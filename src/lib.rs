// TODO - don't make everything public, make it private and expose only what is needed

pub mod files;
pub mod iterators;
pub mod mapping_file;
pub mod output;
pub mod parse_ansi_text;
pub mod parse_file;
pub mod test_utils;
pub mod types;


// use peak_alloc::PeakAlloc;
//
// #[global_allocator]
// static PEAK_ALLOC: PeakAlloc = PeakAlloc;

// https://crates.io/crates/peak_alloc
// let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
// println!("The max amount that was used {}", peak_mem);
