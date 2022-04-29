use dioxus::prelude::*;
use crate::ship::Ship;
use super::selector;
use crate::item::Slot;
use crate::equipment::EquipmentType;

#[inline_props]
pub fn equipment<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-3 justify-items-center",

            selector::selector {
                slot: Slot::Equipment(EquipmentType::Forward),
                ship: ship,
            },
            selector::selector {
                slot: Slot::Equipment(EquipmentType::Deck),
                ship: ship
            },
            selector::selector {
                slot: Slot::Equipment(EquipmentType::Aft),
                ship: ship
            },
            selector::selector {
                slot: Slot::Equipment(EquipmentType::Bridge),
                ship: ship
            },
            selector::selector {
                slot: Slot::Equipment(EquipmentType::Auxiliary),
                ship: ship
            },
            selector::selector {
                slot: Slot::Equipment(EquipmentType::Engine),
                ship: ship
            },
        }
    })
}