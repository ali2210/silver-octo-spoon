use rand::prelude::*;


fn main(){

    // random  
    let mut rnd_op = rand::thread_rng();
    
    // each dice roll 1...6 
    let y : i64 = rnd_op.gen_range(1..6);
    let dice  = |x : i64| -> i64{
      x
    };
    
    // if dice got 6 then lucky
    match dice(y){
        6 => {println!("game started...({:#?})", dice(y))},
        _ => {println!("try again... ({:#?})", dice(y))}
    }
    
}


