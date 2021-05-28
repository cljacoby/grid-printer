#[derive(Debug, Clone, PartialEq)]
pub struct StyleOpt {
    pub fg: Option<FgColor>,
    pub bg: Option<BgColor>, 
    // pub style: Option<Style>,
}

pub fn reset() -> &'static str {
    "\x1b[1;0m"
}

pub fn colorize(s: &String, fg: FgColor) -> String {
    format!("{}{}{}", fg.escape_code(), s, reset())
}

// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
#[derive(Debug, Clone, PartialEq)]
pub enum FgColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Reset,
}

impl FgColor {
    fn escape_code(&self) -> &'static str {
        match self {
            FgColor::Black     => "\x1b[1;30m",
            FgColor::Red       => "\x1b[1;31m",
            FgColor::Green     => "\x1b[1;32m",
            FgColor::Yellow    => "\x1b[1;33m",
            FgColor::Blue      => "\x1b[1;34m",
            FgColor::Magenta   => "\x1b[1;35m",
            FgColor::Cyan      => "\x1b[1;36m",
            FgColor::White     => "\x1b[1;37m",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BgColor {
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
}

impl BgColor {
    pub fn escape_code(&self) -> &'static str {
        match self {
            BgColor::Black          => "\x1b[1;40m",
            BgColor::Red            => "\x1b[1;41m",
            BgColor::Green          => "\x1b[1;42m",
            BgColor::Yellow         => "\x1b[1;43m",
            BgColor::Blue           => "\x1b[1;44m",
            BgColor::Magenta        => "\x1b[1;45m",
            BgColor::Cyan           => "\x1b[1;46m",
            BgColor::White          => "\x1b[1;47m",
            BgColor::BrightBlack    => "\x1b[1;40;1m",
            BgColor::BrightRed      => "\x1b[1;41;1m",
            BgColor::BrightGreen    => "\x1b[1;42;1m",
            BgColor::BrightYellow   => "\x1b[1;43;1m",
            BgColor::BrightBlue     => "\x1b[1;44;1m",
            BgColor::BrightMagenta  => "\x1b[1;45;1m",
            BgColor::BrightCyan     => "\x1b[1;46;1m",
            BgColor::BrightWhite    => "\x1b[1;47;1m",
        }
    }
}
