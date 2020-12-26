use iced::{button, Align, Button, Column, Container, Element, Length, Row, Rule, Sandbox, Text};

#[derive(Default)]
pub struct SideDrawer {
    open_drawer: button::State,
    is_drawer_open: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenDrawer,
}

impl Sandbox for SideDrawer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Side Drawer")
    }

    fn update(&mut self, _message: Message) {
        self.is_drawer_open = !self.is_drawer_open;
    }

    fn view(&mut self) -> Element<Message> {
        let body: Element<_> = Column::new()
            .padding(20)
            .spacing(20)
            .width(Length::FillPortion(3))
            .align_items(Align::Center)
            .push(Text::new("Side Drawer").size(50))
            .push(
                Button::new(&mut self.open_drawer, Text::new("Settings"))
                    .on_press(Message::OpenDrawer),
            )
            .into();

        let settings: Element<_> = Column::new()
            .padding(20)
            .spacing(20)
            .width(Length::FillPortion(1))
            .push(Text::new("Settings").size(25))
            .into();

        let content = Row::with_children(if self.is_drawer_open {
            vec![body, Rule::vertical(0).into(), settings]
        } else {
            vec![body]
        });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
