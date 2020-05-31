//use cc;

fn main() {
    //println!("rustc-flags=-ltll");
    //println!("cargo:rustc-link-lib=dylib=tll");
    println!("cargo:rustc-link-lib=tll");
    println!("cargo:rustc-link-search=native=/home/psha/src/tll/build/lib");
    //println!("cargo:rerun-if-changed=src/channel/inline.c");
    //println!("cargo:rustc-link-search=native=/home/psha/src/tll-export/build/lib");
    //cc::Build::new().flag("-I/home/psha/src/tll/src").file("src/channel/inline.c").compile("tll-channel-static")
}
