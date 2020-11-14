use env_logger;

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    let _ = env_logger::builder().is_test(true).try_init();
}
