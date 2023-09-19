pub mod geometry{
    use glam::{Vec2, Vec3};

    use crate::util::util::Ray;

    pub trait Hittable {
        fn hit(&self,r: &Ray, t_bounds: Vec2, rec: &mut HitRecord)->bool{
            false
        }
    }
    pub struct HitRecord{
        pub p:Vec3,
        pub normal:Vec3,
        pub t:f32
    }

    pub struct Sphere{
        pub center:Vec3,
        pub radius:f32
    }
    impl Sphere{
        pub fn new(center:Vec3,radius:f32)->Self{
            Self { center, radius }
        }
    }
    impl Hittable for Sphere{
        fn hit(&self,r: &Ray, ray_bounds: Vec2, rec: &mut HitRecord)->bool{
            let oc=r.origin-self.center;
            let a= r.dir.length_squared();
            let half_b=oc.dot(r.dir);
            let c=oc.length_squared()-(self.radius*self.radius);

            let discriminant=half_b*half_b-a*c;
            if discriminant<0.0{
                return false;
            }
            let sqrtd= discriminant.sqrt();

            let mut root = (-half_b-sqrtd)/a;
            if root <= ray_bounds.x || ray_bounds.y <= root{
                root=(-half_b+sqrtd)/a;
                if root <= ray_bounds.x || ray_bounds.y <= root{
                    return false;
                }
            }
            rec.t=root;
            rec.p=r.at(root);
            rec.normal=(rec.p-self.center)/self.radius;
            true
        }
    }
}