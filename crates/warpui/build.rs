// We can use `std::process:Command` here because this is invoked within a build script,
// _not_ within the Warp binary (where it could cause a terminal to temporarily flash on
// Windows).
#![allow(clippy::disallowed_types)]

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use cfg_aliases::cfg_aliases;
use sha2::{Digest, Sha256};

const HEADER_PATH: &str = "src/platform/mac/rendering/metal/shaders/shader_types.h";
const METAL_PATH: &str = "src/platform/mac/rendering/metal/shaders/shaders.metal";
const PRECOMPILED_METAL_DIR: &str = "src/platform/mac/rendering/metal/shaders/precompiled";
const PRECOMPILED_METAL_DIR_ENV: &str = "WARP_PRECOMPILED_METAL_DIR";

fn main() {
    cfg_aliases! {
        macos: { target_os = "macos" },
        // We use winit on all platforms other than mac, where we have a custom
        // AppKit-based platform implementation.
        winit: { not(macos) },
        // We use wgpu for rendering on all platforms where we use winit, but
        // we can also use it on macOS, if enabled.
        wgpu: { any(winit, feature = "experimental-wgpu-renderer") },
        native: { not(target_family = "wasm") },
    }

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        bindgen_shader_types();
        compile_metal_shaders();
        compile_objc_lib();
    }
}

fn bindgen_shader_types() {
    println!("cargo:rerun-if-changed={HEADER_PATH}");
    let bindings = bindgen::Builder::default()
        .header(HEADER_PATH)
        .allowlist_type("vector_float2")
        .allowlist_type("Uniforms")
        .allowlist_type("PerRectUniforms")
        .allowlist_type("PerGlyphUniforms")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Disable intrinsic headers that define types containing 16-bit floats (`_Float16`,
        // `__m512h`, etc.) via preprocessor directive. `bindgen` doesn't know how to process
        // these types and panics at compile time. The types aren't used by our shader headers,
        // so suppressing the declarations is safe.
        //
        // - Xcode 15+: avx512fp16intrin.h, avx512vlfp16intrin.h
        // - Xcode 26+ (clang 21): amxavx512intrin.h, avx10_2convertintrin.h,
        //   avx10_2_512convertintrin.h
        //
        // TODO(charlespierce): Remove once https://github.com/rust-lang/rust-bindgen/issues/2500
        // is resolved.
        .clang_args([
            "-D__AVX512VLFP16INTRIN_H",
            "-D__AVX512FP16INTRIN_H",
            "-D__AMX_AVX512INTRIN_H",
            "-D__AVX10_2CONVERTINTRIN_H",
            "-D__AVX10_2_512CONVERTINTRIN_H",
            "-D__AVX10_2_512MINMAXINTRIN_H",
            "-D__AVX10_2_512NIINTRIN_H",
            "-D__AVX10_2_512SATCVTINTRIN_H",
        ])
        .generate()
        .unwrap_or_else(|_| panic!("unable to generate bindings for {HEADER_PATH}"));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("shader_types.rs"))
        .expect("Couldn't write shader type bindings!");
}

fn compile_metal_shaders() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let air_path = out_path.join("shaders.air");
    let air_path = air_path.to_str().unwrap();

    let lib_path = out_path.join("shaders.metallib");
    let lib_path = lib_path.to_str().unwrap();

    println!("cargo:rerun-if-changed={HEADER_PATH}");
    println!("cargo:rerun-if-changed={METAL_PATH}");
    println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo:rerun-if-env-changed={PRECOMPILED_METAL_DIR_ENV}");

    // Pin the AIR bytecode target to `MACOSX_DEPLOYMENT_TARGET`. Without an
    // explicit `-mmacosx-version-min`, `xcrun metal` on recent Xcode toolchains
    // emits AIR for the current SDK (e.g. `air64_v27-apple-macosx15.0.0`)
    // regardless of the env var, and older Metal driver stacks reject the
    // resulting `.metallib` during pipeline state creation. See #11700.
    let min_macos_version = env::var("MACOSX_DEPLOYMENT_TARGET")
        .expect("MACOSX_DEPLOYMENT_TARGET must be set for macOS builds");

    let explicit_precompiled_dir = env::var_os(PRECOMPILED_METAL_DIR_ENV).map(PathBuf::from);
    if let Some(precompiled_dir) = explicit_precompiled_dir
        .or_else(|| (!metal_tools_available()).then(|| PathBuf::from(PRECOMPILED_METAL_DIR)))
    {
        install_precompiled_metal_library(
            &precompiled_dir,
            Path::new(lib_path),
            &min_macos_version,
        );
        return;
    }

    let min_version_arg = format!("-mmacosx-version-min={min_macos_version}");

    let mut compile_args = vec![
        "-sdk",
        "macosx",
        "metal",
        "-c",
        METAL_PATH,
        "-o",
        air_path,
        &min_version_arg,
    ];
    if cfg!(feature = "enable-metal-frame-capture") {
        compile_args.push("-frecord-sources");
        compile_args.push("-gline-tables-only");
    }
    let result = Command::new("xcrun")
        .args(&compile_args)
        .output()
        .expect("error compiling metal shaders to .air");
    assert!(
        result.status.success(),
        "error compiling metal shaders to .air; {}",
        std::str::from_utf8(&result.stderr).unwrap(),
    );

    let result = Command::new("xcrun")
        .args(["-sdk", "macosx", "metallib", air_path, "-o", lib_path])
        .output()
        .expect("error compiling metal shaders to .metallib");
    assert!(
        result.status.success(),
        "error compling metal shaders to .metallib; {}",
        std::str::from_utf8(&result.stderr).unwrap(),
    );
}

