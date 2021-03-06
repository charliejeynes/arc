//! Banner functions.

use colored::Colorize;
use terminal_size::terminal_size;

/// Fallback terminal width if it cannot be determined at runtime.
const FALLBACK_TERM_WIDTH: usize = 80;

/// Index of the section.
static mut SECTION: i32 = 0;

/// Determine the terminal width.
#[inline]
#[must_use]
pub fn term_width() -> usize {
    let ts = terminal_size();

    if let Some(ts) = ts {
        (ts.0).0 as usize
    } else {
        FALLBACK_TERM_WIDTH
    }
}

/// Print a main title bar.
#[inline]
pub fn title(title: &str) {
    let title = title.to_uppercase();

    let term_width = term_width();

    let (left_bar, right_bar) = if term_width < ((title.len() * 2) + 11) {
        (4, 4)
    } else {
        let left_bar = (term_width - (title.len() * 2) - 3) / 2;
        (left_bar, term_width - (title.len() * 2) - 3 - left_bar)
    };

    print!("{} ", "\u{2588}".repeat(left_bar));

    for (pos, ch) in title.chars().enumerate() {
        match pos % 6 {
            0 => print!(" {}", format!("{}", ch).bright_red().bold()),
            1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
            2 => print!(" {}", format!("{}", ch).bright_green().bold()),
            3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
            4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
            5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
            _ => unreachable!(),
        }
    }

    println!("  {}", "\u{2588}".repeat(right_bar));
}

/// Print a closing bar.
#[inline]
pub fn end(message: &str) {
    let message = message.to_lowercase();

    let term_width = term_width();

    let (left_bar, right_bar) = if term_width < ((message.len() * 2) + 11) {
        (4, 4)
    } else {
        let left_bar = (term_width - (message.len() * 2) - 3) / 2;
        (left_bar, term_width - (message.len() * 2) - 3 - left_bar)
    };

    println!();
    println!();
    print!("{} ", "\u{2588}".repeat(left_bar));

    for (pos, ch) in message.chars().enumerate() {
        match pos % 6 {
            0 => print!(" {}", format!("{}", ch).bright_red().bold()),
            1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
            2 => print!(" {}", format!("{}", ch).bright_green().bold()),
            3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
            4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
            5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
            _ => unreachable!(),
        }
    }

    println!("  {}", "\u{2588}".repeat(right_bar));
}

/// Print a section bar.
#[inline]
pub fn section(title: &str) {
    unsafe {
        SECTION += 1;
    }

    println!();
    println!();
    print!("====");
    print!(" {}", colour(title).bold());

    let term_width = term_width();
    let mut cur_len = 5 + title.len();
    if cur_len >= term_width {
        println!();
        return;
    }

    print!(" ");
    cur_len += 1;
    while cur_len < term_width {
        print!("=");
        cur_len += 1;
    }

    println!();
}

/// Print a sub-section bar.
#[inline]
pub fn sub_section(title: &str) {
    println!();
    println!("---- {} ----", colour(title));
}

/// Colour a given string with the appropriate section colour.
#[inline]
#[must_use]
fn colour(string: &str) -> String {
    match unsafe { SECTION } % 6 {
        0 => format!("{}", string.bright_magenta()),
        1 => format!("{}", string.bright_red()),
        2 => format!("{}", string.bright_yellow()),
        3 => format!("{}", string.bright_green()),
        4 => format!("{}", string.bright_cyan()),
        5 => format!("{}", string.bright_blue()),
        _ => unreachable!(),
    }
}
