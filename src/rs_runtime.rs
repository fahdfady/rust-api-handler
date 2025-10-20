use metacall::initialize;

pub fn rust_runtime(file: &str) {
    let _metacall = initialize().unwrap();

    metacall::load::from_single_file("rust", file).expect("couldn't load rs file");
}
