fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
}

fn if_statement (){
  let temp = 35;

  if temp > 30
  {
    println!("Really hot outside!");
  }
  else if temp < 10
  {
    println!("Really cold!");
  }
  else
  {
    println!("Temp is ok!");
  }
  let day = if temp > 20 {"sunny"} else {"cloudy"};
  println!("Temp is {} and day is {}", temp, day);

  println!("It is {}",
    if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});
}

fn while_and_loop() {
  let mut x = 1;
  while x < 1000 {
    x *= 2;

    if x == 64 { continue; }

    println!("x is {}", x);
  }

  let mut y = 1;
  loop {
    y *= 2;
    if y == 64 { continue; }
    println!("y is {}", y);
    if y == 1 <<10 { break; }
  }
}

fn for_loop() {
  for x in 1..11 {
    if x == 3 { continue; }
    println!("x = {}", x);
    if x == 7 { break; }
  }

  for (i, y) in (30..41).enumerate()
  {
    println!("Num in {} is {}", i, y);  
  }
}

fn match_statement() {
  let ccode = 1;

  let country = match ccode {
    44 => "UK",
    46 => "SWE",
    7 => "RUS",
    1..=999 => "unknown",
    _ => "invalid"
  };

  println!("The country for {} is {}", ccode, country);
}