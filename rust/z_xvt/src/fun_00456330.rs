// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005248f8: [u8; 1];
    static mut DAT_0070ad5d: [u8; 1];
    static mut DAT_00524900: [u8; 1];
    static mut DAT_007092d4: u32;
    static mut DAT_0070b252: [u8; 1];
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad8d: [u8; 1];
    static mut DAT_0070ad8c: [u8; 1];
    static mut DAT_006d4a60: [u8; 1];
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4a61: [u8; 1];
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d49a0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00456330() -> u32 {
    // SAFETY: all globals are the game's own memory; indices and offsets
    // are computed exactly as the original does.
    unsafe {
        let mut u_var2: u32 = 0;
        if 0 < DAT_006d49a0 {
            loop {
                let i_var3: u32 = u_var2.wrapping_mul(0x566);
                let a = (&raw mut DAT_005248f8).cast::<u8>();
                let b = (&raw mut DAT_0070ad5d).cast::<u8>();
                let c = (&raw mut DAT_00524900).cast::<u8>();
                let d = (&raw mut DAT_0070b252).cast::<u8>();
                let e = (&raw mut DAT_0070ad8d).cast::<u8>();
                let f = (&raw mut DAT_0070ad8c).cast::<u8>();
                let idx = b.byte_add(i_var3 as usize).read();
                let first = (a.byte_add(idx as usize).read()
                    & c.byte_add(DAT_007092d4 as usize).read())
                    != 0
                    || d.byte_add(i_var3 as usize).cast::<i32>().read_unaligned() != -1;
                if first
                    && (u_var2 as u16) != DAT_006d9d60
                    && e.byte_add(i_var3 as usize).read() != 0
                    && (f.byte_add(i_var3 as usize).read() as u16) == DAT_006d9d60
                {
                    let off = u_var2.wrapping_mul(0x126) as usize;
                    let p60 = (&raw mut DAT_006d4a60).cast::<u8>();
                    let p61 = (&raw mut DAT_006d4a61).cast::<u8>();
                    if p60.byte_add(off).read() == 0 {
                        return 0;
                    }
                    let mut u_var1: u16 = DAT_00732a3c;
                    if p61.byte_add(off).read() != 0 {
                        return 0;
                    }
                    while (u_var1 as i32) < DAT_006d4980 {
                        let base = (u_var1 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32);
                        let p = base as usize as *const u8;
                        if p.byte_add(3).read() != 0 && (p.byte_add(0x16).read() as u32) == u_var2 {
                            return 0;
                        }
                        u_var1 = u_var1.wrapping_add(1);
                    }
                }
                let n = (u_var2 as u16).wrapping_add(1);
                u_var2 = n as u32;
                if !((n as i32) < DAT_006d49a0) {
                    break;
                }
            }
        }
        1
    }
}
