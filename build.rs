extern crate cmake;

fn main() {
    let dst = cmake::build("binaryen");

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=binaryen");
}
