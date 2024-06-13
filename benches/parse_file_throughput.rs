use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};
use nom::combinator::complete;
use nom::IResult;

use ansi_parser_extended::files::file_reader::FileReaderOptions;
use ansi_parser_extended::parse_ansi_text::parse_options::ParseOptions;
use ansi_parser_extended::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines;
use ansi_parser_extended::parse_file::file_to_spans::read_ansi_file_to_spans;
use ansi_parser_extended::parse_file::text_to_lines_of_spans::buffer_to_lines;
use ansi_parser_extended::parse_file::text_to_spans::buffer_to_spans;
use ansi_parser_extended::parse_file::types::ReadAnsiFileOptions;

fn parse_ansi_file_spans_throughput(c: &mut Criterion, path: String) {
    let mut group = c.benchmark_group("Parse ansi file spans throughput");

    let file_size = std::fs::metadata(path.to_string()).expect("should open").len();

    group.throughput(Throughput::Bytes(file_size));
    group.bench_function("Spans", |b| b.iter(||
        read_ansi_file_to_spans(create_options_for_parse(path.to_string())).for_each(|_| {
            // Noop
        })
    ));
    group.finish();
}

fn parse_ansi_file_lines_throughput(c: &mut Criterion, path: String) {
    let mut group = c.benchmark_group("Parse ansi file lines throughput");

    let file_size = std::fs::metadata(path.to_string()).expect("should open").len();
    group.throughput(Throughput::Bytes(file_size));
    group.bench_function("Lines", |b| b.iter(||
        read_ansi_file_to_lines(create_options_for_parse(path.to_string())).for_each(|_| {
            // Noop
        })
    ));
    group.finish();
}


fn parse_ansi_text_spans_throughput(c: &mut Criterion, path: String) {
    let mut group = c.benchmark_group("Parse ansi text spans throughput");

    let file_content = std::fs::read(path).expect("Failed to read file");
    let content = file_content.as_slice();

    group.throughput(Throughput::Bytes(file_content.len() as u64));
    group.bench_function("Spans", |b| b.iter(||
        buffer_to_spans(content).for_each(|_| {
            // Noop
        })
    ));
    group.finish();
}

fn parse_ansi_text_lines_throughput(c: &mut Criterion, path: String) {
    let mut group = c.benchmark_group("Parse ansi text lines throughput");

    let file_content = std::fs::read(path).expect("Failed to read file");
    let content = file_content.as_slice();

    group.throughput(Throughput::Bytes(file_content.len() as u64));
    group.bench_function("Lines", |b| b.iter(||
        buffer_to_lines(content).for_each(|_| {
            // Noop
        })
    ));
    group.finish();
}


pub fn run_parse_file_throughput(c: &mut Criterion, path: String) {
    parse_ansi_file_spans_throughput(c, path.to_string());
    parse_ansi_file_lines_throughput(c, path.to_string());

    parse_ansi_text_spans_throughput(c, path.to_string());
    parse_ansi_text_lines_throughput(c, path.to_string());
}


fn create_options_for_parse(path: String) -> ReadAnsiFileOptions {
    let file_reader_options = FileReaderOptions {
        file_path: path,
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        from_bytes: None,
        to_bytes: None,
    };
    let parse_options = ParseOptions::default();

    return ReadAnsiFileOptions {
        file_options: file_reader_options,
        parse_options,
    };
}
