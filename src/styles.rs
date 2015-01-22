#[derive(Copy)]
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

#[derive(Copy)]
pub struct StylePoint {
    prefix: int,
    suffix: int
}

impl StylePoint {
    pub fn new(style: Styles) -> StylePoint {
        match style {
            Styles::Reset         => StylePoint { prefix: 0i, suffix: 0i },
            Styles::Bold          => StylePoint { prefix: 1i, suffix: 22i },
            Styles::Dim           => StylePoint { prefix: 2i, suffix: 22i },
            Styles::Italic        => StylePoint { prefix: 3i, suffix: 23i },
            Styles::Underline     => StylePoint { prefix: 4i, suffix: 24i },
            Styles::Inverse       => StylePoint { prefix: 7i, suffix: 27i },
            Styles::Hidden        => StylePoint { prefix: 8i, suffix: 28i },
            Styles::Strikethrough => StylePoint { prefix: 9i, suffix: 29i },
            Styles::Black         => StylePoint { prefix: 30i, suffix: 39i },
            Styles::Red           => StylePoint { prefix: 31i, suffix: 39i },
            Styles::Green         => StylePoint { prefix: 32i, suffix: 39i },
            Styles::Yellow        => StylePoint { prefix: 33i, suffix: 39i },
            Styles::Blue          => StylePoint { prefix: 34i, suffix: 39i },
            Styles::Magenta       => StylePoint { prefix: 35i, suffix: 39i },
            Styles::Cyan          => StylePoint { prefix: 36i, suffix: 39i },
            Styles::White         => StylePoint { prefix: 37i, suffix: 39i },
            Styles::Gray          => StylePoint { prefix: 90i, suffix: 39i },
            Styles::Grey          => StylePoint { prefix: 90i, suffix: 39i },
            Styles::BgBlack       => StylePoint { prefix: 40i, suffix: 49i },
            Styles::BgRed         => StylePoint { prefix: 41i, suffix: 49i },
            Styles::BgGreen       => StylePoint { prefix: 42i, suffix: 49i },
            Styles::BgYellow      => StylePoint { prefix: 43i, suffix: 49i },
            Styles::BgBlue        => StylePoint { prefix: 44i, suffix: 49i },
            Styles::BgMagenta     => StylePoint { prefix: 45i, suffix: 49i },
            Styles::BgCyan        => StylePoint { prefix: 46i, suffix: 49i },
            Styles::BgWhite       => StylePoint { prefix: 47i, suffix: 49i }
        }
    }

    pub fn get_prefix(&self) -> String {
        String::new() + "\u{001b}[" + self.prefix.to_string().as_slice() + "m"
    }

    pub fn get_suffix(&self) -> String {
        String::new() + "\u{001b}[" + self.suffix.to_string().as_slice() + "m"
    }
}
