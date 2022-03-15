use iced::{Column, Element, pick_list, PickList, Sandbox};

use crate::ship::{Ship, Shiptype};
use crate::data::*;
use crate::equipment::{Equipment, Slot};

#[derive(Default)]
pub struct SunlessStats {
    ship: Ship,
    pick_list: pick_list::State<Shiptype>
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeShiptype(Shiptype),
    ChangeEquipment(Equipment, Slot),
    Nothing
}

impl Sandbox for SunlessStats {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Sunless Stats")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeShiptype(shiptype) => {
                self.ship.shiptype = shiptype;
            },
            Message::ChangeEquipment(_, _) => {

            },
            Message::Nothing => { }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                PickList::new(
                    &mut self.pick_list,
                    &*SHIPTYPES,
                    Some(self.ship.shiptype.clone()),
                    |shiptype: Shiptype| { Message::ChangeShiptype(shiptype) }
                )
            )
            .into()
    }
}