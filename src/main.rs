mod seq;
mod xbin_header;
use clap::Parser;
use seq::{C64Color, IntoSeqIterator, Seq2XBinError, SeqElement};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};
use xbin_header::{XBinColor, XBinHeader};

fn convert(
    input: PathBuf,
    output: PathBuf,
    background: Option<u8>,
    columns: Option<u16>,
    shifted: bool,
) -> Result<(), Box<dyn Error>> {
    let seq_iter = {
        let file = File::open(input)?;
        BufReader::new(file).bytes().flatten().into_seq_iter()
    };
    let mut screen_bytes = Vec::new();
    let background = background.unwrap_or(u8::from(C64Color::Blue)) << 4;
    let mut color = u8::from(C64Color::LightBlue);
    for seq in seq_iter {
        match seq {
            SeqElement::ClearScreen => continue,
            SeqElement::Color(value) => color = u8::from(value),
            SeqElement::Reverse(_) => continue,
            SeqElement::Character(value) => {
                screen_bytes.push(value);
                screen_bytes.push(color + background);
            }
        }
    }
    let width = match columns {
        Some(0) => return Err(Box::new(Seq2XBinError::InvalidColumnValue)),
        Some(width) => width,
        None => 40,
    };
    let height = (screen_bytes.len() / 2 / width as usize) as u16;
    let palette = include_str!("palette/commodore64.hex")
        .lines()
        .map(|line| XBinColor::from_hex(line).expect("legal hex"))
        .collect();
    let font = if shifted {
        include_bytes!("fonts/PETSCII shifted.F08").to_vec()
    } else {
        include_bytes!("fonts/PETSCII unshifted.F08").to_vec()
    };
    let mut file = File::create(output)?;
    let header = XBinHeader {
        width,
        height,
        font_height: 8,
        palette: Some(palette),
        font: Some(font),
        compressed: false,
        non_blink: true,
    };
    file.write_all(&header.as_bytes())?;
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
