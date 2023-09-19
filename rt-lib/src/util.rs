pub mod util{
    use glam::Vec3;

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
}