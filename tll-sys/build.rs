use pkg_config;

fn main() {
    pkg_config::probe_library("tll").unwrap();
}
