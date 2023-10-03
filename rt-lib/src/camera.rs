use glam::{UVec2, Vec3, vec3, uvec2, vec2};

use crate::{util::util::Ray, geometry::geometry::{HitRecord, Sphere, Hittable, HittableList}};

pub struct Camera{
    pub aspect_ratio:f32,
    pub cam_dim: UVec2,
    pub focal_length: f32,
    center: Vec3,
    pixel00: Vec3,
    pixel_du:Vec3,
    pixel_dv:Vec3
}
impl Camera {
    pub fn new(x:f32,y:f32,z:f32,cam_w:u32,aspect_ratio:f32,focal_length:f32)->Self{
        Self { aspect_ratio, cam_dim: uvec2(cam_w, 10), focal_length, center: vec3(x, y, z), pixel00: Vec3::ZERO, pixel_du: Vec3::ZERO, pixel_dv: Vec3::ZERO }
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
        let pixel_c=self.pixel00+(self.pixel_du*i)+(self.pixel_dv*j);
        let r_dir=pixel_c-self.center;
        let r=Ray::new(self.center, r_dir);
        let color=self.render_ray(&r,world);
        color
    }
    fn render_ray(&self,r:&Ray, world: &HittableList)->Vec3{
        //let sphere=Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);

        let mut rec:HitRecord=HitRecord { p: Vec3::ZERO, normal: Vec3::ZERO, t: 0.0 , front_face: true};
        if world.hit(&r, vec2(0.0, f32::MAX), &mut rec){
            return (rec.normal+vec3(1.0, 1.0, 1.0))*0.5;
        }

        let unit_dir=r.dir.normalize();
        let a = (unit_dir.y+1.0)*0.5;
        vec3(1.0, 1.0, 1.0)*(1.0-a)+vec3(0.5, 0.7, 1.0)*a
    }
}