// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075dc4e: u16;
    static mut DAT_0075965b: i8;
    static mut DAT_0075dc4a: i32;
    static mut DAT_0075dc46: i32;
}

#[inline(always)]
unsafe fn udiv(a: u32, b: u32) -> u32 {
    let q: u32;
    // SAFETY: unsigned divide; a zero divisor faults as the original's does.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) b,
            inout("eax") a => q,
            inout("edx") 0u32 => _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df890(param_1: u32) -> u32 {
    // SAFETY: the globals and the table are the game's own memory; the table
    // entry is read and written where the original does.
    unsafe {
        let entry = (&raw mut DAT_0075dc4e)
            .cast::<u8>()
            .byte_add(param_1.wrapping_mul(2) as usize)
            .cast::<u16>();
        let u_var1 = entry.read_unaligned();
        if u_var1 != 0 {
            return u_var1 as u32;
        }
        let mut u_var4: u32;
        let mut u_var3: u32;
        if DAT_0075965b == 0 {
            u_var4 = param_1 & 0x7e0;
            u_var3 = (param_1 & 0xf800) >> 0xb;
        } else {
            u_var4 = param_1 & 0x3e0;
            u_var3 = (param_1 & 0x7c00) >> 10;
        }
        let i_var5: u32 = DAT_0075dc4a.wrapping_sub(DAT_0075dc46) as u32;
        let d = DAT_0075dc4a as u32;
        u_var4 = udiv((u_var4 >> 5).wrapping_mul(i_var5), d);
        u_var3 = udiv(u_var3.wrapping_mul(i_var5), d);
        let i_var2: u32;
        if DAT_0075965b == 0 {
            u_var4 &= 0x3f;
            i_var2 = (u_var3 & 0x1f) << 6;
        } else {
            u_var4 &= 0x1f;
            i_var2 = (u_var3 & 0x1f) << 5;
        }
        let last = udiv(i_var5.wrapping_mul(param_1 & 0x1f), d) & 0x1f;
        let mut r = i_var2
            .wrapping_add(u_var4)
            .wrapping_mul(0x20)
            .wrapping_add(last);
        if r == 0 {
            r = 1;
        }
        entry.write_unaligned(r as u16);
        r
    }
}
