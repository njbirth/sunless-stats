use dioxus::prelude::*;
use crate::equipment::Slot;
use crate::officers::Position;
use crate::ship::Ship;

pub static MODAL: Atom<Option<Modal>> = |_| None;

pub struct Modal {
    pub ship: UseRef<Ship>,
    pub selected: Selected
}

pub enum Selected {
    Equipment(Slot),
    Officer(Position)
}

pub fn modal(cx: Scope) -> Element {
    let modal = use_read(&cx, MODAL);
    let set_modal = use_set(&cx, MODAL);

    if modal.is_none() { return None; }
    let modal = modal.as_ref().unwrap();

    cx.render(rsx! {
        div {
            class: "modal modal-open",
            div {
                class:"modal-box",

                div { "Officer modal" },
                button {
                    onclick: |_| set_modal(None),
                    "Close modal"
                }
            }
        }
    })
}