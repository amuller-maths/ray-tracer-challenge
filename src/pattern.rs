use crate::{
    canvas::Color,
    geometry::Point,
    object::Object,
    transform::{Transform, Transformable},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternType,
    transform: Transform,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PatternType {
    Stripe(Color, Color),
    Gradient(Color, Color),
    Ring(Color, Color),
    Checkers(Color, Color),
}

impl Pattern {
    pub fn pattern_at(self, p: Point) -> Color {
        match self.pattern {
            PatternType::Stripe(a, b) => {
                if p.0.floor() as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            PatternType::Gradient(a, b) => {
                let distance = b - a;
                let fraction = p.0 - (p.0.floor());
                a + distance * fraction
            }
            PatternType::Ring(a, b) => {
                if (p.0.powi(2) + p.2.powi(2)).sqrt().floor() as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            PatternType::Checkers(a, b) => {
                if (p.0.floor() + p.1.floor() + p.2.floor()) as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
        }
    }

    pub fn pattern_at_object(self, object: &Object, world_point: Point) -> Color {
        let object_point = world_point.transform(object.transform.inverse());
        let pattern_point = object_point.transform(self.transform.inverse());
        self.pattern_at(pattern_point)
    }

    pub fn set_transform(&mut self, t: Transform) -> Self {
        self.transform = t;
        *self
    }

    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Stripe(a, b),
            transform: Transform::default(),
        }
    }

    pub fn gradient_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Gradient(a, b),
            transform: Transform::default(),
        }
    }

    pub fn ring_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Ring(a, b),
            transform: Transform::default(),
        }
    }

    pub fn checkers_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Checkers(a, b),
            transform: Transform::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{object::Object, transform::Transform};

    use super::*;
    const WHITE: Color = Color(1., 1., 1.);
    const BLACK: Color = Color(0., 0., 0.);
    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(0., 2., 0.)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(0., 0., 2.)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Point(-1.1, 0., 0.)), WHITE);
    }
    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Object::sphere().set_transform(Transform::scaling(2., 2., 2.));
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(
            pattern.pattern_at_object(&object, Point(1.5, 0., 0.)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::sphere();
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black())
            .set_transform(Transform::scaling(2., 2., 2.));
        assert_eq!(
            pattern.pattern_at_object(&object, Point(1.5, 0., 0.)),
            Color::white()
        );
    }
    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::sphere().set_transform(Transform::scaling(2., 2., 2.));
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black())
            .set_transform(Transform::translation(0.5, 0., 0.));
        assert_eq!(
            pattern.pattern_at_object(&object, Point(2.5, 0., 0.)),
            Color::white()
        );
    }
    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient_pattern(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Point(0.25, 0., 0.)),
            Color(0.75, 0.75, 0.75)
        );
        assert_eq!(pattern.pattern_at(Point(0.5, 0., 0.)), Color(0.5, 0.5, 0.5));
        assert_eq!(
            pattern.pattern_at(Point(0.75, 0., 0.)),
            Color(0.25, 0.25, 0.25)
        );
    }
    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring_pattern(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(1., 0., 0.)), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 1.)), Color::black());
        assert_eq!(pattern.pattern_at(Point(0.708, 0., 0.708)), Color::black());
    }
    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checkers_pattern(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(0.99, 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(1.01, 0., 0.)), Color::black());
    }
    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checkers_pattern(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(0., 0.99, 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(0., 1.01, 0.)), Color::black());
    }
    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checkers_pattern(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Point(0., 0., 0.99)), Color::white());
        assert_eq!(pattern.pattern_at(Point(0., 0., 1.01)), Color::black());
    }
}
