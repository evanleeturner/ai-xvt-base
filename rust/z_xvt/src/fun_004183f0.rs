// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [i16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004183f0(param_1: u16) -> i16 {
    let i_var1: usize = ((param_1 & 0x7fc0) >> 6) as usize;
    // SAFETY: DAT_00523c60 is the game's own table; the index is at most 0x1ff,
    // so index + 1 stays within the table the original reads.
    unsafe {
        let base = (&raw mut DAT_00523c60).cast::<i16>();
        let a = base.add(i_var1 + 1).read() as u16;
        let b = base.add(i_var1).read() as u16;
        let u_var2: u16 = a.wrapping_sub(b);
        let mut u_var3: u16 = u_var2;
        if (u_var2 as i16) < 0 {
            u_var3 = (0u32.wrapping_sub(u_var2 as u32)) as u16;
        }
        let mut s_var4: i16 =
            ((((param_1 << 10) as u32).wrapping_mul(u_var3 as u32)) >> 0x10) as i16;
        if (u_var2 as i16) < 0 {
            s_var4 = (0u32.wrapping_sub(s_var4 as u16 as u32)) as i16;
        }
        ((s_var4 as u16 as u32).wrapping_add(b as u32)) as i16
    }
}
