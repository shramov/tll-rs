fn main() {
    //println!("rustc-flags=-ltll");
    //println!("cargo:rustc-link-lib=dylib=tll");
    println!("cargo:rustc-link-lib=tll");
    println!("cargo:rustc-link-search=native=/home/psha/src/tll/build/lib");
}
