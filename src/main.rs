mod draggable;

use clap::{App, Arg};
use iced::{Sandbox, Settings};

use draggable::Draggable;

fn main() -> iced::Result {
    let matches = App::new("Iced Experiments")
        .version("0.1")
        .about("Run simple iced applications")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Name of the example to run")
                .possible_values(&["draggable"])
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("name").unwrap_or("draggable") {
        "draggable" => Draggable::run(Settings::default()),
        _ => unreachable!(),
    }
}
