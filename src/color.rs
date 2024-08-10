#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Hue {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    Gray,
    Grey,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Background {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    Gray,
    Grey,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Formatting {
    Reset,
    Bold,
    Dim,
    Strikethrough,
    Italic,
    Underline,
    Inverted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Text {
    pub fg: Option<Hue>,
    pub decor: Option<Formatting>,
    pub bg: Option<Background>,
}

impl Text {
    pub fn new() -> Self {
        Self {
            fg: Option::None,
            bg: Option::None,
            decor: Option::None,
        }
    }

    pub fn black(mut self) -> Self {
        self.fg = Option::Some(Hue::Black);
        self
    }
    pub fn red(mut self) -> Self {
        self.fg = Option::Some(Hue::Red);
        self
    }
    pub fn green(mut self) -> Self {
        self.fg = Option::Some(Hue::Green);
        self
    }
    pub fn yellow(mut self) -> Self {
        self.fg = Option::Some(Hue::Yellow);
        self
    }
    pub fn blue(mut self) -> Self {
        self.fg = Option::Some(Hue::Blue);
        self
    }
    pub fn magenta(mut self) -> Self {
        self.fg = Option::Some(Hue::Magenta);
        self
    }
    pub fn cyan(mut self) -> Self {
        self.fg = Option::Some(Hue::Cyan);
        self
    }
    pub fn white(mut self) -> Self {
        self.fg = Option::Some(Hue::White);
        self
    }
    pub fn brightblack(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightBlack);
        self
    }
    pub fn gray(mut self) -> Self {
        self.fg = Option::Some(Hue::Gray);
        self
    }
    pub fn grey(mut self) -> Self {
        self.fg = Option::Some(Hue::Grey);
        self
    }
    pub fn bright_red(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightRed);
        self
    }
    pub fn bright_green(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightGreen);
        self
    }
    pub fn bright_yellow(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightYellow);
        self
    }
    pub fn bright_blue(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightBlue);
        self
    }
    pub fn bright_magenta(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightMagenta);
        self
    }
    pub fn bright_cyan(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightCyan);
        self
    }
    pub fn bright_white(mut self) -> Self {
        self.fg = Option::Some(Hue::BrightWhite);
        self
    }

    pub fn bg_black(mut self) -> Self {
        self.bg = Option::Some(Background::Black);
        self
    }
    pub fn bg_red(mut self) -> Self {
        self.bg = Option::Some(Background::Red);
        self
    }
    pub fn bg_green(mut self) -> Self {
        self.bg = Option::Some(Background::Green);
        self
    }
    pub fn bg_yellow(mut self) -> Self {
        self.bg = Option::Some(Background::Yellow);
        self
    }
    pub fn bg_blue(mut self) -> Self {
        self.bg = Option::Some(Background::Blue);
        self
    }
    pub fn bg_magenta(mut self) -> Self {
        self.bg = Option::Some(Background::Magenta);
        self
    }
    pub fn bg_cyan(mut self) -> Self {
        self.bg = Option::Some(Background::Cyan);
        self
    }
    pub fn bg_white(mut self) -> Self {
        self.bg = Option::Some(Background::White);
        self
    }
    pub fn bg_brightblack(mut self) -> Self {
        self.bg = Option::Some(Background::BrightBlack);
        self
    }
    pub fn bg_gray(mut self) -> Self {
        self.bg = Option::Some(Background::Gray);
        self
    }
    pub fn bg_grey(mut self) -> Self {
        self.bg = Option::Some(Background::Grey);
        self
    }
    pub fn bg_bright_red(mut self) -> Self {
        self.bg = Option::Some(Background::BrightRed);
        self
    }
    pub fn bg_bright_green(mut self) -> Self {
        self.bg = Option::Some(Background::BrightGreen);
        self
    }
    pub fn bg_bright_yellow(mut self) -> Self {
        self.bg = Option::Some(Background::BrightYellow);
        self
    }
    pub fn bg_bright_blue(mut self) -> Self {
        self.bg = Option::Some(Background::BrightBlue);
        self
    }
    pub fn bg_bright_magenta(mut self) -> Self {
        self.bg = Option::Some(Background::BrightMagenta);
        self
    }
    pub fn bg_bright_cyan(mut self) -> Self {
        self.bg = Option::Some(Background::BrightCyan);
        self
    }
    pub fn bg_bright_white(mut self) -> Self {
        self.bg = Option::Some(Background::BrightWhite);
        self
    }

    pub fn reset(mut self) -> Self {
        self.decor = Option::Some(Formatting::Reset);
        self
    }
    pub fn bold(mut self) -> Self {
        self.decor = Option::Some(Formatting::Bold);
        self
    }
    pub fn underline(mut self) -> Self {
        self.decor = Option::Some(Formatting::Underline);
        self
    }
    pub fn inverted(mut self) -> Self {
        self.decor = Option::Some(Formatting::Inverted);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.decor = Option::Some(Formatting::Strikethrough);
        self
    }

    pub fn build(self) -> String {
        if self.decor == Some(Formatting::Reset) {
            return String::from("\x1b[0m");
        }
        String::from(format!(
            "\x1b[{}{}{}m",
            match self.decor {
                Some(i) => match i {
                    Formatting::Bold => "1;",
                    Formatting::Dim => "2;",
                    Formatting::Italic => "3;",
                    Formatting::Underline => "4;",
                    Formatting::Strikethrough => "9;",
                    Formatting::Inverted => "7;",
                    Formatting::Reset => "0;",
                },
                None => "",
            },
            match self.fg {
                Some(i) => match i {
                    Hue::Black => "30",
                    Hue::Red => "31",
                    Hue::Green => "32",
                    Hue::Yellow => "33",
                    Hue::Blue => "34",
                    Hue::Magenta => "35",
                    Hue::Cyan => "36",
                    Hue::White => "37",
                    Hue::BrightBlack => "90",
                    Hue::Gray => "90",
                    Hue::Grey => "90",
                    Hue::BrightRed => "91",
                    Hue::BrightGreen => "92",
                    Hue::BrightYellow => "93",
                    Hue::BrightBlue => "94",
                    Hue::BrightMagenta => "95",
                    Hue::BrightCyan => "96",
                    Hue::BrightWhite => "97",
                },
                None => "",
            },
            match self.bg {
                Some(i) => match i {
                    Background::Black => "40",
                    Background::Red => "41",
                    Background::Green => "42",
                    Background::Yellow => "43",
                    Background::Blue => "44",
                    Background::Magenta => "45",
                    Background::Cyan => "46",
                    Background::White => "47",
                    Background::BrightBlack => "100",
                    Background::Gray => "100",
                    Background::Grey => "100",
                    Background::BrightRed => "101",
                    Background::BrightGreen => "102",
                    Background::BrightYellow => "103",
                    Background::BrightBlue => "104",
                    Background::BrightMagenta => "105",
                    Background::BrightCyan => "106",
                    Background::BrightWhite => "107",
                },
                None => "",
            }
        ))
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.build()
    }
}

