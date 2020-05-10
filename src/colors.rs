use termion::color::*;

pub const TIME_COLOR: Fg<LightBlack> = Fg(LightBlack);
pub const WARN_COLOR: Fg<Yellow> = Fg(Yellow);
pub const ERROR_COLOR: Fg<Red> = Fg(Red);
pub const INFO_COLOR: Fg<Reset> = Fg(Reset);
pub const OTHER_COLOR: Fg<Green> = Fg(Green);
pub const MESSAGE_COLOR: Fg<Reset> = Fg(Reset);
pub const STACKTRACE_COLOR: Fg<Reset> = Fg(Reset);
