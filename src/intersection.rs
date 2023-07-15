use crate::shape::Shape;
use core::cmp::Ordering;

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Intersection {}

pub struct Intersections(Vec<Intersection>);

impl Intersections {
    fn push(&mut self, element: Intersection) {
        let Intersections(v) = self;
        let pos = v.binary_search(&element).unwrap_or_else(|e| e);
        v.insert(pos, element);
    }

    fn hit(self) -> Option<Intersection> {
        self.0.into_iter().find(|i| i.t >= 0.)
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersection::Intersections, shape::Shape};

    use super::Intersection;
    #[test]
    fn intersections_pos1() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(intersections.hit(), Some(Intersection { t: 1., object: s }));
    }
    #[test]
    fn intersections_pos2() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(intersections.hit(), Some(Intersection { t: 1., object: s }));
    }
    #[test]
    fn intersections_pos_neg() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(intersections.hit(), Some(Intersection { t: 1., object: s }));
    }
    #[test]
    fn intersections_neg_pos() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(intersections.hit(), Some(Intersection { t: 1., object: s }));
    }
    #[test]
    fn intersections_neg() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: -2., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(intersections.hit(), None);
    }
    #[test]
    fn intersections_pos3() {
        let mut intersections = Intersections(vec![]);
        let s = Shape::default_sphere();
        let i1 = Intersection { t: 5., object: s };
        let i2 = Intersection { t: 7., object: s };
        let i3 = Intersection { t: -3., object: s };
        let i4 = Intersection { t: 2., object: s };
        intersections.push(i1);
        intersections.push(i2);
        intersections.push(i3);
        intersections.push(i4);
        assert_eq!(intersections.hit(), Some(Intersection { t: 2., object: s }));
    }
}
