
pub trait Dim{
    fn Square(&self) -> i64;
}

#[derive(Debug)]
struct Area<T> {
    h : T,
    w : T,
}

impl<T> Dim for Area<T>{
    fn Square(&self) -> i64{
          let num : i64 = self.h * self.w; //mismatched types
          num
    }
}

pub fn traitcall <T: Dim>(item : &T) -> i64{
    let num = item.Square();
    num
}
  
  
