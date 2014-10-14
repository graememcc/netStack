pub fn print_byte_table<'a, T: Iterator<&'a u8>>(iter: T) {
    print!("    ");
    for x in range::<u8>(0, 16) {
        print!("  {:02x}", x);

        if x == 15 {
            print!("\n");
        }
    }

    for (i, value) in iter.enumerate() {
        if i % 16 == 0 {
            if i > 0 {
                println!("");
            }
            print!("{:04x}", i);
        }
        print!("  {:02x}", *value);
    }

    println!("\n");
}
