fn main() {
    let mut config = cmake::Config::new(".");
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release");
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=LIEF");
    println!("cargo:rustc-link-lib=static=sui");

    println!("cargo:rustc-link-lib=dylib=stdc++");
}