use dioxus::prelude::*;
use crate::ship::Ship;
use crate::item::{Item, Slot};

pub static MODAL: Atom<Option<Modal>> = |_| None;

pub struct Modal {
    pub ship: UseRef<Ship>,
    pub selected: Slot
}

#[inline_props]
pub fn selector_set<'a>(cx: Scope, ship: &'a UseRef<Ship>, slots: Vec<Slot>) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-3 justify-items-center",

            slots.iter().map(|slot| { rsx! {
                self::selector {
                    slot: slot.clone(),
                    ship: ship
                }
            }})
        }
    })
}

#[inline_props]
pub fn selector<'a>(cx: Scope, slot: Slot, ship: &'a UseRef<Ship>) -> Element {
    let set_modal = use_set(&cx, MODAL);

    let img = if let Some(item) = ship.with(|s| s.item(&slot)) {
        item.img
    } else { "none.png".to_string() };

    cx.render(rsx! {
        div {
            class: "w-40 pb-4",
            label {
                class: "label",
                span {
                    class: "label-text m-auto",
                    b { "{slot}" }
                }
            }
            img {
                class: "w-24 m-auto",
                border: "3px solid #000",
                src: format_args!("data/img/{}", img),
                onclick: |_| set_modal(Some(Modal { ship: ship.clone().clone(), selected: slot.clone() }))
            }
        }
    })
}

pub fn modal(cx: Scope) -> Element {
    let modal = use_read(&cx, MODAL);
    let set_modal = use_set(&cx, MODAL);

    if modal.is_none() { return None; }
    let modal = modal.as_ref().unwrap();

    let ship = &modal.ship;
    let slot = &modal.selected;

    let elements = crate::data::items(slot);
    let current = ship.with(|s| s.item(slot));

    cx.render(rsx! {
        div {
            class: "modal modal-open",
            div {
                class:"modal-box",

                div {
                    ul {
                        li {
                            onclick: |_| {
                                set_modal(None);
                                ship.with_mut(|s| s.set_item(slot, None));
                            },

                            button {
                                font_weight: format_args!("{}", if current == Option::<Item>::None { "bold" } else { "normal" }),

                                "None"
                            }
                        },

                        elements.iter().map(|elem| {
                            let is_current = current.as_ref() == Some(elem);
                            // I have no idea why this is working while all the other stuff I tried did not.
                            let elem_clone = elem.clone();

                            rsx! {
                                li {
                                    onclick: move |_| {
                                        set_modal(None);
                                        ship.write().set_item(slot, Some(elem_clone.clone()));
                                    },
                                    button {
                                        font_weight: format_args!("{}", if is_current { "bold" } else { "normal" }),

                                        "{elem}"
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    })
}