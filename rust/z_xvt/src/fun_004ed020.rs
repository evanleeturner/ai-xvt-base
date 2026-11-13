// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758620: u32;
    static mut DAT_00757cf6: u32;
    static mut DAT_00757e44: u32;
    static mut DAT_00757680: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ed020() -> u32 {
    // SAFETY: all globals and the callee are the game's own memory; the
    // reads and writes happen at the same addresses as in the original.
    unsafe {
        let base = (&raw mut DAT_00758620) as *mut u8;
        let mut i_var5: i32 = 1;
        DAT_00758620 = 0x61;
        let mut p6 = (&raw mut DAT_00757cf6) as *const u8;
        let limit = ((&raw mut DAT_00757e44) as usize).wrapping_add(2);
        loop {
            let old = (i_var5 as usize).wrapping_mul(4);
            let v0 = p6.cast::<u32>().read_unaligned();
            let v1 = p6.byte_add(4).cast::<u32>().read_unaligned();
            base.wrapping_add(old).cast::<u32>().write_unaligned(v0);
            let v2 = p6.byte_add(8).cast::<u32>().read_unaligned();
            let p7 = p6.byte_add(0x2a);
            base.wrapping_add(old.wrapping_add(4))
                .cast::<u32>()
                .write_unaligned(v1);
            i_var5 = i_var5.wrapping_add(5);
            let v3 = p6.byte_add(12).cast::<u32>().read_unaligned();
            base.wrapping_add(old.wrapping_add(8))
                .cast::<u32>()
                .write_unaligned(v2);
            let v4 = p6.byte_add(16).cast::<u32>().read_unaligned();
            base.wrapping_add(old.wrapping_add(12))
                .cast::<u32>()
                .write_unaligned(v3);
            base.wrapping_add(old.wrapping_add(16))
                .cast::<u32>()
                .write_unaligned(v4);
            p6 = p7;
            if !((p7 as usize) < limit) {
                break;
            }
        }

        let mut p10 = base.wrapping_add((i_var5 as usize).wrapping_mul(4));
        let mut p8 = (&raw mut DAT_00757680) as *const u8;
        let mut i_var11: i32 = 10;
        loop {
            let mut i_var4: i32 = 8;
            let mut p9 = p10;
            loop {
                let b = p8.read();
                p10 = p9.wrapping_add(1);
                p8 = p8.wrapping_add(4);
                i_var4 -= 1;
                p9.write(b);
                p9 = p10;
                if i_var4 == 0 {
                    break;
                }
            }
            i_var11 -= 1;
            if i_var11 == 0 {
                break;
            }
        }

        FUN_004cb830(
            0,
            &raw mut DAT_00758620,
            (i_var5 as u32).wrapping_mul(4).wrapping_add(0x50),
        );
        1
    }
}
