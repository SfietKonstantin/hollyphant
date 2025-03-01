use cxx::{let_cxx_string, CxxString, UniquePtr};
use hollyphant::{new_database, Hollyphant};
use hollyphant_dispatch::{ErrorFormatter, EventPublisher, HollyphantDispatch};
use reqwest::Client;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[cxx::bridge]
pub mod cxxbridge {
    unsafe extern "C++" {
        include!("rust/rusteventpublisher.h");
        include!("rust/rusterrorformatter.h");

        type RustEventPublisher;
        fn publish_set(self: &RustEventPublisher, key: &CxxString, value: &CxxString);
        fn publish_append(self: &RustEventPublisher, key: &CxxString, value: &CxxString);

        // Error formatting
        fn format_error_unexpected() -> String;
        fn format_error_mas_application_register(instance: &str) -> String;
        fn format_error_database() -> String;
    }

    extern "Rust" {
        type RustEventProcessor;

        fn hollyphant_init();
        fn hollyphant_event_processor_new(database_url: String) -> Box<RustEventProcessor>;
        fn execute(
            self: &RustEventProcessor,
            publisher: UniquePtr<RustEventPublisher>,
            key: &CxxString,
            args: &CxxString,
        );
    }
}

struct RustEventProcessor(HollyphantDispatch<CxxErrorFormatter>);

impl RustEventProcessor {
    pub fn execute(
        &self,
        publisher: UniquePtr<cxxbridge::RustEventPublisher>,
        key: &CxxString,
        args: &CxxString,
    ) {
        let publisher = CxxEventPublisher(publisher);
        self.0.execute(publisher, key.as_bytes(), args.as_bytes());
    }
}

fn hollyphant_init() {
    env_logger::init();
}

fn hollyphant_event_processor_new(database_url: String) -> Box<RustEventProcessor> {
    let db_connection = new_database(&database_url).unwrap();
    let runtime = Runtime::new().unwrap();
    let hollyphant = Hollyphant::new(db_connection, Client::new());
    let hollyphant = Arc::new(Mutex::new(hollyphant));
    Box::new(RustEventProcessor(HollyphantDispatch::new(
        runtime, hollyphant,
    )))
}

struct CxxErrorFormatter;

impl ErrorFormatter for CxxErrorFormatter {
    fn format_error_unexpected() -> String {
        cxxbridge::format_error_unexpected()
    }

    fn format_error_mas_application_register(instance: &str) -> String {
        cxxbridge::format_error_mas_application_register(instance)
    }

    fn format_error_database() -> String {
        cxxbridge::format_error_database()
    }
}

struct CxxEventPublisher(UniquePtr<cxxbridge::RustEventPublisher>);

unsafe impl Send for CxxEventPublisher {}
unsafe impl Sync for CxxEventPublisher {}

impl EventPublisher for CxxEventPublisher {
    fn publish_set(&self, key: &[u8], value: &[u8]) {
        let_cxx_string!(cxx_key = key);
        let_cxx_string!(cxx_value = value);
        self.0.publish_set(&cxx_key, &cxx_value)
    }

    fn publish_append(&self, key: &[u8], value: &[u8]) {
        let_cxx_string!(cxx_key = key);
        let_cxx_string!(cxx_value = value);
        self.0.publish_append(&cxx_key, &cxx_value)
    }
}
