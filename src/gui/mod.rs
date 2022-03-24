pub mod stats;

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
                class: "basis-1/3",
                select {
                    class: "select select-bordered select-sm w-full max-w-xs",
                    onchange: |evt| {
                        ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                    },
                    self::select_options { arr: stringvec(&SHIPTYPES) }
                },
            },
            div {
                class: "basis-1/3",
                stats::stats { ship: ship.clone() }
            }
        }
    })
}

#[inline_props]
fn select_options(cx: Scope, arr: Vec<String>) -> Element {
    cx.render(rsx! {
        arr.iter().enumerate().map(|(idx, ship)| {
            rsx!{
                option {
                    value: "{idx}",
                    [format_args!("{ship}")]
                }
            }
        })
    })
}

fn stringvec(v: &Vec<impl Display>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}