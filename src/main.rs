mod seq2xb_error;
mod seq_iterator;
use clap::Parser;
use seq2xb_error::Seq2XBinError;
use seq_iterator::{C64Color, IntoSeqIterator};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

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
            seq_iterator::SeqElement::ClearScreen => continue,
            seq_iterator::SeqElement::Color(value) => color = u8::from(value),
            seq_iterator::SeqElement::Reverse(_) => continue,
            seq_iterator::SeqElement::Character(value) => {
                screen_bytes.push(value);
                screen_bytes.push(color + background);
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
