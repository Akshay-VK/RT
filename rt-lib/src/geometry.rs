pub mod geometry{
    use glam::{Vec2, Vec3};
    
    use crate::util::util::Ray;

    pub trait Hittable: Sync+Send {
        fn hit(&self,r: &Ray, ray_bounds: Vec2, rec: &mut HitRecord)->bool{
            false
        }
    }
    pub struct HitRecord{
        pub p:Vec3,
        pub normal:Vec3,
        pub t:f32,
        pub front_face:bool,
    }
    impl HitRecord{
        pub fn set_face_normal(&mut self,ray: &Ray, out: Vec3){
            self.front_face = ray.dir.dot(out)<0.0;
            self.normal= if self.front_face {
                out
            }else{
                -out
            }
        }
    }

    pub struct HittableList{
        pub objects: Vec<Box<dyn Hittable>>
    }
    impl HittableList{
        pub fn new()->Self{
            Self { objects: Vec::new() }
        }
        pub fn clear(&mut self){
            self.objects.clear();
        }
        pub fn add(&mut self, object: Box<dyn Hittable>){
            self.objects.push(object);
        }
    }
    impl Hittable for HittableList{
        fn hit(&self,r: &Ray, ray_bounds: Vec2, rec: &mut HitRecord)->bool{
            let mut temp_rec =HitRecord{ p: Vec3::ZERO, normal: Vec3::ZERO, t: 0.0, front_face: true };
            let mut hitanything=false;
            let mut closest = ray_bounds.y;
            for obj in self.objects.iter(){
                if obj.hit(r,Vec2 { x: ray_bounds.x, y: closest }, &mut temp_rec){
                    hitanything=true;
                    closest=temp_rec.t;
                    rec.p=temp_rec.p;
                    rec.normal=temp_rec.normal;
                    rec.t=temp_rec.t;
                    rec.front_face=temp_rec.front_face;
                }
            }
            hitanything
        }
    }

    pub struct Sphere{
        pub center:Vec3,
        pub radius:f32
    }
    impl Sphere{
        pub fn new(center:(f32,f32,f32),radius:f32)->Self{
            Self { center:Vec3 { x: center.0, y: center.1, z: center.2 }, radius }
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
            let onormal=(rec.p-self.center)/self.radius;
            rec.set_face_normal(r, onormal);
            true
        }
    }
}