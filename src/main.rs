mod draggable;
mod line_plot;
mod scatter_plot;

use clap::{App, Arg};
use iced::{Sandbox, Settings};

use draggable::Draggable;
use line_plot::LinePlot;
use scatter_plot::ScatterPlot;

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
                .possible_values(&["draggable", "lineplot", "scatterplot"])
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("name").unwrap_or("draggable") {
        "draggable" => Draggable::run(Settings::default()),
        "lineplot" => LinePlot::run(Settings::default()),
        "scatterplot" => ScatterPlot::run(Settings::default()),
        _ => unreachable!(),
    }
}
