use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};
use super::{select_options, stringvec};

#[inline_props]
pub fn equipment<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    let select_class = "select select-bordered select-sm w-full max-w-xs";

    cx.render(rsx! {
        div {
            fieldset {
                class: "border-2 rounded p-2",
                legend { "Equipment" },

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Forward" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Deck" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Bridge" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Auxiliary" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Engines" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "Aft" }
                    }

                    select {
                        class: "{select_class}",
                        onchange: |evt| {
                            ship.with_mut(|s| s.shiptype = SHIPTYPES[evt.data.value.parse::<usize>().unwrap()].clone())
                        },
                        self::select_options { arr: stringvec(&OFFICERS) }
                    },
                }
            }
        },
    })
}

