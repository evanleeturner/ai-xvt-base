// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebf9: [u8; 4];
    static mut DAT_00acebf1: [u8; 4];
    static mut DAT_00acebf5: [u8; 4];
    static mut DAT_00acf145: [u8; 4];
    static mut DAT_00acec45: [u8; 0];
    static mut DAT_00b69a89: [u8; 4];
    static mut DAT_00b641c5: [u8; 0];
    static mut DAT_00b63fa1: [u8; 0];
    static mut DAT_00b63f99: [u8; 0];
    static mut DAT_00b53579: [u8; 4];
    static mut DAT_00b53571: [u8; 4];
    static mut DAT_00acf571: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d23b0(param_1: i32) -> u32;
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[inline(always)]
unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: caller passes a pointer into the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: caller passes a pointer into the game's own memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d25d0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets mirror the original's table strides.
    unsafe {
        let a_b641c5 = &raw mut DAT_00b641c5 as *mut u8;
        let a_b63fa1 = &raw mut DAT_00b63fa1 as *mut u8;
        let a_b63f99 = &raw mut DAT_00b63f99 as *mut u8;
        let a_cnt = &raw mut DAT_00b53579 as *mut u8;
        let a_idx = &raw mut DAT_00b53571 as *mut u8;
        let a_ent = &raw mut DAT_00acf571 as *mut u8;
        let a_cur = &raw mut DAT_00acebf1 as *mut u8;

        if r32(&raw mut DAT_00acebf9 as *const u8) == 0 {
            let uvar2 = FUN_004d23b0(r32(a_cur as *const u8) as i32);
            if r32(&raw mut DAT_00b69a89 as *const u8) <= uvar2 {
                return 0;
            }
            let off = uvar2.wrapping_mul(0x238) as usize;
            let dvar3 = GetTickCount();
            let t = r32(a_b641c5.byte_add(off));
            if 45000 < dvar3.wrapping_sub(t) {
                let dvar3 = GetTickCount();
                w32(a_b641c5.byte_add(off), dvar3);
                let cnt = r32(a_cnt);
                if cnt < 0x400 {
                    let idx = r32(a_idx);
                    let e = a_ent.byte_add(idx.wrapping_mul(0x210) as usize);
                    w32(e.byte_add(0x10), 0x46);
                    w32(e, r32(a_cur as *const u8));
                    w32(e.byte_add(4), 4);
                    w32(e.byte_add(8), 0);
                    e.byte_add(0xc).write(0);
                    e.byte_add(0xf).write(0);
                    e.byte_add(0xd).write(0);
                    let t = r32(a_b63fa1.byte_add(off));
                    let mut uvar4 = t.wrapping_add(1);
                    if 0x7f < uvar4 {
                        uvar4 = 0;
                    }
                    w32(a_b63fa1.byte_add(off), uvar4);
                    e.byte_add(0xe).write(uvar4 as u8);
                    let cnt = r32(a_cnt).wrapping_add(1);
                    w32(a_cnt, cnt);
                    let mut idx = r32(a_idx).wrapping_add(1);
                    w32(a_idx, idx);
                    if 0x3ff < idx {
                        idx = 0;
                        w32(a_idx, idx);
                    }
                }
            }
        } else {
            let end = &raw mut DAT_00acf145 as *mut u8;
            let mut p5 = &raw mut DAT_00acec45 as *mut u8;
            loop {
                let ivar1 = r32(p5 as *const u8) as i32;
                if ivar1 != 0
                    && r32(end as *const u8) != ivar1 as u32
                    && r32(&raw mut DAT_00acebf5 as *const u8) != ivar1 as u32
                    && r32(p5.byte_add(4) as *const u8) != 0
                {
                    let uvar2 = FUN_004d23b0(ivar1);
                    if uvar2 < r32(&raw mut DAT_00b69a89 as *const u8) {
                        let off = uvar2.wrapping_mul(0x238) as usize;
                        let dvar3 = GetTickCount();
                        let t = r32(a_b641c5.byte_add(off));
                        if 45000 < dvar3.wrapping_sub(t) {
                            let dvar3 = GetTickCount();
                            w32(a_b641c5.byte_add(off), dvar3);
                            let cnt = r32(a_cnt);
                            if cnt < 0x400 {
                                let mut local_8: [u32; 2] = [0; 2];
                                local_8[0] = 0x5b;
                                let t2 = r32(p5 as *const u8);
                                FUN_004cef70(t2 as i32, local_8.as_mut_ptr(), 4);
                                let idx = r32(a_idx);
                                let e = a_ent.byte_add(idx.wrapping_mul(0x210) as usize);
                                w32(e.byte_add(0x10), 0x47);
                                let t2 = r32(p5 as *const u8);
                                w32(e, t2);
                                w32(e.byte_add(4), 4);
                                w32(e.byte_add(8), 0);
                                e.byte_add(0xc).write(0);
                                e.byte_add(0xf).write(0);
                                e.byte_add(0xd).write(1);
                                let t = r32(a_b63f99.byte_add(off));
                                let mut uvar4 = t.wrapping_add(1);
                                if 0x7f < uvar4 {
                                    uvar4 = 0;
                                }
                                w32(a_b63f99.byte_add(off), uvar4);
                                e.byte_add(0xe).write(uvar4 as u8);
                                let cnt = r32(a_cnt).wrapping_add(1);
                                w32(a_cnt, cnt);
                                let mut idx = r32(a_idx).wrapping_add(1);
                                w32(a_idx, idx);
                                if 0x3ff < idx {
                                    idx = 0;
                                    w32(a_idx, idx);
                                }
                            }
                        }
                    }
                }
                p5 = p5.byte_add(40);
                if !((p5 as usize) < (end as usize)) {
                    break;
                }
            }
        }
        1
    }
}
