mod officers;
mod equipment;

use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};

#[inline_props]
pub fn ship_config<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    let select_class = "select select-bordered select-sm w-full max-w-xs";

    cx.render(rsx! {
        div {
            class: "flex flex-col",

            div {
                class: "flex flex-row justify-center",

                select {
                    class: "{select_class}",
                    onchange: |evt| {
                        ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                    },
                    self::select_options { arr: stringvec(&SHIPTYPES) }
                },
            }

            div {
                class: "flex flex-row",

                div {
                    class: "m-2",

                    equipment::equipment { ship: ship },
                },

                div {
                    class: "m-2",

                    officers::officers { ship: ship },
                }
            }
        },
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