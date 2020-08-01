use std::mem;

struct Point {
  x: f64,
  y: f64
}

enum Color {
  Red,
  Green,
  Blue,
  RGB(u8,u8,u8), // tuple like
  CMYK{cyan: u8, magenta: u8, yellow: u8, black: u8}, // struct like
}

struct Line {
  start: Point,
  end: Point,
  color: Color
}

fn structures_and_enums() {
  let p = Point {x: 1.0, y: 1.0};
  println!("point p is at ({}, {})", p.x, p.y);
  let p2 = Point {x: 5.0, y: 10.0};
  let line = Line { start: p, end: p2, color: Color::CMYK{cyan: 0, magenta: 128, yellow: 0, black: 255} };
  match line.color
  {
    Color::Red => println!("The line is Red!"),
    Color::Green => println!("The line is Green!"),
    Color::Blue => println!("The line is Blue!"),
    Color::RGB(0,0,0)
      | Color::CMYK{cyan:_,magenta:_,yellow:_, black: 255} => println!("The line is Black!"),
    Color::RGB(r,g,b) => println!("rgb({},{},{})", r,g,b),
    _ => println!("The line is  on some other color!"),
  }
}

fn options()
{
  // Option<T>

  let x = 3.0;
  let y = 2.0;
  
  //Can contain Some(z) or None

  let r:Option<f64> =
    if y != 0.0 { Some(x/y) } else { None };
  println!("{:?}", r);

  match r {
    Some(z) => println!("{}/{} is {}", x, y, z),
    None => println!("Cannot divide by zero"),
  }

  if let Some(z) = r { println!("value of {}", z); }
}

fn arrays()
{
  let mut a = [1,2,3,4,5]; //:[i32;5]
  println!("{:?}", a);
  a[0] = 15;
  println!("a has {} elements and the values is {}", a.len(), a[0]);
  println!("{:?}", a);

  if a != [1,2,3,4,5] { println!("Does not match"); } else { println!("Does match"); }
  let b = [1u16; 10];

  for i in 0..b.len()
  {
    println!("Item at {} is {}", i, b[i]);
  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  // matrix

  let mtx:[[f32;3];3] = [
    [1.0,2.0,3.0],
    [4.0,5.0,6.0],
    [4.0,5.0,6.0]
  ];
  println!("{:?}", mtx);

  for i in 0..mtx.len()
  {
    for j in 0..mtx[i].len()
    {
      if i == j {
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
      }
    }
  }

  for r in &mtx
  {
    for v in r
    {
      println!("{}", v);
    }
  }
}

fn vectors()
{
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);
  println!("a is {:?}", a);

  a.push(44);
  println!("a is {:?}", a);
  
  let idx:usize = 0; // wont work with a 32 bit or a signed type
  println!("a[0] is {}", a[idx]);

  if let Some(z) = a.get(6) { println!("{}", z); } else { println!("Not element at {}th position", 6); }
  match a.get(6)
  {
    Some(x) => println!("a[6] = {}", x),
    None => println!("Not elem at 6th position"),
  }

  for x in &a { println!("{}", x); }

  a.push(77);
  println!("{:?}", a);

  let last_elem = a.pop();
  println!("last elem is {:?} and a is {:?}", last_elem, a);

  while let Some(x) = a.pop()
  {
    println!("{}", x);
  }

  a.push(1);
  a.push(2);
  a.push(3);

  loop {
    match a.pop() {
      Some(x) => println!("{}", x),
      None => break,
    }
  }

}

fn use_slice(slice: &[i32])
{
  println!("f = {}, l = {}", slice[0], slice.len());
}


fn slices() {
  let mut data = [1,2,3,4,5];

  use_slice(&data[1..4]);
}

fn main() {
  structures_and_enums();
  options();
  arrays();
  vectors();
  slices();
}

