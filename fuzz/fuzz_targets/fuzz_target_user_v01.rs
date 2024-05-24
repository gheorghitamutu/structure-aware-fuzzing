#![no_main]

use libfuzzer_sys::fuzz_target;

// models::UserV01

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
