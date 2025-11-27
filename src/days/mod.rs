use log::trace;

use crate::framework::Registry;

macro_rules! register_day {
    ($reg:expr, $daymod:ident) => {
        let name = stringify!($daymod);
        let short = &name[3..];

        $reg.register(short, $daymod::part1, $daymod::part2);
        trace!("Registered day {name}")
    };
}

pub mod day00;
// more

pub fn register_all(reg: &mut Registry) {
    register_day!(reg, day00);
}
