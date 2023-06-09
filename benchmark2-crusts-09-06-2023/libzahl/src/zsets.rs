use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __errno_location() -> *mut i32;
    fn __ctype_b_loc() -> *mut *const u16;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
pub type z_t = [C2RustUnnamed; 1];
pub const _ISdigit: u32 = 2048;
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[no_mangle]
pub extern "C" fn zsets(mut a: *mut C2RustUnnamed, mut str: *const i8) -> i32 {
    unsafe {
        let mut temp = 0;
        let mut neg = (*str as i32 == '-' as i32) as i32;
        let mut str_end = 0 as *const i8;
        str = str.offset((neg != 0 || *str as i32 == '+' as i32) as i32 as isize);
        if *str == 0 {
            *__errno_location() = 22;
            return -1;
        }
        str_end = str;
        while *str_end != 0 {
            if *(*__ctype_b_loc()).offset(*str_end as i32 as isize) as i32 & _ISdigit as i32 == 0 {
                *__errno_location() = 22;
                return -1;
            }
            str_end = str_end.offset(1);
        }
        (*a).sign = 0;
        zset(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_const_1e19.as_mut_ptr(),
        );
        's_234: {
            let mut current_block_39: u64;
            match str_end.offset_from(str) as i64 % 19 {
                0 => {
                    current_block_39 = 2009900886467833939;
                }
                18 => {
                    current_block_39 = 15121320231496893747;
                }
                17 => {
                    current_block_39 = 16368488404610608819;
                }
                16 => {
                    current_block_39 = 2817806250783016588;
                }
                15 => {
                    current_block_39 = 8436272415664805267;
                }
                14 => {
                    current_block_39 = 17260602820350443736;
                }
                13 => {
                    current_block_39 = 13506216435796342560;
                }
                12 => {
                    current_block_39 = 3798081891250826146;
                }
                11 => {
                    current_block_39 = 3337010891255527766;
                }
                10 => {
                    current_block_39 = 14701426646074699728;
                }
                9 => {
                    current_block_39 = 7675967895758458727;
                }
                8 => {
                    current_block_39 = 17228944538289729950;
                }
                7 => {
                    current_block_39 = 16906717074776190969;
                }
                6 => {
                    current_block_39 = 5148048065308018341;
                }
                5 => {
                    current_block_39 = 18210558662916816231;
                }
                4 => {
                    current_block_39 = 13503438682059240994;
                }
                3 => {
                    current_block_39 = 5618369753603485945;
                }
                2 => {
                    current_block_39 = 12749676338018479376;
                }
                1 => {
                    current_block_39 = 13992101357592761495;
                }
                _ => {
                    current_block_39 = 1434579379687443766;
                }
            }
            loop {
                match current_block_39 {
                    1434579379687443766 => {
                        break 's_234;
                    }
                    2009900886467833939 => {
                        temp = temp.wrapping_mul(10);
                        let fresh0 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh0 as i32 & 15i32) as u64);
                        current_block_39 = 15121320231496893747;
                    }
                    15121320231496893747 => {
                        temp = temp.wrapping_mul(10);
                        let fresh1 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh1 as i32 & 15i32) as u64);
                        current_block_39 = 16368488404610608819;
                    }
                    16368488404610608819 => {
                        temp = temp.wrapping_mul(10);
                        let fresh2 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh2 as i32 & 15i32) as u64);
                        current_block_39 = 2817806250783016588;
                    }
                    2817806250783016588 => {
                        temp = temp.wrapping_mul(10);
                        let fresh3 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh3 as i32 & 15i32) as u64);
                        current_block_39 = 8436272415664805267;
                    }
                    8436272415664805267 => {
                        temp = temp.wrapping_mul(10);
                        let fresh4 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh4 as i32 & 15i32) as u64);
                        current_block_39 = 17260602820350443736;
                    }
                    17260602820350443736 => {
                        temp = temp.wrapping_mul(10);
                        let fresh5 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh5 as i32 & 15i32) as u64);
                        current_block_39 = 13506216435796342560;
                    }
                    13506216435796342560 => {
                        temp = temp.wrapping_mul(10);
                        let fresh6 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh6 as i32 & 15i32) as u64);
                        current_block_39 = 3798081891250826146;
                    }
                    3798081891250826146 => {
                        temp = temp.wrapping_mul(10);
                        let fresh7 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh7 as i32 & 15i32) as u64);
                        current_block_39 = 3337010891255527766;
                    }
                    3337010891255527766 => {
                        temp = temp.wrapping_mul(10);
                        let fresh8 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh8 as i32 & 15i32) as u64);
                        current_block_39 = 14701426646074699728;
                    }
                    14701426646074699728 => {
                        temp = temp.wrapping_mul(10);
                        let fresh9 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh9 as i32 & 15i32) as u64);
                        current_block_39 = 7675967895758458727;
                    }
                    7675967895758458727 => {
                        temp = temp.wrapping_mul(10);
                        let fresh10 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh10 as i32 & 15i32) as u64);
                        current_block_39 = 17228944538289729950;
                    }
                    17228944538289729950 => {
                        temp = temp.wrapping_mul(10);
                        let fresh11 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh11 as i32 & 15i32) as u64);
                        current_block_39 = 16906717074776190969;
                    }
                    16906717074776190969 => {
                        temp = temp.wrapping_mul(10);
                        let fresh12 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh12 as i32 & 15i32) as u64);
                        current_block_39 = 5148048065308018341;
                    }
                    5148048065308018341 => {
                        temp = temp.wrapping_mul(10);
                        let fresh13 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh13 as i32 & 15i32) as u64);
                        current_block_39 = 18210558662916816231;
                    }
                    18210558662916816231 => {
                        temp = temp.wrapping_mul(10);
                        let fresh14 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh14 as i32 & 15i32) as u64);
                        current_block_39 = 13503438682059240994;
                    }
                    13503438682059240994 => {
                        temp = temp.wrapping_mul(10);
                        let fresh15 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh15 as i32 & 15i32) as u64);
                        current_block_39 = 5618369753603485945;
                    }
                    5618369753603485945 => {
                        temp = temp.wrapping_mul(10);
                        let fresh16 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh16 as i32 & 15i32) as u64);
                        current_block_39 = 12749676338018479376;
                    }
                    12749676338018479376 => {
                        temp = temp.wrapping_mul(10);
                        let fresh17 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh17 as i32 & 15i32) as u64);
                        current_block_39 = 13992101357592761495;
                    }
                    _ => {
                        temp = temp.wrapping_mul(10);
                        let fresh18 = str;
                        str = str.offset(1);
                        temp = temp.wrapping_add((*fresh18 as i32 & 15i32) as u64);
                        if !(temp == 0) {
                            *((*libzahl_tmp_str_num.as_mut_ptr()).chars).offset(0 as isize) =
                                temp as u32;
                            temp >>= 32;
                            *((*libzahl_tmp_str_num.as_mut_ptr()).chars).offset(1 as isize) =
                                temp as u32;
                            (*libzahl_tmp_str_num.as_mut_ptr()).used =
                                (1 + (temp != 0) as i32) as u64;
                            zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
                        }
                        if !(*str != 0) {
                            current_block_39 = 1434579379687443766;
                            continue;
                        }
                        zmul(a, a, libzahl_const_1e19.as_mut_ptr());
                        temp = 0;
                        current_block_39 = 2009900886467833939;
                    }
                }
            }
        }
        if neg != 0 {
            (*a).sign = -zsignum(a);
        }
        return 0;
    }
}
