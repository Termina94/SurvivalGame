use graphics::{
    line_from_to,
    math::{Matrix2d, Vec2d},
    types::Color,
};
use opengl_graphics::GlGraphics;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}
impl Dimensions {
    pub fn new<T: Into<f64>>(width: T, height: T) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

pub fn draw_rect(color: Color, rect: [f64; 4], transform: Matrix2d, gl: &mut GlGraphics) {
    let [x, y, width, height] = rect;
    let points: Vec<Vec2d> = vec![
        [x, y],
        [x + width, y],
        [x + width, y + height],
        [x, y + height],
    ];

    draw_points(color, points, transform, gl);
}

fn draw_points(color: Color, points: Vec<Vec2d>, transform: Matrix2d, gl: &mut GlGraphics) {
    for i in 1..=points.len() {
        let (from, to) = {
            if i < points.len() {
                let from: Vec2d = *points.get(i).unwrap();
                let to: Vec2d = *points.get(i - 1).unwrap();
                (from, to)
            } else {
                let from: Vec2d = *points.get(3).unwrap();
                let to: Vec2d = *points.get(0).unwrap();
                (from, to)
            }
        };

        line_from_to(color, 0.5, from, to, transform, gl);
    }
}
