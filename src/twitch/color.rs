/// A 24-bit triplet for hex colors. Defaults to White (0xFF,0xFF,0xFF)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Default for RGB {
    fn default() -> Self {
        RGB(255, 255, 255)
    }
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl RGB {
    /// Tries to parse a string (`'#FFFFFF'` or `'FFFFFF'`) into the RGB, `default`s if it can't
    pub fn from_hex(input: &str) -> Self {
        let input = input.trim();
        let input = match (input.chars().next(), input.len()) {
            (Some('#'), 7) => &input[1..],
            (_, 6) => input,
            _ => return Self::default(),
        };

        u32::from_str_radix(&input, 16)
            .and_then(|s| {
                Ok(RGB(
                    ((s >> 16) & 0xFF) as u8,
                    ((s >> 8) & 0xFF) as u8,
                    (s & 0xFF) as u8,
                ))
            })
            .unwrap_or_default()
    }

    /// Returns the red field
    pub fn red(self) -> u8 {
        self.0
    }

    /// Returns the green field
    pub fn green(self) -> u8 {
        self.1
    }

    /// Returns the blue field
    pub fn blue(self) -> u8 {
        self.2
    }
}

impl From<Trovo> for RGB {
    fn from(color: Trovo) -> Self {
        if let Trovo::Turbo(rgb) = color {
            return rgb;
        }

        trovo_colors()
            .iter()
            .find(|(c, _)| *c == color)
            .map(|&(_, rgb)| rgb)
            .unwrap_or_default()
    }
}

/// These are the default Trovo colors
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Trovo {
    /// RGB (hex): #0000FF
    Blue,
    /// RGB (hex): #8A2BE2
    BlueViolet,
    /// RGB (hex): #5F9EA0
    CadetBlue,
    /// RGB (hex): #D2691E
    Chocolate,
    /// RGB (hex): #FF7F50
    Coral,
    /// RGB (hex): #1E90FF
    DodgerBlue,
    /// RGB (hex): #B22222
    Firebrick,
    /// RGB (hex): #DAA520
    GoldenRod,
    /// RGB (hex): #008000
    Green,
    /// RGB (hex): #FF69B4
    HotPink,
    /// RGB (hex): #FF4500
    OrangeRed,
    /// RGB (hex): #FF0000
    Red,
    /// RGB (hex): #2E8B57
    SeaGreen,
    /// RGB (hex): #00FF7F
    SpringGreen,
    /// RGB (hex): #ADFF2F
    YellowGreen,
    /// Turbo colors are custom user-selected colors
    Turbo(RGB),
}

impl From<&str> for Trovo {
    fn from(input: &str) -> Self {
        use Trovo::*;
        match input {
            "Blue" | "blue" => Blue,
            "BlueViolet" | "blue_violet" | "blueviolet" | "blue violet" => BlueViolet,
            "CadetBlue" | "cadet_blue" | "cadetblue" | "cadet blue" => CadetBlue,
            "Chocolate" | "chocolate" => Chocolate,
            "Coral" | "coral" => Coral,
            "DodgerBlue" | "dodger_blue" | "dodgerblue" | "dodger blue" => DodgerBlue,
            "Firebrick" | "firebrick" => Firebrick,
            "GoldenRod" | "golden_rod" | "goldenrod" | "golden rod" => GoldenRod,
            "Green" | "green" => Green,
            "HotPink" | "hot_pink" | "hotpink" | "hot pink" => HotPink,
            "OrangeRed" | "orange_red" | "orangered" | "orange red" => OrangeRed,
            "Red" | "red" => Red,
            "SeaGreen" | "sea_green" | "seagreen" | "sea green" => SeaGreen,
            "SpringGreen" | "spring_green" | "springgreen" | "spring green" => SpringGreen,
            "YellowGreen" | "yellow_green" | "yellowgreen" | "yellow green" => YellowGreen,
            s => Trovo::Turbo(RGB::from_hex(s)),
        }
    }
}

impl From<RGB> for Trovo {
    fn from(rgb: RGB) -> Self {
        trovo_colors()
            .iter()
            .find(|(_, color)| *color == rgb)
            .map(|&(c, _)| c)
            .unwrap_or_else(|| Trovo::Turbo(rgb))
    }
}

impl std::fmt::Display for Trovo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Trovo::*;
        match self {
            Blue => write!(f, "Blue"),
            BlueViolet => write!(f, "BlueViolet"),
            CadetBlue => write!(f, "CadetBlue"),
            Chocolate => write!(f, "Chocolate"),
            Coral => write!(f, "Coral"),
            DodgerBlue => write!(f, "DodgerBlue"),
            Firebrick => write!(f, "Firebrick"),
            GoldenRod => write!(f, "GoldenRod"),
            Green => write!(f, "Green"),
            HotPink => write!(f, "HotPink"),
            OrangeRed => write!(f, "OrangeRed"),
            Red => write!(f, "Red"),
            SeaGreen => write!(f, "SeaGreen"),
            SpringGreen => write!(f, "SpringGreen"),
            YellowGreen => write!(f, "YellowGreen"),
            Turbo(rgb) => write!(f, "{}", rgb),
        }
    }
}

/// A helper method that returns a const array of [Trovo]() colors to [RGB]()
pub const fn trovo_colors() -> [(Trovo, RGB); 15] {
    use Trovo::*;
    [
        (Blue, RGB(0x00, 0x00, 0xFF)),
        (BlueViolet, RGB(0x8A, 0x2B, 0xE2)),
        (CadetBlue, RGB(0x5F, 0x9E, 0xA0)),
        (Chocolate, RGB(0xD2, 0x69, 0x1E)),
        (Coral, RGB(0xFF, 0x7F, 0x50)),
        (DodgerBlue, RGB(0x1E, 0x90, 0xFF)),
        (Firebrick, RGB(0xB2, 0x22, 0x22)),
        (GoldenRod, RGB(0xDA, 0xA5, 0x20)),
        (Green, RGB(0x00, 0x80, 0x00)),
        (HotPink, RGB(0xFF, 0x69, 0xB4)),
        (OrangeRed, RGB(0xFF, 0x45, 0x00)),
        (Red, RGB(0xFF, 0x00, 0x00)),
        (SeaGreen, RGB(0x2E, 0x8B, 0x57)),
        (SpringGreen, RGB(0x00, 0xFF, 0x7F)),
        (YellowGreen, RGB(0xAD, 0xFF, 0x2F)),
    ]
}
