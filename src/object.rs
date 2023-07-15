use crate::{matrix::Matrix, shape::Shape};

#[derive(Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix,
}

impl Object {
    pub fn default_sphere() -> Self {
        Self {
            shape: Shape::default_sphere(),
            transform: Matrix::id(),
        }
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use crate::geometry::Point;
    use crate::matrix::Matrix;
    use crate::shape::Shape;
    use crate::transform::translation;

    #[test]
    fn default_sphere() {
        let s = Object::default_sphere();
        assert_eq!(
            s,
            Object {
                shape: Shape::Sphere {
                    center: Point(0., 0., 0.),
                    radius: 1.
                },
                transform: Matrix::id(),
            }
        );
    }
    #[test]
    fn changing_sphere_transform() {
        let mut s = Object::default_sphere();
        let t = translation(2., 3., 4.);
        s.set_transform(t);
        assert_eq!(
            s,
            Object {
                shape: Shape::Sphere {
                    center: Point(0., 0., 0.),
                    radius: 1.
                },
                transform: t,
            }
        )
    }
}
