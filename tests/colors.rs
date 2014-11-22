#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate core;
extern crate colors;

use colors::styles::Styles;
use colors::styles::StylePoint;
use colors::style;
use colors::Style;

const STYLES: [Styles, ..26] = [
    Styles::Reset,
    Styles::Bold,
    Styles::Dim,
    Styles::Italic,
    Styles::Underline,
    Styles::Inverse,
    Styles::Hidden,
    Styles::Strikethrough,
    Styles::Black,
    Styles::Red,
    Styles::Green,
    Styles::Yellow,
    Styles::Blue,
    Styles::Magenta,
    Styles::Cyan,
    Styles::White,
    Styles::Gray,
    Styles::Grey,
    Styles::BgBlack,
    Styles::BgRed,
    Styles::BgGreen,
    Styles::BgYellow,
    Styles::BgBlue,
    Styles::BgMagenta,
    Styles::BgCyan,
    Styles::BgWhite
];

const STYLE_NAMES: [&'static str, ..26] = [
    "Reset",
    "Bold",
    "Dim",
    "Italic",
    "Underline",
    "Inverse",
    "Hidden",
    "Strikethrough",
    "Black",
    "Red",
    "Green",
    "Yellow",
    "Blue",
    "Magenta",
    "Cyan",
    "White",
    "Gray",
    "Grey",
    "BgBlack",
    "BgRed", 
    "BgGreen",
    "BgYellow",
    "BgBlue",
    "BgMagenta",
    "BgCyan",
    "BgWhite"
];

// TODO: Make a test specific logging mechanism that prints "greetings", it
// looks like Cargo/Rust doesn't run tests in a synchronous manner so the
// logging gets all messed up

#[test]
fn it_styles_strings_using_direct_calling() {
    for zip in STYLES.iter().zip(STYLE_NAMES.iter()) {
        let (style_type, style_name): (&Styles, &&str) = zip;
        let style_point = StylePoint::new(*style_type);
        let styled = style(*style_type, "greetings");

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(style_point.get_prefix().as_slice()),
            "Coloring prefix failed for {}", styled);

        assert!(styled.as_slice().contains("greetings"));

        assert!(styled.ends_with(style_point.get_suffix().as_slice()),
            "Coloring suffix failed for {}", styled);
    }
}

#[test]
fn it_styles_strings_using_static_strings() {
    for zip in STYLES.iter().zip(STYLE_NAMES.iter()) {
        let (style_type, style_name): (&Styles, &&str) = zip;
        let style_point = StylePoint::new(*style_type);
        let styled = "greetings".style(*style_type);

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(style_point.get_prefix().as_slice()),
            "Styling prefix failed for color {} with {}", style_name, styled);
        assert!(styled.as_slice().contains("greetings"),
            "Styling dropped the text for {}", style_name);
        assert!(styled.ends_with(style_point.get_suffix().as_slice()),
            "Styling suffix failed for color {} with {}", style_name, styled);
    }

    // TODO: Figure out how to test individual colors without resorting to
    // calling them all one by one. It seems like the only way to do
    // metaprogramming in Rust is with macros, the guide to which comes with a
    // big disclaimer that they're about to change (date today 11/21/14), so I
    // don't want to deal with it.
    let styled = "greetings".red();
    let style_point = StylePoint::new(Styles::Red);

    assert!(styled.starts_with(style_point.get_prefix().as_slice()),
        "Styling prefix failed for color {} with {}", "Red", styled);
    assert!(styled.as_slice().contains("greetings"),
        "Styling dropped the text for {}", "Red");
    assert!(styled.ends_with(style_point.get_suffix().as_slice()),
        "Styling suffix failed for color {} with {}", "Red", styled);
}

#[test]
fn it_styles_strings_using_string_instances() {
    for zip in STYLES.iter().zip(STYLE_NAMES.iter()) {
        let (style_type, style_name): (&Styles, &&str) = zip;
        let style_point = StylePoint::new(*style_type);
        let styled = (String::new() + "greetings").style(*style_type);

        debug!("{} should be style type {}", styled, style_name);

        assert!(styled.starts_with(style_point.get_prefix().as_slice()),
            "Styling prefix failed for color {} with {}", style_name, styled);
        assert!(styled.as_slice().contains("greetings"),
            "Styling dropped the text for {}", style_name);
        assert!(styled.ends_with(style_point.get_suffix().as_slice()),
            "Styling suffix failed for color {} with {}", style_name, styled);
    }

    let styled = (String::new() + "greetings").red();
    let style_point = StylePoint::new(Styles::Red);

    assert!(styled.starts_with(style_point.get_prefix().as_slice()),
        "Styling prefix failed for color {} with {}", "Red", styled);
    assert!(styled.as_slice().contains("greetings"),
        "Styling dropped the text for {}", "Red");
    assert!(styled.ends_with(style_point.get_suffix().as_slice()),
        "Styling suffix failed for color {} with {}", "Red", styled);
}
