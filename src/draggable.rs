use iced::{Align, Column, Container, Element, Length, Sandbox, Text};

use drag::Square;

#[derive(Default)]
pub struct Draggable {
    drag: Square,
}

impl Sandbox for Draggable {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Draggable")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Draggable Square").size(50))
            .push(self.drag.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod drag {
    use iced::{
        canvas::event::{self, Event},
        canvas::Cache,
        canvas::{Cursor, Geometry, Path, Program, Stroke},
        mouse, Canvas, Color, Element, Length, Point, Rectangle, Size,
    };

    #[derive(Default)]
    pub struct Square {
        cache: Cache,
        top_left: Point,
        side: f32,
        grabbed: bool,
    }

    impl Square {
        pub fn view<'a>(&'a mut self) -> Element<'a, ()> {
            Canvas::new(Square {
                side: 50.0,
                ..Square::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }

    impl Program<()> for Square {
        fn update(
            &mut self,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<()>) {
            let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
                position
            } else {
                return (event::Status::Ignored, None);
            };

            let square = Rectangle::new(self.top_left, Size::new(self.side, self.side));

            if square.contains(cursor_position) {
                match event {
                    Event::Mouse(mouse_event) => match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            self.grabbed = true;
                            return (event::Status::Captured, None);
                        }
                        mouse::Event::ButtonReleased(mouse::Button::Left) => {
                            self.grabbed = false;
                            return (event::Status::Captured, None);
                        }
                        _ => return (event::Status::Ignored, None),
                    },
                    _ => return (event::Status::Ignored, None),
                }
            } else {
                match event {
                    Event::Mouse(mouse_event) => match mouse_event {
                        mouse::Event::CursorMoved { .. } => {
                            if self.grabbed {
                                self.top_left = cursor_position;
                                self.cache.clear();
                            }
                            return (event::Status::Captured, None);
                        }
                        _ => return (event::Status::Ignored, None),
                    },
                    _ => return (event::Status::Ignored, None),
                }
            };
        }

        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            let square = self.cache.draw(bounds.size(), |frame| {
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );
                let square = Path::rectangle(self.top_left, Size::new(self.side, self.side));
                frame.fill(&square, Color::BLACK);
            });

            vec![square]
        }
    }
}
