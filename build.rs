fn main() {
    let mut build = cc::Build::new();

    // Use pkg-config to get information about packagekit-glib2
    if let Ok(package) = pkg_config::Config::new().probe("packagekit-glib2") {
        for include_path in &package.include_paths {
            build.include(include_path);
        }
        for library_path in &package.link_paths {
            println!("cargo:rustc-link-search=native={}", library_path.display());
        }
        for library in &package.libs {
            println!("cargo:rustc-link-lib={}", library);
        }
    } else {
        eprintln!("packagekit-glib2 library not found.");
        std::process::exit(1);
    }

    // Use pkg-config to get information about glib-2.0
    if let Ok(package) = pkg_config::Config::new().probe("glib-2.0") {
        for include_path in &package.include_paths {
            build.include(include_path);
        }
        for library_path in &package.link_paths {
            println!("cargo:rustc-link-search=native={}", library_path.display());
        }
        for library in &package.libs {
            println!("cargo:rustc-link-lib={}", library);
        }
    } else {
        eprintln!("glib-2.0 library not found.");
        std::process::exit(1);
    }

    build
        .file("src/c/pk_rs.c")
        .shared_flag(true)
        .flag("-pthread")
        .compile("pk_rs");
}
