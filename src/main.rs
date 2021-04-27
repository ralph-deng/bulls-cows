use num_traits::pow;
use rand::Rng;
use std::convert::TryFrom;
use std::io;

fn main() {
  // get the number of digits in the number which will be guessed
  println!("How many digits in the guessed number you want(1-10)?");
  let digits: String;
  let secret_max_limit: u32; 
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(..) => {},
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };
    match input.trim().parse::<u32>() {
      Ok(num) => {
        if num > 0 && num < 11 {
          digits = input;
          // calculate max value of random secret number
          secret_max_limit = pow(10, usize::try_from(num).unwrap());
          break;
        } 
      },
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };
  }

  // generate secret number
  let secert_number = rand::thread_rng().gen_range(0..secret_max_limit);
  // println!("secret_number: {} {}", secert_number, secret_max_limit);

  println!("Please guess number:");
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess) {
      Ok(..) => {},
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };

    // todo:users can choose to get the smaller/bigger hints
    // use std::cmp::Ordering;
    // match guess.cmp(&secert_number) {
    //   Ordering::Less => println!("too small"),
    //   Ordering::Greater => println!("too big"),
    //   Ordering::Equal => {}
    // }
  }
}
