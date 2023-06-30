#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn rand() -> i32;
}
fn main_0() -> i32 {
    let mut i: i32 = 0;
    print! ("<table style=\"text-align:center; border: 1px solid\"><th></th><th>X</th><th>Y</th><th>Z</th>");
    i = 0_i32;
    unsafe {
        while i < 4_i32 {
            print!(
                "<tr><th>{}</th><td>{}</td><td>{}</td><td>{}</td></tr>",
                i,
                rand() % 10_000_i32,
                rand() % 10_000_i32,
                rand() % 10_000_i32
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("</table>");
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
