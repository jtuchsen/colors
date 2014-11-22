use super::{Styles,style};

pub trait Style {
    fn style(self, style: Styles) -> String;
    fn reset(self) -> String;
    fn bold(self) -> String;
    fn dim(self) -> String;
    fn italic(self) -> String;
    fn underline(self) -> String;
    fn inverse(self) -> String;
    fn hidden(self) -> String;
    fn strikethrough(self) -> String;
    fn black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn white(self) -> String;
    fn gray(self) -> String;
    fn grey(self) -> String;
    fn bg_black(self) -> String;
    fn bg_red(self) -> String;
    fn bg_green(self) -> String;
    fn bg_yellow(self) -> String;
    fn bg_blue(self) -> String;
    fn bg_magenta(self) -> String;
    fn bg_cyan(self) -> String;
    fn bg_white(self) -> String;
}

impl Style for &'static str {
    fn style(self, choice: Styles) -> String { style(choice, *&self) }
    fn reset(self) -> String { style(Styles::Reset, *&self) }
    fn bold(self) -> String { style(Styles::Bold, *&self) }
    fn dim(self) -> String { style(Styles::Dim, *&self) }
    fn italic(self) -> String { style(Styles::Italic, *&self) }
    fn underline(self) -> String { style(Styles::Underline, *&self) }
    fn inverse(self) -> String { style(Styles::Inverse, *&self) }
    fn hidden(self) -> String { style(Styles::Hidden, *&self) }
    fn strikethrough(self) -> String { style(Styles::Strikethrough, *&self) }
    fn black(self) -> String { style(Styles::Black, *&self) }
    fn red(self) -> String { style(Styles::Red, *&self) }
    fn green(self) -> String { style(Styles::Green, *&self) }
    fn yellow(self) -> String { style(Styles::Yellow, *&self) }
    fn blue(self) -> String { style(Styles::Blue, *&self) }
    fn magenta(self) -> String { style(Styles::Magenta, *&self) }
    fn cyan(self) -> String { style(Styles::Cyan, *&self) }
    fn white(self) -> String { style(Styles::White, *&self) }
    fn gray(self) -> String { style(Styles::Gray, *&self) }
    fn grey(self) -> String { style(Styles::Grey, *&self) }
    fn bg_black(self) -> String { style(Styles::BgBlack, *&self) }
    fn bg_red(self) -> String { style(Styles::BgRed, *&self) }
    fn bg_green(self) -> String { style(Styles::BgGreen, *&self) }
    fn bg_yellow(self) -> String { style(Styles::BgYellow, *&self) }
    fn bg_blue(self) -> String { style(Styles::BgBlue, *&self) }
    fn bg_magenta(self) -> String { style(Styles::BgMagenta, *&self) }
    fn bg_cyan(self) -> String { style(Styles::BgCyan, *&self) }
    fn bg_white(self) -> String { style(Styles::BgWhite, *&self) }
}

impl Style for String {
    fn style(self, choice: Styles) -> String { style(choice, self.as_slice()) }
    fn reset(self) -> String { style(Styles::Reset, self.as_slice()) }
    fn bold(self) -> String { style(Styles::Bold, self.as_slice()) }
    fn dim(self) -> String { style(Styles::Dim, self.as_slice()) }
    fn italic(self) -> String { style(Styles::Italic, self.as_slice()) }
    fn underline(self) -> String { style(Styles::Underline, self.as_slice()) }
    fn inverse(self) -> String { style(Styles::Inverse, self.as_slice()) }
    fn hidden(self) -> String { style(Styles::Hidden, self.as_slice()) }
    fn strikethrough(self) -> String { style(Styles::Strikethrough, self.as_slice()) }
    fn black(self) -> String { style(Styles::Black, self.as_slice()) }
    fn red(self) -> String { style(Styles::Red, self.as_slice()) }
    fn green(self) -> String { style(Styles::Green, self.as_slice()) }
    fn yellow(self) -> String { style(Styles::Yellow, self.as_slice()) }
    fn blue(self) -> String { style(Styles::Blue, self.as_slice()) }
    fn magenta(self) -> String { style(Styles::Magenta, self.as_slice()) }
    fn cyan(self) -> String { style(Styles::Cyan, self.as_slice()) }
    fn white(self) -> String { style(Styles::White, self.as_slice()) }
    fn gray(self) -> String { style(Styles::Gray, self.as_slice()) }
    fn grey(self) -> String { style(Styles::Grey, self.as_slice()) }
    fn bg_black(self) -> String { style(Styles::BgBlack, self.as_slice()) }
    fn bg_red(self) -> String { style(Styles::BgRed, self.as_slice()) }
    fn bg_green(self) -> String { style(Styles::BgGreen, self.as_slice()) }
    fn bg_yellow(self) -> String { style(Styles::BgYellow, self.as_slice()) }
    fn bg_blue(self) -> String { style(Styles::BgBlue, self.as_slice()) }
    fn bg_magenta(self) -> String { style(Styles::BgMagenta, self.as_slice()) }
    fn bg_cyan(self) -> String { style(Styles::BgCyan, self.as_slice()) }
    fn bg_white(self) -> String { style(Styles::BgWhite, self.as_slice()) }
}
