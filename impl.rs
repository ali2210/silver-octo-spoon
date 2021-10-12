


pub trait Megadoom_Tower{
    fn visible(&self) -> &megadoom;
}

#[derive(Debug)]
pub struct megadoom{
    pub height : i64 ,
    pub  width : i64,
    pub depth : i64,
}


impl Megadoom_Tower for megadoom  {
    fn visible(&self) -> &megadoom {
        println!(" megadoom structure ({:#?})", self);
        self
    }
}


fn new_object(item : &impl Megadoom_Tower) -> &megadoom{
    item.visible()
}

fn main(){
    let skytower = megadoom{height : 300, width: 720, depth : 100};
    new_object(&skytower);
}
