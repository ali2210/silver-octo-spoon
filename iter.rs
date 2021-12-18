






fn main(){

    let v : Vec<i32> = vec![1,2,3,4];
    let v2 = v.iter().filter(|&&s| 0 == s% 2).map(|&x| x*2).collect::<Vec<_>>(); // [4,8]
    
    println!("value: ({:#?})", v2);
    
}
