// SPDX-License-Identifier: MIT

#[repr(C)]
struct LocalObj {
    local_90: [i16; 2],
    local_8c: i32,
    local_88: i16,
    local_84: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005348a0: *const u8;
    static mut DAT_00534aac: i32;
    static mut DAT_0081bcd0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut u32, param_2: *mut u32);
    fn __isctype(c: u32, mask: i32) -> u32;
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32) -> u32;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

unsafe fn isc(c: u8) -> u32 {
    // SAFETY: reads the game's own ctype globals and table.
    unsafe {
        if DAT_00534aac < 2 {
            (DAT_005348a0
                .byte_add((c as usize) * 2)
                .cast::<u16>()
                .read_unaligned() as u32)
                & 8
        } else {
            __isctype(c as u32, 8)
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f3f10(param_1: *mut u32, param_2: i32, param_3: i16) {
    let mut obj = LocalObj {
        local_90: [0; 2],
        local_8c: 0,
        local_88: 0,
        local_84: 0,
    };
    let mut local_80: [i8; 128] = [0; 128];
    let l80 = local_80.as_mut_ptr();
    let objp = &raw mut obj;
    let b = |off: i16| -> u8 {
        // SAFETY: param_2 is the caller's string address.
        unsafe { ((param_2.wrapping_add(off as i32)) as *const u8).read() }
    };
    // SAFETY: callees and globals are the game's own; buffers are local and
    // indexed exactly as the original does.
    unsafe {
        FUN_004dd000(objp as *mut u32, param_1);
        (*objp).local_84 = ((*objp).local_8c as u32).wrapping_add(0xd) as i32;
        let mut s10: i16 = -1;
        let mut s12: i16 = 0;
        let mut s11: i16 = 0;
        *l80 = 0;
        let mut local_96: i16 = -1;
        let mut local_98: i16 = 0;
        let mut b5 = false;
        loop {
            let i6 = s12 as i32;
            let b1 = b(s12);
            let mut s8: i16;
            if b1 == 0x24 || b1 == 0 {
                local_96 = s12;
                if param_3 == 0 && b1 == 0x24 {
                    local_96 = s12.wrapping_add(-1);
                }
                s12 = s12.wrapping_add(1);
                s8 = s12;
                s10 = s11;
                if b(s12) == 0 {
                    b5 = true;
                }
            } else {
                if b1 == 0x20 {
                    local_98 = s12;
                }
                let u7 = isc(b1);
                if u7 == 0 {
                    loop {
                        if isc(b(s12)) != 0 {
                            break;
                        }
                        let i6b = s12 as i32;
                        let c = b(s12);
                        if c == b'$' || c == 0 {
                            break;
                        }
                        let idx = i6b.wrapping_sub(s11 as i32) as isize;
                        if c == b'[' {
                            *l80.offset(idx) = 2;
                        } else if c == b']' {
                            *l80.offset(idx) = 1;
                        } else {
                            *l80.offset(idx) = c as i8;
                        }
                        s12 = s12.wrapping_add(1);
                    }
                } else {
                    s12 = s12.wrapping_add(1);
                    *l80.offset(i6.wrapping_sub(s11 as i32) as isize) = b(i6 as i16) as i8;
                }
                *l80.offset((s12 as i32).wrapping_sub(s11 as i32) as isize) = 0;
                let r = FUN_004db2f0(l80, 0xc);
                s8 = s11;
                if ((*objp).local_88.wrapping_sub((*objp).local_90[0])) <= (r as i16) {
                    s12 = local_98.wrapping_add(1);
                    local_96 = local_98;
                    s8 = s12;
                    s10 = s11;
                }
            }
            s11 = s8;
            let mut bvar3 = false;
            let mut s8b: i16 = 0;
            let mut bvar4 = false;
            if 0 < s10 {
                loop {
                    bvar3 = bvar4;
                    if b(s8b) == b'[' {
                        bvar3 = true;
                    }
                    if b(s8b) == b']' {
                        bvar3 = false;
                    }
                    s8b = s8b.wrapping_add(1);
                    bvar4 = bvar3;
                    if !(s8b < s10) {
                        break;
                    }
                }
            }
            if s10 != -1 {
                if bvar3 {
                    *l80 = 2;
                }
                let mut u9: u16 = bvar3 as u16;
                *l80.offset(u9 as i16 as isize) = 0;
                while s10 <= local_96 {
                    let c = b(s10);
                    let p = l80.offset(u9 as i16 as isize);
                    if c == b'[' {
                        *p = 2;
                    } else if c == b']' {
                        *p = 1;
                    } else {
                        *p = c as i8;
                    }
                    u9 = u9.wrapping_add(1);
                    s10 = s10.wrapping_add(1);
                }
                *l80.offset(u9 as i16 as isize) = 0;
                if param_3 == 0 && *l80 == b'>' as i8 {
                    FUN_004dd030(objp as *mut i32, 0, 1);
                    FUN_004da920(10, l80.add(1), objp as *mut i32, DAT_0081bcd0);
                    FUN_004dd030(objp as *mut i32, 0, -1);
                } else {
                    FUN_004daac0(10, l80, objp as *mut i32, 0, 1, 0xffff);
                }
                s10 = -1;
                local_96 = -1;
                FUN_004dd030(objp as *mut i32, 0, 0xe);
            }
            if b5 {
                return;
            }
        }
    }
}
