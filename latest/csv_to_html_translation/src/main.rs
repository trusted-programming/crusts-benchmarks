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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub static mut input : * const i8 =
(b"Character,Speech\nThe multitude,The messiah! Show us the messiah!\nBrians mother,<angry>Now you listen here! He's not the messiah; he's a very naughty boy! Now go away!</angry>\nThe multitude,Who are you?\nBrians mother,I'm his mother; that's who!\nThe multitude,Behold his mother! Behold his mother!\0" as * const u8).cast::<i8>();
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s: *const i8 = std::ptr::null::<i8>();
        print!("<table>\n<tr><td>");
        s = input;
        while *s != 0 {
            match i32::from(*s) {
                10_i32 => {
                    print!("</td></tr>\n<tr><td>");
                }
                44_i32 => {
                    print!("</td><td>");
                }
                60_i32 => {
                    print!("&lt;");
                }
                62_i32 => {
                    print!("&gt;");
                }
                38_i32 => {
                    print!("&amp;");
                }
                _ => {
                    print!("{}", i32::from(*s));
                }
            }
            s = s.offset(1);
            s;
        }
        puts((b"</td></tr>\n</table>\0" as *const u8).cast::<i8>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
