use libc;
extern "C" {
    fn log2(_: f64) -> f64;
    static kBrotliLog2Table: [f64; 256];
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
#[inline(always)]
extern "C" fn brotli_max_uint32_t(mut a: u32, mut b: u32) -> u32 {
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
extern "C" fn HistogramDataSizeLiteral() -> u64 {
    return 256;
}

#[inline(always)]
extern "C" fn HistogramDataSizeCommand() -> u64 {
    return 704;
}

#[inline(always)]
extern "C" fn HistogramDataSizeDistance() -> u64 {
    return 544;
}

#[inline(always)]
extern "C" fn ShannonEntropy(
    mut population: *const u32,
    mut size: u64,
    mut total: *mut u64,
) -> f64 {
    unsafe {
        let mut current_block: u64;
        let mut sum = 0;
        let mut retval = 0 as f64;
        let mut population_end = population.offset(size as isize);
        let mut p: u64 = 0;
        if size & 1 != 0 {
            current_block = 15800646830660818732;
        } else {
            current_block = 715039052867723359;
        }
        loop {
            match current_block {
                715039052867723359 => {
                    if !(population < population_end) {
                        break;
                    }
                    let fresh0 = population;
                    population = population.offset(1);
                    p = *fresh0 as u64;
                    sum = (sum as u64).wrapping_add(p) as u64;
                    retval -= p as f64 * FastLog2(p);
                    current_block = 15800646830660818732;
                }
                _ => {
                    let fresh1 = population;
                    population = population.offset(1);
                    p = *fresh1 as u64;
                    sum = (sum as u64).wrapping_add(p) as u64;
                    retval -= p as f64 * FastLog2(p);
                    current_block = 715039052867723359;
                }
            }
        }
        if sum != 0 {
            retval += sum as f64 * FastLog2(sum);
        }
        *total = sum;
        return retval;
    }
}

#[inline(always)]
extern "C" fn BitsEntropy(mut population: *const u32, mut size: u64) -> f64 {
    unsafe {
        let mut sum: u64 = 0;
        let mut retval = ShannonEntropy(population, size, &mut sum);
        if retval < sum as f64 {
            retval = sum as f64;
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn BrotliPopulationCostDistance(mut histogram: *const HistogramDistance) -> f64 {
    unsafe {
        static mut kOneSymbolHistogramCost: f64 = 12 as f64;
        static mut kTwoSymbolHistogramCost: f64 = 20 as f64;
        static mut kThreeSymbolHistogramCost: f64 = 28 as f64;
        static mut kFourSymbolHistogramCost: f64 = 37 as f64;
        let data_size = HistogramDataSizeDistance();
        let mut count = 0;
        let mut s: [u64; 5] = [0; 5];
        let mut bits = 0.0f64;
        let mut i: u64 = 0;
        if (*histogram).total_count_ == 0 {
            return kOneSymbolHistogramCost;
        }
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                s[count as usize] = i;
                count += 1;
                if count > 4 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if count == 1 {
            return kOneSymbolHistogramCost;
        }
        if count == 2 {
            return kTwoSymbolHistogramCost + (*histogram).total_count_ as f64;
        }
        if count == 3 {
            let histo0 = (*histogram).data_[s[0 as usize] as usize];
            let histo1 = (*histogram).data_[s[1 as usize] as usize];
            let histo2 = (*histogram).data_[s[2 as usize] as usize];
            let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
            return kThreeSymbolHistogramCost
                + 2u32.wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as f64
                - histomax as f64;
        }
        if count == 4 {
            let mut histo: [u32; 4] = [0; 4];
            let mut h23: u32 = 0;
            let mut histomax_0: u32 = 0;
            i = 0;
            while i < 4 {
                histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
                i = i.wrapping_add(1);
            }
            i = 0;
            while i < 4 {
                let mut j: u64 = 0;
                j = i.wrapping_add(1);
                while j < 4 {
                    if histo[j as usize] > histo[i as usize] {
                        let mut __brotli_swap_tmp = histo[j as usize];
                        histo[j as usize] = histo[i as usize];
                        histo[i as usize] = __brotli_swap_tmp;
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            h23 = (histo[2 as usize]).wrapping_add(histo[3 as usize]);
            histomax_0 = brotli_max_uint32_t(h23, histo[0 as usize]);
            return kFourSymbolHistogramCost
                + 3u32.wrapping_mul(h23) as f64
                + 2u32.wrapping_mul((histo[0 as usize]).wrapping_add(histo[1 as usize])) as f64
                - histomax_0 as f64;
        }
        let mut max_depth = 1;
        let mut depth_histo: [u32; 18] = [0; 18];
        let log2total = FastLog2((*histogram).total_count_);
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                let mut log2p = log2total - FastLog2((*histogram).data_[i as usize] as u64);
                let mut depth = (log2p + 0.5f64) as u64;
                bits += (*histogram).data_[i as usize] as f64 * log2p;
                if depth > 15 {
                    depth = 15;
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                depth_histo[depth as usize] = (depth_histo[depth as usize]).wrapping_add(1);
                i = i.wrapping_add(1);
            } else {
                let mut reps = 1;
                let mut k: u64 = 0;
                k = i.wrapping_add(1);
                while k < data_size && (*histogram).data_[k as usize] == 0 {
                    reps = reps.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
                i = (i).wrapping_add(reps as u64) as u64;
                if i == data_size {
                    break;
                }
                if reps < 3 {
                    depth_histo[0 as usize] =
                        (depth_histo[0 as usize] as u32).wrapping_add(reps) as u32;
                } else {
                    reps = (reps as u32).wrapping_sub(2) as u32;
                    while reps > 0 {
                        depth_histo[17 as usize] = (depth_histo[17 as usize]).wrapping_add(1);
                        bits += 3 as f64;
                        reps >>= 3;
                    }
                }
            }
        }
        bits += 18u64.wrapping_add(2u64.wrapping_mul(max_depth)) as f64;
        bits += BitsEntropy(depth_histo.as_mut_ptr(), (17 + 1i32) as u64);
        return bits;
    }
}

#[no_mangle]
pub extern "C" fn BrotliPopulationCostCommand(mut histogram: *const HistogramCommand) -> f64 {
    unsafe {
        static mut kOneSymbolHistogramCost: f64 = 12 as f64;
        static mut kTwoSymbolHistogramCost: f64 = 20 as f64;
        static mut kThreeSymbolHistogramCost: f64 = 28 as f64;
        static mut kFourSymbolHistogramCost: f64 = 37 as f64;
        let data_size = HistogramDataSizeCommand();
        let mut count = 0;
        let mut s: [u64; 5] = [0; 5];
        let mut bits = 0.0f64;
        let mut i: u64 = 0;
        if (*histogram).total_count_ == 0 {
            return kOneSymbolHistogramCost;
        }
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                s[count as usize] = i;
                count += 1;
                if count > 4 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if count == 1 {
            return kOneSymbolHistogramCost;
        }
        if count == 2 {
            return kTwoSymbolHistogramCost + (*histogram).total_count_ as f64;
        }
        if count == 3 {
            let histo0 = (*histogram).data_[s[0 as usize] as usize];
            let histo1 = (*histogram).data_[s[1 as usize] as usize];
            let histo2 = (*histogram).data_[s[2 as usize] as usize];
            let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
            return kThreeSymbolHistogramCost
                + 2u32.wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as f64
                - histomax as f64;
        }
        if count == 4 {
            let mut histo: [u32; 4] = [0; 4];
            let mut h23: u32 = 0;
            let mut histomax_0: u32 = 0;
            i = 0;
            while i < 4 {
                histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
                i = i.wrapping_add(1);
            }
            i = 0;
            while i < 4 {
                let mut j: u64 = 0;
                j = i.wrapping_add(1);
                while j < 4 {
                    if histo[j as usize] > histo[i as usize] {
                        let mut __brotli_swap_tmp = histo[j as usize];
                        histo[j as usize] = histo[i as usize];
                        histo[i as usize] = __brotli_swap_tmp;
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            h23 = (histo[2 as usize]).wrapping_add(histo[3 as usize]);
            histomax_0 = brotli_max_uint32_t(h23, histo[0 as usize]);
            return kFourSymbolHistogramCost
                + 3u32.wrapping_mul(h23) as f64
                + 2u32.wrapping_mul((histo[0 as usize]).wrapping_add(histo[1 as usize])) as f64
                - histomax_0 as f64;
        }
        let mut max_depth = 1;
        let mut depth_histo: [u32; 18] = [0; 18];
        let log2total = FastLog2((*histogram).total_count_);
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                let mut log2p = log2total - FastLog2((*histogram).data_[i as usize] as u64);
                let mut depth = (log2p + 0.5f64) as u64;
                bits += (*histogram).data_[i as usize] as f64 * log2p;
                if depth > 15 {
                    depth = 15;
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                depth_histo[depth as usize] = (depth_histo[depth as usize]).wrapping_add(1);
                i = i.wrapping_add(1);
            } else {
                let mut reps = 1;
                let mut k: u64 = 0;
                k = i.wrapping_add(1);
                while k < data_size && (*histogram).data_[k as usize] == 0 {
                    reps = reps.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
                i = (i).wrapping_add(reps as u64) as u64;
                if i == data_size {
                    break;
                }
                if reps < 3 {
                    depth_histo[0 as usize] =
                        (depth_histo[0 as usize] as u32).wrapping_add(reps) as u32;
                } else {
                    reps = (reps as u32).wrapping_sub(2) as u32;
                    while reps > 0 {
                        depth_histo[17 as usize] = (depth_histo[17 as usize]).wrapping_add(1);
                        bits += 3 as f64;
                        reps >>= 3;
                    }
                }
            }
        }
        bits += 18u64.wrapping_add(2u64.wrapping_mul(max_depth)) as f64;
        bits += BitsEntropy(depth_histo.as_mut_ptr(), (17 + 1i32) as u64);
        return bits;
    }
}

#[no_mangle]
pub extern "C" fn BrotliPopulationCostLiteral(mut histogram: *const HistogramLiteral) -> f64 {
    unsafe {
        static mut kOneSymbolHistogramCost: f64 = 12 as f64;
        static mut kTwoSymbolHistogramCost: f64 = 20 as f64;
        static mut kThreeSymbolHistogramCost: f64 = 28 as f64;
        static mut kFourSymbolHistogramCost: f64 = 37 as f64;
        let data_size = HistogramDataSizeLiteral();
        let mut count = 0;
        let mut s: [u64; 5] = [0; 5];
        let mut bits = 0.0f64;
        let mut i: u64 = 0;
        if (*histogram).total_count_ == 0 {
            return kOneSymbolHistogramCost;
        }
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                s[count as usize] = i;
                count += 1;
                if count > 4 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if count == 1 {
            return kOneSymbolHistogramCost;
        }
        if count == 2 {
            return kTwoSymbolHistogramCost + (*histogram).total_count_ as f64;
        }
        if count == 3 {
            let histo0 = (*histogram).data_[s[0 as usize] as usize];
            let histo1 = (*histogram).data_[s[1 as usize] as usize];
            let histo2 = (*histogram).data_[s[2 as usize] as usize];
            let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
            return kThreeSymbolHistogramCost
                + 2u32.wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as f64
                - histomax as f64;
        }
        if count == 4 {
            let mut histo: [u32; 4] = [0; 4];
            let mut h23: u32 = 0;
            let mut histomax_0: u32 = 0;
            i = 0;
            while i < 4 {
                histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
                i = i.wrapping_add(1);
            }
            i = 0;
            while i < 4 {
                let mut j: u64 = 0;
                j = i.wrapping_add(1);
                while j < 4 {
                    if histo[j as usize] > histo[i as usize] {
                        let mut __brotli_swap_tmp = histo[j as usize];
                        histo[j as usize] = histo[i as usize];
                        histo[i as usize] = __brotli_swap_tmp;
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            h23 = (histo[2 as usize]).wrapping_add(histo[3 as usize]);
            histomax_0 = brotli_max_uint32_t(h23, histo[0 as usize]);
            return kFourSymbolHistogramCost
                + 3u32.wrapping_mul(h23) as f64
                + 2u32.wrapping_mul((histo[0 as usize]).wrapping_add(histo[1 as usize])) as f64
                - histomax_0 as f64;
        }
        let mut max_depth = 1;
        let mut depth_histo: [u32; 18] = [0; 18];
        let log2total = FastLog2((*histogram).total_count_);
        i = 0;
        while i < data_size {
            if (*histogram).data_[i as usize] > 0 {
                let mut log2p = log2total - FastLog2((*histogram).data_[i as usize] as u64);
                let mut depth = (log2p + 0.5f64) as u64;
                bits += (*histogram).data_[i as usize] as f64 * log2p;
                if depth > 15 {
                    depth = 15;
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                depth_histo[depth as usize] = (depth_histo[depth as usize]).wrapping_add(1);
                i = i.wrapping_add(1);
            } else {
                let mut reps = 1;
                let mut k: u64 = 0;
                k = i.wrapping_add(1);
                while k < data_size && (*histogram).data_[k as usize] == 0 {
                    reps = reps.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
                i = (i).wrapping_add(reps as u64) as u64;
                if i == data_size {
                    break;
                }
                if reps < 3 {
                    depth_histo[0 as usize] =
                        (depth_histo[0 as usize] as u32).wrapping_add(reps) as u32;
                } else {
                    reps = (reps as u32).wrapping_sub(2) as u32;
                    while reps > 0 {
                        depth_histo[17 as usize] = (depth_histo[17 as usize]).wrapping_add(1);
                        bits += 3 as f64;
                        reps >>= 3;
                    }
                }
            }
        }
        bits += 18u64.wrapping_add(2u64.wrapping_mul(max_depth)) as f64;
        bits += BitsEntropy(depth_histo.as_mut_ptr(), (17 + 1i32) as u64);
        return bits;
    }
}
