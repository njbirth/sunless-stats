use std::fmt::Display;
use dioxus::prelude::*;
use crate::data::*;
use crate::ship::{Ship, Shiptype};

#[inline_props]
pub fn equipment<'a>(cx: Scope, ship: &'a UseRef<Ship>) -> Element {
    cx.render(rsx! {
        div {
            "WORK IN PROGRESS"
        }
    })
}