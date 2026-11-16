// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d34: u16;
    static mut DAT_0052b08f: [u8; 0];
    static mut DAT_0052b091: [u8; 0];
    static mut DAT_0071e6be: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478320(a: u32, b: u16) -> u32;
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: the address is game memory written where the original writes it.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00445920(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // in the same order as the original.
    unsafe {
        let i_var5 = (param_1 as u32).wrapping_mul(0x23);
        let mut i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
        let w = rd16(i_var3.wrapping_add(0x79));
        let mut u_var9 = (param_2 as u32).wrapping_sub(w as u32);
        if u_var9 == 0 {
            return;
        }
        let idx = (DAT_006d9d34 as u32).wrapping_mul(0xef);
        let hi = (idx >> 16) << 16;
        if u_var9 < 0x8000 {
            let base = (&raw mut DAT_0052b08f).cast::<u8>();
            let val = base.add(idx as usize).cast::<u16>().read_unaligned();
            let mut u_var6 = FUN_00478320(hi | val as u32, 0x4000);
            u_var6 &= 0xffff;
            if u_var6 == 0 {
                u_var6 = 1;
            }
            let val = base.add(idx as usize).cast::<u16>().read_unaligned();
            let u_var7 = FUN_00478320(val.wrapping_sub(u_var6 as u16) as u32, param_4);
            u_var6 = u_var6.wrapping_add(u_var7 & 0xffff);
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let tmp = rd32(i_var3.wrapping_add(0xa7));
            let s = rd16(tmp.wrapping_add(0xbf));
            if s == 0 {
                u_var6 = u_var6.wrapping_mul(3);
            }
            if u_var6 <= u_var9 {
                u_var9 = u_var6;
            }
            let ua = rd16(i_var3.wrapping_add(0x7b));
            let u_var2 = ua as u32;
            let prod = (DAT_0071e6be as u32).wrapping_mul(u_var9);
            let u_var4 = prod / 0xec;
            let rem = u_var4
                .wrapping_mul(0xec_u32.wrapping_neg())
                .wrapping_add(prod);
            let f = (rem.wrapping_mul(0x10000) / 0xec) as u16;
            let ub = f.wrapping_add(u_var2 as u16);
            wr16(i_var3.wrapping_add(0x7b), ub);
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let ub = rd16(i_var3.wrapping_add(0x7b));
            if ub < u_var2 as u16 {
                let s = rd16(i_var3.wrapping_add(0x79));
                wr16(i_var3.wrapping_add(0x79), s.wrapping_add(1));
            }
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let s = rd16(i_var3.wrapping_add(0x79));
            wr16(i_var3.wrapping_add(0x79), s.wrapping_add(u_var4 as u16));
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let s = rd16(i_var3.wrapping_add(0x79));
            if 0xe10 < s {
                wr16(i_var3.wrapping_add(0x79), 0xe10);
                return;
            }
        } else if param_3 == 1 {
            let base = (&raw mut DAT_0052b091).cast::<u8>();
            let val = base.add(idx as usize).cast::<u16>().read_unaligned();
            let mut u_var6 = FUN_00478320(hi | val as u32, 0x4000);
            u_var6 &= 0xffff;
            if u_var6 == 0 {
                u_var6 = 1;
            }
            let val = base.add(idx as usize).cast::<u16>().read_unaligned();
            let arg2 = (0u32.wrapping_sub(param_4 as u32)).wrapping_sub(1) as u16;
            let u_var7 = FUN_00478320(hi | val.wrapping_sub(u_var6 as u16) as u32, arg2);
            u_var6 = u_var6.wrapping_add(u_var7 & 0xffff);
            let mut u_var7 = 0u32.wrapping_sub(u_var9);
            if u_var6 <= u_var7 {
                u_var7 = u_var6;
            }
            let prod = (DAT_0071e6be as u32).wrapping_mul(u_var7);
            let u_var4 = prod / 0xec;
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let ua = rd16(i_var3.wrapping_add(0x7b));
            let u_var2 = ua as u32;
            let rem = u_var4
                .wrapping_mul(0xec_u32.wrapping_neg())
                .wrapping_add(prod);
            let f = (rem.wrapping_mul(0x10000) / 0xec) as u16;
            let ub = (u_var2 as u16).wrapping_sub(f);
            wr16(i_var3.wrapping_add(0x7b), ub);
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let ub = rd16(i_var3.wrapping_add(0x7b));
            if (u_var2 as u16) < ub {
                let s = rd16(i_var3.wrapping_add(0x79));
                wr16(i_var3.wrapping_add(0x79), s.wrapping_sub(1));
            }
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let s = rd16(i_var3.wrapping_add(0x79));
            wr16(i_var3.wrapping_add(0x79), s.wrapping_sub(u_var4 as u16));
            i_var3 = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let s = rd16(i_var3.wrapping_add(0x79));
            if 0x8000 < s {
                wr16(i_var3.wrapping_add(0x79), 0);
            }
        }
    }
}
