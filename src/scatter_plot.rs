use iced::{pick_list, Align, Canvas, Column, Container, Element, Length, PickList, Sandbox, Text};

use plot::{Scatter, Shape};

#[derive(Default)]
pub struct ScatterPlot {
    pick_list: pick_list::State<Shape>,
    selected_shape: Shape,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectedShape(Shape),
}

impl Sandbox for ScatterPlot {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Scatter Plot")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectedShape(shape) => {
                self.selected_shape = shape;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let pick_list = PickList::new(
            &mut self.pick_list,
            &Shape::ALL[..],
            Some(self.selected_shape),
            Message::SelectedShape,
        );

        let scatter = Canvas::new(Scatter::default().shape(self.selected_shape))
            .width(Length::Fill)
            .height(Length::Fill);

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Scatter Plot").size(50))
            .push(pick_list)
            .push(scatter);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod plot {
    use iced::{
        canvas::{Cache, Cursor, Geometry, Path, Program, Stroke},
        Point, Rectangle, Size,
    };

    #[derive(Default)]
    pub struct Scatter {
        cache: Cache,
        shape: Shape,
    }

    impl Scatter {
        pub fn shape(mut self, shape: Shape) -> Self {
            self.shape = shape;
            self
        }
    }

    impl<Message> Program<Message> for Scatter {
        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            const MARGIN: f32 = 0.15;

            // Sample data for plotting
            const X: &[usize] = &[1, 2, 3, 4, 5];
            const Y: &[usize] = &[1, 3, 4, 1, 2];

            let scatter = self.cache.draw(bounds.size(), |frame| {
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, bounds.size()),
                    Stroke::default(),
                );

                // Drawing the axes
                let axis_width = frame.width() - (2.0 * MARGIN * frame.width());
                let axis_height = frame.height() - (2.0 * MARGIN * frame.height());

                let xscale = axis_width / (*X.iter().max().unwrap() as f32);
                let yscale = axis_height / (*Y.iter().max().unwrap() as f32);

                let xscaled: Vec<_> = X.iter().map(|&x| (x as f32) * xscale).collect();
                let yscaled: Vec<_> = Y.iter().map(|&y| (y as f32) * yscale).collect();

                for (&x, &y) in xscaled.iter().zip(yscaled.iter()) {
                    frame.stroke(&self.shape.path(Point::new(x, y), 10.0), Stroke::default())
                }
            });

            vec![scatter]
        }
    }

    #[derive(Eq, PartialEq, Clone, Debug, Copy)]
    pub enum Shape {
        Circle,
        Square,
    }

    impl Shape {
        pub const ALL: [Shape; 2] = [Shape::Circle, Shape::Square];

        fn path(&self, center: Point, size: f32) -> Path {
            match self {
                Self::Circle => Path::circle(center, size),
                Self::Square => Path::rectangle(
                    Point::new(center.x - size, center.y - size),
                    Size::new(2. * size, 2. * size),
                ),
            }
        }
    }

    impl Default for Shape {
        fn default() -> Shape {
            Shape::Circle
        }
    }

    impl std::fmt::Display for Shape {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Shape::Circle => "Circle",
                    Shape::Square => "Square",
                }
            )
        }
    }
}
