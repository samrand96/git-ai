#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub error: Rgb,
    pub success: Rgb,
    pub warning: Rgb,
    pub info: Rgb,
    pub header: Rgb,
    pub highlight: Rgb,
    pub dim: Rgb,
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltInTheme {
    pub name: &'static str,
    pub description: &'static str,
    pub palette: ThemePalette,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_array(value: [u8; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }

    pub fn to_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub(crate) fn fg(self) -> String {
        format!("\u{001b}[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        let value = input.trim();
        if value.is_empty() {
            return Err("RGB value cannot be empty".to_string());
        }

        let hex = value.strip_prefix('#').unwrap_or(value);
        if hex.len() == 6 && hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|err| err.to_string())?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|err| err.to_string())?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|err| err.to_string())?;
            return Ok(Self::new(r, g, b));
        }

        let parts = value
            .split([',', ' ', ';'])
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err("Use R,G,B or #RRGGBB".to_string());
        }

        let mut channels = [0u8; 3];
        for (idx, part) in parts.iter().enumerate() {
            channels[idx] = part
                .trim()
                .parse::<u8>()
                .map_err(|_| format!("'{part}' is not a valid 0-255 channel"))?;
        }
        Ok(Self::from_array(channels))
    }
}

pub(crate) const DARK_THEME: ThemePalette = ThemePalette {
    error: Rgb::new(208, 80, 72),
    success: Rgb::new(95, 160, 110),
    warning: Rgb::new(196, 154, 74),
    info: Rgb::new(92, 132, 176),
    header: Rgb::new(84, 175, 170),
    highlight: Rgb::new(225, 225, 220),
    dim: Rgb::new(120, 124, 128),
};

pub(crate) const LIGHT_THEME: ThemePalette = ThemePalette {
    error: Rgb::new(166, 46, 48),
    success: Rgb::new(44, 118, 72),
    warning: Rgb::new(142, 104, 28),
    info: Rgb::new(48, 92, 142),
    header: Rgb::new(38, 122, 130),
    highlight: Rgb::new(28, 32, 36),
    dim: Rgb::new(104, 108, 112),
};

pub(crate) const BUILT_IN_THEMES: [BuiltInTheme; 6] = [
    BuiltInTheme {
        name: "auto",
        description: "match the system dark/light appearance",
        palette: DARK_THEME,
    },
    BuiltInTheme {
        name: "terminal",
        description: "muted steel and phosphor, low glare",
        palette: DARK_THEME,
    },
    BuiltInTheme {
        name: "daylight",
        description: "high-contrast palette for light terminals",
        palette: LIGHT_THEME,
    },
    BuiltInTheme {
        name: "matrix",
        description: "green phosphor, black-terminal energy",
        palette: ThemePalette {
            error: Rgb::new(255, 92, 92),
            success: Rgb::new(60, 220, 110),
            warning: Rgb::new(190, 210, 95),
            info: Rgb::new(96, 190, 135),
            header: Rgb::new(80, 255, 145),
            highlight: Rgb::new(226, 255, 230),
            dim: Rgb::new(86, 122, 96),
        },
    },
    BuiltInTheme {
        name: "midnight",
        description: "blue/cyan operations console",
        palette: ThemePalette {
            error: Rgb::new(218, 92, 112),
            success: Rgb::new(100, 180, 150),
            warning: Rgb::new(218, 176, 88),
            info: Rgb::new(120, 154, 220),
            header: Rgb::new(110, 210, 232),
            highlight: Rgb::new(235, 240, 248),
            dim: Rgb::new(116, 124, 144),
        },
    },
    BuiltInTheme {
        name: "amber",
        description: "warm CRT amber with red alerts",
        palette: ThemePalette {
            error: Rgb::new(220, 88, 72),
            success: Rgb::new(180, 170, 92),
            warning: Rgb::new(230, 165, 68),
            info: Rgb::new(205, 145, 70),
            header: Rgb::new(255, 186, 82),
            highlight: Rgb::new(255, 232, 190),
            dim: Rgb::new(145, 118, 82),
        },
    },
];

#[cfg(test)]
mod tests {
    use super::Rgb;

    #[test]
    fn parses_rgb_triples_and_hex_values() {
        assert_eq!(Rgb::parse("80,255,145").unwrap().to_array(), [80, 255, 145]);
        assert_eq!(Rgb::parse("80 255 145").unwrap().to_array(), [80, 255, 145]);
        assert_eq!(Rgb::parse("#50ff91").unwrap().to_array(), [80, 255, 145]);
    }

    #[test]
    fn rejects_invalid_rgb_values() {
        assert!(Rgb::parse("80,255").is_err());
        assert!(Rgb::parse("300,1,2").is_err());
        assert!(Rgb::parse("not-rgb").is_err());
    }

    #[test]
    fn rgb_round_trips_to_array() {
        assert_eq!(Rgb::new(1, 2, 3).to_array(), [1, 2, 3]);
    }
}
