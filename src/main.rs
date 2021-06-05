use rand::Rng;
use std::io;

fn main() {
  const MAX_DIGITS: u32 = 9;

  // get the number of digits in the number which will be guessed
  println!("How many digits in the guessed number you want(1-{})?", MAX_DIGITS);

  let digits: u32;
  let secret_max: u32; 
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(..) => {},
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };

    match input.trim().parse::<u32>() {
      Ok(num) => {
        // 
        if num > 0 && num <= MAX_DIGITS {
          digits = num;
          // calculate max value of random secret number
          // 10 ^ 10 > u32 max value 4_294_967_295 so if MAX_DIGITS >= 10 u64 is needed 
          secret_max = 10u32.pow(num);
          break;
        } else {
          println!("Error: The number of digits must between 1 and {}", MAX_DIGITS);
        }
      },
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };
  }

  // generate secret number
  let secert_number = {
    let mut random;
    loop {
      random = prepend(rand::thread_rng().gen_range(0..secret_max).to_string(), digits);
      // clone otherwise get "use of moved value: random" issue on return statement
      if !is_duplicate(random.clone()) {
        break;
      }
    }
    
    random
  };
  let guess_min = prepend("0".to_string(), digits);
  // println!("Secret_number: {}", secert_number);

  println!("Please guess number({}-{}):", guess_min, secret_max - 1);
  loop {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
      Ok(..) => {},
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };

    match guess.trim().parse::<u32>() {
      Ok(num) => {
        // minus /r/n to get real guess length 
        if (guess.len() - 2) != digits as usize || num >= secret_max {
          println!("Error: Please guess number between {} and {} with {} digits", guess_min, secret_max - 1, digits);
        } else if is_duplicate(guess.clone()) {
          // clone otherwise get "use of moved value: guess" issue on next compare statement
          println!("Error: The digits must be all different");
        } else {  
          // clone otherwise get "use of moved value: secert_number in previous iteration of loop" issue
          let result = compare(guess, secert_number.clone());
          if result[0] == 999 {
            println!("You win!");
            break;
          } else {
            println!("Bull(s): {}, Cow(s): {}", result[0], result[1]);
          }
        } 
      },
      Err(err) => println!("Error[{}][{}]: {}", file!(), line!(), err)
    };
  }
}

// prepend '0' to str with total max_chars length
// return: str with prepended '0'
fn prepend(str: String, max_chars: u32) -> String {
  let mut str = String::from(str);
  let limit = max_chars - str.len() as u32;

  for _ in 0..limit {
    str.insert(0, '0');
  }
  
  str
}

// check if any char of str is duplicate
// return: true if duplicate
fn is_duplicate(str: String) -> bool {
  let mut chars: Vec<&str> = str.trim().split("").collect();
  
  // remove first and last chars due to empty after split("")
  chars.remove(0);
  chars.pop();

  for item in chars {
    if str.matches(item).count() > 1 {
      return true;
    }
  }

  return false;
}

// compare guessed number by user with secrect one by this program 
// return: numbers of bulls and cows 
fn compare(guess: String, secret: String) -> [u32; 2] {
  let mut bulls: u32 = 0;
  let mut cows: u32 = 0;

  if guess.trim() == secret {
    return [999, 999];
  }

  for item in guess.trim().char_indices() {
    let (guess_index, guess_char) = item;
    let mut secret_chars: Vec<&str> = secret.split("").collect();

    // remove first and last chars due to empty after split("")
    secret_chars.remove(0);
    secret_chars.pop();

    let mut is_bull = false;
    for i in 0..secret_chars.len() {
      if i == guess_index && secret_chars[i] == guess_char.to_string() {
        bulls += 1;
        is_bull = true;
        break;
      } 
    }

    if is_bull == true {
      continue;
    }

    for i in 0..secret_chars.len() {
      if i != guess_index && secret_chars[i] == guess_char.to_string() {
        cows += 1;
        break;
      }
    } 
  }

  [bulls, cows]
}
