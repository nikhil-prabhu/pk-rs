fn main() {
    let mut c_build = cc::Build::new();

    // Probe required system libraries with pkg-config.
    let sys_deps = system_deps::Config::new().probe().unwrap();

    // Add -I, -L and -l flags using probed values.
    c_build.includes(&sys_deps.all_include_paths());

    for path in &sys_deps.all_link_paths() {
        c_build.flag(&format!("-L{:?}", path));
    }

    for path in &sys_deps.all_libs() {
        c_build.flag(&format!("-l{}", path));
    }

    // Compile everything.
    c_build
        .file("src/c/pk_rs.c")
        .shared_flag(true)
        .flag("-pthread")
        .compile("pk_rs");

    println!("cargo:rerun-if-changed=src/c/pk_rs.c");
}
