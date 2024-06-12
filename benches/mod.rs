use criterion::{Criterion, criterion_group, criterion_main};

use parse_file_throughput::run_parse_file_throughput;
use crate::parse_raw_ansi::raw_parse_ansi_fn_compare;

pub mod parse_file_throughput;
pub mod parse_raw_ansi;

const file_path: &str = "/Users/rluvaton/dev/personal/ansi-viewer/examples/fixtures/tiny.ans";


fn run_bench(c: &mut Criterion) {
    // run_parse_file_throughput(c, file_path.to_string());
    raw_parse_ansi_fn_compare(c, file_path.to_string());
}


criterion_group!(benches, run_bench);
criterion_main!(benches);
