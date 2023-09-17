use glam::{UVec2, Vec3, vec3, uvec2};
use image::{RgbImage, ImageBuffer, Rgb};
use rayon::prelude::*;

pub struct Engine{
    state:State
}
impl Engine{
    pub fn init(x:u32,y:u32)->Self{
        Self{state: State::new(uvec2(x, y))}
    }
    pub fn run(&mut self)->bool{
        self.state.img.par_iter_mut().enumerate().for_each(|x| {
            let y=x.0;
            x.1.iter_mut().enumerate().for_each(|x|{
                *x.1=vec3(x.0 as f32/256.0,y as f32/256.0,0.0);
            });
        });
        true
    }
    pub fn save(&self){
        let mut img = ImageBuffer::from_fn(self.state.dim.x, self.state.dim.y, |x, y| {
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
    img: Vec<Vec<Vec3>>
}
impl State{
    pub fn new(dim: UVec2)->Self{
        Self { img: vec!(vec!(vec3(0.0, 0.0, 0.0);dim.x as usize);dim.y as usize) , dim}
    }
}