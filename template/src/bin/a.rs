// use flexi_logger;
// use log::{info, warn};

fn setup_tracing() {
    flexi_logger::Logger::try_with_str("info").unwrap()
    .log_to_file(flexi_logger::FileSpec::default())         // write logs to file
    .duplicate_to_stderr(flexi_logger::Duplicate::Warn)     // print warnings and errors also to the console
    .start().unwrap();
}

fn main() {
    setup_tracing();
    log::info!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    log::info!("File: {:?}", file);
    return 1;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.1"));
    assert_eq!(8, get_answer("test.2"));
}
