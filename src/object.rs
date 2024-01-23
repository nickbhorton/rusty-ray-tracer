pub mod objects {
    use crate::linear_algebra::vector::Vec3;
    pub struct Ray {
        origin: Vec3<f32>,
        direction: Vec3<f32>,
    }
    impl Ray {
        pub fn from(origin: Vec3<f32>, direction: Vec3<f32>) -> Ray {
            Ray { origin, direction }
        }
    }
    pub struct Sphere {
        position: Vec3<f32>,
        radius: f32,
    }
    impl Sphere {
        pub fn from(x: f32, y: f32, z: f32, r: f32) -> Sphere {
            Sphere {
                position: Vec3::from(x, y, z),
                radius: r,
            }
        }
        pub fn intersect(&self, ray: &Ray) -> Option<f32> {
            let b = 2.0
                * (ray.direction.x * (ray.origin.x - self.position.x)
                    + ray.direction.y * (ray.origin.y - self.position.y)
                    + ray.direction.z * (ray.origin.z - self.position.z));
            let c = (ray.origin.x - self.position.x).powi(2)
                + (ray.origin.y - self.position.y).powi(2)
                + (ray.origin.z - self.position.z).powi(2)
                - self.radius.powi(2);
            let discriminant = b.powi(2) - 4.0 * c;
            if discriminant >= 0.0 {
                let t1 = 0.5 * (-b + discriminant.sqrt());
                let t2 = 0.5 * (-b - discriminant.sqrt());
                if t1 >= 0.0 {
                    // t2 ? 0
                    if t2 >= 0.0 {
                        if t1 < t2 {
                            return Some(t1);
                        } else {
                            return Some(t2);
                        }
                    }
                    return Some(t1);
                } else if t2 >= 0.0 {
                    // t1 < 0 && t2 >= 0
                    return Some(t2);
                } else {
                    // t1 < 0 && t2 < 0
                    return None;
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        linear_algebra::linear_algebra::Vec3,
        object::objects::{Ray, Sphere},
    };

    #[test]
    fn sphere_intersection_simple() {
        let origin = Vec3::from(0.0, 0.0, 0.0);
        let direction = Vec3::from(0.0, 0.0, 1.0);
        let ray = Ray::from(origin, direction);

        let sphere = Sphere::from(0.0, 0.0, 4.0, 2.0);

        let result = sphere.intersect(&ray);
        assert_eq!(result, Some(2.0));
    }
    #[test]
    fn sphere_intersection_miss() {
        let origin = Vec3::from(0.0, 0.0, 0.0);
        let direction = Vec3::from(0.0, 0.0, -1.0);
        let ray = Ray::from(origin, direction);

        let sphere = Sphere::from(0.0, 0.0, 4.0, 2.0);

        let result = sphere.intersect(&ray);
        assert_eq!(result, None);
    }

    #[test]
    fn sphere_intersection_inside() {
        let origin = Vec3::from(0.0, 0.0, 0.0);
        let direction = Vec3::from(0.0, 0.0, 1.0);
        let ray = Ray::from(origin, direction);

        let sphere = Sphere::from(0.0, 0.0, 0.0, 2.0);

        let result = sphere.intersect(&ray);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn sphere_intersection_glance() {
        let origin = Vec3::from(0.0, 0.0, 0.0);
        let direction = Vec3::from(0.0, 0.0, 1.0);
        let ray = Ray::from(origin, direction);

        let sphere = Sphere::from(0.0, -1.0, 2.0, 1.0);

        let result = sphere.intersect(&ray);
        assert_eq!(result, Some(2.0));
    }
}
