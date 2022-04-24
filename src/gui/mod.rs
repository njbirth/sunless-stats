pub mod stats;
pub mod ship_config;

use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};

pub fn app(cx: Scope) -> Element {
    let ship = use_ref(&cx, || Ship::default());

    cx.render(rsx! {
        style { [include_str!("../../css/tailwind.min.css")] }
        style { [include_str!("../../css/daisyui.css")] }

        div {
            padding: "20px",
            class: "flex flex-row",

            div {
                class: "basis-1/2",
                ship_config::ship_config { ship: ship },
            }

            /*div {
                class: "basis-1/2",
                stats::stats { ship: ship }
            }*/
        }
    })
}