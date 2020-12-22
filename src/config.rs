use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};

#[derive(Debug)]
pub enum ArchitectChoice {
    Random,
    Rooms,
    Drunkard,
    CellularAutomata,
}

#[derive(Debug, Clone)]
pub struct ArchitectChoiceParseError {
    msg: String,
}

#[derive(Debug)]
pub struct Config {
    pub architect: ArchitectChoice,
}

pub fn parse_command_line_args() -> Config {
    let matches = App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .author(crate_authors!("\n"))
        .version(format!("v{}", crate_version!()).as_ref())
        .about(crate_description!())
        .arg(
            Arg::with_name("architect")
                // .about("Which architect should be responsible for constructing the dungeon")
                .short("a")
                .long("architect")
                .default_value("Random")
                .possible_values(&["Rooms", "Drunkard", "CellularAutomata", "Random"])
                .value_name("architect"),
        )
        .get_matches();

    let arch = matches.value_of("architect");
    let architect = match arch.unwrap() {
        "Random" => ArchitectChoice::Random,
        "Rooms" => ArchitectChoice::Rooms,
        "Drunkard" => ArchitectChoice::Drunkard,
        "CellularAutomata" => ArchitectChoice::CellularAutomata,
        val => panic!(format!("{:?} is not a valid Architect choice.", val)),
    };
    let c = Config { architect };
    println!("Config = {:?}", c);
    c
}
