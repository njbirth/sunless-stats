use iced::{Sandbox, Settings};
use sunless_stats::data::*;

fn main() {
    dbg!(&*SHIPTYPES);
    dbg!(&*OFFICERS);
    dbg!(&*EQUIPMENT);

    sunless_stats::gui::SunlessStats::run(Settings::default()).ok();
}