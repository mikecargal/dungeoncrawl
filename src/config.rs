use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use std::{fmt, str::FromStr};

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

#[derive(Debug, Clone, PartialEq)]
pub struct WorldDimensions {
    pub world_width: i32,
    pub world_height: i32,
    pub display_width: i32,
    pub display_height: i32,
}

impl WorldDimensions {
    pub fn num_tiles(&self) -> usize {
        (self.world_width * self.world_width) as usize
    }
}

#[derive(Debug, PartialEq)]
enum WorldDimensionParseErrorCodes {
    BadFormat,
    InvalidWidth,
    InvalidHeight,
}
#[derive(Debug, PartialEq)]
pub struct WorldDimensionParseError {
    code: WorldDimensionParseErrorCodes,
    msg: String,
}

impl WorldDimensionParseError {
    fn new(code: WorldDimensionParseErrorCodes, msg: String) -> Self {
        Self { code, msg }
    }
}

impl fmt::Display for WorldDimensionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorldDimensionParseError: ({})", self.msg) // TODO: actual error details
    }
}

impl FromStr for WorldDimensions {
    type Err = WorldDimensionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wh: Vec<&str> = s.split('x').collect();
        if wh.len() != 2 {
            return Err(WorldDimensionParseError::new(WorldDimensionParseErrorCodes::BadFormat, "World Dimensions should we expressed as WxH where W and H are the width and height.".to_string() ));
        }
        let world_width_r = wh[0].parse::<i32>();
        if world_width_r.is_err() {
            return Err(WorldDimensionParseError::new(
                WorldDimensionParseErrorCodes::InvalidWidth,
                "World Dimensions width is invalid".to_string(),
            ));
        }
        let world_width = world_width_r.unwrap();
        let world_height_r = wh[1].parse::<i32>();
        if world_height_r.is_err() {
            return Err(WorldDimensionParseError::new(
                WorldDimensionParseErrorCodes::InvalidHeight,
                "World Dimensions height is invalid".to_string(),
            ));
        }
        let world_height = world_height_r.unwrap();
        Ok(WorldDimensions {
            world_width,
            world_height,
            display_width: world_width / 2,
            display_height: world_height / 2,
        })
    }
}

#[test]
fn test_w_d_from_str_valid() {
    let res = "40x40".parse::<WorldDimensions>();
    assert_eq!(
        Ok(WorldDimensions {
            world_width: 40,
            world_height: 40,
            display_width: 20,
            display_height: 20
        }),
        res
    );
}

#[test]
fn test_w_d_from_str_no_x() {
    let res = "40.40".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::BadFormat);
    }
}
#[test]
fn test_w_d_from_str_bad_w() {
    let res = "mx40".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::InvalidWidth);
    }
}
#[test]
fn test_w_d_from_str_bad_h() {
    let res = "40xm".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::InvalidHeight);
    }
}

#[derive(Debug)]
pub struct Config {
    pub architect: ArchitectChoice,
    pub world_dimensions: WorldDimensions,
}

pub fn parse_command_line_args() -> Config {
    let matches = App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .author(crate_authors!("\n"))
        .version(format!("v{}", crate_version!()).as_ref())
        .about(crate_description!())
        .arg(
            Arg::with_name("architect")
                .short("a")
                .long("architect")
                .default_value("Random")
                .possible_values(&["Rooms", "Drunkard", "CellularAutomata", "Random"])
                .value_name("architect"),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .default_value("50x80")
                .help("size of world expressed as WxH (example: 80x50 is 80 tiles wide by 80 tiles wide")
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
    let world_dimensions: WorldDimensions = matches
        .value_of("size")
        .unwrap()
        .parse::<WorldDimensions>()
        .unwrap();
    let config = Config {
        architect,
        world_dimensions,
    };
    println!("Config = {:?}", config);
    config
}
