// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_006da9f0: i16;
    static mut DAT_006db580: u32;
    static mut DAT_00732e40: i16;
    static mut DAT_00732e54: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00497f00(param_1: u32);
    fn FUN_0042a060();
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0042b250(param_1: i32) -> u32;
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address is the game's own memory, read where the original reads.
    unsafe { (a as *const u8).read() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address is the game's own memory, read where the original reads.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address is the game's own memory, read where the original reads.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: address is the game's own memory, written where the original writes.
    unsafe { (a as *mut u8).write(v) }
}

unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address is the game's own memory, written where the original writes.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

unsafe fn ptr6() -> u32 {
    // SAFETY: reads the game's packed pointer global by value.
    unsafe { u32::from_ne_bytes(DAT_006d9d56) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00497c40(param_1: i16) {
    // SAFETY: all globals, callees and addressed memory are the game's own;
    // reads and writes follow the original's order and offsets.
    unsafe {
        let mut uvar3: u32 = DAT_006d9d50 as u32;
        let p = ptr6();
        if rd8(p.wrapping_add(0x34)) == 0x17 {
            FUN_0042a060();
        } else {
            let local_1 = rd16(p.wrapping_add(0x18));
            let local_2 = rd32(
                (local_1 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988),
            );
            if local_2 == 0 {
                FUN_0042a060();
            } else {
                FUN_00497f00(local_1 as u32);
            }
        }
        let i_var5 = DAT_006d4988.wrapping_add(uvar3.wrapping_mul(0x23));
        let p = ptr6();
        let local_2 = rd32(p.wrapping_add(0x1f));
        let local_3 = rd32(i_var5.wrapping_add(4));
        let local_4 = rd32(p.wrapping_add(0x23));
        let local_5 = rd32(i_var5.wrapping_add(8));
        let local_6 = rd32(p.wrapping_add(0x27));
        let local_7 = rd32(i_var5.wrapping_add(0xc));
        FUN_00418cd0(
            local_2.wrapping_sub(local_3) as i32,
            local_4.wrapping_sub(local_5) as i32,
            local_6.wrapping_sub(local_7) as i32,
        );
        let p = ptr6();
        let local_8 = ((DAT_006da9f0 as i32 as u32).wrapping_add(param_1 as i32 as u32)) as i16;
        wr16(p.wrapping_add(0x32), local_8 as u16);

        let mut uvar2: u16 = 0;
        // 0 = success path, 1 = LAB_00497e3c, 2 = LAB_00497e56
        let action: u8;
        if rd8(DAT_00732e54.wrapping_add(0xa4)) == 2 {
            let l8 = rd16(
                DAT_006d4988
                    .wrapping_add(uvar3.wrapping_mul(0x23))
                    .wrapping_add(0x10),
            ) as i16;
            let p = ptr6();
            let l9 = rd16(p.wrapping_add(0x32)) as i16;
            uvar2 = (l8 as i32 as u32).wrapping_sub(l9 as i32 as u32) as u16;
            if 0x8000 < uvar2 {
                uvar2 = (0u32.wrapping_sub(uvar2 as u32)) as u16;
            }
            if uvar2 < 0x2000 && 0xffff < DAT_006db580 {
                action = 2;
            } else {
                uvar3 = FUN_0042b250(DAT_00732e54 as i32);
                uvar2 = (uvar3 >> 1) as u16;
                let psvar1 = (DAT_006d9d50 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x10)
                    .wrapping_add(DAT_006d4988);
                let l8 = rd16(psvar1) as i16;
                let p = ptr6();
                let l9 = rd16(p.wrapping_add(0x32)) as i16;
                let mut uvar4 = (l8 as i32 as u32).wrapping_sub(l9 as i32 as u32) as u16;
                if 0x7fff < uvar4 {
                    uvar4 = (0u32.wrapping_sub(uvar4 as u32)) as u16;
                }
                if 0x300 < uvar4 {
                    action = 1;
                } else {
                    let p = ptr6();
                    let l8 = rd16(p.wrapping_add(0x32));
                    wr16(psvar1, l8);
                    action = 0;
                }
            }
        } else {
            uvar3 = FUN_0042b250(DAT_00732e54 as i32);
            uvar2 = (uvar3 >> 1) as u16;
            let psvar1 = (DAT_006d9d50 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x10)
                .wrapping_add(DAT_006d4988);
            let l8 = rd16(psvar1) as i16;
            let p = ptr6();
            let l9 = rd16(p.wrapping_add(0x32)) as i16;
            let mut uvar4 = (l8 as i32 as u32).wrapping_sub(l9 as i32 as u32) as u16;
            if 0x7fff < uvar4 {
                uvar4 = (0u32.wrapping_sub(uvar4 as u32)) as u16;
            }
            if 0x300 < uvar4 {
                action = 1;
            } else {
                let p = ptr6();
                let l8 = rd16(p.wrapping_add(0x32));
                wr16(psvar1, l8);
                action = 0;
            }
        }

        if action == 0 {
            let base = (DAT_006d9d50 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988);
            let i5 = rd32(base);
            wr8(i5.wrapping_add(0x90), 1);
            let base = (DAT_006d9d50 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988);
            let i5 = rd32(base);
            let v = rd8(i5.wrapping_add(0x90));
            wr8(i5.wrapping_add(0x89), v);
            wr8(DAT_00732e54.wrapping_add(0xab), 3);
        } else if action == 1 {
            wr8(DAT_00732e54.wrapping_add(0xab), 2);
            let l10 = ((uvar2 & 0x7fff) as u32).wrapping_add(0x8000) as u16;
            wr16(DAT_00732e54.wrapping_add(0xac), l10);
        }

        if rd8(DAT_00732e54.wrapping_add(0xa4)) == 3 {
            wr8(DAT_00732e54.wrapping_add(0xa4), 0);
        }
        let l8 = rd16(DAT_00732e54.wrapping_add(0xd)) as i16;
        if l8 != DAT_00732e40 {
            let p = ptr6();
            wr16(p.wrapping_add(0x2e), DAT_00732e40 as u16);
            wr16(DAT_00732e54.wrapping_add(0x9e), 0xffff);
            let l1 = rd16(DAT_00732e54.wrapping_add(0xd));
            let p = ptr6();
            let u4 = rd16(p.wrapping_add(0x2e));
            if l1 < u4 {
                wr8(DAT_00732e54.wrapping_add(0x9c), 2);
            } else {
                wr8(DAT_00732e54.wrapping_add(0x9c), 1);
            }
            wr16(DAT_00732e54.wrapping_add(0xbd), 0xffff);
            wr8(DAT_00732e54.wrapping_add(0x96), 0);
            wr8(DAT_00732e54.wrapping_add(0x97), 0);
            wr8(DAT_00732e54.wrapping_add(0x9d), 0);
        }
    }
}
