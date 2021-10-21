

// lifetime 

fn main(){
    let prefix = String::from("hello");
    let suffx = String::from("world");
    println!("value : ({})", append(&prefix, &suffx));
}

fn append<'a>(s : &'a str, st : &'a str) -> String{
    s.to_owned()+st
}
