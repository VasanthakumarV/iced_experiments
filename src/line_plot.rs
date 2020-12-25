use iced::{Align, Canvas, Column, Container, Element, Length, Sandbox, Text};

use plot::Line;

#[derive(Default)]
pub struct LinePlot {}

impl Sandbox for LinePlot {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Line Plot")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        let line = Canvas::new(Line::default())
            .width(Length::Fill)
            .height(Length::Fill);

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Line Plot").size(50))
            .push(line);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod plot {
    use iced::{
        canvas::{Cache, Cursor, Geometry, Path, Program, Stroke},
        Point, Rectangle, Vector,
    };

    #[derive(Default)]
    pub struct Line {
        cache: Cache,
    }

    impl Program<()> for Line {
        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            // Space between the border and the axis as a fraction
            const MARGIN: f32 = 0.15;

            // Sample data for plotting
            const X: &[usize] = &[1, 2, 3, 4, 5];
            const Y: &[usize] = &[1, 3, 4, 1, 2];

            let line = self.cache.draw(bounds.size(), |frame| {
                // Drawinng the border
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );

                frame.with_save(|frame| {
                    let translate_origin =
                        Vector::new(frame.width() * MARGIN, frame.height() * MARGIN);
                    frame.translate(translate_origin);

                    // Drawing the axes
                    let axis_width = frame.width() - (2.0 * MARGIN * frame.width());
                    let axis_height = frame.height() - (2.0 * MARGIN * frame.height());
                    let axes = Path::new(|p| {
                        p.move_to(Point::new(0.0, axis_height));
                        p.line_to(Point::ORIGIN);
                        p.line_to(Point::new(axis_width, 0.0));
                    });
                    frame.stroke(&axes, Stroke::default().with_width(2.0));

                    let xscale = axis_width / (*X.iter().max().unwrap() as f32);
                    let yscale = axis_height / (*Y.iter().max().unwrap() as f32);

                    let xscaled: Vec<_> = X.iter().map(|&x| (x as f32) * xscale).collect();
                    let yscaled: Vec<_> = Y.iter().map(|&y| (y as f32) * yscale).collect();

                    // Drawing the line plot
                    let line = Path::new(|p| {
                        let mut points_iter = xscaled.iter().zip(yscaled.iter());

                        let (&x, &y) = points_iter.next().unwrap();
                        p.move_to(Point::new(x as f32, y as f32));

                        for (&x, &y) in xscaled.iter().zip(yscaled.iter()) {
                            p.line_to(Point::new(x as f32, y as f32));
                        }
                    });
                    frame.stroke(&line, Stroke::default())
                })
            });

            vec![line]
        }
    }
}
