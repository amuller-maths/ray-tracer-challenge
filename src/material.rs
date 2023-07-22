use crate::canvas::Color;
use crate::geometry::{Point, Vector};
use crate::light::PointLight;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}

impl Material {
    pub fn lighting(
        self,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0. || in_shadow {
            diffuse = Color(0., 0., 0.);
            specular = Color(0., 0., 0.);
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0. {
                specular = Color(0., 0., 0.);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{Point, Vector},
        light::PointLight,
    };

    use super::*;

    fn almost_eq(c1: Color, c2: Color) -> bool {
        (c1.0 - c2.0) < 1e6 && (c1.1 - c2.1) < 1e6 && (c1.2 - c2.2) < 1e6
    }
    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., 0., -1.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 0., -10.),
            intensity: Color(1., 1., 1.),
        };
        let result = m.lighting(light, position, eyev, normalv, false);
        assert_eq!(result, Color(1.9, 1.9, 1.9));
    }
    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 0., -10.),
            intensity: Color(1., 1., 1.),
        };
        let result = m.lighting(light, position, eyev, normalv, false);
        assert_eq!(result, Color(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., 0., -1.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 10., -10.),
            intensity: Color(1., 1., 1.),
        };
        let result = m.lighting(light, position, eyev, normalv, false);
        assert!(almost_eq(result, Color(0.7364, 0.7364, 0.7364)));
    }
    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 10., -10.),
            intensity: Color(1., 1., 1.),
        };
        let result = m.lighting(light, position, eyev, normalv, false);
        assert!(almost_eq(result, Color(1.6364, 1.6364, 1.6364)));
    }
    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., 0., -1.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 0., 10.),
            intensity: Color(1., 1., 1.),
        };
        let result = m.lighting(light, position, eyev, normalv, false);
        assert_eq!(result, Color(0.1, 0.1, 0.1));
    }
    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = Point(0., 0., 0.);
        let eyev = Vector(0., 0., -1.);
        let normalv = Vector(0., 0., -1.);
        let light = PointLight {
            position: Point(0., 0., -10.),
            intensity: Color(1., 1., 1.),
        };
        let in_shadow = true;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color(0.1, 0.1, 0.1));
    }
}
