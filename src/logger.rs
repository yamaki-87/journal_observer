use flexi_logger::Logger;
use log::Record;

pub fn init() {
    Logger::try_with_env()
        .unwrap()
        .format(custome_format)
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .start()
        .unwrap();
}

fn custome_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}][{}][{}] {}",
        record.level(),
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.target(),
        &record.args()
    )
}
