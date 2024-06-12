use criterion::{BenchmarkId, Criterion};
use nom::IResult;

use ansi_parser_extended::parse_ansi_text::raw_ansi_parse::{
    AnsiSequence,
    parse_escape_complete,
    parse_escape_only_text_and_graphics,
    parse_escape_only_text_and_graphics_manual,
    parse_escape_only_text_and_graphics_manual_simd,
};

pub fn raw_parse_ansi_fn_compare(c: &mut Criterion, path: String) {
    let file_content = std::fs::read(path).expect("Failed to read file");
    let content = file_content.as_slice();

    let mut group = c.benchmark_group("Raw parse ansi compare");


    fn run_parse_fn_res<F>(mut content: &[u8], parse_fn: F) where F: Fn(&[u8], bool) -> IResult<&[u8], AnsiSequence> {
        let mut res = parse_fn(content, true);

        loop {
            if res.is_err() {
                break;
            }

            content = res.unwrap().0;

            res = parse_fn(content, true);
        }
    }
    fn run_parse_fn_option<F>(mut content: &[u8], parse_fn: F) where F: Fn(&[u8], bool) -> Option<(&[u8], AnsiSequence)> {
        let mut res = parse_fn(content, true);

        loop {
            if res.is_none() {
                break;
            }

            content = res.unwrap().0;

            res = parse_fn(content, true);
        }
    }

    group.bench_function(BenchmarkId::new("parse_escape_complete", 0),
                         |b| b.iter(|| run_parse_fn_res(content, parse_escape_complete)));
    group.bench_function(BenchmarkId::new("parse_escape_only_text_and_graphics", 0),
                         |b| b.iter(|| run_parse_fn_res(content, parse_escape_only_text_and_graphics)));
    group.bench_function(BenchmarkId::new("parse_escape_only_text_and_graphics_manual", 0),
                         |b| b.iter(|| run_parse_fn_res(content, parse_escape_only_text_and_graphics_manual)));
    group.bench_function(BenchmarkId::new("parse_escape_only_text_and_graphics_manual_simd", 0),
                         |b| b.iter(|| run_parse_fn_option(content, parse_escape_only_text_and_graphics_manual_simd)));
    group.finish();
}

