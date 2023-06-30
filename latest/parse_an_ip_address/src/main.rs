#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
}
extern "C" fn _parseDecimal(mut pchCursor: *mut *const i8) -> u32 {
    unsafe {
        let mut nVal: u32 = 0;
        let mut chNow: i8 = 0;
        loop {
            chNow = **pchCursor;
            if !(i32::from(chNow) >= '0' as i32 && i32::from(chNow) <= '9' as i32) {
                break;
            }
            nVal = nVal.wrapping_mul(10);
            nVal = nVal.wrapping_add((i32::from(chNow) - '0' as i32) as u32);
            *pchCursor = (*pchCursor).offset(1);
            *pchCursor;
        }
        nVal
    }
}

extern "C" fn _parseHex(mut pchCursor: *mut *const i8) -> u32 {
    unsafe {
        let mut nVal: u32 = 0;
        let mut chNow: i8 = 0;
        loop {
            chNow = (i32::from(**pchCursor) & 0x5fi32) as i8;
            if !(i32::from(chNow) >= '0' as i32 & 0x005f_i32 && i32::from(chNow) <= '9' as i32 & 0x005f_i32
                || i32::from(chNow) >= 'A' as i32 && i32::from(chNow) <= 'F' as i32)
            {
                break;
            }
            let mut nybbleValue: u8 = 0;
            chNow = (i32::from(chNow) - 0x10i32) as i8;
            nybbleValue = (if i32::from(chNow) > 9i32 {
                i32::from(chNow) - (0x0031_i32 - 0x000a_i32)
            } else {
                i32::from(chNow)
            }) as u8;
            nVal <<= 4_i32;
            nVal = nVal.wrapping_add(u32::from(nybbleValue));
            *pchCursor = (*pchCursor).offset(1);
            *pchCursor;
        }
        nVal
    }
}

