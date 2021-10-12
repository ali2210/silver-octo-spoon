
pub trait Dim{
    fn Square(&self) -> i64;
}

#[derive(Debug)]
struct Area {
    h : i64,
    w : i64,
}

impl Dim for Area{
    fn Square(&self) -> i64{
          let num : i64 = self.h * self.w;
          num
    }
}

pub fn traitcall <T: Dim>(item : &T) -> i64{
    let num = item.Square();
    num
}

fn main(){
    let rectangle = Area{h : 3, w : 3};
    println!("({})", traitcall(&rectangle));
}
