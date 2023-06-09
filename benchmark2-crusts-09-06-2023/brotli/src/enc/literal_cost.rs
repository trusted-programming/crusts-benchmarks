use libc;
extern "C" {
    fn log2(_: f64) -> f64;
    static kBrotliLog2Table: [f64; 256];
    fn BrotliIsMostlyUTF8(
        data: *const u8,
        pos: u64,
        mask: u64,
        length: u64,
        min_fraction: f64,
    ) -> i32;
}
#[inline(always)]
extern "C" fn brotli_min_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn FastLog2(mut v: u64) -> f64 {
    unsafe {
        if v < 256 {
            return kBrotliLog2Table[v as usize];
        }
        return log2(v as f64);
    }
}

static mut kMinUTF8Ratio: f64 = 0.75f64;
extern "C" fn UTF8Position(mut last: u64, mut c: u64, mut clamp: u64) -> u64 {
    if c < 128 {
        return 0;
    } else if c >= 192 {
        return brotli_min_size_t(1, clamp);
    } else if last < 0xe0 {
        return 0;
    } else {
        return brotli_min_size_t(2, clamp);
    };
}

extern "C" fn DecideMultiByteStatsLevel(
    mut pos: u64,
    mut len: u64,
    mut mask: u64,
    mut data: *const u8,
) -> u64 {
    unsafe {
        let mut counts: [u64; 3] = [0; 3];
        let mut max_utf8 = 1;
        let mut last_c = 0;
        let mut i: u64 = 0;
        i = 0;
        while i < len {
            let mut c = *data.offset((pos.wrapping_add(i) & mask) as isize) as u64;
            counts[UTF8Position(last_c, c, 2) as usize] =
                (counts[UTF8Position(last_c, c, 2u64) as usize]).wrapping_add(1);
            last_c = c;
            i = i.wrapping_add(1);
        }
        if counts[2 as usize] < 500 {
            max_utf8 = 1;
        }
        if (counts[1 as usize]).wrapping_add(counts[2 as usize]) < 25 {
            max_utf8 = 0;
        }
        return max_utf8;
    }
}

