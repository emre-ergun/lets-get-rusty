mod front_of_house;


pub fn eat_at_resturant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative
    front_of_house::hosting::add_to_wait_list();
}