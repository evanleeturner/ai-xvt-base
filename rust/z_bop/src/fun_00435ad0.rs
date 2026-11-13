// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_0051c56e: [u8; 0];
    static mut DAT_0051c5ac: [u8; 0];
    static mut DAT_0051c5aa: [u8; 0];
    static mut DAT_0051a3de: [u16; 0];
    static mut DAT_00521188: [i32; 0];
    static mut DAT_00521170: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004269e0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435ad0(param_1: i32) -> i32 {
    // SAFETY: all addresses are the game's own memory, accessed where the
    // original accesses them.
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x23);
        let d = DAT_009a1fe8;
        let inner = (d.wrapping_add(0x1f).wrapping_add(i_var2) as *const i32).read_unaligned();
        let i_var1 = (inner.wrapping_add(0xa9) as *const i32).read_unaligned();
        let b3 = (d.wrapping_add(i_var2).wrapping_add(3) as *const u8).read();
        let arg = ((((d as u32) >> 8) << 8) | b3 as u32) & 0xffff00ff;
        let u_var4 = FUN_004269e0(arg);
        let i_var3 = ((u_var4 & 0xffff) as i32).wrapping_mul(0x101);
        let t = (&raw mut DAT_0051c56e)
            .cast::<u8>()
            .byte_offset(i_var3 as isize)
            .read();
        let mut i_var5 = (t as i32).wrapping_mul(0x28);
        if (d.wrapping_add(3).wrapping_add(i_var2) as *const i8).read() == 0x36 {
            i_var5 = (t as i32).wrapping_mul(0x50);
        }
        let count = |p: i32| (p.wrapping_add(0x232) as *const u8).read();
        if count(i_var1) != 0 {
            let mut u_var4: u32 = 0;
            loop {
                let u_var6 = u_var4.wrapping_add(1);
                let idx = (i_var3 as u32).wrapping_add(u_var4) as usize;
                let a = (&raw mut DAT_0051c5ac).cast::<u8>().byte_add(idx).read() as u32;
                let b = (&raw mut DAT_0051c5aa).cast::<u8>().byte_add(idx).read() as u32;
                let va = (i_var1
                    .wrapping_add(0x372)
                    .wrapping_add(a.wrapping_mul(4) as i32) as *const u8)
                    .read() as u32;
                let vb = (i_var1
                    .wrapping_add(0x372)
                    .wrapping_add(b.wrapping_mul(4) as i32) as *const u8)
                    .read() as u32;
                let k =
                    (i_var1.wrapping_add(0x233).wrapping_add(u_var4 as i32) as *const u8).read();
                let w = (&raw mut DAT_0051a3de).cast::<u16>().add(k as usize).read() as u32;
                let term = va.wrapping_add(vb).wrapping_mul(w);
                i_var5 = i_var5.wrapping_add(term as i32);
                u_var4 = u_var6;
                if !(u_var6 < count(i_var1) as u32) {
                    break;
                }
            }
        }
        let k1 = (i_var1.wrapping_add(0x246) as *const u8).read();
        let k2 = (i_var1.wrapping_add(0x23d) as *const u8).read();
        let g1 = (&raw mut DAT_00521188)
            .cast::<i32>()
            .add(k1 as usize)
            .read();
        let g2 = (&raw mut DAT_00521170)
            .cast::<i32>()
            .add(k2 as usize)
            .read();
        i_var5.wrapping_add(g1).wrapping_add(g2)
    }
}
