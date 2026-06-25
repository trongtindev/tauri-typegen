fn main() {
    if std::env::var("CARGO_FEATURE_BUILD_TIME_GENERATION").is_ok() {
        run_build_time_generation();
    }
}

fn run_build_time_generation() {
    println!("cargo:rerun-if-changed=src-tauri");
    println!("cargo:rerun-if-changed=tauri.conf.json");
}
