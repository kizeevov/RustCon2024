use eframe::egui::Shape;
use eframe::emath::Rot2;

pub trait Rotate {
    fn rotate(&mut self, angle: f32);
}

impl Rotate for Shape {
    fn rotate(&mut self, angle: f32) {
        match self {
            Shape::LineSegment { points, .. } => {
                let rotation = Rot2::from_angle(angle);

                points[0] = (rotation * points[0].to_vec2()).to_pos2();
                points[1] = (rotation * points[1].to_vec2()).to_pos2();
            }
            _ => {}
        }
    }
}
