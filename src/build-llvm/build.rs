// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::boxed::Box;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use cc::Build;
use cmake::Config;

mod external;
use external::llvm_sys;

extern crate cc;
#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=config.cmake");
  println!("cargo:rerun-if-changed=CMakeLists.txt");

  let install_dir = get_llvm_install_dir();
  println!("cargo:rerun-if-changed={:?}", install_dir);

  // llvm-sys components
  println!("cargo:rerun-if-changed=external.rs");

  // Download vars passed to cmake
  println!("cargo:rerun-if-env-changed=RSQL_DOWNLOAD_LLVM");
  println!("cargo:rerun-if-env-changed=RSQL_LLVM_BUILDS_URL");
  println!("cargo:rerun-if-env-changed=RSQL_LLVM_PKG_NAME");

  // Package vars used only in here
  println!("cargo:rerun-if-env-changed=RSQL_PKG_DEST");

  // Build vars passed to cmake
  println!("cargo:rerun-if-env-changed=RSQL_LLVM_TAG");

  // maps to CPACK_PACKAGE_FILE_NAME
  println!("cargo:rerun-if-env-changed=RSQL_PACKAGE_FILE_NAME");

  // maps to CMAKE_INSTALL_PREFIX passed to cmake in build and download
  println!("cargo:rerun-if-env-changed=RSQL_CACHE_DIR");

  if cfg!(feature = "download-llvm") {
    println!("Downloading llvm");
    download_llvm()?;
  } else if cfg!(feature = "build-llvm") {
    println!("Building llvm");
    compile_llvm()?;
  }
  if cfg!(feature = "rsql-llvm-linking") {
    println!("Linking llvm");
    link_llvm();
    let build_dir = get_build_dir()?;
    compile_target_wrappers(&build_dir)?;
  } else if cfg!(feature = "external-llvm-linking") {
    println!("LLVM_SYS_{{}}_PREFIX will provide the LLVM linking");
  } else {
    println!("No LLVM linking");
  }

  Ok(())
}

fn download_llvm() -> Result<(), Box<dyn Error>> {
  // If the download url isn't set, we need to immediately fail.
  let url = env::var("RSQL_LLVM_BUILDS_URL")?;

  let enable_download = env::var("RSQL_DOWNLOAD_LLVM").unwrap_or_else(|_| "true".to_owned());

  let build_dir = get_build_dir()?;

  let mut config = Config::new(build_dir);
  config
    .generator("Ninja")
    .no_build_target(true)
    .env("RSQL_LLVM_PKG_NAME", get_package_file_name()?)
    .env("RSQL_LLVM_BUILDS_URL", url)
    .env("RSQL_DOWNLOAD_LLVM", enable_download)
    .define("CPACK_PACKAGE_FILE_NAME", get_package_name()?)
    .define("CMAKE_INSTALL_PREFIX", get_llvm_install_dir())
    .very_verbose(true);
  let _ = config.build();

  Ok(())
}

fn get_llvm_compile_target() -> String {
  // We always install unless package is chosen.
  // The user's choices for CMAKE_INSTALL_PREFIX will choose whether
  // the installation goes into the target folder for linking or
  // into another dir for potential reuse
  if cfg!(feature = "package-llvm") {
    "llvm-prefix/src/llvm-stamp/llvm-package".to_owned()
  } else {
    "llvm-prefix/src/llvm-stamp/llvm-install".to_owned()
  }
}

fn compile_llvm() -> Result<(), Box<dyn Error>> {
  let build_dir = get_build_dir()?;
  let mut config = Config::new(build_dir);

  config
    .generator("Ninja")
    .build_target(get_llvm_compile_target().as_str())
    .env("RSQL_LLVM_TAG", get_llvm_tag())
    .define("CPACK_PACKAGE_FILE_NAME", get_package_name()?)
    .define("CMAKE_INSTALL_PREFIX", get_llvm_install_dir());
  let _ = config.build();

  if cfg!(feature = "package-llvm") {
    package_llvm()?;
  }
  Ok(())
}

fn package_llvm() -> Result<(), Box<dyn Error>> {
  let out_dir = env::var("OUT_DIR").expect("Could not get OUT_DIR environment variable");
  let output = PathBuf::from(out_dir)
    .join("build")
    .join("llvm-prefix")
    .join("src")
    .join("llvm-build")
    .join(get_package_file_name()?);

  if let Ok(dest_dir) = env::var("RSQL_PKG_DEST") {
    let dest = PathBuf::from(dest_dir).join(get_package_file_name()?);
    println!(
      "Moving {} to {}.",
      output.as_path().display(),
      dest.as_path().display()
    );
    fs::rename(output, dest)?;
  } else {
    println!("Not moving package output. RSQL_PKG_DEST not set.");
  }

  Ok(())
}

