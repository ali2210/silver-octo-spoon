use rand::prelude::*;


#[derive(Debug)]
struct LuckSpin<T>
where 
    T : Fn(i64) -> i64,
    {
        spin : T,
        outcome : Option<i64>,
    }


impl<T> LuckSpin<T>
where
    T : Fn(i64) -> i64,
    {
        // create new object 
        fn roll(turn : T)-> LuckSpin<T>{
            LuckSpin{
                spin : turn,
                outcome : None,
            }
        }
        
        fn throw_dice(&mut self, x : i64) -> Option<i64>{
            
            // store value in closures
            match self.outcome{
                Some(x) => {Some(x)}, 
                None =>{
                    let otc = (self.spin)(x);
                    self.outcome = Some(otc);
                    self.outcome
                }
            }
        }
        
        fn rule(&mut self, x : Option<i64>) -> (Option<i64>, String){
            
            // if dice got 6 then lucky
            match self.outcome{
                Some(6) => {
                    (self.outcome, String::from("game started!"))
                }
                _ => {
                    (x, String::from("try again!"))
                }
            }
        }
    }
fn main(){

    // random  
    let mut rnd_op = rand::thread_rng();
    
    // each dice roll 1...6 
    let y : i64 = rnd_op.gen_range(1..6);
    
    // closures with trait
    let mut dice = LuckSpin::roll(|x : i64| -> i64{x});
  
    // store closures value
    let roll = dice.throw_dice(y);
  
    // predicit closures value .. that's is the dice game ! wel-done
    let (result, msg) = dice.rule(roll);
    
    println!("({:#?}), {:#?}", msg, result);
    
}
