use flexi_logger::Logger;

pub fn init() {
    Logger::try_with_env()
        .unwrap()
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .start()
        .unwrap();
}