#[no_mangle]
pub extern "C" fn ParseIPv4OrIPv6(
    mut ppszText: *mut *const i8,
    mut abyAddr: *mut u8,
    mut pnPort: *mut i32,
    mut pbIsIPv6: *mut i32,
) -> i32 {
    unsafe {
        let mut abyAddrLocal: *mut u8 = std::ptr::null_mut::<u8>();
        let mut abyDummyAddr: [u8; 16] = [0; 16];
        let mut pchColon: *const i8 = strchr(*ppszText, ':' as i32);
        let mut pchDot: *const i8 = strchr(*ppszText, '.' as i32);
        let mut pchOpenBracket: *const i8 = strchr(*ppszText, '[' as i32);
        let mut pchCloseBracket: *const i8 = std::ptr::null::<i8>();
        let mut bIsIPv6local: i32 = i32::from(!pchOpenBracket.is_null()
            || pchDot.is_null()
            || !pchColon.is_null() && (pchDot.is_null() || pchColon < pchDot));
        if bIsIPv6local != 0_i32 {
            pchCloseBracket = strchr(*ppszText, ']' as i32);
            if !pchOpenBracket.is_null()
                && (pchCloseBracket.is_null() || pchCloseBracket < pchOpenBracket)
            {
                return 0_i32;
            }
        } else if pchDot.is_null() || !pchColon.is_null() && pchColon < pchDot {
            return 0_i32;
        }
        if !pbIsIPv6.is_null() {
            *pbIsIPv6 = bIsIPv6local;
        }
        abyAddrLocal = abyAddr;
        if abyAddrLocal.is_null() {
            abyAddrLocal = abyDummyAddr.as_mut_ptr();
        }
        if bIsIPv6local == 0_i32 {
            let mut pbyAddrCursor: *mut u8 = abyAddrLocal;
            let mut nVal: u32 = 0;
            let mut pszTextBefore: *const i8 = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != i32::from(**ppszText) || nVal > 255 || pszTextBefore == *ppszText {
                return 0_i32;
            }
            let fresh0 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh0 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != i32::from(**ppszText) || nVal > 255 || pszTextBefore == *ppszText {
                return 0_i32;
            }
            let fresh1 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh1 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != i32::from(**ppszText) || nVal > 255 || pszTextBefore == *ppszText {
                return 0_i32;
            }
            let fresh2 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh2 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if nVal > 255 || pszTextBefore == *ppszText {
                return 0_i32;
            }
            let fresh3 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh3 = nVal as u8;
            if ':' as i32 == i32::from(**ppszText) && !pnPort.is_null() {
                let mut usPortNetwork: u16 = 0;
                *ppszText = (*ppszText).offset(1);
                *ppszText;
                pszTextBefore = *ppszText;
                nVal = _parseDecimal(ppszText);
                if nVal > 65535 || pszTextBefore == *ppszText {
                    return 0_i32;
                };
                *(&mut usPortNetwork as *mut u16).cast::<u8>().offset(0_isize) =
                    ((nVal & 0xff00u32) >> 8_i32) as u8;
                *(&mut usPortNetwork as *mut u16).cast::<u8>().offset(1_isize) =
                    (nVal & 0xffu32) as u8;
                *pnPort = i32::from(usPortNetwork);
                1_i32
            } else {
                if !pnPort.is_null() {
                    *pnPort = 0_i32;
                }
                1_i32
            }
        } else {
            let mut pbyAddrCursor_0: *mut u8 = std::ptr::null_mut::<u8>();
            let mut pbyZerosLoc: *mut u8 = std::ptr::null_mut::<u8>();
            let mut bIPv4Detected: i32 = 0;
            let mut nIdx: i32 = 0;
            if !pchOpenBracket.is_null() {
                *ppszText = pchOpenBracket.offset(1_isize);
            }
            pbyAddrCursor_0 = abyAddrLocal;
            pbyZerosLoc = std::ptr::null_mut::<u8>();
            bIPv4Detected = 0_i32;
            nIdx = 0_i32;
            while nIdx < 8_i32 {
                let mut pszTextBefore_0: *const i8 = *ppszText;
                let mut nVal_0: u32 = _parseHex(ppszText);
                if pszTextBefore_0 == *ppszText {
                    if !pbyZerosLoc.is_null() {
                        if pbyZerosLoc == pbyAddrCursor_0 {
                            nIdx = nIdx.wrapping_sub(1);
                            nIdx;
                            break;
                        } else {
                            return 0_i32;
                        }
                    } else {
                        if ':' as i32 != i32::from(**ppszText) {
                            return 0_i32;
                        }
                        if 0_i32 == nIdx {
                            *ppszText = (*ppszText).offset(1);
                            *ppszText;
                            if ':' as i32 != i32::from(**ppszText) {
                                return 0_i32;
                            }
                        }
                        pbyZerosLoc = pbyAddrCursor_0;
                        *ppszText = (*ppszText).offset(1);
                        *ppszText;
                    }
                } else if '.' as i32 == i32::from(**ppszText) {
                    let mut pszTextlocal: *const i8 = pszTextBefore_0;
                    let mut abyAddrlocal: [u8; 16] = [0; 16];
                    let mut bIsIPv6local_0: i32 = 0;
                    let mut bParseResultlocal: i32 = ParseIPv4OrIPv6(
                        &mut pszTextlocal,
                        abyAddrlocal.as_mut_ptr(),
                        std::ptr::null_mut::<i32>(),
                        &mut bIsIPv6local_0,
                    );
                    *ppszText = pszTextlocal;
                    if bParseResultlocal == 0_i32 || bIsIPv6local_0 != 0_i32 {
                        return 0_i32;
                    }
                    let fresh4 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh4 = abyAddrlocal[0_usize];
                    let fresh5 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh5 = abyAddrlocal[1_usize];
                    let fresh6 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh6 = abyAddrlocal[2_usize];
                    let fresh7 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh7 = abyAddrlocal[3_usize];
                    nIdx = nIdx.wrapping_add(1);
                    nIdx;
                    bIPv4Detected = 1_i32;
                    break;
                } else {
                    if nVal_0 > 65535 {
                        return 0_i32;
                    }
                    let fresh8 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh8 = (nVal_0 >> 8i32) as u8;
                    let fresh9 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh9 = (nVal_0 & 0xffu32) as u8;
                    if ':' as i32 != i32::from(**ppszText) {
                        break;
                    }
                    *ppszText = (*ppszText).offset(1);
                    *ppszText;
                }
                nIdx = nIdx.wrapping_add(1);
                nIdx;
            }
            if !pbyZerosLoc.is_null() {
                let mut nHead: i32 = pbyZerosLoc.offset_from(abyAddrLocal) as i32;
                let mut nTail: i32 = nIdx * 2 - pbyZerosLoc.offset_from(abyAddrLocal) as i32;
                let mut nZeros: i32 = 16 - nTail - nHead;
                memmove(
                    (&mut *abyAddrLocal.offset((16 - nTail) as isize) as *mut u8).cast::<libc::c_void>(),
                    pbyZerosLoc as *const libc::c_void,
                    nTail as u64,
                );
                memset(pbyZerosLoc.cast::<libc::c_void>(), 0, nZeros as u64);
            }
            if bIPv4Detected != 0_i32 {
                static mut abyPfx: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff];
                if 0_i32 != memcmp(
                    abyAddrLocal as *const libc::c_void,
                    abyPfx.as_ptr().cast::<libc::c_void>(),
                    ::core::mem::size_of::<[u8; 12]>() as u64,
                ) {
                    return 0_i32;
                }
            }
            if !pchOpenBracket.is_null() {
                if ']' as i32 != i32::from(**ppszText) {
                    return 0_i32;
                }
                *ppszText = (*ppszText).offset(1);
                *ppszText;
            }
            if ':' as i32 == i32::from(**ppszText) && !pnPort.is_null() {
                let mut pszTextBefore_1: *const i8 = std::ptr::null::<i8>();
                let mut nVal_1: u32 = 0;
                let mut usPortNetwork_0: u16 = 0;
                *ppszText = (*ppszText).offset(1);
                *ppszText;
                pszTextBefore_1 = *ppszText;
                pszTextBefore_1 = *ppszText;
                nVal_1 = _parseDecimal(ppszText);
                if nVal_1 > 65535 || pszTextBefore_1 == *ppszText {
                    return 0_i32;
                };
                *(&mut usPortNetwork_0 as *mut u16).cast::<u8>().offset(0_isize) =
                    ((nVal_1 & 0xff00u32) >> 8_i32) as u8;
                *(&mut usPortNetwork_0 as *mut u16).cast::<u8>().offset(1_isize) =
                    (nVal_1 & 0xffu32) as u8;
                *pnPort = i32::from(usPortNetwork_0);
                1_i32
            } else {
                if !pnPort.is_null() {
                    *pnPort = 0_i32;
                }
                1_i32
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn ParseIPv4OrIPv6_2(
    mut pszText: *const i8,
    mut abyAddr: *mut u8,
    mut pnPort: *mut i32,
    mut pbIsIPv6: *mut i32,
) -> i32 {
    unsafe {
        let mut pszTextLocal: *const i8 = pszText;
        ParseIPv4OrIPv6(&mut pszTextLocal, abyAddr, pnPort, pbIsIPv6)
    }
}

#[no_mangle]
pub extern "C" fn htons(mut us: u16) -> u16 {
    unsafe {
        ((i32::from(*(&mut us as *mut u16).cast::<u8>().offset(0_isize)) << 8i32)
            + i32::from(*(&mut us as *mut u16).cast::<u8>().offset(1_isize))) as u16
    }
}

#[no_mangle]
pub extern "C" fn dumpbin(mut pbyBin: *mut u8, mut nLen: i32) {
    unsafe {
        let mut i: i32 = 0;
        i = 0_i32;
        while i < nLen {
            print!("{:02x}", i32::from(*pbyBin.offset(i as isize)));
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn testcase(mut pszTest: *const i8) {
    unsafe {
        let mut abyAddr: [u8; 16] = [0; 16];
        let mut bIsIPv6: i32 = 0;
        let mut nPort: i32 = 0;
        let mut bSuccess: i32 = 0;
        println!("Test case {}", build_str_from_raw_ptr(pszTest as *mut u8));
        let mut pszTextCursor: *const i8 = pszTest;
        bSuccess = ParseIPv4OrIPv6(
            &mut pszTextCursor,
            abyAddr.as_mut_ptr(),
            &mut nPort,
            &mut bIsIPv6,
        );
        if bSuccess == 0_i32 {
            println!(
                "parse failed, at about index {}; rest: {}",
                pszTextCursor.offset_from(pszTest) as i64,
                build_str_from_raw_ptr(pszTextCursor as *mut u8)
            );
            return;
        }
        print!("addr:  ");
        dumpbin(abyAddr.as_mut_ptr(), if bIsIPv6 != 0 { 16 } else { 4 });
        println!();
        if 0_i32 == nPort {
            print!("port absent");
        } else {
            print!("port:  {}", i32::from(htons(nPort as u16)));
        }
        print!("\n\n");
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
    unsafe {
        testcase((b"127.0.0.1\0" as *const u8).cast::<i8>());
        testcase((b"127.0.0.1:80\0" as *const u8).cast::<i8>());
        testcase((b"::1\0" as *const u8).cast::<i8>());
        testcase((b"[::1]:80\0" as *const u8).cast::<i8>());
        testcase((b"2605:2700:0:3::4713:93e3\0" as *const u8).cast::<i8>());
        testcase((b"[2605:2700:0:3::4713:93e3]:80\0" as *const u8).cast::<i8>());
        testcase((b"::ffff:192.168.173.22\0" as *const u8).cast::<i8>());
        testcase((b"[::ffff:192.168.173.22]:80\0" as *const u8).cast::<i8>());
        testcase((b"1::\0" as *const u8).cast::<i8>());
        testcase((b"[1::]:80\0" as *const u8).cast::<i8>());
        testcase((b"::\0" as *const u8).cast::<i8>());
        testcase((b"[::]:80\0" as *const u8).cast::<i8>());
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
