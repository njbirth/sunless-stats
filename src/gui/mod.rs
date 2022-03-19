use iced::{Alignment, Column, Container, Element, Length, pick_list, PickList, Row, Rule, Sandbox, Text};

use crate::ship::{Ship, Shiptype};
use crate::data::*;
use crate::equipment::{Equipment, Slot};
use crate::officers::{Officer, Position};

#[derive(Default)]
pub struct SunlessStats {
    ship: Ship,
    pick_list_shiptype: pick_list::State<Shiptype>,
    pick_list_first_officer: pick_list::State<Officer>
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeShiptype(Shiptype),
    ChangeEquipment(Equipment, Slot),
    ChangeOfficer(Officer, Position),
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
            Message::ChangeOfficer(_, _) => {

            },
            Message::Nothing => { }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(5)
            .width(Length::Units(500))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(Text::new("Ship").width(Length::FillPortion(2)))
                    .push(
                        PickList::new(
                            &mut self.pick_list_shiptype,
                            &*SHIPTYPES,
                            Some(self.ship.shiptype.clone()),
                            |shiptype: Shiptype| { Message::ChangeShiptype(shiptype) }
                        )
                            .width(Length::FillPortion(5))
                    )
            )
            .push(Rule::horizontal(20))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(Text::new("First Officer").width(Length::FillPortion(2)))
                    .push(
                        PickList::new(
                            &mut self.pick_list_first_officer,
                            &*OFFICERS,
                            self.ship.officers.first_officer.clone(),
                            |officer: Officer| { Message::ChangeOfficer(officer, Position::FirstOfficer) }
                        ).width(Length::FillPortion(5)).placeholder("No ship")
                    )
            )
            .push(Rule::horizontal(20))
            .push(
                Text::new(format!("Weight: {}", self.ship.shiptype.stats.weight))
            )
            .into()
    }
}