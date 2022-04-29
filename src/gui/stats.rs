use dioxus::prelude::*;
use crate::ship::Ship;

#[inline_props]
pub fn stats<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    let ship = ship.read();

    let weight = ship.shiptype.stats.weight;
    let engine_power = ship.engine_power();
    let quarters = ship.quarters();

    cx.render(rsx! {
        div {
            class: "border-2 mt-4 p-2",

            table {
                class: "table table-compact",
                tbody {
                    tr { td { "Weight:" }, td { "{weight}" } },
                    tr { td { "Engine Power:" }, td { "{engine_power}" } },
                    tr { td { "Quarters:" }, td { "{quarters}" } }
                }
            }
        }
    })
}