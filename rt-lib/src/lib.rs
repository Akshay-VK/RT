pub mod util;
pub mod camera;
pub mod geometry;

use camera::Camera;
use geometry::geometry::{Hittable, HittableList};
use glam::{UVec2, Vec3, vec3, uvec2};
use image::ImageBuffer;
use rayon::prelude::*;

pub struct Engine{
    state:State
}
impl Engine{
    pub fn init(x:u32,y:u32,cam:Camera)->Self{
        Self{state: State::new(uvec2(x, y),cam)}
    }
    pub fn add_object(&mut self,obj:Box<dyn Hittable>){
        self.state.world.add(obj);
    }
    pub fn run(&mut self)->bool{
        //let mut img = vec!(vec!(vec3(0.0, 0.0, 0.0);self.state.dim.x as usize);self.state.dim.y as usize);
        //let mut w=&self.state.world;
        self.state.cam.initialize();
        let w=&self.state.world;
        //let w=self.state.world;
        /*img.par_iter_mut().enumerate().for_each(|x| {
            let y=x.0;
            x.1.iter_mut().enumerate().for_each(|x|{
                //println!("{}  ,  {}",x.0,y);
                let c=self.state.cam.render_pixel(x.0 as f32, y as f32,&Box::new(self.state.world));
                *x.1=c;
            });
        });*/
        for y in 0..self.state.img.len(){
            /*img[y].par_iter_mut().enumerate().for_each(|x|{
                //println!("{}  ,  {}",x.0,y);
                let c=self.state.cam.render_pixel(x.0 as f32, y as f32,&Box::new(self.state.world));
                *x.1=c;
            });*/
            for x in 0..self.state.img[y].len(){
                let c=self.state.cam.render_pixel(x as f32, y as f32,w);
                self.state.img[y][x]=c;
            }
        }
        true
    }
    pub fn save(&self){
        let img = ImageBuffer::from_fn(self.state.dim.x, self.state.dim.y, |x, y| {
            let e:Vec3=self.state.img[y as usize][x as usize];
            let r=(e.x*255.0) as u8;
            let g=(e.y*255.0) as u8;
            let b=(e.z*255.0) as u8;
            image::Rgb([r,g,b])
        });
        img.save("render.png").unwrap();
    }
}

pub struct State{
    dim: UVec2,
    img: Vec<Vec<Vec3>>,
    cam:Camera,
    world:HittableList
}
impl State{
    pub fn new(dim: UVec2,cam:Camera)->Self{
        let world=HittableList::new();
        Self { img: vec!(vec!(vec3(0.0, 0.0, 0.0);dim.x as usize);dim.y as usize) , dim,cam,world}
    }
}
