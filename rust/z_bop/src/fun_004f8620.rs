// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052dc84: i32;
    static mut PTR_DAT_0052da78: *mut u8;
    static mut DAT_00b6a2c0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut u32, param_2: *mut u32);
    fn __isctype(c: u32, mask: i32) -> i32;
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> u32;
}

unsafe fn isc8(c: u8) -> u32 {
    // SAFETY: the game's own ctype table and function, used as the original does.
    unsafe {
        if DAT_0052dc84 < 2 {
            (PTR_DAT_0052da78
                .byte_add((c as usize) * 2)
                .cast::<u16>()
                .read_unaligned() as u32)
                & 8
        } else {
            __isctype(c as u32, 8) as u32
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f8620(param_1: *mut u32, param_2: i32, param_3: i16) {
    let mut local_150: [i32; 4] = [0; 4];
    let mut local_140: [u8; 320] = [0; 320];
    let buf = local_140.as_mut_ptr();
    let l150 = local_150.as_mut_ptr();
    let mut sv8: i16;
    let mut sv10: i16;
    let mut sv11: i16;
    let mut sv12: i16;
    let mut local_15c: i16;
    let mut local_15a: i16;
    let mut b4: i32;
    // SAFETY: the callees are the game's own; param_2 points at the caller's
    // string and the local buffer is indexed exactly as the original does.
    unsafe {
        FUN_004d6480(l150 as *mut u32, param_1);
        local_150[3] = local_150[1].wrapping_add(0xd);
        sv10 = -1;
        sv12 = 0;
        sv11 = 0;
        *buf = 0;
        local_15a = -1;
        local_15c = 0;
        b4 = 0;
        loop {
            let mut i5 = sv12 as i32;
            let p6 = param_2.wrapping_add(i5) as *const u8;
            let mut b1 = p6.read();
            if b1 == 0x24 || b1 == 0 {
                local_15a = sv12;
                if param_3 == 0 && b1 == 0x24 {
                    local_15a = sv12.wrapping_sub(1);
                }
                sv12 = sv12.wrapping_add(1);
                sv8 = sv12;
                sv10 = sv11;
                if *(param_2.wrapping_add(sv12 as i32) as *const u8) == 0 {
                    b4 = 1;
                }
            } else {
                if b1 == 0x20 {
                    local_15c = sv12;
                }
                let u7 = isc8(b1);
                if u7 == 0 {
                    loop {
                        let u = isc8(*(param_2.wrapping_add(sv12 as i32) as *const u8));
                        if u != 0 {
                            break;
                        }
                        i5 = sv12 as i32;
                        b1 = *(param_2.wrapping_add(i5) as *const u8);
                        if b1 == 0x24 || b1 == 0 {
                            break;
                        }
                        let idx = i5.wrapping_sub(sv11 as i32) as isize;
                        if b1 == 0x5b {
                            *buf.offset(idx) = 2;
                        } else if b1 == 0x5d {
                            *buf.offset(idx) = 1;
                        } else {
                            *buf.offset(idx) = b1;
                        }
                        sv12 = sv12.wrapping_add(1);
                    }
                } else {
                    sv12 = sv12.wrapping_add(1);
                    *buf.offset(i5.wrapping_sub(sv11 as i32) as isize) = p6.read();
                }
                *buf.offset((sv12 as i32).wrapping_sub(sv11 as i32) as isize) = 0;
                let r = FUN_004dbfa0(buf as *mut i8, 10);
                sv8 = sv11;
                let d = (local_150[2] as i16).wrapping_sub(local_150[0] as i16);
                if d <= (r as i16) {
                    sv12 = local_15c.wrapping_add(1);
                    local_15a = local_15c;
                    sv8 = sv12;
                    sv10 = sv11;
                }
            }
            sv11 = sv8;
            let mut b2: i32 = 0;
            sv8 = 0;
            let mut b3: i32 = 0;
            if 0 < sv10 {
                loop {
                    b2 = b3;
                    let c = *(param_2.wrapping_add(sv8 as i32) as *const u8);
                    if c == b'[' {
                        b2 = 1;
                    }
                    if c == b']' {
                        b2 = 0;
                    }
                    sv8 = sv8.wrapping_add(1);
                    b3 = b2;
                    if !(sv8 < sv10) {
                        break;
                    }
                }
            }
            if sv10 != -1 {
                if b2 != 0 {
                    *buf = 2;
                }
                let mut u9 = b2 as u16;
                *buf.offset(u9 as i16 as isize) = 0;
                while sv10 <= local_15a {
                    b1 = *(param_2.wrapping_add(sv10 as i32) as *const u8);
                    let o = u9 as i16 as isize;
                    if b1 == 0x5b {
                        *buf.offset(o) = 2;
                    } else if b1 == 0x5d {
                        *buf.offset(o) = 1;
                    } else {
                        *buf.offset(o) = b1;
                    }
                    u9 = u9.wrapping_add(1);
                    sv10 = sv10.wrapping_add(1);
                }
                *buf.offset(u9 as i16 as isize) = 0;
                if param_3 == 0 && *buf == 0x3e {
                    FUN_004d64a0(l150, 0, 1);
                    FUN_004db640(10, buf.add(1) as *mut i8, l150, DAT_00b6a2c0);
                    FUN_004d64a0(l150, 0, -1);
                } else {
                    FUN_004db7d0(10, buf as *mut i8, l150, 0, 1, 0xffff);
                }
                sv10 = -1;
                local_15a = -1;
                FUN_004d64a0(l150, 0, 0xe);
            }
            if b4 != 0 {
                return;
            }
        }
    }
}
