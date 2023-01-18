mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//use crate::front_of_house::hosting;
// restaurant_02::front_of_house::hosting::add_to_waitlist();

// IDIOMATIC USE PATH
//use crate::front_of_house::hosting::add_to_waitlist;

pub use crate::front_of_house::hosting;
// restaurant_02::hosting::add_to_waitlist();

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    //add_to_waitlist();
}
/*
mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
*/
