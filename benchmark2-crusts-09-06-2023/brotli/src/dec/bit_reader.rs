use libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReader {
    pub val_: u64,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReaderState {
    pub val_: u64,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: u64,
}
#[inline(always)]
extern "C" fn BrotliBitReaderSaveState(
    from: *mut BrotliBitReader,
    mut to: *mut BrotliBitReaderState,
) {
    unsafe {
        (*to).val_ = (*from).val_;
        (*to).bit_pos_ = (*from).bit_pos_;
        let ref mut fresh0 = (*to).next_in;
        *fresh0 = (*from).next_in;
        (*to).avail_in = (*from).avail_in;
    }
}

#[inline(always)]
extern "C" fn BrotliGetBitsUnmasked(br: *mut BrotliBitReader) -> u64 {
    unsafe {
        return (*br).val_ >> (*br).bit_pos_;
    }
}

#[inline(always)]
extern "C" fn BrotliDropBits(br: *mut BrotliBitReader, mut n_bits: u32) {
    unsafe {
        let ref mut fresh1 = (*br).bit_pos_;
        *fresh1 = (*fresh1 as u32).wrapping_add(n_bits) as u32;
    }
}

#[inline(always)]
extern "C" fn BrotliTakeBits(br: *mut BrotliBitReader, mut n_bits: u32, mut val: *mut u32) {
    unsafe {
        *val = BrotliGetBitsUnmasked(br) as u32 & BitMask(n_bits);
        BrotliDropBits(br, n_bits);
    }
}

#[inline(always)]
extern "C" fn BrotliSafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: u32,
    mut val: *mut u32,
) -> i32 {
    unsafe {
        while BrotliGetAvailableBits(br) < n_bits {
            if BrotliPullByte(br) == 0 {
                return 0;
            }
        }
        BrotliTakeBits(br, n_bits, val);
        return 1;
    }
}

#[inline(always)]
extern "C" fn BrotliBitReaderRestoreState(
    to: *mut BrotliBitReader,
    mut from: *mut BrotliBitReaderState,
) {
    unsafe {
        (*to).val_ = (*from).val_;
        (*to).bit_pos_ = (*from).bit_pos_;
        let ref mut fresh2 = (*to).next_in;
        *fresh2 = (*from).next_in;
        (*to).avail_in = (*from).avail_in;
    }
}

#[inline(always)]
extern "C" fn BrotliGetAvailableBits(mut br: *const BrotliBitReader) -> u32 {
    unsafe {
        return ((if 1 != 0 { 64i32 } else { 32 }) as u32).wrapping_sub((*br).bit_pos_);
    }
}

#[inline(always)]
extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> i32 {
    unsafe {
        if (*br).avail_in == 0 {
            return 0;
        };
        (*br).val_ >>= 8;
        let ref mut fresh3 = (*br).val_;
        *fresh3 |= (*(*br).next_in as u64) << 56;
        let ref mut fresh4 = (*br).bit_pos_;
        *fresh4 = (*fresh4 as u32).wrapping_sub(8) as u32;
        let ref mut fresh5 = (*br).avail_in;
        *fresh5 = (*fresh5).wrapping_sub(1);
        let ref mut fresh6 = (*br).next_in;
        *fresh6 = (*fresh6).offset(1);
        return 1;
    }
}

#[inline(always)]
extern "C" fn BitMask(mut n: u32) -> u32 {
    unsafe {
        if 0 != 0 || 0 != 0 {
            return !(0xffffffff << n);
        } else {
            return kBrotliBitMask[n as usize];
        };
    }
}

#[no_mangle]
pub static mut kBrotliBitMask: [u32; 33] = [
    0, 0x1, 0x3, 0x7, 0xf, 0x1f, 0x3f, 0x7f, 0xff, 0x1ff, 0x3ff, 0x7ff, 0xfff, 0x1fff, 0x3fff,
    0x7fff, 0xffff, 0x1ffff, 0x3ffff, 0x7ffff, 0xfffff, 0x1fffff, 0x3fffff, 0x7fffff, 0xffffff,
    0x1ffffff, 0x3ffffff, 0x7ffffff, 0xfffffff, 0x1fffffff, 0x3fffffff, 0x7fffffff, 0xffffffff,
];
#[no_mangle]
pub extern "C" fn BrotliInitBitReader(br: *mut BrotliBitReader) {
    unsafe {
        (*br).val_ = 0;
        (*br).bit_pos_ = ((::std::mem::size_of::<u64>() as u64) << 3i32) as u32;
    }
}

#[no_mangle]
pub extern "C" fn BrotliWarmupBitReader(br: *mut BrotliBitReader) -> i32 {
    unsafe {
        let mut aligned_read_mask = (::std::mem::size_of::<u64>() as u64 >> 1i32).wrapping_sub(1);
        if 0 == 0 {
            aligned_read_mask = 0;
        }
        if BrotliGetAvailableBits(br) == 0 {
            if BrotliPullByte(br) == 0 {
                return 0;
            }
        }
        while (*br).next_in as u64 & aligned_read_mask != 0 {
            if BrotliPullByte(br) == 0 {
                return 1;
            }
        }
        return 1;
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn BrotliSafeReadBits32Slow(
    br: *mut BrotliBitReader,
    mut n_bits: u32,
    mut val: *mut u32,
) -> i32 {
    unsafe {
        let mut low_val: u32 = 0;
        let mut high_val: u32 = 0;
        let mut memento = BrotliBitReaderState {
            val_: 0,
            bit_pos_: 0,
            next_in: 0 as *const u8,
            avail_in: 0,
        };
        BrotliBitReaderSaveState(br, &mut memento);
        if BrotliSafeReadBits(br, 16, &mut low_val) == 0
            || BrotliSafeReadBits(br, n_bits.wrapping_sub(16), &mut high_val) == 0
        {
            BrotliBitReaderRestoreState(br, &mut memento);
            return 0;
        }
        *val = low_val | high_val << 16;
        return 1;
    }
}
