use sunless_stats::gui::app;

fn main() {
    dioxus::desktop::launch_cfg(
        app,
        |c|
            c.with_window(
                |w| w.with_title("Sunless Stats")
            )
    );
}