use libc;
extern "C" {
    static _kBrotliContextLookupTable: [u8; 2048];
}
pub const CONTEXT_SIGNED: u32 = 3;
pub const CONTEXT_UTF8: u32 = 2;
pub const CONTEXT_MSB6: u32 = 1;
pub const CONTEXT_LSB6: u32 = 0;
pub type ContextLut = *const u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: u32,
    pub copy_len_: u32,
    pub dist_extra_: u32,
    pub cmd_prefix_: u16,
    pub dist_prefix_: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types: u64,
    pub num_blocks: u64,
    pub types: *mut u8,
    pub lengths: *mut u32,
    pub types_alloc_size: u64,
    pub lengths_alloc_size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [u32; 256],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [u32; 704],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_: [u32; 544],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitIterator {
    pub split_: *const BlockSplit,
    pub idx_: u64,
    pub type_: u64,
    pub length_: u64,
}
#[inline(always)]
extern "C" fn HistogramAddDistance(mut self_0: *mut HistogramDistance, mut val: u64) {
    unsafe {
        let ref mut fresh0 = (*self_0).data_[val as usize];
        *fresh0 = (*fresh0).wrapping_add(1);
        let ref mut fresh1 = (*self_0).total_count_;
        *fresh1 = (*fresh1).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn HistogramAddCommand(mut self_0: *mut HistogramCommand, mut val: u64) {
    unsafe {
        let ref mut fresh2 = (*self_0).data_[val as usize];
        *fresh2 = (*fresh2).wrapping_add(1);
        let ref mut fresh3 = (*self_0).total_count_;
        *fresh3 = (*fresh3).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn HistogramAddLiteral(mut self_0: *mut HistogramLiteral, mut val: u64) {
    unsafe {
        let ref mut fresh4 = (*self_0).data_[val as usize];
        *fresh4 = (*fresh4).wrapping_add(1);
        let ref mut fresh5 = (*self_0).total_count_;
        *fresh5 = (*fresh5).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn CommandDistanceContext(mut self_0: *const Command) -> u32 {
    unsafe {
        let mut r = ((*self_0).cmd_prefix_ as i32 >> 6i32) as u32;
        let mut c = ((*self_0).cmd_prefix_ as i32 & 7i32) as u32;
        if (r == 0 || r == 2 || r == 4 || r == 7) && c <= 2 {
            return c;
        }
        return 3;
    }
}

#[inline(always)]
extern "C" fn CommandCopyLen(mut self_0: *const Command) -> u32 {
    unsafe {
        return (*self_0).copy_len_ & 0x1ffffff;
    }
}

extern "C" fn InitBlockSplitIterator(
    mut self_0: *mut BlockSplitIterator,
    mut split: *const BlockSplit,
) {
    unsafe {
        let ref mut fresh6 = (*self_0).split_;
        *fresh6 = split;
        (*self_0).idx_ = 0;
        (*self_0).type_ = 0;
        (*self_0).length_ = (if !((*split).lengths).is_null() {
            *((*split).lengths).offset(0 as isize)
        } else {
            0
        }) as u64;
    }
}

extern "C" fn BlockSplitIteratorNext(mut self_0: *mut BlockSplitIterator) {
    unsafe {
        if (*self_0).length_ == 0 {
            let ref mut fresh7 = (*self_0).idx_;
            *fresh7 = (*fresh7).wrapping_add(1);
            (*self_0).type_ = *((*(*self_0).split_).types).offset((*self_0).idx_ as isize) as u64;
            (*self_0).length_ =
                *((*(*self_0).split_).lengths).offset((*self_0).idx_ as isize) as u64;
        }
        let ref mut fresh8 = (*self_0).length_;
        *fresh8 = (*fresh8).wrapping_sub(1);
    }
}

#[no_mangle]
pub extern "C" fn BrotliBuildHistogramsWithContext(
    mut cmds: *const Command,
    num_commands: u64,
    mut literal_split: *const BlockSplit,
    mut insert_and_copy_split: *const BlockSplit,
    mut dist_split: *const BlockSplit,
    mut ringbuffer: *const u8,
    mut start_pos: u64,
    mut mask: u64,
    mut prev_byte: u8,
    mut prev_byte2: u8,
    mut context_modes: *const u32,
    mut literal_histograms: *mut HistogramLiteral,
    mut insert_and_copy_histograms: *mut HistogramCommand,
    mut copy_dist_histograms: *mut HistogramDistance,
) {
    unsafe {
        let mut pos = start_pos;
        let mut literal_it = BlockSplitIterator {
            split_: 0 as *const BlockSplit,
            idx_: 0,
            type_: 0,
            length_: 0,
        };
        let mut insert_and_copy_it = BlockSplitIterator {
            split_: 0 as *const BlockSplit,
            idx_: 0,
            type_: 0,
            length_: 0,
        };
        let mut dist_it = BlockSplitIterator {
            split_: 0 as *const BlockSplit,
            idx_: 0,
            type_: 0,
            length_: 0,
        };
        let mut i: u64 = 0;
        InitBlockSplitIterator(&mut literal_it, literal_split);
        InitBlockSplitIterator(&mut insert_and_copy_it, insert_and_copy_split);
        InitBlockSplitIterator(&mut dist_it, dist_split);
        i = 0;
        while i < num_commands {
            let mut cmd: *const Command = &*cmds.offset(i as isize) as *const Command;
            let mut j: u64 = 0;
            BlockSplitIteratorNext(&mut insert_and_copy_it);
            HistogramAddCommand(
                &mut *insert_and_copy_histograms.offset(insert_and_copy_it.type_ as isize),
                (*cmd).cmd_prefix_ as u64,
            );
            j = (*cmd).insert_len_ as u64;
            while j != 0 {
                let mut context: u64 = 0;
                BlockSplitIteratorNext(&mut literal_it);
                context = literal_it.type_;
                if !context_modes.is_null() {
                    let mut lut: ContextLut = &*_kBrotliContextLookupTable
                        .as_ptr()
                        .offset(((*context_modes.offset(context as isize) as u32) << 9i32) as isize)
                        as *const u8;
                    context = (context << 6i32).wrapping_add(
                        (*lut.offset(prev_byte as isize) as i32
                            | *lut.offset(256 as isize).offset(prev_byte2 as isize) as i32)
                            as u64,
                    );
                }
                HistogramAddLiteral(
                    &mut *literal_histograms.offset(context as isize),
                    *ringbuffer.offset((pos & mask) as isize) as u64,
                );
                prev_byte2 = prev_byte;
                prev_byte = *ringbuffer.offset((pos & mask) as isize);
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
            pos = (pos as u64).wrapping_add(CommandCopyLen(cmd) as u64) as u64;
            if CommandCopyLen(cmd) != 0 {
                prev_byte2 = *ringbuffer.offset((pos.wrapping_sub(2u64) & mask) as isize);
                prev_byte = *ringbuffer.offset((pos.wrapping_sub(1u64) & mask) as isize);
                if (*cmd).cmd_prefix_ as i32 >= 128 {
                    let mut context_0: u64 = 0;
                    BlockSplitIteratorNext(&mut dist_it);
                    context_0 =
                        (dist_it.type_ << 2i32).wrapping_add(CommandDistanceContext(cmd) as u64);
                    HistogramAddDistance(
                        &mut *copy_dist_histograms.offset(context_0 as isize),
                        ((*cmd).dist_prefix_ as i32 & 0x3ffi32) as u64,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