extern "C" fn EstimateBitCostsForLiteralsUTF8(
    mut pos: u64,
    mut len: u64,
    mut mask: u64,
    mut data: *const u8,
    mut cost: *mut libc::c_float,
) {
    unsafe {
        let max_utf8 = DecideMultiByteStatsLevel(pos, len, mask, data);
        let mut histogram: [[u64; 256]; 3] = [[0; 256], [0; 256], [0; 256]];
        let mut window_half = 495;
        let mut in_window = brotli_min_size_t(window_half, len);
        let mut in_window_utf8: [u64; 3] = [0; 3];
        let mut i: u64 = 0;
        let mut last_c = 0;
        let mut utf8_pos = 0;
        i = 0;
        while i < in_window {
            let mut c = *data.offset((pos.wrapping_add(i) & mask) as isize) as u64;
            histogram[utf8_pos as usize][c as usize] =
                (histogram[utf8_pos as usize][c as usize]).wrapping_add(1);
            in_window_utf8[utf8_pos as usize] = (in_window_utf8[utf8_pos as usize]).wrapping_add(1);
            utf8_pos = UTF8Position(last_c, c, max_utf8);
            last_c = c;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < len {
            if i >= window_half {
                let mut c_0 = (if i < window_half.wrapping_add(1u64) {
                    0
                } else {
                    *data.offset(
                        (pos.wrapping_add(i)
                            .wrapping_sub(window_half)
                            .wrapping_sub(1u64)
                            & mask) as isize,
                    ) as i32
                }) as u64;
                let mut last_c_0 = (if i < window_half.wrapping_add(2u64) {
                    0
                } else {
                    *data.offset(
                        (pos.wrapping_add(i)
                            .wrapping_sub(window_half)
                            .wrapping_sub(2u64)
                            & mask) as isize,
                    ) as i32
                }) as u64;
                let mut utf8_pos2 = UTF8Position(last_c_0, c_0, max_utf8);
                histogram[utf8_pos2 as usize][*data
                    .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                    as usize] = (histogram[utf8_pos2 as usize][*data
                    .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                    as usize])
                    .wrapping_sub(1);
                in_window_utf8[utf8_pos2 as usize] =
                    (in_window_utf8[utf8_pos2 as usize]).wrapping_sub(1);
            }
            if i.wrapping_add(window_half) < len {
                let mut c_1 = *data.offset(
                    (pos.wrapping_add(i)
                        .wrapping_add(window_half)
                        .wrapping_sub(1u64)
                        & mask) as isize,
                ) as u64;
                let mut last_c_1 = *data.offset(
                    (pos.wrapping_add(i)
                        .wrapping_add(window_half)
                        .wrapping_sub(2u64)
                        & mask) as isize,
                ) as u64;
                let mut utf8_pos2_0 = UTF8Position(last_c_1, c_1, max_utf8);
                histogram[utf8_pos2_0 as usize][*data
                    .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                    as usize] = (histogram[utf8_pos2_0 as usize][*data
                    .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                    as usize])
                    .wrapping_add(1);
                in_window_utf8[utf8_pos2_0 as usize] =
                    (in_window_utf8[utf8_pos2_0 as usize]).wrapping_add(1);
            }
            let mut c_2 = (if i < 1u64 {
                0
            } else {
                *data.offset((pos.wrapping_add(i).wrapping_sub(1u64) & mask) as isize) as i32
            }) as u64;
            let mut last_c_2 = (if i < 2u64 {
                0
            } else {
                *data.offset((pos.wrapping_add(i).wrapping_sub(2u64) & mask) as isize) as i32
            }) as u64;
            let mut utf8_pos_0 = UTF8Position(last_c_2, c_2, max_utf8);
            let mut masked_pos = pos.wrapping_add(i) & mask;
            let mut histo =
                histogram[utf8_pos_0 as usize][*data.offset(masked_pos as isize) as usize];
            let mut lit_cost: f64 = 0.;
            if histo == 0 {
                histo = 1;
            }
            lit_cost = FastLog2(in_window_utf8[utf8_pos_0 as usize]) - FastLog2(histo);
            lit_cost += 0.02905f64;
            if lit_cost < 1.0f64 {
                lit_cost *= 0.5f64;
                lit_cost += 0.5f64;
            }
            if i < 2000 {
                lit_cost += 0.7f64 - 2000u64.wrapping_sub(i) as f64 / 2000.0f64 * 0.35f64;
            };
            *cost.offset(i as isize) = lit_cost as libc::c_float;
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliEstimateBitCostsForLiterals(
    mut pos: u64,
    mut len: u64,
    mut mask: u64,
    mut data: *const u8,
    mut cost: *mut libc::c_float,
) {
    unsafe {
        if BrotliIsMostlyUTF8(data, pos, mask, len, kMinUTF8Ratio) != 0 {
            EstimateBitCostsForLiteralsUTF8(pos, len, mask, data, cost);
            return;
        } else {
            let mut histogram: [u64; 256] = [0; 256];
            let mut window_half = 2000;
            let mut in_window = brotli_min_size_t(window_half, len);
            let mut i: u64 = 0;
            i = 0;
            while i < in_window {
                histogram[*data.offset((pos.wrapping_add(i) & mask) as isize) as usize] =
                    (histogram[*data.offset((pos.wrapping_add(i) & mask) as isize) as usize])
                        .wrapping_add(1);
                i = i.wrapping_add(1);
            }
            i = 0;
            while i < len {
                let mut histo: u64 = 0;
                if i >= window_half {
                    histogram[*data
                        .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                        as usize] = (histogram[*data
                        .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                        as usize])
                        .wrapping_sub(1);
                    in_window = in_window.wrapping_sub(1);
                }
                if i.wrapping_add(window_half) < len {
                    histogram[*data
                        .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                        as usize] = (histogram[*data
                        .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                        as usize])
                        .wrapping_add(1);
                    in_window = in_window.wrapping_add(1);
                }
                histo = histogram[*data.offset((pos.wrapping_add(i) & mask) as isize) as usize];
                if histo == 0 {
                    histo = 1;
                }
                let mut lit_cost = FastLog2(in_window) - FastLog2(histo);
                lit_cost += 0.029f64;
                if lit_cost < 1.0f64 {
                    lit_cost *= 0.5f64;
                    lit_cost += 0.5f64;
                };
                *cost.offset(i as isize) = lit_cost as libc::c_float;
                i = i.wrapping_add(1);
            }
        };
    }
}
