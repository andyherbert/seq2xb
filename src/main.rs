mod seq2xb_error;
use clap::Parser;
use seq2xb_error::Seq2XBinError;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

fn convert(
    input: PathBuf,
    output: PathBuf,
    background: Option<u8>,
    columns: Option<u16>,
    shifted: bool,
) -> Result<(), Box<dyn Error>> {
    let bytes = {
        let mut file = File::open(input)?;
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes)?;
        bytes
    };
    let mut colour = 14; // Light Blue
    let mut reverse = false;
    let background = background.unwrap_or(6) << 4;
    let mut screen_bytes = Vec::new();
    for byte in bytes {
        match byte {
            0x93 => continue,    // Clear screen is ignored
            0x90 => colour = 0,  //Black,
            0x05 => colour = 1,  //White,
            0x1c => colour = 2,  //Red,
            0x9f => colour = 3,  //Cyan,
            0x9c => colour = 4,  //Purple,
            0x1e => colour = 5,  //Green,
            0x1f => colour = 6,  //Blue,
            0x9e => colour = 7,  //Yellow,
            0x81 => colour = 8,  //Orange,
            0x95 => colour = 9,  //Brown,
            0x96 => colour = 10, //Pink,
            0x97 => colour = 11, //DarkGrey,
            0x98 => colour = 12, //MidGrey,
            0x99 => colour = 13, //LightGreen,
            0x9a => colour = 14, //LightBlue,
            0x9b => colour = 15, //LightGrey,
            0x12 => reverse = true,
            0x92 => reverse = false,
            _ => {
                let code = match byte {
                    0x40..=0x5f | 0x80..=0xbf => byte - 0x40,
                    0xc0..=0xfe => byte - 0x80,
                    0xff => 0x5e,
                    _ => byte,
                };
                if reverse {
                    screen_bytes.push(code + 0x80);
                } else {
                    screen_bytes.push(code);
                }
                screen_bytes.push(colour + background);
            }
        }
    }
    let mut file = File::create(output)?;
    let mut xbin_bytes = if shifted {
        include_bytes!("bin/header-shifted.bin").to_vec()
    } else {
        include_bytes!("bin/header-unshifted.bin").to_vec()
    };
    let width = match columns {
        Some(0) => return Err(Box::new(Seq2XBinError::InvalidColumnValue)),
        Some(width) => width as usize,
        None => 40,
    };
    let height = screen_bytes.len() / 2 / width;
    xbin_bytes[5] = u8::try_from(width & 0xff).expect("conversion");
    xbin_bytes[6] = u8::try_from((width >> 8) & 0xff).expect("conversion");
    xbin_bytes[7] = u8::try_from(height & 0xff).expect("conversion");
    xbin_bytes[8] = u8::try_from((height >> 8) & 0xff).expect("conversion");
    file.write_all(&xbin_bytes)?;
    file.write_all(&screen_bytes)?;
    Ok(())
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Use background color 0-15
    #[clap(short, long, value_name = "0 to 15")]
    background: Option<u8>,
    /// Use shifted glyphs
    #[clap(short, long, action, value_name = "Defaults to unshifted")]
    shifted: bool,
    /// Use columns 1-many
    #[clap(short, long, value_name = "1 to 65535")]
    columns: Option<u16>,
    #[clap(value_name = "SEQ file")]
    input: PathBuf,
    #[clap(value_name = "XBIN file")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = convert(
        cli.input,
        cli.output,
        cli.background,
        cli.columns,
        cli.shifted,
    ) {
        eprintln!("{}", error);
    }
}
