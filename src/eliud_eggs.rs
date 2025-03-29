pub fn egg_count(display_value: u32) -> usize {
    display_value.count_ones() as usize
    // OR use the implementation below
    // let mut count = 0usize;
    // let mut oprnd = display_value as usize;
    // while oprnd > 0 {
    //     println!("x: {} | x & 1: {}", x, x & 1);
    //     count += oprnd & 1;
    //     oprnd >>= 1;
    // }
    // count
}
