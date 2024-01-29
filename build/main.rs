mod consts;
mod helpers;

extern crate bindgen;

use crate::consts::LINUX_FEATURE_VERSIONS;
use crate::helpers::{bindgen, parse_linux_version_h};
use std::env;
use std::path::Path;

fn main() {
    // Check target OS
    match env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => match target_os.as_str() {
            "linux" | "android" => {}
            target_os => panic!("Invalid target OS: {:?}", target_os),
        },
        Err(e) => {
            panic!("Unknown target OS: {}", e);
        }
    };

    let linux_headers_path = if let Ok(path) = env::var("LINUX_HEADERS_PATH") {
        let path = format!("{}/include", path);
        let path = Path::new(&path).canonicalize().unwrap();
        path.to_str().unwrap().to_string()
    } else {
        // TODO: get the right location of libc in the building system.
        // as different linux distros have different locations of libc header files.
        // on Ubuntu or Fedora, the default location is `/usr/include`
        // while on other distros like nix, they may have different locations.
        "/usr/include".to_string()
    };

    let linux_version_h_path = format!("{}/{}", linux_headers_path, "linux/version.h");
    let (major, patch_level, sub_level) = parse_linux_version_h(&linux_version_h_path);

    let selected_linux_feature_versions: Vec<(usize, usize)> = LINUX_FEATURE_VERSIONS
        .into_iter()
        .filter(|(.., is_selected)| *is_selected)
        .map(|(m, p, _)| (m, p))
        .collect();

    let enabled_linux_features = if selected_linux_feature_versions.is_empty() {
        // Apply default features based on parsed linux version
        LINUX_FEATURE_VERSIONS
            .into_iter()
            .filter_map(|(m, p, _)| {
                if (major == m && patch_level >= p) || major > m {
                    println!("cargo:rustc-cfg=feature=\"linux-{}.{}\"", m, p);
                    Some((m, p))
                } else {
                    None
                }
            })
            .collect()
    } else {
        // Features are manually selected by the user
        // show warning if major or patch_level dose not match
        let (selected_major, selected_patch_level) = selected_linux_feature_versions[0];
        if selected_major != major || selected_patch_level != patch_level {
            let selected_linux_feature =
                format!("linux-{}.{}", selected_major, selected_patch_level,);
            println!(
                "cargo:warning=\
                Selected feature `{}` may not compatible with compile against Linux version `{}.{}.{}`",
                selected_linux_feature,
                major,
                patch_level,
                sub_level,
            );
            println!(
                "cargo:warning=\
                To set another Linux headers path, run `LINUX_HEADERS_PATH=/path/to/directory cargo build --features {}`",
                selected_linux_feature
            );
        }
        selected_linux_feature_versions
    };

    bindgen(&linux_headers_path, &enabled_linux_features)
}