#[macro_export]
macro_rules! draw {
    ($r:ident @loc: $loc:ident $($tail:tt)*) => {
        draw!($r @loc: ($loc.0, $loc.1))
        draw!($r $($tail)*);
    };
    ($r:ident @loc: ( $($loc:tt)* ) $($tail:tt)*) => {
        draw!($r -> "{}", termion::cursor::Goto($($loc)*));
        draw!($r $($tail)*);
    };
    ($r:ident @bold $($tail:tt)*) => {
        draw!($r -> "{}", termion::style::Bold);
        draw!($r $($tail)*);
        draw!($r -> "{}", termion::style::Reset);
    };
    ($r:ident @style: $style:ident $($tail:tt)*) => {
        draw!($r -> "{}{}", $r.styles.$style.bg, $r.styles.$style.fg);
        draw!($r $($tail)*);
        draw!($r -> "{}{}", termion::color::Fg(termion::color::Reset), termion::color::Bg(termion::color::Reset));
    };
    ($r:ident -> $text:literal, $($tail:tt)*) => {
        write!($r.term, $text, $($tail)*)?;
    };
    ($r:ident <<) => {}; //End tail call if no text to write
}
