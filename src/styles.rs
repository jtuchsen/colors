#[derive(Clone, Copy)]
pub enum Styles {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Inverse,
    Hidden,
    Strikethrough,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Grey,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite
}

#[derive(Clone, Copy)]
pub struct StylePoint {
    prefix: u8,
    suffix: u8
}

impl StylePoint {
    pub fn new(style: Styles) -> StylePoint {
        match style {
            Styles::Reset         => StylePoint { prefix: 0, suffix: 0 },
            Styles::Bold          => StylePoint { prefix: 1, suffix: 22 },
            Styles::Dim           => StylePoint { prefix: 2, suffix: 22 },
            Styles::Italic        => StylePoint { prefix: 3, suffix: 23 },
            Styles::Underline     => StylePoint { prefix: 4, suffix: 24 },
            Styles::Inverse       => StylePoint { prefix: 7, suffix: 27 },
            Styles::Hidden        => StylePoint { prefix: 8, suffix: 28 },
            Styles::Strikethrough => StylePoint { prefix: 9, suffix: 29 },
            Styles::Black         => StylePoint { prefix: 30, suffix: 39 },
            Styles::Red           => StylePoint { prefix: 31, suffix: 39 },
            Styles::Green         => StylePoint { prefix: 32, suffix: 39 },
            Styles::Yellow        => StylePoint { prefix: 33, suffix: 39 },
            Styles::Blue          => StylePoint { prefix: 34, suffix: 39 },
            Styles::Magenta       => StylePoint { prefix: 35, suffix: 39 },
            Styles::Cyan          => StylePoint { prefix: 36, suffix: 39 },
            Styles::White         => StylePoint { prefix: 37, suffix: 39 },
            Styles::Gray          => StylePoint { prefix: 90, suffix: 39 },
            Styles::Grey          => StylePoint { prefix: 90, suffix: 39 },
            Styles::BgBlack       => StylePoint { prefix: 40, suffix: 49 },
            Styles::BgRed         => StylePoint { prefix: 41, suffix: 49 },
            Styles::BgGreen       => StylePoint { prefix: 42, suffix: 49 },
            Styles::BgYellow      => StylePoint { prefix: 43, suffix: 49 },
            Styles::BgBlue        => StylePoint { prefix: 44, suffix: 49 },
            Styles::BgMagenta     => StylePoint { prefix: 45, suffix: 49 },
            Styles::BgCyan        => StylePoint { prefix: 46, suffix: 49 },
            Styles::BgWhite       => StylePoint { prefix: 47, suffix: 49 }
        }
    }

    pub fn get_prefix(&self) -> String {
        format!("\u{001b}[{}m", self.prefix)
    }

    pub fn get_suffix(&self) -> String {
        format!("\u{001b}[{}m", self.suffix)
    }
}
