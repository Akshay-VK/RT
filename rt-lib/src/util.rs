pub mod util{
    use std::f32::consts::PI;

    use glam::{Vec3, Vec2, vec2};
    use rand::random;

    pub struct Ray{
        pub origin: Vec3,
        pub dir: Vec3
    }
    impl Ray {
        pub fn new(o:Vec3,dir:Vec3)->Self{
            Self { origin: o, dir }
        }
        pub fn at(&self, t:f32)->Vec3{
            self.origin+(self.dir*t)
        }
    }

    pub fn random_vec3()->Vec3{
        Vec3 { x: random::<f32>(), y: random::<f32>(), z: random::<f32>() }
    }
    pub fn random_vec3_range(range:Vec2)->Vec3{
        (random_vec3()*(range.y-range.x))+range.x
    }
    pub fn random_in_unit_sphere()->Vec3{
        for i in 0..50{
            let p = random_vec3_range(vec2(-1.0, 1.0));
            if p.length_squared()<1.0{
                return p;
            }
        }
        random_vec3().normalize()
    }
    pub fn random_unit_vector()->Vec3{
        random_in_unit_sphere().normalize()
    }
    pub fn random_on_hemisphere(n:Vec3)->Vec3{
        let u = random_unit_vector();
        if u.dot(n)>0.0{
            u
        }else {
            -u
        }
    }
    
}