const MCTOTEXT: std::sync::LazyLock<std::collections::HashMap<String, Text>> =
    std::sync::LazyLock::new(|| {
        let mut output = std::collections::HashMap::new();
        output.insert("0".to_string(), Text::new().black());
        output.insert("1".to_string(), Text::new().blue());
        output.insert("2".to_string(), Text::new().green());
        output.insert("3".to_string(), Text::new().cyan());
        output.insert("4".to_string(), Text::new().red());
        output.insert("5".to_string(), Text::new().magenta());
        output.insert("6".to_string(), Text::new().yellow());
        output.insert("7".to_string(), Text::new().gray());
        output.insert("8".to_string(), Text::new().gray());
        output.insert("9".to_string(), Text::new().blue());
        output.insert("a".to_string(), Text::new().bright_green());
        output.insert("b".to_string(), Text::new().bright_cyan());
        output.insert("c".to_string(), Text::new().bright_red());
        output.insert("d".to_string(), Text::new().bright_magenta());
        output.insert("e".to_string(), Text::new().bright_yellow());
        output.insert("f".to_string(), Text::new().white());
        output.insert("r".to_string(), Text::new().reset());
        output.insert("l".to_string(), Text::new().bold());
        output.insert("n".to_string(), Text::new().underline());
        output.insert("m".to_string(), Text::new().strikethrough());
        output
    });

pub fn to_color(code: String) -> Text {
    MCTOTEXT.get_key_value(&code).unwrap().1.to_owned()
}

pub fn mccolor(text: &str) -> String {
    mccolor_esc(text, "&")
}

pub fn mccolor_esc(text: &str, esc_char: &str) -> String {
    let mut output: String = String::new();
    let mut is_escape_seq: bool = false;
    for c in text.chars() {
        if c.to_string() == esc_char {
            is_escape_seq = true;
            continue;
        }

        if is_escape_seq {
            output.push_str(&to_color(c.to_string()).build());
            is_escape_seq = false;
        } else {
            output.push(c);
        }
    }

    output
}
