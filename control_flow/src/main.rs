fn main() {
    let mut counter :i32= 0;
    let x = 4;
    let x = if x < 2 {x + 1} else {x - 1};
    let result = loop {
        counter += 1;
        if counter >= 10 {
            let x = x + 2;
            println!("x in the inner loop x={x}");
            break counter / 3;
        };
    };
    println!("x in main x={x}");
    println!("The result from the loop is {result}");

    let mut cnt = 0;
    'outer_loop: loop {
        println!("Outer loop counter: {cnt}");
        let mut barrier_value = 10;
        let reset_val = loop {
            println!("barrier value {barrier_value}");
            if barrier_value < 8 {
                break cnt;
            }
            if cnt == barrier_value {
                break 'outer_loop;
            }
            barrier_value -= 1;
            cnt += 1;
        };
    }
    println!("Counter upon exit from outer loop: {cnt}");

    let a = [11, 21, 31, 41, 51];
    let mut idx = 0;
    while idx < 5 {
        println!("The value is: {}", a[idx]);
        idx += 1
    }

}
