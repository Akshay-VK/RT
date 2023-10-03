use glam::{UVec2, Vec3, vec3, uvec2, vec2, Vec2};
use rand::random;
use crate::{util::util::{Ray, random_on_hemisphere, random_unit_vector}, geometry::geometry::{HitRecord, Sphere, Hittable, HittableList}};

pub struct Camera{
    pub aspect_ratio:f32,
    pub cam_dim: UVec2,
    pub focal_length: f32,
    pub samples_per_pixel:i32,
    center: Vec3,
    pixel00: Vec3,
    pixel_du:Vec3,
    pixel_dv:Vec3,
    pub max_bounce:i32
}
impl Camera {
    pub fn new(
            x:f32,
            y:f32,
            z:f32,
            cam_w:u32,
            aspect_ratio:f32,
            focal_length:f32,
            samples_per_pixel:i32,
            max_bounce:i32
        )->Self{
        Self { aspect_ratio, cam_dim: uvec2(cam_w, 10), focal_length, samples_per_pixel, center: vec3(x, y, z), pixel00: Vec3::ZERO, pixel_du: Vec3::ZERO, pixel_dv: Vec3::ZERO,max_bounce}
    }
    pub fn initialize(&mut self){
        self.cam_dim.y=(self.cam_dim.x as f32/self.aspect_ratio) as u32;
        self.cam_dim.y= if self.cam_dim.y<1 {
            1
        }else{
            self.cam_dim.y
        };
        //self.center=;

        let view_dim_y=2.0;
        let view_dim_x=view_dim_y*((self.cam_dim.x as f32/self.cam_dim.y as f32) as f32);

        let view_u=vec3(view_dim_x, 0.0, 0.0);
        let view_v=vec3(0.0, -view_dim_y, 0.0);

        self.pixel_du=view_u/(self.cam_dim.x as f32);
        self.pixel_dv=view_v/(self.cam_dim.y as f32);
        
        let view_up_left=self.center-vec3(0.0, 0.0, self.focal_length)-(view_u/2.0)-(view_v/2.0);

        self.pixel00=view_up_left+((self.pixel_du+self.pixel_dv)*0.5);
    }
    
    pub fn render_pixel(&self,i:f32,j:f32, world: &HittableList)->Vec3{
        let mut color=Vec3::ZERO;
        for s in 0..self.samples_per_pixel{
            let r=self.get_ray(i, j);
            color+=self.render_ray(&r,world,self.max_bounce);
        }
        color
    }
    fn render_ray(&self,r:&Ray, world: &HittableList, depth:i32)->Vec3{
        if depth<=0{
            return Vec3::ZERO;
        }

        let mut rec:HitRecord=HitRecord { p: Vec3::ZERO, normal: Vec3::ZERO, t: 0.0 , front_face: true};
        if world.hit(&r, vec2(0.001, f32::MAX), &mut rec){
            let dir=rec.normal+random_unit_vector();
            return (self.render_ray(&Ray::new(rec.p, dir), world,depth-1))*0.1;
        }

        let unit_dir=r.dir.normalize();
        let a = (unit_dir.y+1.0)*0.5;
        vec3(1.0, 1.0, 1.0)*(1.0-a)+vec3(0.5, 0.7, 1.0)*a
    }
    fn get_ray(&self,i:f32,j:f32)->Ray{
        let pixel_c=self.pixel00+(self.pixel_du*i)+(self.pixel_dv*j);
        let px:f32=random::<f32>()-0.5;
        let py:f32=random::<f32>()-0.5;
        let pix_sample_square=(px*self.pixel_du)+(py*self.pixel_dv);

        let pix_sample=pixel_c+pix_sample_square;


        let r_dir=pix_sample-self.center;
        return Ray::new(self.center, r_dir);
    }
}