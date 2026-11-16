// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn free(p: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc6c0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7320(
    param_1: *mut core::ffi::c_void,
    param_2: i32,
    param_3: u32,
    param_4: i32,
) -> u32 {
    let _ = param_3;
    let hdr = param_4 as u32 as usize;
    // SAFETY: param_4 is the address of the caller's header; param_2 is the
    // caller's destination buffer; the game's own functions and heap are used
    // as the original does.
    unsafe {
        let rd = |off: usize| -> i32 { ((hdr + off) as *const i32).read_unaligned() };
        let width = rd(4);
        let s = (width >> 31) as u32;
        let mut local_c: u32 = ((((width as u32 ^ s).wrapping_sub(s)) & 3) ^ s).wrapping_sub(s);
        if local_c != 0 {
            local_c = 4u32.wrapping_sub(local_c);
        }
        let mut mem: *mut core::ffi::c_void = core::ptr::null_mut();
        if rd(0x10) != 0 {
            mem = malloc(rd(0x14) as u32);
            if mem.is_null() {
                return 0;
            }
        }
        let mut local_4 = [0u8; 4];
        if rd(0x10) == 0 {
            let mut s9: i16 = 0;
            if 0 < rd(8) {
                loop {
                    let i8_ = s9 as i32;
                    s9 = s9.wrapping_add(1);
                    let dst = (rd(8) as u32)
                        .wrapping_sub(i8_ as u32)
                        .wrapping_sub(1)
                        .wrapping_mul(rd(4) as u32)
                        .wrapping_add(param_2 as u32);
                    FUN_004cc6c0(
                        param_1,
                        dst as usize as *mut core::ffi::c_void,
                        rd(4) as u32,
                    );
                    FUN_004cc6c0(param_1, local_4.as_mut_ptr().cast(), local_c);
                    if !((s9 as i32) < rd(8)) {
                        break;
                    }
                }
            }
        } else if rd(0x10) == 1 {
            FUN_004cc6c0(param_1, mem, rd(0x14) as u32);
            let m = mem as *const u8;
            let base = param_2 as u32;
            let mut done = false;
            let mut i10: i32 = (rd(8) as u32).wrapping_sub(1).wrapping_mul(rd(4) as u32) as i32;
            let mut i8_: i32 = 0;
            let mut local_8: i32 = i10;
            loop {
                let b2 = *m.add(i8_ as u32 as usize);
                let b3 = *m.add((i8_ as u32).wrapping_add(1) as usize);
                let mut i6: i32;
                if b2 == 0 {
                    i6 = (i8_ as u32).wrapping_add(2) as i32;
                    let u4 = b3 as u16;
                    if b3 == 0 {
                        i10 = (local_8 as u32).wrapping_sub(rd(4) as u32) as i32;
                        local_8 = i10;
                    } else if u4 == 1 {
                        done = true;
                    } else if u4 == 2 {
                        let v1 = *m.add(i6 as u32 as usize);
                        i6 = (i8_ as u32).wrapping_add(4) as i32;
                        let v3 = *m.add((i8_ as u32).wrapping_add(3) as usize);
                        i8_ = (local_8 as u32).wrapping_sub((v3 as u32).wrapping_mul(rd(4) as u32))
                            as i32;
                        i10 = (i10 as u32)
                            .wrapping_sub(local_8 as u32)
                            .wrapping_add(v1 as u32)
                            .wrapping_add(i8_ as u32) as i32;
                        local_8 = i8_;
                    } else {
                        let mut s9: i16 = 0;
                        if b3 != 0 {
                            loop {
                                s9 = s9.wrapping_add(1);
                                let v = *m.add(i6 as u32 as usize);
                                *(base.wrapping_add(i10 as u32) as usize as *mut u8) = v;
                                i6 = (i6 as u32).wrapping_add(1) as i32;
                                i10 = (i10 as u32).wrapping_add(1) as i32;
                                if !(s9 < u4 as i16) {
                                    break;
                                }
                            }
                        }
                        if (b3 & 1) != 0 {
                            i6 = (i6 as u32).wrapping_add(1) as i32;
                        }
                    }
                } else {
                    let mut s9: i16 = 0;
                    i6 = (i8_ as u32).wrapping_add(2) as i32;
                    loop {
                        s9 = s9.wrapping_add(1);
                        *(base.wrapping_add(i10 as u32) as usize as *mut u8) = b3;
                        i10 = (i10 as u32).wrapping_add(1) as i32;
                        if !(s9 < b2 as u16 as i16) {
                            break;
                        }
                    }
                }
                i8_ = i6;
                if done {
                    break;
                }
            }
        }
        if !mem.is_null() {
            free(mem);
        }
    }
    1
}
