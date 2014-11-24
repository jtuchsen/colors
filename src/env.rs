use std::os;
use libc;

const COLORS_OFF : [&'static str, ..2] = ["--no-color", "--color=false"];
const COLORS_ON  : [&'static str, ..3] = ["--color", "--color=always", "--color=true"];

pub fn supports_colors() -> bool {
    supports_colors_from_env()
}

fn supports_colors_from_env() -> bool {
    let os_args = os::args();
    let colors: Vec<&str> = COLORS_OFF.iter().chain(COLORS_ON.iter()).map(|&str| str).collect();
    let args: Vec<&str> = os_args
        .iter()
        .filter_map(|arg| colors.iter().find(|&&c| arg.as_slice() == c).map(|&c| c))
        .collect();

    // TODO: Figure out just how cross platform this is, the answer is probably not very
    let is_tty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
    let is_color_term = os::getenv("COLOR_TERM").is_some();
    let term = os::getenv("TERM").unwrap_or(String::new());

    supports_colors_pure(args, is_tty, is_color_term, term)
}

fn supports_colors_pure(args: Vec<&str>, is_tty: bool, is_color_term: bool, term: String) -> bool {
    if args.iter().any(|arg| { COLORS_OFF.iter().any(|&neg| { neg == arg.as_slice() }) }) {
        return false
    }

    if args.iter().any(|arg| { COLORS_ON.iter().any(|&pos| { pos == arg.as_slice() }) }) {
        return true
    }

    if !is_tty {
        return false
    }
    
    if is_color_term {
        return true
    }

    if term.as_slice() == "dumb" {
        return false
    }

    return regex!(r"/^screen|^xterm|^vt100|color|ansi|cygwin|linux/i").is_match(term.as_slice())
}

#[test]
fn it_detects_support_for_terminal_colors() {
    assert!(!supports_colors_pure(vec!["--no-color"], true, true, String::new()));
    assert!(!supports_colors_pure(vec![], false, false, String::new()));
    assert!(!supports_colors_pure(vec![], true, false, "dumb".to_string()));
    assert!(!supports_colors_pure(vec![], true, false, String::new()));

    assert!(supports_colors_pure(vec!["--color"], false, false, String::new()));
    assert!(supports_colors_pure(vec![], true, true, String::new()));
    assert!(supports_colors_pure(vec![], true, false, "xterm".to_string()));
}
