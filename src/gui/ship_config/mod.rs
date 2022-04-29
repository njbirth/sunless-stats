pub mod officers;
pub mod equipment;
pub mod selector;

use dioxus::prelude::*;
use crate::data::*;
use crate::ship::Ship;

#[inline_props]
pub fn ship_config<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    let active_tab = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            class: "flex flex-col",

            div {
                class: "border-2 mt-2 p-2",

                div {
                    class: "flex flex-row justify-center mb-8",

                    select {
                        class: "select select-bordered select-sm w-full max-w-xs",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },

                        SHIPTYPES.iter().enumerate().map(|(idx, ship)| {
                            rsx! { option { value: "{idx}", "{ship}" } }
                        })
                    },
                }

                div {
                    class: "tabs border-b",
                    a {
                        class: format_args!("tab {}", if *active_tab.current() == 0 { "tab-active" } else { "" }),
                        onclick: |_| active_tab.set(0),
                        "Equipment"
                    },
                    a {
                        class: format_args!("tab {}", if *active_tab.current() == 1 { "tab-active" } else { "" }),
                        onclick: |_| active_tab.set(1),
                        "Officers"
                    }
                }

                div {
                    class: "m-2",

                    match *active_tab.current() {
                        0 => rsx! { equipment::equipment { ship: ship } },
                        1 => rsx! { officers::officers { ship: ship } },
                        _ => unreachable!()
                    }
                },
            }
        },
    })
}