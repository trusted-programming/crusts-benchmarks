use libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: u32,
    pub index_left_: i16,
    pub index_right_or_value_: i16,
}
pub type HuffmanTreeComparator =
    Option<unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> i32>;
#[inline(always)]
extern "C" fn brotli_max_uint32_t(mut a: u32, mut b: u32) -> u32 {
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn InitHuffmanTree(
    mut self_0: *mut HuffmanTree,
    mut count: u32,
    mut left: i16,
    mut right: i16,
) {
    unsafe {
        (*self_0).total_count_ = count;
        (*self_0).index_left_ = left;
        (*self_0).index_right_or_value_ = right;
    }
}

#[inline(always)]
extern "C" fn SortHuffmanTreeItems(
    mut items: *mut HuffmanTree,
    n: u64,
    mut comparator: HuffmanTreeComparator,
) {
    unsafe {
        if n < 13 {
            let mut i: u64 = 0;
            i = 1;
            while i < n {
                let mut tmp = *items.offset(i as isize);
                let mut k = i;
                let mut j = i.wrapping_sub(1);
                while comparator.expect("non-null function pointer")(
                    &mut tmp,
                    &mut *items.offset(j as isize),
                ) != 0
                {
                    *items.offset(k as isize) = *items.offset(j as isize);
                    k = j;
                    let fresh0 = j;
                    j = j.wrapping_sub(1);
                    if fresh0 == 0 {
                        break;
                    }
                }
                *items.offset(k as isize) = tmp;
                i = i.wrapping_add(1);
            }
            return;
        } else {
            let mut g = if n < 57 { 2 } else { 0 };
            while g < 6 {
                let mut gap = kBrotliShellGaps[g as usize];
                let mut i_0: u64 = 0;
                i_0 = gap;
                while i_0 < n {
                    let mut j_0 = i_0;
                    let mut tmp_0 = *items.offset(i_0 as isize);
                    while j_0 >= gap
                        && comparator.expect("non-null function pointer")(
                            &mut tmp_0,
                            &mut *items.offset(j_0.wrapping_sub(gap) as isize),
                        ) != 0
                    {
                        *items.offset(j_0 as isize) = *items.offset(j_0.wrapping_sub(gap) as isize);
                        j_0 = (j_0 as u64).wrapping_sub(gap) as u64;
                    }
                    *items.offset(j_0 as isize) = tmp_0;
                    i_0 = i_0.wrapping_add(1);
                }
                g += 1;
            }
        };
    }
}

#[no_mangle]
pub static mut kBrotliShellGaps: [u64; 6] = [132, 57, 23, 10, 4, 1];
#[no_mangle]
pub extern "C" fn BrotliSetDepth(
    mut p0: i32,
    mut pool: *mut HuffmanTree,
    mut depth: *mut u8,
    mut max_depth: i32,
) -> i32 {
    unsafe {
        let mut stack: [i32; 16] = [0; 16];
        let mut level = 0;
        let mut p = p0;
        stack[0 as usize] = -1;
        loop {
            if (*pool.offset(p as isize)).index_left_ as i32 >= 0 {
                level += 1;
                if level > max_depth {
                    return 0;
                }
                stack[level as usize] = (*pool.offset(p as isize)).index_right_or_value_ as i32;
                p = (*pool.offset(p as isize)).index_left_ as i32;
            } else {
                *depth.offset((*pool.offset(p as isize)).index_right_or_value_ as isize) =
                    level as u8;
                while level >= 0 && stack[level as usize] == -1 {
                    level -= 1;
                }
                if level < 0 {
                    return 1;
                }
                p = stack[level as usize];
                stack[level as usize] = -1;
            }
        }
    }
}

#[inline(always)]
extern "C" fn SortHuffmanTree(mut v0: *const HuffmanTree, mut v1: *const HuffmanTree) -> i32 {
    unsafe {
        if (*v0).total_count_ != (*v1).total_count_ {
            return if (*v0).total_count_ < (*v1).total_count_ {
                1
            } else {
                0
            };
        }
        return if (*v0).index_right_or_value_ as i32 > (*v1).index_right_or_value_ as i32 {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliCreateHuffmanTree(
    mut data: *const u32,
    length: u64,
    tree_limit: i32,
    mut tree: *mut HuffmanTree,
    mut depth: *mut u8,
) {
    unsafe {
        let mut count_limit: u32 = 0;
        let mut sentinel = HuffmanTree {
            total_count_: 0,
            index_left_: 0,
            index_right_or_value_: 0,
        };
        InitHuffmanTree(&mut sentinel, !0, -1 as i16, -1 as i16);
        count_limit = 1;
        loop {
            let mut n = 0;
            let mut i: u64 = 0;
            let mut j: u64 = 0;
            let mut k: u64 = 0;
            i = length;
            while i != 0 {
                i = i.wrapping_sub(1);
                if *data.offset(i as isize) != 0 {
                    let count = brotli_max_uint32_t(*data.offset(i as isize), count_limit);
                    let fresh1 = n;
                    n = n.wrapping_add(1);
                    InitHuffmanTree(
                        &mut *tree.offset(fresh1 as isize),
                        count,
                        -1 as i16,
                        i as i16,
                    );
                }
            }
            if n == 1 {
                *depth.offset((*tree.offset(0 as isize)).index_right_or_value_ as isize) = 1;
                break;
            } else {
                SortHuffmanTreeItems(
                    tree,
                    n,
                    Some(
                        SortHuffmanTree
                            as unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> i32,
                    ),
                );
                *tree.offset(n as isize) = sentinel;
                *tree.offset(n.wrapping_add(1) as isize) = sentinel;
                i = 0;
                j = n.wrapping_add(1);
                k = n.wrapping_sub(1);
                while k != 0 {
                    let mut left: u64 = 0;
                    let mut right: u64 = 0;
                    if (*tree.offset(i as isize)).total_count_
                        <= (*tree.offset(j as isize)).total_count_
                    {
                        left = i;
                        i = i.wrapping_add(1);
                    } else {
                        left = j;
                        j = j.wrapping_add(1);
                    }
                    if (*tree.offset(i as isize)).total_count_
                        <= (*tree.offset(j as isize)).total_count_
                    {
                        right = i;
                        i = i.wrapping_add(1);
                    } else {
                        right = j;
                        j = j.wrapping_add(1);
                    }
                    let mut j_end = 2u64.wrapping_mul(n).wrapping_sub(k);
                    (*tree.offset(j_end as isize)).total_count_ = ((*tree.offset(left as isize))
                        .total_count_)
                        .wrapping_add((*tree.offset(right as isize)).total_count_);
                    (*tree.offset(j_end as isize)).index_left_ = left as i16;
                    (*tree.offset(j_end as isize)).index_right_or_value_ = right as i16;
                    *tree.offset(j_end.wrapping_add(1) as isize) = sentinel;
                    k = k.wrapping_sub(1);
                }
                if BrotliSetDepth(
                    2u64.wrapping_mul(n).wrapping_sub(1) as i32,
                    &mut *tree.offset(0 as isize),
                    depth,
                    tree_limit,
                ) != 0
                {
                    break;
                }
                count_limit = (count_limit).wrapping_mul(2) as u32;
            }
        }
    }
}

extern "C" fn Reverse(mut v: *mut u8, mut start: u64, mut end: u64) {
    unsafe {
        end = end.wrapping_sub(1);
        while start < end {
            let mut tmp = *v.offset(start as isize);
            *v.offset(start as isize) = *v.offset(end as isize);
            *v.offset(end as isize) = tmp;
            start = start.wrapping_add(1);
            end = end.wrapping_sub(1);
        }
    }
}

extern "C" fn BrotliWriteHuffmanTreeRepetitions(
    previous_value: u8,
    value: u8,
    mut repetitions: u64,
    mut tree_size: *mut u64,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    unsafe {
        if previous_value as i32 != value as i32 {
            *tree.offset(*tree_size as isize) = value;
            *extra_bits_data.offset(*tree_size as isize) = 0;
            *tree_size = (*tree_size).wrapping_add(1);
            repetitions = repetitions.wrapping_sub(1);
        }
        if repetitions == 7 {
            *tree.offset(*tree_size as isize) = value;
            *extra_bits_data.offset(*tree_size as isize) = 0;
            *tree_size = (*tree_size).wrapping_add(1);
            repetitions = repetitions.wrapping_sub(1);
        }
        if repetitions < 3 {
            let mut i: u64 = 0;
            i = 0;
            while i < repetitions {
                *tree.offset(*tree_size as isize) = value;
                *extra_bits_data.offset(*tree_size as isize) = 0;
                *tree_size = (*tree_size).wrapping_add(1);
                i = i.wrapping_add(1);
            }
        } else {
            let mut start = *tree_size;
            repetitions = (repetitions as u64).wrapping_sub(3) as u64;
            loop {
                *tree.offset(*tree_size as isize) = 16;
                *extra_bits_data.offset(*tree_size as isize) = (repetitions & 0x3u64) as u8;
                *tree_size = (*tree_size).wrapping_add(1);
                repetitions >>= 2;
                if repetitions == 0 {
                    break;
                }
                repetitions = repetitions.wrapping_sub(1);
            }
            Reverse(tree, start, *tree_size);
            Reverse(extra_bits_data, start, *tree_size);
        };
    }
}

extern "C" fn BrotliWriteHuffmanTreeRepetitionsZeros(
    mut repetitions: u64,
    mut tree_size: *mut u64,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    unsafe {
        if repetitions == 11 {
            *tree.offset(*tree_size as isize) = 0;
            *extra_bits_data.offset(*tree_size as isize) = 0;
            *tree_size = (*tree_size).wrapping_add(1);
            repetitions = repetitions.wrapping_sub(1);
        }
        if repetitions < 3 {
            let mut i: u64 = 0;
            i = 0;
            while i < repetitions {
                *tree.offset(*tree_size as isize) = 0;
                *extra_bits_data.offset(*tree_size as isize) = 0;
                *tree_size = (*tree_size).wrapping_add(1);
                i = i.wrapping_add(1);
            }
        } else {
            let mut start = *tree_size;
            repetitions = (repetitions as u64).wrapping_sub(3) as u64;
            loop {
                *tree.offset(*tree_size as isize) = 17;
                *extra_bits_data.offset(*tree_size as isize) = (repetitions & 0x7u64) as u8;
                *tree_size = (*tree_size).wrapping_add(1);
                repetitions >>= 3;
                if repetitions == 0 {
                    break;
                }
                repetitions = repetitions.wrapping_sub(1);
            }
            Reverse(tree, start, *tree_size);
            Reverse(extra_bits_data, start, *tree_size);
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliOptimizeHuffmanCountsForRle(
    mut length: u64,
    mut counts: *mut u32,
    mut good_for_rle: *mut u8,
) {
    unsafe {
        let mut nonzero_count = 0;
        let mut stride: u64 = 0;
        let mut limit: u64 = 0;
        let mut sum: u64 = 0;
        let streak_limit = 1240;
        let mut i: u64 = 0;
        i = 0;
        while i < length {
            if *counts.offset(i as isize) != 0 {
                nonzero_count = nonzero_count.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if nonzero_count < 16 {
            return;
        }
        while length != 0 && *counts.offset(length.wrapping_sub(1) as isize) == 0 {
            length = length.wrapping_sub(1);
        }
        if length == 0 {
            return;
        }
        let mut nonzeros = 0;
        let mut smallest_nonzero = (1i32 << 30) as u32;
        i = 0;
        while i < length {
            if *counts.offset(i as isize) != 0 {
                nonzeros = nonzeros.wrapping_add(1);
                if smallest_nonzero > *counts.offset(i as isize) {
                    smallest_nonzero = *counts.offset(i as isize);
                }
            }
            i = i.wrapping_add(1);
        }
        if nonzeros < 5 {
            return;
        }
        if smallest_nonzero < 4 {
            let mut zeros = length.wrapping_sub(nonzeros);
            if zeros < 6 {
                i = 1;
                while i < length.wrapping_sub(1) {
                    if *counts.offset(i.wrapping_sub(1) as isize) != 0
                        && *counts.offset(i as isize) == 0
                        && *counts.offset(i.wrapping_add(1) as isize) != 0
                    {
                        *counts.offset(i as isize) = 1;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        if nonzeros < 28 {
            return;
        }
        memset(good_for_rle as *mut libc::c_void, 0, length);
        let mut symbol = *counts.offset(0 as isize);
        let mut step = 0;
        i = 0;
        while i <= length {
            if i == length || *counts.offset(i as isize) != symbol {
                if symbol == 0 && step >= 5 || symbol != 0 && step >= 7 {
                    let mut k: u64 = 0;
                    k = 0;
                    while k < step {
                        *good_for_rle.offset(i.wrapping_sub(k).wrapping_sub(1) as isize) = 1;
                        k = k.wrapping_add(1);
                    }
                }
                step = 1;
                if i != length {
                    symbol = *counts.offset(i as isize);
                }
            } else {
                step = step.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        stride = 0;
        limit = 256u32
            .wrapping_mul(
                (*counts.offset(0 as isize))
                    .wrapping_add(*counts.offset(1 as isize))
                    .wrapping_add(*counts.offset(2 as isize)),
            )
            .wrapping_div(3)
            .wrapping_add(420) as u64;
        sum = 0;
        i = 0;
        while i <= length {
            if i == length
                || *good_for_rle.offset(i as isize) as i32 != 0
                || i != 0 && *good_for_rle.offset(i.wrapping_sub(1) as isize) as i32 != 0
                || (256u32.wrapping_mul(*counts.offset(i as isize)) as u64)
                    .wrapping_sub(limit)
                    .wrapping_add(streak_limit)
                    >= 2u64.wrapping_mul(streak_limit)
            {
                if stride >= 4 || stride >= 3 && sum == 0 {
                    let mut k_0: u64 = 0;
                    let mut count = sum
                        .wrapping_add(stride.wrapping_div(2))
                        .wrapping_div(stride);
                    if count == 0 {
                        count = 1;
                    }
                    if sum == 0 {
                        count = 0;
                    }
                    k_0 = 0;
                    while k_0 < stride {
                        *counts.offset(i.wrapping_sub(k_0).wrapping_sub(1) as isize) = count as u32;
                        k_0 = k_0.wrapping_add(1);
                    }
                }
                stride = 0;
                sum = 0;
                if i < length.wrapping_sub(2) {
                    limit = 256u32
                        .wrapping_mul(
                            (*counts.offset(i as isize))
                                .wrapping_add(*counts.offset(i.wrapping_add(1) as isize))
                                .wrapping_add(*counts.offset(i.wrapping_add(2) as isize)),
                        )
                        .wrapping_div(3)
                        .wrapping_add(420) as u64;
                } else if i < length {
                    limit = 256u32.wrapping_mul(*counts.offset(i as isize)) as u64;
                } else {
                    limit = 0;
                }
            }
            stride = stride.wrapping_add(1);
            if i != length {
                sum = (sum).wrapping_add(*counts.offset(i as isize) as u64) as u64;
                if stride >= 4 {
                    limit = 256u64
                        .wrapping_mul(sum)
                        .wrapping_add(stride.wrapping_div(2))
                        .wrapping_div(stride);
                }
                if stride == 4 {
                    limit = (limit).wrapping_add(120) as u64;
                }
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn DecideOverRleUse(
    mut depth: *const u8,
    length: u64,
    mut use_rle_for_non_zero: *mut i32,
    mut use_rle_for_zero: *mut i32,
) {
    unsafe {
        let mut total_reps_zero = 0;
        let mut total_reps_non_zero = 0;
        let mut count_reps_zero = 1;
        let mut count_reps_non_zero = 1;
        let mut i: u64 = 0;
        i = 0;
        while i < length {
            let value = *depth.offset(i as isize);
            let mut reps = 1;
            let mut k: u64 = 0;
            k = i.wrapping_add(1);
            while k < length && *depth.offset(k as isize) as i32 == value as i32 {
                reps = reps.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            if reps >= 3 && value as i32 == 0 {
                total_reps_zero = (total_reps_zero as u64).wrapping_add(reps) as u64;
                count_reps_zero = count_reps_zero.wrapping_add(1);
            }
            if reps >= 4 && value as i32 != 0 {
                total_reps_non_zero = (total_reps_non_zero as u64).wrapping_add(reps) as u64;
                count_reps_non_zero = count_reps_non_zero.wrapping_add(1);
            }
            i = (i).wrapping_add(reps) as u64;
        }
        *use_rle_for_non_zero = if total_reps_non_zero > count_reps_non_zero.wrapping_mul(2) {
            1
        } else {
            0
        };
        *use_rle_for_zero = if total_reps_zero > count_reps_zero.wrapping_mul(2) {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliWriteHuffmanTree(
    mut depth: *const u8,
    mut length: u64,
    mut tree_size: *mut u64,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    unsafe {
        let mut previous_value = 8;
        let mut i: u64 = 0;
        let mut use_rle_for_non_zero = 0;
        let mut use_rle_for_zero = 0;
        let mut new_length = length;
        i = 0;
        while i < length {
            if !(*depth.offset(length.wrapping_sub(i).wrapping_sub(1) as isize) as i32 == 0) {
                break;
            }
            new_length = new_length.wrapping_sub(1);
            i = i.wrapping_add(1);
        }
        if length > 50 {
            DecideOverRleUse(
                depth,
                new_length,
                &mut use_rle_for_non_zero,
                &mut use_rle_for_zero,
            );
        }
        i = 0;
        while i < new_length {
            let value = *depth.offset(i as isize);
            let mut reps = 1;
            if value as i32 != 0 && use_rle_for_non_zero != 0
                || value as i32 == 0 && use_rle_for_zero != 0
            {
                let mut k: u64 = 0;
                k = i.wrapping_add(1);
                while k < new_length && *depth.offset(k as isize) as i32 == value as i32 {
                    reps = reps.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
            }
            if value as i32 == 0 {
                BrotliWriteHuffmanTreeRepetitionsZeros(reps, tree_size, tree, extra_bits_data);
            } else {
                BrotliWriteHuffmanTreeRepetitions(
                    previous_value,
                    value,
                    reps,
                    tree_size,
                    tree,
                    extra_bits_data,
                );
                previous_value = value;
            }
            i = (i).wrapping_add(reps) as u64;
        }
    }
}

extern "C" fn BrotliReverseBits(mut num_bits: u64, mut bits: u16) -> u16 {
    unsafe {
        static mut kLut: [u64; 16] = [
            0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe, 0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf,
        ];
        let mut retval = kLut[(bits as i32 & 0xfi32) as usize];
        let mut i: u64 = 0;
        i = 4;
        while i < num_bits {
            retval <<= 4;
            bits = (bits as i32 >> 4i32) as u16;
            retval |= kLut[(bits as i32 & 0xfi32) as usize];
            i = (i).wrapping_add(4) as u64;
        }
        retval >>= 0u64.wrapping_sub(num_bits) & 0x3;
        return retval as u16;
    }
}

#[no_mangle]
pub extern "C" fn BrotliConvertBitDepthsToSymbols(
    mut depth: *const u8,
    mut len: u64,
    mut bits: *mut u16,
) {
    unsafe {
        let mut bl_count: [u16; 16] = [0; 16];
        let mut next_code: [u16; 16] = [0; 16];
        let mut i: u64 = 0;
        let mut code = 0;
        i = 0;
        while i < len {
            bl_count[*depth.offset(i as isize) as usize] =
                (bl_count[*depth.offset(i as isize) as usize]).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        bl_count[0 as usize] = 0;
        next_code[0 as usize] = 0;
        i = 1;
        while i < 16 {
            code = (code + bl_count[i.wrapping_sub(1) as usize] as i32) << 1;
            next_code[i as usize] = code as u16;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < len {
            if *depth.offset(i as isize) != 0 {
                let fresh2 = next_code[*depth.offset(i as isize) as usize];
                next_code[*depth.offset(i as isize) as usize] =
                    (next_code[*depth.offset(i as isize) as usize]).wrapping_add(1);
                *bits.offset(i as isize) =
                    BrotliReverseBits(*depth.offset(i as isize) as u64, fresh2);
            }
            i = i.wrapping_add(1);
        }
    }
}
