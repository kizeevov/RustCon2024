use iced::{
    advanced::{graphics::gradient, mouse},
    alignment,
    font::Weight,
    theme::palette::Extended,
    widget::canvas,
    widget::canvas::{stroke, Cache, Frame, LineCap, Path, Stroke, Style, Text},
    Color, Degrees, Element, Font, Length, Point, Radians, Rectangle, Renderer, Theme, Vector,
};

pub struct Speedometer {
    speed: u32,
    min_speed: u32,
    max_speed: u32,
    width: Length,
    height: Length,
    cache: Cache,
}

impl Speedometer {
    pub fn new(min_speed: u32, max_speed: u32, speed: u32) -> Speedometer {
        Self {
            speed: speed.min(max_speed),
            min_speed,
            max_speed,
            width: Length::Fixed(20.0),
            height: Length::Fixed(20.0),
            cache: Default::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn view<'a>(self) -> Element<'a, ()> {
        let width = self.width;
        let height = self.height;

        canvas(self).width(width).height(height).into()
    }
}

pub fn speedometer(min_speed: u32, max_speed: u32, speed: u32) -> Speedometer {
    Speedometer::new(min_speed, max_speed, speed)
}

impl canvas::Program<()> for Speedometer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let speedometer = self.cache.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0 - 9.0;
            let scale = frame.width().min(frame.height()) / 694.0;
            let width = radius / 100.0;

            draw_background(frame, center, radius, palette);
            draw_needle_cap(frame, center, radius);

            frame.translate(Vector::new(center.x, center.y));
            draw_ticks(frame, self.min_speed, self.max_speed, width, scale, radius, &palette);
            draw_needle(frame, self.speed, self.max_speed, width, radius, &palette);
        });

        vec![speedometer]
    }
}

fn draw_background(frame: &mut Frame<Renderer>, center: Point, radius: f32, palette: &Extended) {
    let background = Path::circle(center, radius);
    frame.fill(&background, palette.background.weak.color);
    frame.stroke(
        &background,
        Stroke::default()
            .with_color(palette.background.strong.color)
            .with_width(9.0),
    );
}

fn draw_needle(
    frame: &mut Frame<Renderer>,
    speed: u32,
    max_speed: u32,
    width: f32,
    radius: f32,
    palette: &Extended,
) {
    let needle = Path::line(
        Point::new(0.0, 0.12 * radius),
        Point::new(0.0, -0.88 * radius),
    );

    let needle_stroke = || -> Stroke {
        Stroke {
            width: width * 1.6,
            style: Style::Solid(palette.secondary.base.color),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    };

    frame.with_save(|frame| {
        frame.rotate(hand_rotation(speed, max_speed));
        frame.stroke(&needle, needle_stroke());
    });
}

fn draw_ticks(
    frame: &mut Frame<Renderer>,
    min_speed: u32,
    max_speed: u32,
    width: f32,
    scale: f32,
    radius: f32,
    palette: &Extended,
) {
    let short_ticks = Path::line(
        Point::new(0.0, -radius + 9.0 + 16.0),
        Point::new(0.0, -0.86 * radius),
    );
    let long_ticks = Path::line(
        Point::new(0.0, -radius + 9.0 + 16.0),
        Point::new(0.0, -0.85 * radius),
    );

    let thin_stroke = || -> Stroke {
        Stroke {
            width,
            style: stroke::Style::Solid(palette.secondary.base.text),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    };

    let wide_stroke = || -> Stroke {
        Stroke {
            width: width * 2.0,
            style: stroke::Style::Solid(palette.secondary.base.text),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    };

    for tick in (min_speed..=max_speed).step_by(10) {
        if tick % 20 == 0 {
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(tick, max_speed));
                frame.stroke(&long_ticks, wide_stroke());
            });
            draw_tick_label(frame, tick, radius, scale, max_speed);
        } else {
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(tick, max_speed));
                frame.stroke(&short_ticks, thin_stroke());
            });
        }
    }
}

fn draw_needle_cap(frame: &mut Frame<Renderer>, center: Point, radius: f32) {
    let needle_cap = Path::circle(center, radius * 0.1);
    frame.fill(
        &needle_cap,
        Color::from_rgb(
            0x14 as f32 / 255.0,
            0x18 as f32 / 255.0,
            0x1B as f32 / 255.0,
        ),
    );

    frame.stroke(
        &needle_cap,
        Stroke {
            style: Style::Gradient(
                gradient::Linear::new(
                    Point::new(center.x, center.y - radius * 0.1),
                    Point::new(center.x, center.y + radius * 0.1),
                )
                .add_stop(
                    0.0,
                    Color::from_rgb(
                        0x14 as f32 / 255.0,
                        0x18 as f32 / 255.0,
                        0x1C as f32 / 255.0,
                    ),
                )
                .add_stop(
                    0.46,
                    Color::from_rgb(
                        0x24 as f32 / 255.0,
                        0x24 as f32 / 255.0,
                        0x26 as f32 / 255.0,
                    ),
                )
                .add_stop(0.98, Color::BLACK)
                .into(),
            ),
            ..Default::default()
        }
        .with_width(4.0),
    );
}

fn draw_tick_label(
    frame: &mut Frame<Renderer>,
    tick: u32,
    radius: f32,
    scale: f32,
    max_speed: u32,
) {
    let radians: Radians = tick_label_rotation(tick, max_speed).into();
    let text = Text {
        content: format!("{tick:<3}"),
        color: Color::WHITE,
        // size: (0.07 * radius).into(),
        size: (24.0 * scale).into(),
        position: Point::new(
            0.76 * radius * radians.0.cos(),
            0.76 * radius * radians.0.sin(),
        ),
        horizontal_alignment: alignment::Horizontal::Center,
        vertical_alignment: alignment::Vertical::Center,
        font: Font {
            family: Default::default(),
            weight: Weight::Bold,
            stretch: Default::default(),
            style: Default::default(),
        },
        ..Text::default()
    };

    text.draw_with(|path, color| {
        frame.fill(&path, color);
    });
}

const SECTOR_DEGREES: f32 = 100.0;

fn hand_rotation(n: u32, total: u32) -> Degrees {
    let turns = n as f32 / total as f32;

    Degrees((360.0 - SECTOR_DEGREES) * turns - (180.0 - SECTOR_DEGREES / 2.0))
}

fn tick_label_rotation(n: u32, total: u32) -> Degrees {
    let turns = n as f32 / total as f32;

    Degrees((360.0 - SECTOR_DEGREES) * turns - 220.0)
}
