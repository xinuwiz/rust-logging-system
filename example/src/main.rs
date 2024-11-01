use rust_logging_system::logger;

fn main() -> () {
    let stateful = logger::Stateful::new();
    rust_logging_system::info!(stateful, String::from("Hello World!"));

    rust_logging_system::warn!(stateful, String::from("Hello World!"));

    rust_logging_system::error!(stateful, String::from("Hello World!"));
    stateful.to_file();
}
