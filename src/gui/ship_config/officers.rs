use dioxus::prelude::*;
use crate::data::*;
use crate::ship::Ship;
use crate::officers::Position;
use crate::gui::modal::{Modal, MODAL, Selected};

#[inline_props]
pub fn officers<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-3 justify-items-center",

            self::officer{
                position: Position::FirstOfficer,
                ship: ship,
            },
            self::officer {
                position: Position::Engineer,
                ship: ship
            },
            self::officer {
                position: Position::Gunner,
                ship: ship
            },
            self::officer {
                position: Position::Cook,
                ship: ship
            },
            self::officer {
                position: Position::Surgeon,
                ship: ship
            },
            self::officer {
                position: Position::Mascot,
                ship: ship
            },
        }
    })
}

#[inline_props]
fn officer<'a>(cx: Scope, position: Position, ship: &'a UseRef<Ship>) -> Element {
    let set_modal = use_set(&cx, MODAL);

    cx.render(rsx! {
        div {
            class: "w-40 pb-4",
            label {
                class: "label",
                span {
                    class: "label-text m-auto",
                    b { "{position}" }
                }
            }
            img {
                class: "w-24 m-auto",
                src: "images/SS_costermonger2gaz.png",
                onclick: |_| set_modal(Some(Modal { ship: ship.clone().clone(), selected: Selected::Officer(Position::FirstOfficer) }))
            }
        }
    })
}