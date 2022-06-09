use dioxus::prelude::*;
use crate::ship::Ship;
use crate::item::{Item, Slot};

pub static MODAL: Atom<Option<Modal>> = |_| None;

pub struct Modal {
    pub ship: UseRef<Ship>,
    pub slot: Slot
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

    let locked = ship.with(|s| s.shiptype.locked_slots.contains(&slot));

    let img = match ship.with(|s| s.item(&slot)) {
        Some(item) => item.img,
        None => if locked { "locked.png".to_string() } else { "none.png".to_string() }
    };

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
                onclick: move |_| {
                    if !locked {
                        set_modal(Some(Modal {
                            ship: ship.clone().clone(),
                            slot: slot.clone()
                        }))
                    }
                }
            }
        }
    })
}

pub fn modal(cx: Scope) -> Element {
    // The modal is only rendered if there is one. Therefore we can safely unwrap this.
    let modal = use_read(&cx, MODAL).as_ref().unwrap();
    let set_modal = use_set(&cx, MODAL);

    let ship = &modal.ship;
    let slot = &modal.slot;

    let elements = crate::data::items(slot);

    let selected = use_ref(&cx, || ship.with(|s| s.item(slot)));
    let current = ship.with(|s| s.item(slot));

    cx.render(rsx! {
        div {
            class: "modal modal-open",
            onclick: move |_| {
                set_modal(None);
                selected.set(None);
            },
            div {
                class: "modal-box flex flex-row w-11/12 max-w-5xl h-2/3",
                max_width: "60%",
                onclick: |evt| { evt.cancel_bubble(); },

                div {
                    class: "border-r-2 h-full w-2/3",

                    h3 {
                        class: "font-medium leading-tight text-3xl mt-0 mb-4",
                        style: "text-align: center",
                        [format_args!("{}", slot)]
                    }

                    ul {
                        li {
                            onclick: |_| {
                                set_modal(None);
                                ship.with_mut(|s| s.set_item(slot, None));
                            },

                            button {
                                font_weight: format_args!("{}", if current == Option::<Item>::None { "bold" } else { "normal" }),
                                font_style: format_args!("{}", if selected.with(|s| s.is_none()) { "italic" } else { "normal" }),
                                onmouseover: |_| { selected.set(None); },

                                "None"
                            }
                        },

                        elements.iter().map(|elem| {
                            let is_current = current.as_ref() == Some(elem);
                            let is_selected = selected.read().as_ref() == Some(elem);
                            // I have no idea why this is working while all the other stuff I tried did not.
                            let elem_clone = elem.clone();

                            rsx! {
                                li {
                                    onclick: move |_| {
                                        set_modal(None);
                                        ship.write().set_item(slot, Some(elem_clone.clone()));
                                    },
                                    onmouseover: |_| {
                                        selected.set(Some(elem_clone.clone()));
                                    },
                                    button {
                                        font_weight: format_args!("{}", if is_current { "bold" } else { "normal" }),
                                        font_style: format_args!("{}", if is_selected { "italic" } else { "normal" }),

                                        "{elem}"
                                    }
                                }
                            }
                        })
                    }
                },
                self::info_box { item: selected }
            }
        }
    })
}

#[inline_props]
pub fn info_box<'a>(cx: Scope, item: &'a UseRef<Option<Item>>) -> Element {
    let item = item.read();
    //if item.is_none() { return None; }
    //let item = item.as_ref().unwrap();

    let (title, img) = match item.as_ref() {
        Some(i) => (i.name.clone(), i.img.clone()),
        None => ("None".to_string(), "none.png".to_string())
    };

    cx.render(rsx! {
        div {
            class: "p-4 flex flex-col w-1/3",

            h3 {
                class: "font-medium leading-tight text-m h-12",
                style: "text-align: center",
                [title]
            },

            img {
                class: "w-24 mx-auto my-4",
                border: "3px solid #000",
                src: format_args!("data/img/{}", img),
            },

            hr { class: "mb-4" },

            [if let Some(item) = item.as_ref() { rsx! {
                div {
                    table {
                        class: "table table-compact",
                        tbody {
                            tr { td { "Iron: " } td { [format_args!("{}", item.skills.iron)] } }
                            tr { td { "Mirrors: " } td { [format_args!("{}", item.skills.mirrors)] } }
                            tr { td { "Veils: " } td { [format_args!("{}", item.skills.veils)] } }
                            tr { td { "Pages: " } td { [format_args!("{}", item.skills.pages)] } }
                            tr { td { "Hearts: " } td { [format_args!("{}", item.skills.hearts)] } }
                        }
                    }
                }
            }} else { rsx! { div { } } }]
        }
    })
}