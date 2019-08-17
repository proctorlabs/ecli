use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StyleConfig {
    pub default: Style,
    pub selected: Style,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            default: Style {
                fg: Color::Red,
                bg: Color::None,
            },
            selected: Style {
                fg: Color::Green,
                bg: Color::None,
            },
        }
    }
}

macro_rules! impl_colors {
    ($($name:ident : $tname:ident),*) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum Color {
            $($name),*
        }

        impl fmt::Display for Color {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Color::$name => color::$tname.write_fg(f)),*
                }
            }
        }
    };
}

impl_colors! {
    None:Reset,
    Black:Black,
    Red:Red,
    Green:Green,
    Yellow:Yellow,
    Blue:Blue,
    Magenta:Magenta,
    Cyan:Cyan,
    White:White,
    LightBlack:LightBlack,
    LightRed:LightRed,
    LightGreen:LightGreen,
    LightYellow:LightYellow,
    LightBlue:LightBlue,
    LightMagenta:LightMagenta,
    LightCyan:LightCyan,
    LightWhite:LightWhite
}
