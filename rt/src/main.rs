use rt_lib::Engine;

fn main() {
    println!("Hello, world!");
    let mut eng= Engine::init(256,256);
    eng.run();
    eng.save();
}
