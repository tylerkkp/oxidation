
// copied from tutorialspoint for testing purposes
fn main(){
   let mut line = String::new();
   println!("What is your name?");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}
