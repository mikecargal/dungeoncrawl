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

#[derive(Debug)]
pub enum ThemeChoice {
    Dungeon,
    Forest,
    Random,
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
    NotWideEnough,
    NotTallEnough,
    NotBigEnough,
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
        write!(f, "WorldDimensionParseError: ({})", self.msg)
    }
}

impl FromStr for WorldDimensions {
    type Err = WorldDimensionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const MIN_WIDTH: i32 = 40;
        const MIN_HEIGHT: i32 = 40;
        const MIN_AREA: i32 = 2000;

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

        if world_width < MIN_WIDTH {
            return Err(WorldDimensionParseError::new(
                WorldDimensionParseErrorCodes::NotWideEnough,
                format!(
                    "World must be a minimum of {} tiles wide to be playable.",
                    MIN_WIDTH
                )
                .to_string(),
            ));
        }
        if world_height < MIN_HEIGHT {
            return Err(WorldDimensionParseError::new(
                WorldDimensionParseErrorCodes::NotTallEnough,
                format!(
                    "World must be a minimum of {} tiles high to be playable.",
                    MIN_HEIGHT
                )
                .to_string(),
            ));
        }
        let area = world_height * world_height;
        println!("{}", area);
        if area < MIN_AREA {
            return Err(WorldDimensionParseError::new(
                WorldDimensionParseErrorCodes::NotBigEnough,
                format!(
                    "World must have area (w*h) greater than {} to be playable. ({} is {})",
                    MIN_AREA, s, area
                )
                .to_string(),
            ));
        }
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
    let res = "40x50".parse::<WorldDimensions>();
    assert_eq!(
        Ok(WorldDimensions {
            world_width: 40,
            world_height: 50,
            display_width: 20,
            display_height: 25
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

#[test]
fn test_w_d_too_narrow() {
    let res = "39x80".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::NotWideEnough);
    }
}

#[test]
fn test_w_d_too_short() {
    let res = "80x39".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::NotTallEnough);
    }
}

#[test]
fn test_w_d_too_small() {
    let res = "40x40".parse::<WorldDimensions>();
    assert!(res.is_err());
    if let Err(WorldDimensionParseError { code, .. }) = res {
        assert_eq!(code, WorldDimensionParseErrorCodes::NotBigEnough);
    }
}

#[derive(Debug)]
pub struct Config {
    pub architect: ArchitectChoice,
    pub theme: ThemeChoice,
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
        )        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .default_value("Random")
                .possible_values(&["Dungeon", "Forest", "Random"])
                .value_name("theme"),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .default_value("80x50")
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
    let th = matches.value_of("theme");
    let theme = match th.unwrap() {
        "Random" => ThemeChoice::Random,
        "Dungeon" => ThemeChoice::Dungeon,
        "Forest" => ThemeChoice::Forest,
        val => panic!(format!("{:?} is not a valid Theme choice.", val)),
    };
    let world_dimensions: WorldDimensions = matches
        .value_of("size")
        .unwrap()
        .parse::<WorldDimensions>()
        .unwrap();
    let config = Config {
        architect,
        world_dimensions,
        theme,
    };
    println!("Config = {:?}", config);
    config
}
