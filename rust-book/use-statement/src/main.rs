mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn assign_table() {}
    }
}

use front_of_house::hosting;
mod customer {
    use super::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::assign_table();
    }
}

fn main () {
    crate::customer::eat_at_restaurant();
}
