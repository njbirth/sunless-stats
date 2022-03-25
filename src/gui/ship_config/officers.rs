use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};
use super::{select_options, stringvec};

#[inline_props]
pub fn officers<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    let select_class = "select select-bordered select-sm w-full max-w-xs";

    cx.render(rsx! {
        div {
            fieldset {
                class: "border-2 rounded p-2",
                legend { "Officers" },

                div {
                    class: "form-control w-full max-w-xs",

                    label {
                        class: "label",
                        span { class: "label-text-alt", "First Officer" }
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
                        span { class: "label-text-alt", "Chief Engineer" }
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
                        span { class: "label-text-alt", "Gunnery Officer" }
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
                        span { class: "label-text-alt", "Cook" }
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
                        span { class: "label-text-alt", "Surgeon" }
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
                        span { class: "label-text-alt", "Mascot" }
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

