
// Generic type traits implements  
struct Area<T>{
    h : T, 
    w : T,
}

impl<T : std::ops::Mul<Output = T>+Copy> Area<T>{
    fn GetArea(&self) -> T{
        let area = self.h * self. w;
        area
    }
}


fn main(){
    let rectangle = Area{h : 3, w : 3};
    println!("Area : ({})", rectangle.GetArea());
}
