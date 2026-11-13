// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0052b07e: [u8; 1];
    static mut DAT_0052b0b3: [u8; 1];
    static mut DAT_0052b0b6: [u8; 1];
    static mut DAT_0052ad38: [i32; 1];
    static mut DAT_0052ad20: [i32; 1];
    static mut DAT_00525b86: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7aa0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a1980(param_1: i32) -> i32 {
    // SAFETY: all addresses are the game's own memory, read exactly where the
    // original reads them; the callee and globals belong to the game.
    unsafe {
        let i_var5 = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988) as u32;
        let i_var1a: i32 = ((i_var5.wrapping_add(0x1f)) as usize as *const i32).read_unaligned();
        let i_var2: u32 =
            ((i_var1a as u32).wrapping_add(0xa7) as usize as *const i32).read_unaligned() as u32;
        let b3 = (i_var5.wrapping_add(3) as usize as *const u8).read() as u32;
        let u_var4 = FUN_004a7aa0((((i_var1a as u32) & 0xFFFF_FF00) | b3) & 0xFFFF_00FF);
        let mut u_var6: u32 = 0;
        let i_var1: u32 = (u_var4 & 0xffff).wrapping_mul(0xef);
        let p07e = (&raw mut DAT_0052b07e).cast::<u8>();
        let p0b3 = (&raw mut DAT_0052b0b3).cast::<u8>();
        let p0b6 = (&raw mut DAT_0052b0b6).cast::<u8>();
        let pw = (&raw mut DAT_00525b86).cast::<u16>();
        let p38 = (&raw mut DAT_0052ad38).cast::<i32>();
        let p20 = (&raw mut DAT_0052ad20).cast::<i32>();
        let mut acc: u32 = (p07e.wrapping_add(i_var1 as usize).read() as u32).wrapping_mul(0x28);
        let cnt = |o: u32| -> u32 { (i_var2.wrapping_add(o) as usize as *const u8).read() as u32 };
        if cnt(0x228) != 0 {
            loop {
                let i_var3 = u_var6.wrapping_add(i_var1);
                u_var6 = u_var6.wrapping_add(1);
                let idx_a = p0b3
                    .wrapping_add(u_var6.wrapping_add(i_var1) as usize)
                    .read() as u32;
                let local_8 = cnt(0x346u32.wrapping_add(idx_a.wrapping_mul(4)));
                let idx_b = p0b6.wrapping_add(i_var3 as usize).read() as u32;
                let t = cnt(0x346u32.wrapping_add(idx_b.wrapping_mul(4)));
                let sel = cnt(0x228u32.wrapping_add(u_var6));
                let w = pw.wrapping_add(sel.wrapping_mul(2) as usize).read() as u32;
                acc = acc.wrapping_add(t.wrapping_add(local_8).wrapping_mul(w));
                if !(u_var6 < cnt(0x228)) {
                    break;
                }
            }
        }
        let a = p38.wrapping_add(cnt(0x23c).wrapping_mul(4) as usize).read();
        let b = p20.wrapping_add(cnt(0x233).wrapping_mul(4) as usize).read();
        (acc as i32).wrapping_add(a).wrapping_add(b)
    }
}
