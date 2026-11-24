use std::collections::HashMap;

fn main() {

  let mut h: HashMap<char, usize> = HashMap::new();
  for (i, c) in "hello!".chars().enumerate() {
    print!("i={}; c={}\n", &i, &c);
    let sum = h.entry(c).or_insert(0);
    if *sum > 0 {
        *sum += i;
    }
    else {
        *sum = i;
    }
  }


  println!("{}", h.get(&'l').unwrap());

}
