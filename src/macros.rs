#[macro_export]
macro_rules! draw {
    ($r:expr ; @loc: $loc:ident $($tail:tt)*) => {
        draw!($r ; @loc: ($loc.0, $loc.1))
        draw!($r ; $($tail)*);
    };
    ($r:expr ; @loc: ( $($loc:tt)* ) $($tail:tt)*) => {
        draw!($r ; -> "{}", termion::cursor::Goto($($loc)*));
        draw!($r ; $($tail)*);
    };
    ($r:expr ; @bold $($tail:tt)*) => {
        draw!($r ; -> "{}", termion::style::Bold);
        draw!($r ; $($tail)*);
        draw!($r ; -> "{}", termion::style::Reset);
    };
    ($r:expr ; @style: $style:ident $($tail:tt)*) => {
        draw!($r ; -> "{}{}", $r.styles.$style.bg, $r.styles.$style.fg);
        draw!($r ; $($tail)*);
        draw!($r ; -> "{}{}", termion::color::Fg(termion::color::Reset), termion::color::Bg(termion::color::Reset));
    };
    ($r:expr ; -> $text:literal, $($tail:tt)*) => {
        write!($r.term, $text, $($tail)*)?;
    };
    ($r:expr ; <<) => {}; //End tail call if no text to write
}
