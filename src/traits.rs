use super::{Styles, style};
use std::ops::Deref;

pub trait Style {
    fn style(&self, style: Styles) -> String;
    fn reset(&self) -> String { self.style(Styles::Reset) }
    fn bold(&self) -> String { self.style(Styles::Bold) }
    fn dim(&self) -> String { self.style(Styles::Dim) }
    fn italic(&self) -> String { self.style(Styles::Italic) }
    fn underline(&self) -> String { self.style(Styles::Underline) }
    fn inverse(&self) -> String { self.style(Styles::Inverse) }
    fn hidden(&self) -> String { self.style(Styles::Hidden) }
    fn strikethrough(&self) -> String { self.style(Styles::Strikethrough) }
    fn black(&self) -> String { self.style(Styles::Black) }
    fn red(&self) -> String { self.style(Styles::Red) }
    fn green(&self) -> String { self.style(Styles::Green) }
    fn yellow(&self) -> String { self.style(Styles::Yellow) }
    fn blue(&self) -> String { self.style(Styles::Blue) }
    fn magenta(&self) -> String { self.style(Styles::Magenta) }
    fn cyan(&self) -> String { self.style(Styles::Cyan) }
    fn white(&self) -> String { self.style(Styles::White) }
    fn gray(&self) -> String { self.style(Styles::Gray) }
    fn grey(&self) -> String { self.style(Styles::Grey) }
    fn bg_black(&self) -> String { self.style(Styles::BgBlack) }
    fn bg_red(&self) -> String { self.style(Styles::BgRed) }
    fn bg_green(&self) -> String { self.style(Styles::BgGreen) }
    fn bg_yellow(&self) -> String { self.style(Styles::BgYellow) }
    fn bg_blue(&self) -> String { self.style(Styles::BgBlue) }
    fn bg_magenta(&self) -> String { self.style(Styles::BgMagenta) }
    fn bg_cyan(&self) -> String { self.style(Styles::BgCyan) }
    fn bg_white(&self) -> String { self.style(Styles::BgWhite) }
}

impl<T> Style for T where T: Deref<Target=str> {
    fn style(&self, choice: Styles) -> String { style(choice, self.deref()) }
}
