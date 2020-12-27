use iced::{Align, Column, Container, Element, Length, Sandbox, Text};

use grid::Grid;

#[derive(Default)]
pub struct ZoomGrid {
    grid: Grid,
}

impl Sandbox for ZoomGrid {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Zoomable Grid")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Zoomable Grid").size(50))
            .push(self.grid.view().map(|_| ()));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub mod grid {
    use iced::{
        canvas::{event, Event},
        canvas::{Cache, Cursor, Geometry, Program},
        mouse, Canvas, Color, Element, Length, Point, Rectangle, Size,
    };

    pub struct Grid {
        scale: f32,
        cache: Cache,
    }

    impl Default for Grid {
        fn default() -> Self {
            Self {
                scale: 1.0,
                cache: Cache::default(),
            }
        }
    }

    impl Grid {
        const CELL_SIZE: usize = 20;
        const MIN_SCALING: f32 = 0.5;
        const MAX_SCALING: f32 = 2.0;

        pub fn view<'a>(&self) -> Element<'a, ()> {
            Canvas::new(Grid::default())
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    impl Program<()> for Grid {
        fn update(
            &mut self,
            event: Event,
            _bounds: Rectangle,
            _cursor: Cursor,
        ) -> (event::Status, Option<()>) {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::WheelScrolled { delta } => match delta {
                        mouse::ScrollDelta::Lines { y, .. }
                        | mouse::ScrollDelta::Pixels { y, .. } => {
                            self.scale = (self.scale * (1.0 + y / 100.0))
                                .max(Self::MIN_SCALING)
                                .min(Self::MAX_SCALING);

                            self.cache.clear();

                            (event::Status::Captured, None)
                        }
                    },
                    _ => (event::Status::Ignored, None),
                },
                _ => (event::Status::Ignored, None),
            }
        }

        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            let grid = self.cache.draw(bounds.size(), |frame| {
                frame.scale(Self::CELL_SIZE as f32 * self.scale);

                frame.fill_rectangle(Point::new(0.0, 0.0), Size::UNIT, Color::BLACK);

                let width = 2.0 / Self::CELL_SIZE as f32;
                let color = Color::from_rgb8(70, 74, 83);

                let rows =
                    0..((bounds.height / self.scale) / Self::CELL_SIZE as f32).ceil() as usize;
                let rows_count = rows.clone().count();
                let columns =
                    0..((bounds.width / self.scale) / Self::CELL_SIZE as f32).ceil() as usize;
                let columns_count = columns.clone().count();

                for row in rows {
                    frame.fill_rectangle(
                        Point::new(0.0, row as f32),
                        Size::new(columns_count as f32, width),
                        color,
                    );
                }

                for column in columns {
                    frame.fill_rectangle(
                        Point::new(column as f32, 0.0),
                        Size::new(width, rows_count as f32),
                        color,
                    );
                }
            });

            vec![grid]
        }
    }
}
