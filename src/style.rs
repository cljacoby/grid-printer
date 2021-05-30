#[derive(Debug, Clone, PartialEq)]
pub struct StyleOpt {
    pub fg: Option<Fg>,
    pub bg: Option<Bg>, 
    pub style: Option<Style>,
}

impl StyleOpt {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(self, fg: Fg) -> Self {
        Self { fg: Some(fg), ..self }
    }
    
    pub fn bg(self, bg: Bg) -> Self {
        Self { bg: Some(bg), ..self }
    }

    pub fn style(self, style: Style) -> Self {
        Self { style: Some(style), ..self }
    }
}

impl Default for StyleOpt {
    fn default() -> StyleOpt {
        Self { fg: None, bg: None, style: None }
    }
}

pub fn reset() -> &'static str {
    "\x1b[1;0m"
}


pub fn stylize(s: &str, opt: &StyleOpt) -> String {
    format!("{fg}{bg}{style}{text}{reset}",
    // format!("{fg}{bg}{text}{reset}",
        fg = match opt.fg.as_ref() {
            None => "",
            Some(fg) => fg.escape_code(),
        },
        bg = match opt.bg.as_ref() {
            None => "",
            Some(bg) => bg.escape_code(),
        },
        style = match opt.style.as_ref() {
            None => "",
            Some(style) => style.escape_code(),
        },
        text = s,
        reset = reset(),
    )
}

// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
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
    // Reset,
}

impl Fg {
    fn escape_code(&self) -> &'static str {
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
        }
    }
}

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
}

impl Bg {
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
        }
    }
}

/*
 * Dont work:
 * - slowblink
 *
 * */

#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    Bold,
    Faint,
    Italic,
    Underline,
    StrikeThrough,
}

// https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
impl Style {
    pub fn escape_code(&self) -> &'static str {
        match self {
            Self::Bold          => "\x1b[1;1m",
            Self::Faint         => "\x1b[1;2m",
            Self::Italic        => "\x1b[1;3m",
            Self::Underline     => "\x1b[1;4m",
            Self::StrikeThrough => "\x1b[1;9m",
        }

    }
}
