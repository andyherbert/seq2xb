pub struct XBinColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl XBinColor {
    pub fn from_24bits(r: u8, g: u8, b: u8) -> XBinColor {
        XBinColor {
            r: r / 4,
            g: g / 4,
            b: b / 4,
        }
    }

    pub fn from_hex(string: &str) -> Option<XBinColor> {
        if string.len() != 6 {
            None
        } else {
            let red = u8::from_str_radix(&string[0..=1], 16).ok()?;
            let green = u8::from_str_radix(&string[2..=3], 16).ok()?;
            let blue = u8::from_str_radix(&string[4..=5], 16).ok()?;
            Some(XBinColor::from_24bits(red, green, blue))
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}

pub struct XBinHeader {
    pub width: u16,
    pub height: u16,
    pub font_height: u8,
    pub palette: Option<Vec<XBinColor>>,
    pub font: Option<Vec<u8>>,
    pub compressed: bool,
    pub non_blink: bool,
}

impl XBinHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; 11];
        bytes[0..=4].copy_from_slice(b"XBIN\x1a");
        bytes[5..=6].copy_from_slice(&self.width.to_le_bytes());
        bytes[7..=8].copy_from_slice(&self.height.to_le_bytes());
        bytes[9] = self.font_height;
        if let Some(ref palette) = self.palette {
            bytes[10] = 1;
            for color in palette {
                bytes.append(&mut color.as_bytes());
            }
        }
        if let Some(ref font) = self.font {
            bytes[10] += 1 << 1;
            bytes.append(&mut font.clone());
        }
        if self.compressed {
            bytes[10] += 1 << 2;
        }
        if self.non_blink {
            bytes[10] += 1 << 3;
        }
        bytes
    }
}
