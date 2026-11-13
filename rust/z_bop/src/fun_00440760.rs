// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a1e0: [u16; 0];
    static mut DAT_009ed240: [u32; 0];
    static mut DAT_00a08ca0: [u16; 0];
    static mut DAT_00a08ca2: [u16; 0];
    static mut DAT_00a08ca4: [u16; 0];
    static mut DAT_00a08ca6: [u16; 0];
    static mut DAT_009d80c4: unsafe extern "C" fn(u32, u16, u16, u16, i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00440760(param_1: i32, param_2: u32) {
    // SAFETY: all globals are the game's own memory; pointers are formed
    // with the same scaled arithmetic the original C performs.
    unsafe {
        let cur = (&raw mut DAT_00a0a1e0)
            .cast::<u16>()
            .offset(param_1 as isize);
        if (cur.read() as u32) != param_2 {
            cur.write(param_2 as u16);
            let i_var1 = param_1.wrapping_mul(0xc);
            let off = (i_var1 as isize).wrapping_mul(2);
            let u_var6 = (&raw mut DAT_00a08ca6)
                .cast::<u8>()
                .wrapping_offset(off)
                .cast::<u16>()
                .read_unaligned();
            let u_var5 = (&raw mut DAT_00a08ca2)
                .cast::<u8>()
                .wrapping_offset(off)
                .cast::<u16>()
                .read_unaligned();
            let u_var4 = (&raw mut DAT_00a08ca0)
                .cast::<u8>()
                .wrapping_offset(off)
                .cast::<u16>()
                .read_unaligned();
            let u_var3 = (&raw mut DAT_00a08ca4)
                .cast::<u8>()
                .wrapping_offset(off)
                .cast::<u16>()
                .read_unaligned();
            let idx = (u_var3 as u32).wrapping_add(param_2);
            let u_var2 = (&raw mut DAT_009ed240)
                .cast::<u32>()
                .wrapping_add(idx as usize)
                .read_unaligned();
            let f = DAT_009d80c4;
            f(u_var2, u_var4, u_var5, u_var6, 0);
        }
    }
}
