use dioxus::desktop::tao::dpi::{PhysicalSize, Size};
use sunless_stats::gui::app;

fn main() {
    dioxus::desktop::launch_cfg(
        app,
        |c|
            c.with_window(
                |w| w.with_title("Sunless Stats")
                    .with_inner_size(Size::Physical(PhysicalSize::new(1600, 900)))
            )
    );
}