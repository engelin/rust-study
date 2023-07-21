mod front_of_house; // call(not include) modules in `front_of_house` file

pub use crate::front_of_house::hosting; // use the module shortly 

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