fn get_build_dir() -> Result<PathBuf, Box<dyn Error>> {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
  let build_dir = PathBuf::from(manifest_dir.as_str());
  let normalized_build_dir = fs::canonicalize(build_dir)?;
  println!(
    "llvm build files dir: {}",
    normalized_build_dir.to_str().unwrap()
  );
  Ok(normalized_build_dir)
}

fn link_llvm() {
  let libdir = llvm_sys::llvm_config("--libdir");

  // Export information to other crates
  println!(
    "cargo:config_path={}",
    llvm_sys::LLVM_CONFIG_PATH.clone().unwrap().display()
  ); // will be DEP_RSQL_CONFIG_PATH
  println!("cargo:libdir={}", libdir); // DEP_KM_LIBDIR

  // Link LLVM libraries
  println!("cargo:rustc-link-search=native={}", libdir);
  for name in llvm_sys::get_link_libraries() {
    println!("cargo:rustc-link-lib=static={}", name);
  }

  // Link system libraries
  for name in llvm_sys::get_system_libraries() {
    println!("cargo:rustc-link-lib=dylib={}", name);
  }
}

fn compile_target_wrappers(build_dir: &Path) -> Result<(), Box<dyn Error>> {
  let target_c = build_dir.join("target.c").canonicalize()?;
  env::set_var("CFLAGS", llvm_sys::get_llvm_cflags());
  Build::new().file(target_c).compile("targetwrappers");
  Ok(())
}

fn get_package_file_name() -> Result<String, Box<dyn Error>> {
  let mut base_name = get_package_name()?;

  if llvm_sys::target_os_is("windows") {
    base_name.push_str(".zip");
  } else {
    base_name.push_str(".tar.gz");
  }

  Ok(base_name)
}

fn get_llvm_tag() -> String {
  if let Ok(tag) = env::var("RSQL_LLVM_TAG") {
    tag
  } else if cfg!(feature = "llvm11-0") {
    "llvmorg-11.1.0".to_owned() // 1fdec59bf
  } else if cfg!(feature = "llvm12-0") {
    "llvmorg-12.0.1".to_owned() // fed4134
  } else if cfg!(feature = "llvm13-0") {
    "llvmorg-13.0.1".to_owned() // 75e33f7
  } else if cfg!(feature = "llvm14-0") {
    "llvmorg-14.0.6".to_owned() // 28c006
  } else if cfg!(feature = "llvm15-0") {
    "llvmorg-15.0.7".to_owned()
  } else {
    panic!("Unsupported LLVM version. The LLVM feature flags or RSQL_LLVM_TAG must be set.")
  }
}

fn get_package_name() -> Result<String, Box<dyn Error>> {
  if let Ok(file_name) = env::var("RSQL_PACKAGE_FILE_NAME") {
    Ok(file_name)
  } else {
    let tag = get_llvm_tag();
    let triple = get_target_triple()?;
    let package_name = format!("rqsl-llvm-{}-{}", triple, tag);
    Ok(package_name)
  }
}

fn get_target_triple() -> Result<String, Box<dyn Error>> {
  let target = if llvm_sys::target_os_is("windows") {
    // TODO: remove static linking and just return the TARGET
    "x86_64-pc-windows-msvc-static".to_owned()
  } else {
    env::var("TARGET")?
  };
  Ok(target)
}

fn get_llvm_install_dir() -> PathBuf {
  if let Ok(path) = env::var("RSQL_CACHE_DIR") {
    PathBuf::from(path)
  } else {
    // if we install to OUT_DIR the llvm install task during the extraction
    // of the archive will empty the target directory breaking the build.
    // To avoid that, we put llvm binaries into the OUT_DIR/llvm folder.
    let out_dir = env::var("OUT_DIR").expect("Could not get OUT_DIR environment variable");
    PathBuf::from(out_dir).join("llvm")
  }
}

fn locate_llvm_config() -> Option<PathBuf> {
  let major = if cfg!(feature = "llvm11-0") {
    "11"
  } else if cfg!(feature = "llvm12-0") {
    "12"
  } else if cfg!(feature = "llvm13-0") {
    "13"
  } else if cfg!(feature = "llvm14-0") {
    "14"
  } else if cfg!(feature = "llvm15-0") {
    "15"
  } else {
    "unknown"
  };
  if let Ok(path) = env::var(format!("DEP_LLVM_{major}_CONFIG_PATH")) {
    Some(PathBuf::from(path))
  } else {
    let dir = get_llvm_install_dir();
    println!("Looking in {:?}", dir);
    let prefix = dir.join("bin");
    let binary_name = llvm_config_name();
    let binary_path = prefix.join(binary_name);
    if binary_path.as_path().exists() {
      Some(binary_path)
    } else {
      None
    }
  }
}

pub fn llvm_config_name() -> String {
  let mut base_name = "llvm-config".to_owned();

  // On Windows, also search for llvm-config.exe
  if llvm_sys::target_os_is("windows") {
    base_name.push_str(".exe");
  }

  base_name
}
