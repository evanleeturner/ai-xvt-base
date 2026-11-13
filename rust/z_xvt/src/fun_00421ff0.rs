// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006ccabe: u16;
    static mut DAT_006ccabc: u16;
    static mut DAT_006d4988: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f050(param_1: u16);
    fn FUN_00422f60(param_1: u32, param_2: u32);
    fn FUN_00478470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421ff0(param_1: u32, param_2: i8) -> u32 {
    // SAFETY: all globals and callees belong to the game; the pointers
    // computed from the entity table base are read and written at the
    // offsets the original uses.
    unsafe {
        let at = |p: i32, off: u32| -> *mut u8 { (p as u32).wrapping_add(off) as usize as *mut u8 };
        let u_var1: u16 = DAT_006ccabe;
        let mut u_var4: u16 = 0;
        let mut u_var5: u16 = DAT_006ccabc;
        let mut found = false;
        if DAT_006ccabc < DAT_006ccabe {
            let mut i_var2: u32;
            loop {
                let base = DAT_006d4988 as u32;
                if (base
                    .wrapping_add((u_var5 as u32).wrapping_mul(0x23))
                    .wrapping_add(3) as usize as *const u8)
                    .read()
                    == 0
                {
                    let ptr = base
                        .wrapping_add(0x1f)
                        .wrapping_add((u_var5 as u32).wrapping_mul(0x23))
                        as usize as *const i32;
                    at(ptr.read_unaligned(), 0x83)
                        .cast::<u16>()
                        .write_unaligned(0);
                    i_var2 = DAT_006d4988 as u32;
                    at(ptr.read_unaligned(), 1).write(0);
                    break;
                }
                u_var5 = u_var5.wrapping_add(1);
                i_var2 = DAT_006d4988 as u32;
                if !(u_var5 < DAT_006ccabe) {
                    break;
                }
            }
            u_var4 = (i_var2 >> 16) as u16;
            if u_var5 < u_var1 {
                FUN_0040f050(u_var5);
                found = true;
            }
        }
        if !found {
            u_var5 = 0xffff;
        }
        if u_var5 != 0xffff {
            FUN_00422f60(u_var5 as u32, param_1 & 0xffff);
            let i_var2: u32 = (u_var5 as u32).wrapping_mul(0x23);
            let ptr = (DAT_006d4988 as u32)
                .wrapping_add(0x1f)
                .wrapping_add(i_var2) as usize as *const i32;
            at(ptr.read_unaligned(), 0).write(3);
            at(DAT_006d4988 as i32, 2u32.wrapping_add(i_var2)).write(0xb);
            at(ptr.read_unaligned(), 1).write(0);
            at(DAT_006d4988 as i32, 3u32.wrapping_add(i_var2)).write(0x59);
            let src = ((param_1 & 0xffff).wrapping_mul(0x23))
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32) as usize as *const u8;
            let b = src.read();
            at(ptr.read_unaligned(), 0x85).write(b);
            at(DAT_006d4988 as i32, 0x1bu32.wrapping_add(i_var2))
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            at(ptr.read_unaligned(), 0x81)
                .cast::<u16>()
                .write_unaligned(0);
            let u_var3 = FUN_00478470();
            let m = u_var3 & 0xffff0007;
            let v = ((m as u16 as i16) as i32)
                .wrapping_add(4)
                .wrapping_mul(0xec) as i16;
            at(ptr.read_unaligned(), 0x7f)
                .cast::<i16>()
                .write_unaligned(v);
            at(DAT_006d4988 as i32, 0x19u32.wrapping_add(i_var2))
                .write((param_2 as i32).wrapping_mul(2) as u8);
            at(DAT_006d4988 as i32, 0x1au32.wrapping_add(i_var2)).write(0);
            return ((m >> 16) << 16) | (u_var5 as u32);
        }
        ((u_var4 as u32) << 16) | 0xffff
    }
}
