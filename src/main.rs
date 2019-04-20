extern crate structopt;
extern crate ron;
#[macro_use]
extern crate serde_derive;
use structopt::StructOpt;
use ron::ser::{to_string_pretty as to_ron_string, PrettyConfig};

use png::{Decoder, OutputInfo};
use std::path::PathBuf;
use std::fs::File;
use std::fmt::{Display, Formatter, Error as FmtError};

/// Structure acting as scaffolding for serde when loading a spritesheet file.
/// Positions originate in the top-left corner (bitmap image convention).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SpritePosition {
    /// Horizontal position of the sprite in the sprite sheet
    pub x: u32,
    /// Vertical position of the sprite in the sprite sheet
    pub y: u32,
    /// Width of the sprite
    pub width: u32,
    /// Height of the sprite
    pub height: u32,
}

/// Structure acting as scaffolding for serde when loading a spritesheet file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SerializedSpriteSheet {
    /// Width of the sprite sheet
    pub texture_width: u32,
    /// Height of the sprite sheet
    pub texture_height: u32,
    /// Description of the sprites
    pub sprites: Vec<SpritePosition>,
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {

    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(parse(from_os_str))]
    /// The file to generate the sprite sheet data for
    input: PathBuf
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "generate")]
    /// Generates .ron data based on the file, slices into the given amount of horizontal and vertical slices
    Generate {
        #[structopt(short = "h")]
        horizontal_count: u32,
        #[structopt(short = "v")]
        vertical_count: u32,
        #[structopt(short = "d")]
        direction: ListDirection
    },
    /// Suggests a way of slicing the spritesheet, based on the content.
    /// The output will probably require some tweaking, but it can at least generate a starting point.
    Suggest
}


#[derive(Debug, StructOpt, Eq, PartialEq)]
enum ListDirection {
    #[structopt(name = "horizontal")]
    Horizontal,
    #[structopt(name = "vertical")]
    Vertical
}

impl Default for ListDirection {
    fn default() -> Self {
        ListDirection::Horizontal
    }
}

#[derive(Debug)]
enum ListDirectionParseError {
    UnknownDirection
}

impl Display for ListDirectionParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            ListDirectionParseError::UnknownDirection => write!(f, "Unknown direction")
        }
    }
}

impl std::str::FromStr for ListDirection {
    type Err = ListDirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "horizontal" => Ok(ListDirection::Horizontal),
            "vertical" => Ok(ListDirection::Vertical),
            _ => Err(ListDirectionParseError::UnknownDirection)
        }
    }
}

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    match opt.cmd {
        Command::Generate {horizontal_count, vertical_count, direction} => generate(&opt.input, horizontal_count, vertical_count, direction),
        Command::Suggest => unimplemented!()
    }
}

fn generate(file: &PathBuf, horizontal_count: u32, vertical_count: u32, direction: ListDirection) -> std::io::Result<()> {
    let decoder = Decoder::new(File::open(file)?);

    let (OutputInfo{width, height, ..}, _) = decoder.read_info()?;

    let sprite_width = width / horizontal_count;
    let sprite_height = height / vertical_count;

    let mut spritesheet = SerializedSpriteSheet {
        texture_height: height,
        texture_width: width,
        sprites: vec![],
    };

    let mut sprites = vec![];

    for i in 0..horizontal_count {
        for j in 0..vertical_count {
            let x = i * sprite_width;
            let y = j * sprite_height;

            let sprite = SpritePosition {
                height: sprite_height,
                width: sprite_width,
                x,
                y,
            };

            if i == 0 {
                sprites.push(vec![sprite]);
            } else {
                sprites.get_mut(j as usize).unwrap().push(sprite);
            }
        }
    }

    if direction == ListDirection::Horizontal {
        for row in sprites {
            for sprite in row {
                spritesheet.sprites.push(sprite);
            }
        }
    } else {
        for j in 0..vertical_count {
            for i in 0..horizontal_count {
                spritesheet.sprites.push(sprites[i as usize][j as usize].clone())
            }
        }
    }


    let s = to_ron_string(&spritesheet, PrettyConfig::default()).unwrap();

    println!("{}", s);

    Ok(())
}
