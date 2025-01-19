// this example shows how traits model actions on structs that implement them 
// inspired by https://doc.rust-lang.org/rust-by-example/trait/dyn.html


struct Sheep {}
struct Cow {}

trait Sound {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Sound for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Sound for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

trait Species {
    fn name(&self) ->&'static str;
}

impl Species for Sheep {
    fn name(&self) ->&'static str {
        "sheep"    
    }
}

impl Species for Cow {
    fn name(&self) ->&'static str {
        "cow"    
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn animal_properties(random_number: f64) -> (Box<dyn Sound>, Box<dyn Species>) {
    if random_number < 0.5 {
        ( Box::new(Sheep {}), Box::new(Sheep{}) )
    } else {
        ( Box::new(Cow {}), Box::new(Cow {}) )
    }
}


fn main() {
    let random_number = 0.5234;
    let (sound, species) = animal_properties(random_number);
    println!("You've randomly chosen an animal, and it says {}", sound.noise());
    println!("I am a {}", species.name() );
}
