
use clap::{Arg, ArgAction, Command, ValueHint};
use clap::builder::PossibleValue;

pub fn get_cli() -> Command {

    let parse_command = Command::new("parse")
        .about("Parse ANSI text")
        .arg(Arg::new("file")
            .short('f')
            .long("file")

            .short_alias('i')
            .alias("input")

            .required(true)
            .value_hint(ValueHint::FilePath)
            .help("file to read"))

        .arg(Arg::new("split-lines")
            .short('s')
            .long("split-lines")
            .required(false)
            .help("Whether should have no span that contain multiple lines")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("output")
            .long("output")
            .required(false)
            .default_value("stdout")
            .help("Where to output the result")
            .value_parser([
                PossibleValue::new("stdout").help("output to stdout"),
                PossibleValue::new("sink").help("do not output - useful for benchmarking without writing stdout part"),
            ]))

        .arg(Arg::new("format")
            .long("format")
            .required(false)
            .default_value("json")
            .help("How the output will be formatted")
            .value_parser([
                PossibleValue::new("json").help("output all the span in a valid JSON format this is the default format"),
                PossibleValue::new("json-line").help([
                    "Each line of output is a valid JSON, there are no commas between lines and the output is not wrapped with [ and ].",
                    "When split-lines is true, each line of output will match line in the input, all spans for the same input line will be at the same line in the output",
                ].join("\n")),

                // TODO - this should not be possible when split-lines is false
                PossibleValue::new("flat-json-line").help([
                    "Each line of output is a valid JSON, there are no commas between lines and the output is not wrapped with [ and ].",
                    "Object { \"type\": \"new line\" } will be printed between lines to mark new line",
                    "Only available when 'split-lines' is enabled",
                ].join("\n")),
            ]))
        
        // TODO - add support for reading from line to line and not the entire file + use mapping file
        .arg(Arg::new("from-line")
            .long("from-line")
            .required(false)
            .help("From which line to read (included)")

            .allow_negative_numbers(false)
            .value_parser(clap::value_parser!(usize))
        )
        
        .arg(Arg::new("to-line")
            .long("to-line")
            .required(false)
            .allow_negative_numbers(false) 
            // Must be greater than from-line
            .value_parser(clap::value_parser!(usize))
            .help("until which line to read (excluded)"))

        .arg(Arg::new("mapping-file")
            .long("mapping-file")
            .required(false)
            .value_hint(ValueHint::FilePath)
            .help("mapping file for faster line access, not available if not reading from line to line"));

    let create_mapping_command = Command::new("create")
        .about("Mapping file for easy access")
        .arg(Arg::new("input")

            .short('i')
            .long("input")

            .short_alias('f')
            .alias("file")

            .required(true)
            .value_hint(ValueHint::FilePath)
            .help("file to read"))

        .arg(Arg::new("output")

            .short('o')
            .long("output")

            .required(true)
            .value_hint(ValueHint::FilePath)
            .help("mapping file to output"));
    

    let mapping_command = Command::new("mapping")
        .about("Mapping file for easy access")
        .subcommand(create_mapping_command)
        .subcommand_required(true);

    return Command::new("Ansi Parser CLI")
        .version("1.0.0")
        .author("Raz Luvaton")
        .about("Parse ANSI text")
        .subcommands([parse_command, mapping_command])
        .subcommand_required(true);
}
