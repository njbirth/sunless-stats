use dioxus::prelude::*;
use crate::ship::Ship;
use crate::officers::Position;
use super::selector;
use crate::item::Slot;

#[inline_props]
pub fn officers<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-3 justify-items-center",

            selector::selector {
                slot: Slot::Officer(Position::FirstOfficer),
                ship: ship,
            },
            selector::selector {
                slot: Slot::Officer(Position::Engineer),
                ship: ship
            },
            selector::selector {
                slot: Slot::Officer(Position::Gunner),
                ship: ship
            },
            selector::selector {
                slot: Slot::Officer(Position::Cook),
                ship: ship
            },
            selector::selector {
                slot: Slot::Officer(Position::Surgeon),
                ship: ship
            },
            selector::selector {
                slot: Slot::Officer(Position::Mascot),
                ship: ship
            },
        }
    })
}