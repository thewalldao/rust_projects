use std::any::type_name;

/**
this is new test project
*/

fn main() {
    println!("Hello, world!");
    // let unum: u16 = 2525;
    // assert_eq!(unum as i16, 255_i16); // in range
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);
    assert_eq!(0b101101u8.count_ones(), 4); // population count
    println!("{}", type_of(0.));
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
