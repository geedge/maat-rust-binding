fn main() {
    println!("cargo:rustc-link-search=./lib");
    println!("cargo:rustc-link-lib=dylib=maat4");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./lib")
}
