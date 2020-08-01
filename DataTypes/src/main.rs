mod sh;
use sh::stack_and_heap;
use std::mem;

const MEANING_OF_LIFE:u8 = 42;
static Z:i32 = 123;

static mut CAREFULL:i32 = 123;

fn main(){
  println!("Data Types");
  fundamental_types();
  println!("Operators");
  operators();
  println!("Scope");
  scope_and_shadowing();
  println!("Consts");
  consts();
  println!("Stack and Heap");
  sh::stack_and_heap();
}

fn fundamental_types() {
  let _n256: u8 = 255;
  // _n256 = 128;  immutable
  println!("n256 is {}", _n256);

  let mut m128: i8 = -128;
  println!("m is {}", m128);
  m128 = 127;
  println!("m is {}", m128);

  let c = 123456789; // 32-bit signed
  println!("c is {}, size is {} bytes", c, mem::size_of_val(&c));

  let f = 3.1412; // double-precision, f64
  println!("f is {}, size is {} bytes", f, mem::size_of_val(&f));

  let i:isize = 123;
  let size_of_i = mem::size_of_val(&i);
  println!("i is {}, size is {} bytes, {}-bit OS", i, size_of_i, size_of_i*8);

  let ch = 'x';
  println!("ch is {}, size is {} bytes", ch, mem::size_of_val(&ch));

  let g = false;
  println!("g is {}, size is {} bytes", g, mem::size_of_val(&g));

}

fn operators() {
  // arimethic
  let mut a = 2+3*4;
  println!("a is {}", a);
  a = a + 1;
  a -= 2;
  println!("remainder of {} / {} is {}", a, 3, (a%3));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed is {}, and {} to PI is {}", b, b_cubed, b, b_to_pi);

  // bitwise
  let c = 1 | 2;
  print!("1|2 is {}", c);

  let two_to_ten = 1 << 10; // shift
  println!("2^10 is {}", two_to_ten);

  //logical

  let pi_less_4 = std::f64::consts::PI < 4.0;
  // > >= <= ==
  let x = 5;
  let x_is_5 = x == 5;
  println!("Is pi less than4? {}, Is x = 5? {}", pi_less_4, x_is_5);
}

fn scope_and_shadowing() {
  let a = 123;

  { // new scope
    let b = 456;
    println!("Inside, b = {}, and a = {}", b, a);
    let a = 789;
    println!("Inside again, b = {}, and a = {}", b, a);
  }

  println!("a = {}", a);
}

fn consts() {
  println!("{}, {}", MEANING_OF_LIFE, Z);
  // static mutability is not allowed for memory safety
  // you can use unsafe block if you are carefull
  unsafe
  {
    CAREFULL = 777;
    println!("{}", CAREFULL);
  }
}