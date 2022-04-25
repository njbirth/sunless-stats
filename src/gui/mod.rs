pub mod stats;
pub mod ship_config;
pub mod modal;

use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};

pub fn app(cx: Scope) -> Element {
    let ship = use_ref(&cx, || Ship::default());

    cx.render(rsx! {
        style { [include_str!("../../css/tailwind.min.css")] }
        style { [include_str!("../../css/daisyui.css")] }
        style { [include_str!("../../css/styles.css")] }

        div {
            padding: "20px",
            class: "flex flex-col",
            width: "35%",

            div {
                ship_config::ship_config { ship: ship },
            }

            div {
                stats::stats { ship: ship }
            }

            modal::modal {  }
        }
    })
}