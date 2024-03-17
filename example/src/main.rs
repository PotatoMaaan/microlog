fn main() {
    microlog::init(microlog::LevelFilter::Trace);

    log::trace!("Trace test");
    log::debug!("Debug test");
    log::info!("Info test");
    log::warn!("Info test");
    log::error!("Info test");
}
