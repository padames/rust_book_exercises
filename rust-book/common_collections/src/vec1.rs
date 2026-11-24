fn main() {
    let v1 = vec![1,2,3,4,5];
    // now get a reference to the 3rd element, safely
    let pos = 1;

    let opt_number: Option<&i32> = v1.get(pos-1);
    if let Some(number) = opt_number {
        println!("The element  at position {pos} is {number}");
    }
    else {
        println!("There is no element at position {pos}!");
    }
}
