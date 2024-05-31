pub enum OutputDestination {
    Stdout,

    // Does not output anything
    Sink,
}

pub fn get_output_destination(dest: OutputDestination) -> Box<dyn Fn(String)> {
    match dest {
        OutputDestination::Stdout => Box::new(to_stdout),
        OutputDestination::Sink => Box::new(to_sink),
    }
}

fn to_stdout(output: String) {
    print!("{}", output);
}

fn to_sink(_output: String) {
    // Do nothing
}

pub fn get_output_destination_from_string(output_dest: &str) -> Option<OutputDestination> {
    if output_dest == "stdout" {
        return Some(OutputDestination::Stdout);
    }

    if output_dest == "sink" {
        return Some(OutputDestination::Sink);
    }

    return None;
}
