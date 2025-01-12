use cxx::{CxxString, UniquePtr};
use crate::cxxbridge::RustEventPublisher;

#[cxx::bridge]
pub mod cxxbridge {
    unsafe extern "C++" {
        include!("rust/rusteventpublisher.h");
        type RustEventPublisher;
        fn publish_set(self: &RustEventPublisher, key: &CxxString, value: &CxxString);
    }

    extern "Rust" {
        type RustEventProcessor;

        fn event_processor_new() -> Box<RustEventProcessor>;
        fn execute(self: &RustEventProcessor, publisher: UniquePtr<RustEventPublisher>, key: &CxxString, value: &CxxString);
    }
}

struct RustEventProcessor {}

impl RustEventProcessor {
    pub fn execute(&self, _publisher: UniquePtr<RustEventPublisher>, _key: &CxxString, _value: &CxxString) {

    }
}

fn event_processor_new() -> Box<RustEventProcessor> {
    Box::new(RustEventProcessor {})
}
