// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly 'use' each name so they are available without manual scoping.
    // crate(把...装箱)
    // Explicitly(明确地)
    use crate::Status::{ Poor, Rich };
    // Automatically `use` each name inside `Work`
    use crate::Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        // note the lack of scoping because of the explicit `use` above
        Rich => println!("The rich have lots of money!"),
        Poor => println!("the poor have no money!"),
    }

    match work {
        // note again the lack of scoping
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
