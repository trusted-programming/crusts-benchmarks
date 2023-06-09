use libc;
extern "C" {
    fn log2(_: f64) -> f64;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static kBrotliLog2Table: [f64; 256];
    fn BrotliAllocate(m: *mut MemoryManager, n: u64) -> *mut libc::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
    fn BrotliPopulationCostLiteral(_: *const HistogramLiteral) -> f64;
    fn BrotliPopulationCostCommand(_: *const HistogramCommand) -> f64;
    fn BrotliPopulationCostDistance(_: *const HistogramDistance) -> f64;
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
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
pub struct HistogramPair {
    pub idx1: u32,
    pub idx2: u32,
    pub cost_combo: f64,
    pub cost_diff: f64,
}
#[inline(always)]
extern "C" fn brotli_min_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_max_double(mut a: f64, mut b: f64) -> f64 {
    return if a > b { a } else { b };
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

#[inline(always)]
extern "C" fn HistogramClearLiteral(mut self_0: *mut HistogramLiteral) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 256]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut HistogramLiteral,
    mut v: *const HistogramLiteral,
) {
    unsafe {
        let mut i: u64 = 0;
        let ref mut fresh0 = (*self_0).total_count_;
        *fresh0 = (*fresh0 as u64).wrapping_add((*v).total_count_) as u64;
        i = 0;
        while i < 256 {
            let ref mut fresh1 = (*self_0).data_[i as usize];
            *fresh1 = (*fresh1 as u32).wrapping_add((*v).data_[i as usize]) as u32;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn HistogramClearCommand(mut self_0: *mut HistogramCommand) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 704]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddHistogramCommand(
    mut self_0: *mut HistogramCommand,
    mut v: *const HistogramCommand,
) {
    unsafe {
        let mut i: u64 = 0;
        let ref mut fresh2 = (*self_0).total_count_;
        *fresh2 = (*fresh2 as u64).wrapping_add((*v).total_count_) as u64;
        i = 0;
        while i < 704 {
            let ref mut fresh3 = (*self_0).data_[i as usize];
            *fresh3 = (*fresh3 as u32).wrapping_add((*v).data_[i as usize]) as u32;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn HistogramClearDistance(mut self_0: *mut HistogramDistance) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 544]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut HistogramDistance,
    mut v: *const HistogramDistance,
) {
    unsafe {
        let mut i: u64 = 0;
        let ref mut fresh4 = (*self_0).total_count_;
        *fresh4 = (*fresh4 as u64).wrapping_add((*v).total_count_) as u64;
        i = 0;
        while i < 544 {
            let ref mut fresh5 = (*self_0).data_[i as usize];
            *fresh5 = (*fresh5 as u32).wrapping_add((*v).data_[i as usize]) as u32;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn HistogramPairIsLess(
    mut p1: *const HistogramPair,
    mut p2: *const HistogramPair,
) -> i32 {
    unsafe {
        if (*p1).cost_diff != (*p2).cost_diff {
            return if (*p1).cost_diff > (*p2).cost_diff {
                1
            } else {
                0
            };
        }
        return if ((*p1).idx2).wrapping_sub((*p1).idx1) > ((*p2).idx2).wrapping_sub((*p2).idx1) {
            1
        } else {
            0
        };
    }
}

#[inline(always)]
extern "C" fn ClusterCostDiff(mut size_a: u64, mut size_b: u64) -> f64 {
    let mut size_c = size_a.wrapping_add(size_b);
    return size_a as f64 * FastLog2(size_a) + size_b as f64 * FastLog2(size_b)
        - size_c as f64 * FastLog2(size_c);
}

#[no_mangle]
pub extern "C" fn BrotliCompareAndPushToQueueLiteral(
    mut out: *const HistogramLiteral,
    mut cluster_size: *const u32,
    mut idx1: u32,
    mut idx2: u32,
    mut max_num_pairs: u64,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut u64,
) {
    unsafe {
        let mut is_good_pair = 0;
        let mut p = HistogramPair {
            idx1: 0,
            idx2: 0,
            cost_combo: 0.,
            cost_diff: 0.,
        };
        p.idx2 = 0;
        p.idx1 = p.idx2;
        p.cost_combo = 0 as f64;
        p.cost_diff = p.cost_combo;
        if idx1 == idx2 {
            return;
        }
        if idx2 < idx1 {
            let mut t = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64
            * ClusterCostDiff(
                *cluster_size.offset(idx1 as isize) as u64,
                *cluster_size.offset(idx2 as isize) as u64,
            );
        p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
        p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
        if (*out.offset(idx1 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
            is_good_pair = 1;
        } else if (*out.offset(idx2 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
            is_good_pair = 1;
        } else {
            let mut threshold = if *num_pairs == 0 {
                1e99f64
            } else {
                brotli_max_double(0.0f64, (*pairs.offset(0 as isize)).cost_diff)
            };
            let mut combo = *out.offset(idx1 as isize);
            let mut cost_combo: f64 = 0.;
            HistogramAddHistogramLiteral(&mut combo, &*out.offset(idx2 as isize));
            cost_combo = BrotliPopulationCostLiteral(&mut combo);
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff += p.cost_combo;
            if *num_pairs > 0 && HistogramPairIsLess(&mut *pairs.offset(0 as isize), &mut p) != 0 {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as isize) = *pairs.offset(0 as isize);
                    *num_pairs = (*num_pairs).wrapping_add(1);
                };
                *pairs.offset(0 as isize) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) = p;
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliCompareAndPushToQueueCommand(
    mut out: *const HistogramCommand,
    mut cluster_size: *const u32,
    mut idx1: u32,
    mut idx2: u32,
    mut max_num_pairs: u64,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut u64,
) {
    unsafe {
        let mut is_good_pair = 0;
        let mut p = HistogramPair {
            idx1: 0,
            idx2: 0,
            cost_combo: 0.,
            cost_diff: 0.,
        };
        p.idx2 = 0;
        p.idx1 = p.idx2;
        p.cost_combo = 0 as f64;
        p.cost_diff = p.cost_combo;
        if idx1 == idx2 {
            return;
        }
        if idx2 < idx1 {
            let mut t = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64
            * ClusterCostDiff(
                *cluster_size.offset(idx1 as isize) as u64,
                *cluster_size.offset(idx2 as isize) as u64,
            );
        p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
        p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
        if (*out.offset(idx1 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
            is_good_pair = 1;
        } else if (*out.offset(idx2 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
            is_good_pair = 1;
        } else {
            let mut threshold = if *num_pairs == 0 {
                1e99f64
            } else {
                brotli_max_double(0.0f64, (*pairs.offset(0 as isize)).cost_diff)
            };
            let mut combo = *out.offset(idx1 as isize);
            let mut cost_combo: f64 = 0.;
            HistogramAddHistogramCommand(&mut combo, &*out.offset(idx2 as isize));
            cost_combo = BrotliPopulationCostCommand(&mut combo);
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff += p.cost_combo;
            if *num_pairs > 0 && HistogramPairIsLess(&mut *pairs.offset(0 as isize), &mut p) != 0 {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as isize) = *pairs.offset(0 as isize);
                    *num_pairs = (*num_pairs).wrapping_add(1);
                };
                *pairs.offset(0 as isize) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) = p;
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliCompareAndPushToQueueDistance(
    mut out: *const HistogramDistance,
    mut cluster_size: *const u32,
    mut idx1: u32,
    mut idx2: u32,
    mut max_num_pairs: u64,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut u64,
) {
    unsafe {
        let mut is_good_pair = 0;
        let mut p = HistogramPair {
            idx1: 0,
            idx2: 0,
            cost_combo: 0.,
            cost_diff: 0.,
        };
        p.idx2 = 0;
        p.idx1 = p.idx2;
        p.cost_combo = 0 as f64;
        p.cost_diff = p.cost_combo;
        if idx1 == idx2 {
            return;
        }
        if idx2 < idx1 {
            let mut t = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64
            * ClusterCostDiff(
                *cluster_size.offset(idx1 as isize) as u64,
                *cluster_size.offset(idx2 as isize) as u64,
            );
        p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
        p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
        if (*out.offset(idx1 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
            is_good_pair = 1;
        } else if (*out.offset(idx2 as isize)).total_count_ == 0 {
            p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
            is_good_pair = 1;
        } else {
            let mut threshold = if *num_pairs == 0 {
                1e99f64
            } else {
                brotli_max_double(0.0f64, (*pairs.offset(0 as isize)).cost_diff)
            };
            let mut combo = *out.offset(idx1 as isize);
            let mut cost_combo: f64 = 0.;
            HistogramAddHistogramDistance(&mut combo, &*out.offset(idx2 as isize));
            cost_combo = BrotliPopulationCostDistance(&mut combo);
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff += p.cost_combo;
            if *num_pairs > 0 && HistogramPairIsLess(&mut *pairs.offset(0 as isize), &mut p) != 0 {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as isize) = *pairs.offset(0 as isize);
                    *num_pairs = (*num_pairs).wrapping_add(1);
                };
                *pairs.offset(0 as isize) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) = p;
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramCombineLiteral(
    mut out: *mut HistogramLiteral,
    mut cluster_size: *mut u32,
    mut symbols: *mut u32,
    mut clusters: *mut u32,
    mut pairs: *mut HistogramPair,
    mut num_clusters: u64,
    mut symbols_size: u64,
    mut max_clusters: u64,
    mut max_num_pairs: u64,
) -> u64 {
    unsafe {
        let mut cost_diff_threshold = 0.0f64;
        let mut min_cluster_size = 1;
        let mut num_pairs = 0;
        let mut idx1: u64 = 0;
        idx1 = 0;
        while idx1 < num_clusters {
            let mut idx2: u64 = 0;
            idx2 = idx1.wrapping_add(1);
            while idx2 < num_clusters {
                BrotliCompareAndPushToQueueLiteral(
                    out,
                    cluster_size,
                    *clusters.offset(idx1 as isize),
                    *clusters.offset(idx2 as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as isize),
                    &mut num_pairs,
                );
                idx2 = idx2.wrapping_add(1);
            }
            idx1 = idx1.wrapping_add(1);
        }
        while num_clusters > min_cluster_size {
            let mut best_idx1: u32 = 0;
            let mut best_idx2: u32 = 0;
            let mut i: u64 = 0;
            if (*pairs.offset(0 as isize)).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
            } else {
                best_idx1 = (*pairs.offset(0 as isize)).idx1;
                best_idx2 = (*pairs.offset(0 as isize)).idx2;
                HistogramAddHistogramLiteral(
                    &mut *out.offset(best_idx1 as isize),
                    &mut *out.offset(best_idx2 as isize),
                );
                (*out.offset(best_idx1 as isize)).bit_cost_ =
                    (*pairs.offset(0 as isize)).cost_combo;
                let ref mut fresh6 = *cluster_size.offset(best_idx1 as isize);
                *fresh6 =
                    (*fresh6 as u32).wrapping_add(*cluster_size.offset(best_idx2 as isize)) as u32;
                i = 0;
                while i < symbols_size {
                    if *symbols.offset(i as isize) == best_idx2 {
                        *symbols.offset(i as isize) = best_idx1;
                    }
                    i = i.wrapping_add(1);
                }
                i = 0;
                while i < num_clusters {
                    if *clusters.offset(i as isize) == best_idx2 {
                        memmove(
                            &mut *clusters.offset(i as isize) as *mut u32 as *mut libc::c_void,
                            &mut *clusters.offset(i.wrapping_add(1) as isize) as *mut u32
                                as *const libc::c_void,
                            num_clusters
                                .wrapping_sub(i)
                                .wrapping_sub(1)
                                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1);
                let mut copy_to_idx = 0;
                i = 0;
                while i < num_pairs {
                    let mut p: *mut HistogramPair =
                        &mut *pairs.offset(i as isize) as *mut HistogramPair;
                    if !((*p).idx1 == best_idx1
                        || (*p).idx2 == best_idx1
                        || (*p).idx1 == best_idx2
                        || (*p).idx2 == best_idx2)
                    {
                        if HistogramPairIsLess(&mut *pairs.offset(0 as isize), p) != 0 {
                            let mut front = *pairs.offset(0 as isize);
                            *pairs.offset(0 as isize) = *p;
                            *pairs.offset(copy_to_idx as isize) = front;
                        } else {
                            *pairs.offset(copy_to_idx as isize) = *p;
                        }
                        copy_to_idx = copy_to_idx.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                num_pairs = copy_to_idx;
                i = 0;
                while i < num_clusters {
                    BrotliCompareAndPushToQueueLiteral(
                        out,
                        cluster_size,
                        best_idx1,
                        *clusters.offset(i as isize),
                        max_num_pairs,
                        &mut *pairs.offset(0 as isize),
                        &mut num_pairs,
                    );
                    i = i.wrapping_add(1);
                }
            }
        }
        return num_clusters;
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramCombineDistance(
    mut out: *mut HistogramDistance,
    mut cluster_size: *mut u32,
    mut symbols: *mut u32,
    mut clusters: *mut u32,
    mut pairs: *mut HistogramPair,
    mut num_clusters: u64,
    mut symbols_size: u64,
    mut max_clusters: u64,
    mut max_num_pairs: u64,
) -> u64 {
    unsafe {
        let mut cost_diff_threshold = 0.0f64;
        let mut min_cluster_size = 1;
        let mut num_pairs = 0;
        let mut idx1: u64 = 0;
        idx1 = 0;
        while idx1 < num_clusters {
            let mut idx2: u64 = 0;
            idx2 = idx1.wrapping_add(1);
            while idx2 < num_clusters {
                BrotliCompareAndPushToQueueDistance(
                    out,
                    cluster_size,
                    *clusters.offset(idx1 as isize),
                    *clusters.offset(idx2 as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as isize),
                    &mut num_pairs,
                );
                idx2 = idx2.wrapping_add(1);
            }
            idx1 = idx1.wrapping_add(1);
        }
        while num_clusters > min_cluster_size {
            let mut best_idx1: u32 = 0;
            let mut best_idx2: u32 = 0;
            let mut i: u64 = 0;
            if (*pairs.offset(0 as isize)).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
            } else {
                best_idx1 = (*pairs.offset(0 as isize)).idx1;
                best_idx2 = (*pairs.offset(0 as isize)).idx2;
                HistogramAddHistogramDistance(
                    &mut *out.offset(best_idx1 as isize),
                    &mut *out.offset(best_idx2 as isize),
                );
                (*out.offset(best_idx1 as isize)).bit_cost_ =
                    (*pairs.offset(0 as isize)).cost_combo;
                let ref mut fresh7 = *cluster_size.offset(best_idx1 as isize);
                *fresh7 =
                    (*fresh7 as u32).wrapping_add(*cluster_size.offset(best_idx2 as isize)) as u32;
                i = 0;
                while i < symbols_size {
                    if *symbols.offset(i as isize) == best_idx2 {
                        *symbols.offset(i as isize) = best_idx1;
                    }
                    i = i.wrapping_add(1);
                }
                i = 0;
                while i < num_clusters {
                    if *clusters.offset(i as isize) == best_idx2 {
                        memmove(
                            &mut *clusters.offset(i as isize) as *mut u32 as *mut libc::c_void,
                            &mut *clusters.offset(i.wrapping_add(1) as isize) as *mut u32
                                as *const libc::c_void,
                            num_clusters
                                .wrapping_sub(i)
                                .wrapping_sub(1)
                                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1);
                let mut copy_to_idx = 0;
                i = 0;
                while i < num_pairs {
                    let mut p: *mut HistogramPair =
                        &mut *pairs.offset(i as isize) as *mut HistogramPair;
                    if !((*p).idx1 == best_idx1
                        || (*p).idx2 == best_idx1
                        || (*p).idx1 == best_idx2
                        || (*p).idx2 == best_idx2)
                    {
                        if HistogramPairIsLess(&mut *pairs.offset(0 as isize), p) != 0 {
                            let mut front = *pairs.offset(0 as isize);
                            *pairs.offset(0 as isize) = *p;
                            *pairs.offset(copy_to_idx as isize) = front;
                        } else {
                            *pairs.offset(copy_to_idx as isize) = *p;
                        }
                        copy_to_idx = copy_to_idx.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                num_pairs = copy_to_idx;
                i = 0;
                while i < num_clusters {
                    BrotliCompareAndPushToQueueDistance(
                        out,
                        cluster_size,
                        best_idx1,
                        *clusters.offset(i as isize),
                        max_num_pairs,
                        &mut *pairs.offset(0 as isize),
                        &mut num_pairs,
                    );
                    i = i.wrapping_add(1);
                }
            }
        }
        return num_clusters;
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramCombineCommand(
    mut out: *mut HistogramCommand,
    mut cluster_size: *mut u32,
    mut symbols: *mut u32,
    mut clusters: *mut u32,
    mut pairs: *mut HistogramPair,
    mut num_clusters: u64,
    mut symbols_size: u64,
    mut max_clusters: u64,
    mut max_num_pairs: u64,
) -> u64 {
    unsafe {
        let mut cost_diff_threshold = 0.0f64;
        let mut min_cluster_size = 1;
        let mut num_pairs = 0;
        let mut idx1: u64 = 0;
        idx1 = 0;
        while idx1 < num_clusters {
            let mut idx2: u64 = 0;
            idx2 = idx1.wrapping_add(1);
            while idx2 < num_clusters {
                BrotliCompareAndPushToQueueCommand(
                    out,
                    cluster_size,
                    *clusters.offset(idx1 as isize),
                    *clusters.offset(idx2 as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as isize),
                    &mut num_pairs,
                );
                idx2 = idx2.wrapping_add(1);
            }
            idx1 = idx1.wrapping_add(1);
        }
        while num_clusters > min_cluster_size {
            let mut best_idx1: u32 = 0;
            let mut best_idx2: u32 = 0;
            let mut i: u64 = 0;
            if (*pairs.offset(0 as isize)).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
            } else {
                best_idx1 = (*pairs.offset(0 as isize)).idx1;
                best_idx2 = (*pairs.offset(0 as isize)).idx2;
                HistogramAddHistogramCommand(
                    &mut *out.offset(best_idx1 as isize),
                    &mut *out.offset(best_idx2 as isize),
                );
                (*out.offset(best_idx1 as isize)).bit_cost_ =
                    (*pairs.offset(0 as isize)).cost_combo;
                let ref mut fresh8 = *cluster_size.offset(best_idx1 as isize);
                *fresh8 =
                    (*fresh8 as u32).wrapping_add(*cluster_size.offset(best_idx2 as isize)) as u32;
                i = 0;
                while i < symbols_size {
                    if *symbols.offset(i as isize) == best_idx2 {
                        *symbols.offset(i as isize) = best_idx1;
                    }
                    i = i.wrapping_add(1);
                }
                i = 0;
                while i < num_clusters {
                    if *clusters.offset(i as isize) == best_idx2 {
                        memmove(
                            &mut *clusters.offset(i as isize) as *mut u32 as *mut libc::c_void,
                            &mut *clusters.offset(i.wrapping_add(1) as isize) as *mut u32
                                as *const libc::c_void,
                            num_clusters
                                .wrapping_sub(i)
                                .wrapping_sub(1)
                                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1);
                let mut copy_to_idx = 0;
                i = 0;
                while i < num_pairs {
                    let mut p: *mut HistogramPair =
                        &mut *pairs.offset(i as isize) as *mut HistogramPair;
                    if !((*p).idx1 == best_idx1
                        || (*p).idx2 == best_idx1
                        || (*p).idx1 == best_idx2
                        || (*p).idx2 == best_idx2)
                    {
                        if HistogramPairIsLess(&mut *pairs.offset(0 as isize), p) != 0 {
                            let mut front = *pairs.offset(0 as isize);
                            *pairs.offset(0 as isize) = *p;
                            *pairs.offset(copy_to_idx as isize) = front;
                        } else {
                            *pairs.offset(copy_to_idx as isize) = *p;
                        }
                        copy_to_idx = copy_to_idx.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                num_pairs = copy_to_idx;
                i = 0;
                while i < num_clusters {
                    BrotliCompareAndPushToQueueCommand(
                        out,
                        cluster_size,
                        best_idx1,
                        *clusters.offset(i as isize),
                        max_num_pairs,
                        &mut *pairs.offset(0 as isize),
                        &mut num_pairs,
                    );
                    i = i.wrapping_add(1);
                }
            }
        }
        return num_clusters;
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramBitCostDistanceCommand(
    mut histogram: *const HistogramCommand,
    mut candidate: *const HistogramCommand,
) -> f64 {
    unsafe {
        if (*histogram).total_count_ == 0 {
            return 0.0f64;
        } else {
            let mut tmp = *histogram;
            HistogramAddHistogramCommand(&mut tmp, candidate);
            return BrotliPopulationCostCommand(&mut tmp) - (*candidate).bit_cost_;
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramBitCostDistanceDistance(
    mut histogram: *const HistogramDistance,
    mut candidate: *const HistogramDistance,
) -> f64 {
    unsafe {
        if (*histogram).total_count_ == 0 {
            return 0.0f64;
        } else {
            let mut tmp = *histogram;
            HistogramAddHistogramDistance(&mut tmp, candidate);
            return BrotliPopulationCostDistance(&mut tmp) - (*candidate).bit_cost_;
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram: *const HistogramLiteral,
    mut candidate: *const HistogramLiteral,
) -> f64 {
    unsafe {
        if (*histogram).total_count_ == 0 {
            return 0.0f64;
        } else {
            let mut tmp = *histogram;
            HistogramAddHistogramLiteral(&mut tmp, candidate);
            return BrotliPopulationCostLiteral(&mut tmp) - (*candidate).bit_cost_;
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramRemapLiteral(
    mut in_0: *const HistogramLiteral,
    mut in_size: u64,
    mut clusters: *const u32,
    mut num_clusters: u64,
    mut out: *mut HistogramLiteral,
    mut symbols: *mut u32,
) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < in_size {
            let mut best_out = if i == 0 {
                *symbols.offset(0 as isize)
            } else {
                *symbols.offset(i.wrapping_sub(1) as isize)
            };
            let mut best_bits = BrotliHistogramBitCostDistanceLiteral(
                &*in_0.offset(i as isize),
                &mut *out.offset(best_out as isize),
            );
            let mut j: u64 = 0;
            j = 0;
            while j < num_clusters {
                let cur_bits = BrotliHistogramBitCostDistanceLiteral(
                    &*in_0.offset(i as isize),
                    &mut *out.offset(*clusters.offset(j as isize) as isize),
                );
                if cur_bits < best_bits {
                    best_bits = cur_bits;
                    best_out = *clusters.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
            *symbols.offset(i as isize) = best_out;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < num_clusters {
            HistogramClearLiteral(&mut *out.offset(*clusters.offset(i as isize) as isize));
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            HistogramAddHistogramLiteral(
                &mut *out.offset(*symbols.offset(i as isize) as isize),
                &*in_0.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramRemapDistance(
    mut in_0: *const HistogramDistance,
    mut in_size: u64,
    mut clusters: *const u32,
    mut num_clusters: u64,
    mut out: *mut HistogramDistance,
    mut symbols: *mut u32,
) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < in_size {
            let mut best_out = if i == 0 {
                *symbols.offset(0 as isize)
            } else {
                *symbols.offset(i.wrapping_sub(1) as isize)
            };
            let mut best_bits = BrotliHistogramBitCostDistanceDistance(
                &*in_0.offset(i as isize),
                &mut *out.offset(best_out as isize),
            );
            let mut j: u64 = 0;
            j = 0;
            while j < num_clusters {
                let cur_bits = BrotliHistogramBitCostDistanceDistance(
                    &*in_0.offset(i as isize),
                    &mut *out.offset(*clusters.offset(j as isize) as isize),
                );
                if cur_bits < best_bits {
                    best_bits = cur_bits;
                    best_out = *clusters.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
            *symbols.offset(i as isize) = best_out;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < num_clusters {
            HistogramClearDistance(&mut *out.offset(*clusters.offset(i as isize) as isize));
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            HistogramAddHistogramDistance(
                &mut *out.offset(*symbols.offset(i as isize) as isize),
                &*in_0.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramRemapCommand(
    mut in_0: *const HistogramCommand,
    mut in_size: u64,
    mut clusters: *const u32,
    mut num_clusters: u64,
    mut out: *mut HistogramCommand,
    mut symbols: *mut u32,
) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < in_size {
            let mut best_out = if i == 0 {
                *symbols.offset(0 as isize)
            } else {
                *symbols.offset(i.wrapping_sub(1) as isize)
            };
            let mut best_bits = BrotliHistogramBitCostDistanceCommand(
                &*in_0.offset(i as isize),
                &mut *out.offset(best_out as isize),
            );
            let mut j: u64 = 0;
            j = 0;
            while j < num_clusters {
                let cur_bits = BrotliHistogramBitCostDistanceCommand(
                    &*in_0.offset(i as isize),
                    &mut *out.offset(*clusters.offset(j as isize) as isize),
                );
                if cur_bits < best_bits {
                    best_bits = cur_bits;
                    best_out = *clusters.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
            *symbols.offset(i as isize) = best_out;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < num_clusters {
            HistogramClearCommand(&mut *out.offset(*clusters.offset(i as isize) as isize));
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            HistogramAddHistogramCommand(
                &mut *out.offset(*symbols.offset(i as isize) as isize),
                &*in_0.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramReindexLiteral(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramLiteral,
    mut symbols: *mut u32,
    mut length: u64,
) -> u64 {
    unsafe {
        static mut kInvalidIndex: u32 = !0;
        let mut new_index = if length > 0 {
            BrotliAllocate(m, length.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut next_index: u32 = 0;
        let mut tmp = 0 as *mut HistogramLiteral;
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        i = 0;
        while i < length {
            *new_index.offset(i as isize) = kInvalidIndex;
            i = i.wrapping_add(1);
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
                *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
                next_index = next_index.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        tmp = if next_index > 0 {
            BrotliAllocate(
                m,
                (next_index as u64).wrapping_mul(::std::mem::size_of::<HistogramLiteral>() as u64),
            ) as *mut HistogramLiteral
        } else {
            0 as *mut HistogramLiteral
        };
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
                *tmp.offset(next_index as isize) =
                    *out.offset(*symbols.offset(i as isize) as isize);
                next_index = next_index.wrapping_add(1);
            };
            *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, new_index as *mut libc::c_void);
        new_index = 0 as *mut u32;
        i = 0;
        while i < next_index as u64 {
            *out.offset(i as isize) = *tmp.offset(i as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, tmp as *mut libc::c_void);
        tmp = 0 as *mut HistogramLiteral;
        return next_index as u64;
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramReindexDistance(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramDistance,
    mut symbols: *mut u32,
    mut length: u64,
) -> u64 {
    unsafe {
        static mut kInvalidIndex: u32 = !0;
        let mut new_index = if length > 0 {
            BrotliAllocate(m, length.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut next_index: u32 = 0;
        let mut tmp = 0 as *mut HistogramDistance;
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        i = 0;
        while i < length {
            *new_index.offset(i as isize) = kInvalidIndex;
            i = i.wrapping_add(1);
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
                *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
                next_index = next_index.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        tmp = if next_index > 0 {
            BrotliAllocate(
                m,
                (next_index as u64).wrapping_mul(::std::mem::size_of::<HistogramDistance>() as u64),
            ) as *mut HistogramDistance
        } else {
            0 as *mut HistogramDistance
        };
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
                *tmp.offset(next_index as isize) =
                    *out.offset(*symbols.offset(i as isize) as isize);
                next_index = next_index.wrapping_add(1);
            };
            *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, new_index as *mut libc::c_void);
        new_index = 0 as *mut u32;
        i = 0;
        while i < next_index as u64 {
            *out.offset(i as isize) = *tmp.offset(i as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, tmp as *mut libc::c_void);
        tmp = 0 as *mut HistogramDistance;
        return next_index as u64;
    }
}

#[no_mangle]
pub extern "C" fn BrotliHistogramReindexCommand(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramCommand,
    mut symbols: *mut u32,
    mut length: u64,
) -> u64 {
    unsafe {
        static mut kInvalidIndex: u32 = !0;
        let mut new_index = if length > 0 {
            BrotliAllocate(m, length.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut next_index: u32 = 0;
        let mut tmp = 0 as *mut HistogramCommand;
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        i = 0;
        while i < length {
            *new_index.offset(i as isize) = kInvalidIndex;
            i = i.wrapping_add(1);
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
                *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
                next_index = next_index.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        tmp = if next_index > 0 {
            BrotliAllocate(
                m,
                (next_index as u64).wrapping_mul(::std::mem::size_of::<HistogramCommand>() as u64),
            ) as *mut HistogramCommand
        } else {
            0 as *mut HistogramCommand
        };
        if 0 != 0 || 0 != 0 {
            return 0;
        }
        next_index = 0;
        i = 0;
        while i < length {
            if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
                *tmp.offset(next_index as isize) =
                    *out.offset(*symbols.offset(i as isize) as isize);
                next_index = next_index.wrapping_add(1);
            };
            *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, new_index as *mut libc::c_void);
        new_index = 0 as *mut u32;
        i = 0;
        while i < next_index as u64 {
            *out.offset(i as isize) = *tmp.offset(i as isize);
            i = i.wrapping_add(1);
        }
        BrotliFree(m, tmp as *mut libc::c_void);
        tmp = 0 as *mut HistogramCommand;
        return next_index as u64;
    }
}

#[no_mangle]
pub extern "C" fn BrotliClusterHistogramsDistance(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramDistance,
    in_size: u64,
    mut max_histograms: u64,
    mut out: *mut HistogramDistance,
    mut out_size: *mut u64,
    mut histogram_symbols: *mut u32,
) {
    unsafe {
        let mut cluster_size = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut clusters = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut num_clusters = 0;
        let max_input_histograms = 64;
        let mut pairs_capacity = max_input_histograms
            .wrapping_mul(max_input_histograms)
            .wrapping_div(2);
        let mut pairs = if pairs_capacity.wrapping_add(1) > 0 {
            BrotliAllocate(
                m,
                pairs_capacity
                    .wrapping_add(1)
                    .wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 || 0 != 0 || 0 != 0 {
            return;
        }
        i = 0;
        while i < in_size {
            *cluster_size.offset(i as isize) = 1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            *out.offset(i as isize) = *in_0.offset(i as isize);
            (*out.offset(i as isize)).bit_cost_ =
                BrotliPopulationCostDistance(&*in_0.offset(i as isize));
            *histogram_symbols.offset(i as isize) = i as u32;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            let mut num_to_combine =
                brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
            let mut num_new_clusters: u64 = 0;
            let mut j: u64 = 0;
            j = 0;
            while j < num_to_combine {
                *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as u32;
                j = j.wrapping_add(1);
            }
            num_new_clusters = BrotliHistogramCombineDistance(
                out,
                cluster_size,
                &mut *histogram_symbols.offset(i as isize),
                &mut *clusters.offset(num_clusters as isize),
                pairs,
                num_to_combine,
                num_to_combine,
                max_histograms,
                pairs_capacity,
            );
            num_clusters = (num_clusters as u64).wrapping_add(num_new_clusters) as u64;
            i = (i).wrapping_add(max_input_histograms) as u64;
        }
        let mut max_num_pairs = brotli_min_size_t(
            64u64.wrapping_mul(num_clusters),
            num_clusters.wrapping_div(2).wrapping_mul(num_clusters),
        );
        if pairs_capacity < max_num_pairs.wrapping_add(1) {
            let mut _new_size = if pairs_capacity == 0 {
                max_num_pairs.wrapping_add(1)
            } else {
                pairs_capacity
            };
            let mut new_array = 0 as *mut HistogramPair;
            while _new_size < max_num_pairs.wrapping_add(1) {
                _new_size = (_new_size as u64).wrapping_mul(2) as u64;
            }
            new_array = if _new_size > 0 {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                ) as *mut HistogramPair
            } else {
                0 as *mut HistogramPair
            };
            if 0 == 0 && 0 == 0 && pairs_capacity != 0 {
                memcpy(
                    new_array as *mut libc::c_void,
                    pairs as *const libc::c_void,
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                );
            }
            BrotliFree(m, pairs as *mut libc::c_void);
            pairs = 0 as *mut HistogramPair;
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if 0 != 0 {
            return;
        }
        num_clusters = BrotliHistogramCombineDistance(
            out,
            cluster_size,
            histogram_symbols,
            clusters,
            pairs,
            num_clusters,
            in_size,
            max_histograms,
            max_num_pairs,
        );
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        BrotliFree(m, cluster_size as *mut libc::c_void);
        cluster_size = 0 as *mut u32;
        BrotliHistogramRemapDistance(
            in_0,
            in_size,
            clusters,
            num_clusters,
            out,
            histogram_symbols,
        );
        BrotliFree(m, clusters as *mut libc::c_void);
        clusters = 0 as *mut u32;
        *out_size = BrotliHistogramReindexDistance(m, out, histogram_symbols, in_size);
        if 0 != 0 {
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliClusterHistogramsCommand(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramCommand,
    in_size: u64,
    mut max_histograms: u64,
    mut out: *mut HistogramCommand,
    mut out_size: *mut u64,
    mut histogram_symbols: *mut u32,
) {
    unsafe {
        let mut cluster_size = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut clusters = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut num_clusters = 0;
        let max_input_histograms = 64;
        let mut pairs_capacity = max_input_histograms
            .wrapping_mul(max_input_histograms)
            .wrapping_div(2);
        let mut pairs = if pairs_capacity.wrapping_add(1) > 0 {
            BrotliAllocate(
                m,
                pairs_capacity
                    .wrapping_add(1)
                    .wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 || 0 != 0 || 0 != 0 {
            return;
        }
        i = 0;
        while i < in_size {
            *cluster_size.offset(i as isize) = 1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            *out.offset(i as isize) = *in_0.offset(i as isize);
            (*out.offset(i as isize)).bit_cost_ =
                BrotliPopulationCostCommand(&*in_0.offset(i as isize));
            *histogram_symbols.offset(i as isize) = i as u32;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            let mut num_to_combine =
                brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
            let mut num_new_clusters: u64 = 0;
            let mut j: u64 = 0;
            j = 0;
            while j < num_to_combine {
                *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as u32;
                j = j.wrapping_add(1);
            }
            num_new_clusters = BrotliHistogramCombineCommand(
                out,
                cluster_size,
                &mut *histogram_symbols.offset(i as isize),
                &mut *clusters.offset(num_clusters as isize),
                pairs,
                num_to_combine,
                num_to_combine,
                max_histograms,
                pairs_capacity,
            );
            num_clusters = (num_clusters as u64).wrapping_add(num_new_clusters) as u64;
            i = (i).wrapping_add(max_input_histograms) as u64;
        }
        let mut max_num_pairs = brotli_min_size_t(
            64u64.wrapping_mul(num_clusters),
            num_clusters.wrapping_div(2).wrapping_mul(num_clusters),
        );
        if pairs_capacity < max_num_pairs.wrapping_add(1) {
            let mut _new_size = if pairs_capacity == 0 {
                max_num_pairs.wrapping_add(1)
            } else {
                pairs_capacity
            };
            let mut new_array = 0 as *mut HistogramPair;
            while _new_size < max_num_pairs.wrapping_add(1) {
                _new_size = (_new_size as u64).wrapping_mul(2) as u64;
            }
            new_array = if _new_size > 0 {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                ) as *mut HistogramPair
            } else {
                0 as *mut HistogramPair
            };
            if 0 == 0 && 0 == 0 && pairs_capacity != 0 {
                memcpy(
                    new_array as *mut libc::c_void,
                    pairs as *const libc::c_void,
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                );
            }
            BrotliFree(m, pairs as *mut libc::c_void);
            pairs = 0 as *mut HistogramPair;
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if 0 != 0 {
            return;
        }
        num_clusters = BrotliHistogramCombineCommand(
            out,
            cluster_size,
            histogram_symbols,
            clusters,
            pairs,
            num_clusters,
            in_size,
            max_histograms,
            max_num_pairs,
        );
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        BrotliFree(m, cluster_size as *mut libc::c_void);
        cluster_size = 0 as *mut u32;
        BrotliHistogramRemapCommand(
            in_0,
            in_size,
            clusters,
            num_clusters,
            out,
            histogram_symbols,
        );
        BrotliFree(m, clusters as *mut libc::c_void);
        clusters = 0 as *mut u32;
        *out_size = BrotliHistogramReindexCommand(m, out, histogram_symbols, in_size);
        if 0 != 0 {
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliClusterHistogramsLiteral(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramLiteral,
    in_size: u64,
    mut max_histograms: u64,
    mut out: *mut HistogramLiteral,
    mut out_size: *mut u64,
    mut histogram_symbols: *mut u32,
) {
    unsafe {
        let mut cluster_size = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut clusters = if in_size > 0 {
            BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut num_clusters = 0;
        let max_input_histograms = 64;
        let mut pairs_capacity = max_input_histograms
            .wrapping_mul(max_input_histograms)
            .wrapping_div(2);
        let mut pairs = if pairs_capacity.wrapping_add(1) > 0 {
            BrotliAllocate(
                m,
                pairs_capacity
                    .wrapping_add(1)
                    .wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 || 0 != 0 || 0 != 0 {
            return;
        }
        i = 0;
        while i < in_size {
            *cluster_size.offset(i as isize) = 1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            *out.offset(i as isize) = *in_0.offset(i as isize);
            (*out.offset(i as isize)).bit_cost_ =
                BrotliPopulationCostLiteral(&*in_0.offset(i as isize));
            *histogram_symbols.offset(i as isize) = i as u32;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < in_size {
            let mut num_to_combine =
                brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
            let mut num_new_clusters: u64 = 0;
            let mut j: u64 = 0;
            j = 0;
            while j < num_to_combine {
                *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as u32;
                j = j.wrapping_add(1);
            }
            num_new_clusters = BrotliHistogramCombineLiteral(
                out,
                cluster_size,
                &mut *histogram_symbols.offset(i as isize),
                &mut *clusters.offset(num_clusters as isize),
                pairs,
                num_to_combine,
                num_to_combine,
                max_histograms,
                pairs_capacity,
            );
            num_clusters = (num_clusters as u64).wrapping_add(num_new_clusters) as u64;
            i = (i).wrapping_add(max_input_histograms) as u64;
        }
        let mut max_num_pairs = brotli_min_size_t(
            64u64.wrapping_mul(num_clusters),
            num_clusters.wrapping_div(2).wrapping_mul(num_clusters),
        );
        if pairs_capacity < max_num_pairs.wrapping_add(1) {
            let mut _new_size = if pairs_capacity == 0 {
                max_num_pairs.wrapping_add(1)
            } else {
                pairs_capacity
            };
            let mut new_array = 0 as *mut HistogramPair;
            while _new_size < max_num_pairs.wrapping_add(1) {
                _new_size = (_new_size as u64).wrapping_mul(2) as u64;
            }
            new_array = if _new_size > 0 {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                ) as *mut HistogramPair
            } else {
                0 as *mut HistogramPair
            };
            if 0 == 0 && 0 == 0 && pairs_capacity != 0 {
                memcpy(
                    new_array as *mut libc::c_void,
                    pairs as *const libc::c_void,
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>() as u64),
                );
            }
            BrotliFree(m, pairs as *mut libc::c_void);
            pairs = 0 as *mut HistogramPair;
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if 0 != 0 {
            return;
        }
        num_clusters = BrotliHistogramCombineLiteral(
            out,
            cluster_size,
            histogram_symbols,
            clusters,
            pairs,
            num_clusters,
            in_size,
            max_histograms,
            max_num_pairs,
        );
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        BrotliFree(m, cluster_size as *mut libc::c_void);
        cluster_size = 0 as *mut u32;
        BrotliHistogramRemapLiteral(
            in_0,
            in_size,
            clusters,
            num_clusters,
            out,
            histogram_symbols,
        );
        BrotliFree(m, clusters as *mut libc::c_void);
        clusters = 0 as *mut u32;
        *out_size = BrotliHistogramReindexLiteral(m, out, histogram_symbols, in_size);
        if 0 != 0 {
            return;
        }
    }
}
