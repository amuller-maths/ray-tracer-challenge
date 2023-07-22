use crate::{canvas::Color, geometry::Point};

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}
