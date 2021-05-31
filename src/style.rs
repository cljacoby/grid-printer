//! The style module provides an API to customize the style settings
//! for a GridPrinter instance, such as the font foreground color, the font
//! background color, and the font text style (bold, italic, strike-through, etc.).
//!
//! # Example
//! ```rust
//! use grid_printer::GridPrinter;
//! use grid_printer::style::{Fg, Bg, Sgr, StyleOpt};
//! use std::error::Error;
//! 
//! fn main() -> Result<(), Box<dyn Error>> {
//! 
//!     let grid = vec![
//!         vec![1, 2, 3, 4, ],
//!         vec![5, 6, 7, 8, ],
//!         vec![9, 10, 11, 12, ],
//!     ];
//! 
//!     let rows = grid.len();
//!     let cols = grid[0].len();
//!     
//!     let printer = GridPrinter::builder(rows, cols)
//!         .col_style(0, StyleOpt::new().fg(Fg::Magenta))?
//!         .col_style(1, StyleOpt::new().fg(Fg::Black).bg(Bg::BrightYellow))?
//!         .col_style(2, StyleOpt::new().sgr(Sgr::StrikeThrough))?
//!         .col_style(3, StyleOpt::new().fg(Fg::Cyan))?
//!         .build();
//!     printer.print(&grid);
//! 
//!     Ok(())
//! }
//! ```
//! # Output
//! <div style="background-color:#2A2A2A">
//! <span style="color:magenta">1</span>    <span>&nbsp;&nbsp;&nbsp;&nbsp;</span>   <span style="color:black;background-color:yellow">2</span>      <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span>     <span style="text-decoration:line-through">3</span>   <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span>     <span style="color:cyan;text-decoration:italic">4</span><br/>
//! <span style="color:magenta">5</span>    <span>&nbsp;&nbsp;&nbsp;&nbsp;</span>   <span style="color:black;background-color:yellow">6</span>      <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span>     <span style="text-decoration:line-through">7</span>   <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span>     <span style="color:cyan;text-decoration:italic">8</span><br/>
//! <span style="color:magenta">9</span>    <span>&nbsp;&nbsp;&nbsp;&nbsp;</span>   <span style="color:black;background-color:yellow">10</span>     <span>&nbsp;&nbsp;&nbsp;&nbsp;</span>                 <span style="text-decoration:line-through">11</span>  <span>&nbsp;&nbsp;&nbsp;</span>           <span style="color:cyan;text-decoration:italic">12</span><br/>
//! </div>


/// A struct providing optional customization of the foreground color, background
/// color, and text style of a GridPrinter column.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleOpt {
    pub fg: Option<Fg>,
    pub bg: Option<Bg>, 
    pub sgr: Option<Sgr>,
}

impl StyleOpt {

    /// Create a new StyleOpt with no specified style options. 
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the foreground color.
    pub fn fg(self, fg: Fg) -> Self {
        Self { fg: Some(fg), ..self }
    }
    
    /// Set the background color.
    pub fn bg(self, bg: Bg) -> Self {
        Self { bg: Some(bg), ..self }
    }

    /// Set the Select Graphic Rendition.
    pub fn sgr(self, sgr: Sgr) -> Self {
        Self { sgr: Some(sgr), ..self }
    }
}

impl Default for StyleOpt {
    fn default() -> StyleOpt {
        Self { fg: None, bg: None, sgr: None }
    }
}

// pub fn reset() -> &'static str {
//     "\x1b[1;0m"
// }


/// A function which will print a given &str `s` in accordance to the StylOpt `opt`.
pub fn stylize(s: &str, opt: &StyleOpt) -> String {
    format!("{fg}{bg}{sgr}{text}{reset}",
        fg = match opt.fg.as_ref() {
            None => "",
            Some(fg) => fg.escape_code(),
        },
        bg = match opt.bg.as_ref() {
            None => "",
            Some(bg) => bg.escape_code(),
        },
        sgr = match opt.sgr.as_ref() {
            None => "",
            Some(sgr) => sgr.escape_code(),
        },
        text = s,
        // Note: Using Fg::Reset vs. Bg::Reset makes no difference 
        reset = Fg::Reset.escape_code(),
    )
}

/// An enumeration of foreground color options.
#[derive(Debug, Clone, PartialEq)]
pub enum Fg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl Fg {

    /// A fucntion which will produce the ASCII escape code for a given Fg.
    pub fn escape_code(&self) -> &'static str {
        match self {
            Self::Black           => "\x1b[1;30m",
            Self::Red             => "\x1b[1;31m",
            Self::Green           => "\x1b[1;32m",
            Self::Yellow          => "\x1b[1;33m",
            Self::Blue            => "\x1b[1;34m",
            Self::Magenta         => "\x1b[1;35m",
            Self::Cyan            => "\x1b[1;36m",
            Self::White           => "\x1b[1;37m",
            Self::BrightBlack     => "\x1b[1;90m",
            Self::BrightRed       => "\x1b[1;91m",
            Self::BrightGreen     => "\x1b[1;92m",
            Self::BrightYellow    => "\x1b[1;93m",
            Self::BrightBlue      => "\x1b[1;94m",
            Self::BrightMagenta   => "\x1b[1;95m",
            Self::BrightCyan      => "\x1b[1;96m",
            Self::BrightWhite     => "\x1b[1;97m",
            Self::Reset           => "\x1b[1;0m",
        }
    }
}

/// An enumeration of background color options.
#[derive(Debug, Clone, PartialEq)]
pub enum Bg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl Bg {
    /// A fucntion which will produce the ASCII escape code for a given Bg.
    pub fn escape_code(&self) -> &'static str {
        match self {
            Self::Black          => "\x1b[1;40m",
            Self::Red            => "\x1b[1;41m",
            Self::Green          => "\x1b[1;42m",
            Self::Yellow         => "\x1b[1;43m",
            Self::Blue           => "\x1b[1;44m",
            Self::Magenta        => "\x1b[1;45m",
            Self::Cyan           => "\x1b[1;46m",
            Self::White          => "\x1b[1;47m",
            Self::BrightBlack    => "\x1b[1;100m",
            Self::BrightRed      => "\x1b[1;101m",
            Self::BrightGreen    => "\x1b[1;102m",
            Self::BrightYellow   => "\x1b[1;103m",
            Self::BrightBlue     => "\x1b[1;104m",
            Self::BrightMagenta  => "\x1b[1;105m",
            Self::BrightCyan     => "\x1b[1;106m",
            Self::BrightWhite    => "\x1b[1;107m",
            Self::Reset          => "\x1b[1;0m",
        }
    }
}

/*
 * Dont work:
 * - slowblink
 *
 * */

/// Select Graphic Renditions, i.e. an enumeration of text style options, such
/// as bold, italic, etc.
///
/// This list was obtained from the following source: 
/// * [Select Graphic Rendition](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
///
/// The total list of Select Graphic Renditions were trimmed down to those
/// styles which have general, wide support.
#[derive(Debug, Clone, PartialEq)]
pub enum Sgr {
    Bold,
    Faint,
    Italic,
    Underline,
    StrikeThrough,
    Reset,
}

impl Sgr {
    /// A fucntion which will produce the ASCII escape code for a given Sgr.
    pub fn escape_code(&self) -> &'static str {
        match self {
            Self::Bold          => "\x1b[1;1m",
            Self::Faint         => "\x1b[1;2m",
            Self::Italic        => "\x1b[1;3m",
            Self::Underline     => "\x1b[1;4m",
            Self::StrikeThrough => "\x1b[1;9m",
            Self::Reset         => "\x1b[1;0m",
        }

    }
}
