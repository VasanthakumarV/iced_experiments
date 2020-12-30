use iced::{Align, Column, Container, Element, Length, Sandbox, Text};

use area::Area;

#[derive(Default)]
pub struct Pan {
    area: Area,
}

impl Sandbox for Pan {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Pan Area")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Pan Area").size(50))
            .push(self.area.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod area {
    use iced::{
        canvas::{event, Event},
        canvas::{Cache, Cursor, Geometry, Path, Program, Stroke},
        mouse, Canvas, Color, Element, Length, Point, Rectangle, Size, Vector,
    };

    #[derive(Default)]
    pub struct Area {
        cache: Cache,
        pan_start: Option<Point>,
        translation: Vector,
    }

    impl Area {
        pub fn view<'a>(&self) -> Element<'a, ()> {
            Canvas::new(Self::default())
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    impl Program<()> for Area {
        fn update(
            &mut self,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<()>) {
            let event = match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(button) => match button {
                        mouse::Button::Left => {
                            self.pan_start = cursor.position();
                            event::Status::Captured
                        }
                        _ => event::Status::Ignored,
                    },
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(start) = self.pan_start {
                            if let Some(position) = cursor.position() {
                                self.translation = self.translation
                                    - Vector::new(
                                        (start.x - position.x) / 20.0,
                                        (start.y - position.y) / 20.0,
                                    );
                                self.translation.x = self.translation.x.min(0.0).max(-bounds.width);
                                self.translation.y =
                                    self.translation.y.min(0.0).max(-bounds.height);
                                self.cache.clear();
                            }
                        }
                        event::Status::Captured
                    }
                    mouse::Event::ButtonReleased(button) => match button {
                        mouse::Button::Left => {
                            self.pan_start = None;
                            event::Status::Captured
                        }
                        _ => event::Status::Ignored,
                    },
                    _ => event::Status::Ignored,
                },
                _ => event::Status::Ignored,
            };
            (event, None)
        }

        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            let content = self.cache.draw(bounds.size(), |frame| {
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );

                frame.scale(2.0);

                let points = &[
                    Point::new(100.0, 100.0),
                    Point::new(200.0, 200.0),
                    Point::new(300.0, 100.0),
                    Point::new(400.0, 200.0),
                    Point::new(500.0, 100.0),
                ];

                // NOTE The program doesn't cull the rectangles, it simple moves
                // them relative to the visible region
                for &point in points.iter() {
                    frame.fill_rectangle(
                        point + self.translation,
                        Size::new(100.0, 100.0),
                        Color::from_rgb8(0x40, 0x44, 0x4B),
                    );
                }
            });

            vec![content]
        }
    }
}
