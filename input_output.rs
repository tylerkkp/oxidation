use std::{thread, time};

fn main(){
   let mut line = String::new();
   println!("What is your name?");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();

   let counttime = time::Duration::from_millis(500);

   let countdown = vec!["1...",
                        "2...",
                        "3...",
                        "4...",
                        "5...",
                        "6...",
                        "7...",
                        "8...",
                        "9...",
                        "10!!!"];
   for i in &countdown {
        println!("{}", i);
        // sleep for 500 milliseconds
        thread::sleep(counttime);
   }

   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}