fn metal_tools_available() -> bool {
    ["metal", "metallib"].iter().all(|tool| {
        Command::new("xcrun")
            .args(["--sdk", "macosx", "--find", tool])
            .output()
            .is_ok_and(|output| output.status.success())
    })
}

fn install_precompiled_metal_library(
    precompiled_dir: &Path,
    destination: &Path,
    min_macos_version: &str,
) {
    let metallib_path = precompiled_dir.join("shaders.metallib");
    let manifest_path = precompiled_dir.join("shaders.metallib.manifest");

    println!("cargo:rerun-if-changed={}", metallib_path.display());
    println!("cargo:rerun-if-changed={}", manifest_path.display());

    let manifest = fs::read_to_string(&manifest_path).unwrap_or_else(|error| {
        panic!(
            "Metal toolchain is unavailable and the precompiled shader manifest could not be read at {}: {error}. Run the precompile-metal-shaders GitHub Actions workflow and install its artifact, or set {PRECOMPILED_METAL_DIR_ENV}",
            manifest_path.display(),
        )
    });

    verify_manifest_value(
        &manifest,
        "shader_types_sha256",
        &sha256_file(Path::new(HEADER_PATH)),
    );
    verify_manifest_value(
        &manifest,
        "shaders_metal_sha256",
        &sha256_file(Path::new(METAL_PATH)),
    );
    verify_manifest_value(&manifest, "metallib_sha256", &sha256_file(&metallib_path));
    verify_manifest_value(&manifest, "macosx_deployment_target", min_macos_version);

    fs::copy(&metallib_path, destination).unwrap_or_else(|error| {
        panic!(
            "unable to copy precompiled Metal library from {} to {}: {error}",
            metallib_path.display(),
            destination.display(),
        )
    });

    println!(
        "cargo:warning=using precompiled Metal shaders from {}",
        precompiled_dir.display(),
    );
}

fn verify_manifest_value(manifest: &str, key: &str, expected: &str) {
    let actual = manifest.lines().find_map(|line| {
        let (manifest_key, value) = line.split_once('=')?;
        (manifest_key == key).then_some(value)
    });

    assert_eq!(
        actual,
        Some(expected),
        "precompiled Metal shader manifest has an invalid {key}; regenerate it with the precompile-metal-shaders GitHub Actions workflow",
    );
}

fn sha256_file(path: &Path) -> String {
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("unable to read {} for SHA-256: {error}", path.display()));
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut output, byte| {
            write!(output, "{byte:02x}").unwrap();
            output
        })
}

fn compile_objc_lib() {
    println!("cargo:rustc-link-lib=framework=UserNotifications");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=ServiceManagement");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/app.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/app.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/keycode.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/host_view.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/host_view.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/hotkey.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/hotkey.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/menus.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/menus.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/notifications/notifications.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/notifications/notifications.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/window.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/window_blur.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/window_blur.h");
    // Referenced from https://github.com/tonymillion/Reachability
    println!("cargo:rerun-if-changed=src/platform/mac/objc/reachability.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/reachability.m");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/alert.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/fullscreen_queue.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/fullscreen_queue.m");

    // Link against the clang_rt library so that the @available keyword
    // doesn't produce linker errors.
    //
    // See: https://github.com/alexcrichton/curl-rust/issues/279
    if let Some(path) = macos_link_search_path() {
        println!("cargo:rustc-link-lib=clang_rt.osx");
        println!("cargo:rustc-link-search={path}");
    }

    cc::Build::new()
        .file("src/platform/mac/objc/app.m")
        .file("src/platform/mac/objc/host_view.m")
        .file("src/platform/mac/objc/hotkey.m")
        .file("src/platform/mac/objc/reachability.m")
        .file("src/platform/mac/objc/keycode.m")
        .file("src/platform/mac/objc/menus.m")
        .file("src/platform/mac/objc/notifications/notifications.m")
        .file("src/platform/mac/objc/window.m")
        .file("src/platform/mac/objc/fullscreen_queue.m")
        .file("src/platform/mac/objc/window_blur.m")
        .compile("warp_objc");
}

/// Determine the path containing the macOS standard libraries by querying
/// clang's library search paths.
fn macos_link_search_path() -> Option<String> {
    let output = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            return Some(format!("{path}/lib/darwin"));
        }
    }

    println!("failed to determine link search path, continuing without it");
    None
}
