#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _: Result<stylist_core::ast::Sheet, _> = data.parse();
});
