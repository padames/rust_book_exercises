fn main() {

  let field_values = [
        -5,
        0,
        3,
        -1,
        5,
        -2,
        1,
        -4,
        3,
        4,
        2,
        10,
        1,
        0,
        -4,
        2,
        10,
        -9,
        10,
        5,
        -3,
        1,
        10,
        -8,
        1,
        -2,
        -7,
        4,
        5,
        9,
        -8,
        10,
        -9,
        9,
        -1,
        -3,
        6,
        7,
        -4,
        10
      ];

    if let Some((median, mode)) =  exercises::median_mode(&field_values) {  
        println!("The median is {median} and the mode is {mode}");
    }
}

