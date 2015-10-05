#![feature(rustc_private, core)]

#[macro_use]
extern crate log;
extern crate core;
extern crate colors;

use colors::Styles;
use colors::StylePoint;
use colors::style;
use colors::Style;

const STYLES: [(Styles, &'static str); 26] = [
    (Styles::Reset, "Reset"),
    (Styles::Bold, "Bold"),
    (Styles::Dim, "Dim"),
    (Styles::Italic, "Italic"),
    (Styles::Underline, "Underline"),
    (Styles::Inverse, "Inverse"),
    (Styles::Hidden, "Hidden"),
    (Styles::Strikethrough, "Strikethrough"),
    (Styles::Black, "Black"),
    (Styles::Red, "Red"),
    (Styles::Green, "Green"),
    (Styles::Yellow, "Yellow"),
    (Styles::Blue, "Blue"),
    (Styles::Magenta, "Magenta"),
    (Styles::Cyan, "Cyan"),
    (Styles::White, "White"),
    (Styles::Gray, "Gray"),
    (Styles::Grey, "Grey"),
    (Styles::BgBlack, "BgBlack"),
    (Styles::BgRed, "BgRed"),
    (Styles::BgGreen, "BgGreen"),
    (Styles::BgYellow, "BgYellow"),
    (Styles::BgBlue, "BgBlue"),
    (Styles::BgMagenta, "BgMagenta"),
    (Styles::BgCyan, "BgCyan"),
    (Styles::BgWhite, "BgWhite")
];

// TODO: Make a test specific logging mechanism that prints "greetings", it
// looks like Cargo/Rust doesn't run tests in a synchronous manner so the
// logging gets all messed up

#[test]
fn it_styles_strings_using_direct_calling() {
    for zip in STYLES.iter() {
        let (style_type, style_name): (Styles, &str) = *zip;
        let style_point = StylePoint::new(style_type);
        let styled = style(style_type, "greetings");

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(&*style_point.get_prefix()),
            "Coloring prefix failed for {}", styled);

        assert!(styled.contains("greetings"));

        assert!(styled.ends_with(&*style_point.get_suffix()),
            "Coloring suffix failed for {}", styled);
    }
}

#[test]
fn it_styles_strings_using_static_strings() {
    for zip in STYLES.iter() {
        let (style_type, style_name): (Styles, &str) = *zip;
        let style_point = StylePoint::new(style_type);
        let styled = "greetings".style(style_type);

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(&*style_point.get_prefix()),
            "Styling prefix failed for color {} with {}", style_name, styled);
        assert!(styled.contains("greetings"),
            "Styling dropped the text for {}", style_name);
        assert!(styled.ends_with(&*style_point.get_suffix()),
            "Styling suffix failed for color {} with {}", style_name, styled);
    }

    // TODO: Figure out how to test individual colors without resorting to
    // calling them all one by one. It seems like the only way to do
    // metaprogramming in Rust is with macros, the guide to which comes with a
    // big disclaimer that they're about to change (date today 11/21/14), so I
    // don't want to deal with it.
    let styled = "greetings".red();
    let style_point = StylePoint::new(Styles::Red);

    assert!(styled.starts_with(&*style_point.get_prefix()),
        "Styling prefix failed for color {} with {}", "Red", styled);
    assert!(styled.contains("greetings"),
        "Styling dropped the text for {}", "Red");
    assert!(styled.ends_with(&*style_point.get_suffix()),
        "Styling suffix failed for color {} with {}", "Red", styled);
}

#[test]
fn it_styles_strings_using_string_instances() {
    for zip in STYLES.iter() {
        let (style_type, style_name): (Styles, &str) = *zip;
        let style_point = StylePoint::new(style_type);
        let styled = (String::new() + "greetings").style(style_type);

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(&*style_point.get_prefix()),
            "Styling prefix failed for color {} with {}", style_name, styled);
        assert!(styled.contains("greetings"),
            "Styling dropped the text for {}", style_name);
        assert!(styled.ends_with(&*style_point.get_suffix()),
            "Styling suffix failed for color {} with {}", style_name, styled);
    }

    let styled = (String::new() + "greetings").red();
    let style_point = StylePoint::new(Styles::Red);

    assert!(styled.starts_with(&*style_point.get_prefix()),
        "Styling prefix failed for color {} with {}", "Red", styled);
    assert!(styled.contains("greetings"),
        "Styling dropped the text for {}", "Red");
    assert!(styled.ends_with(&*style_point.get_suffix()),
        "Styling suffix failed for color {} with {}", "Red", styled);
}
