use owo_colors::colors::*;
use owo_colors::OwoColorize;

pub fn direction_str(server_bounded: bool) -> String {
    let s = "S".fg::<CustomColor<255, 164, 0>>();
    let c = "C".bright_blue();

    if server_bounded {
        format!("[{} -> {}]", c, s).bold().to_string()
    } else {
        format!("[{} -> {}]", s, c).bold().to_string()
    }
}
