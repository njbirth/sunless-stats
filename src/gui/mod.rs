use iced::{Column, Element, Sandbox, Text};

#[derive(Default)]
pub struct SunlessStats {

}

#[derive(Debug, Clone, Copy)]
pub enum Message {
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

    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Hello World"))
            .into()
    }
}