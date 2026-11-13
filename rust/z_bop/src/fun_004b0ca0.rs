// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c64: i32;
    static mut DAT_00528c6c: i32;
    static mut DAT_006642f0: u16;
}

#[inline(always)]
unsafe fn rb(base: i32, off: i32) -> u32 {
    // SAFETY: the base is a game-owned structure address; the offset is within it.
    unsafe { ((base.wrapping_add(off)) as *const u8).read() as u32 & 0x1f }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b0ca0(param_1: *mut u16, param_2: u32) {
    // SAFETY: globals are the game's own; param_1 points at the caller's
    // pixel data and the destination is the game's buffer at DAT_006642f0.
    unsafe {
        let i_var2 = DAT_00528c6c;
        let i_var1 = DAT_00528c64;
        let mut src = param_1;
        if i_var2 == i_var1 {
            let mut dst = (&raw mut DAT_006642f0) as *mut u32;
            let mut n = (param_2 & 0x7fff_ffff) >> 1;
            while n != 0 {
                dst.write_unaligned(src.cast::<u32>().read_unaligned());
                src = src.byte_add(4);
                dst = dst.byte_add(4);
                n -= 1;
            }
            let mut n = param_2.wrapping_mul(2) & 3;
            while n != 0 {
                (dst as *mut u8).write(src.read_unaligned() as u8);
                src = src.byte_add(1);
                dst = dst.byte_add(1);
                n -= 1;
            }
            return;
        }
        if 0 < (param_2 as i32) {
            let base = &raw mut DAT_006642f0;
            let mut dst = base;
            let mut cnt = param_2;
            loop {
                let v = src.read_unaligned() as u32;
                let mut r: u16;

                let t = (((v >> rb(i_var1, 0x14)) as u8 as u32) << rb(i_var1, 0x20)) as u8 as u32;
                let t = (t >> rb(i_var2, 0x20)) as u16 as u32;
                r = (t << rb(i_var2, 0x14)) as u16;
                dst.write_unaligned(r);

                let t = (((v >> rb(i_var1, 0x18)) as u8 as u32) << rb(i_var1, 0x24)) as u8 as u32;
                let t = (t >> rb(i_var2, 0x24)) as u16 as u32;
                r = ((t << rb(i_var2, 0x18)) as u16) | r;
                dst.write_unaligned(r);

                let t = (((v >> rb(i_var1, 0x1c)) as u8 as u32) << rb(i_var1, 0x28)) as u8 as u32;
                let t = (t >> rb(i_var2, 0x28)) as u16 as u32;
                r = ((t << rb(i_var2, 0x1c)) as u16) | r;
                dst.write_unaligned(r);

                if dst != base {
                    let a = (0xffu32 >> rb(i_var2, 0x34)) as u16 as u32;
                    dst.write_unaligned(((a << rb(i_var2, 0x30)) as u16) | r);
                }
                src = src.byte_add(2);
                dst = dst.byte_add(2);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
    }
}
