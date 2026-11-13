// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006559b0: [u16; 8];
    static mut DAT_0052f0e0: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043fdf0() -> u32 {
    let mut u_var2: u32 = 0;
    // SAFETY: the globals are the game's own memory; the index stays below 8
    // while the loop runs, as in the original.
    unsafe {
        loop {
            let v = (&raw const DAT_006559b0)
                .cast::<u16>()
                .add(u_var2 as usize)
                .read();
            if !(v == 0 || DAT_0052f0e0 == (u_var2 as i16)) {
                break;
            }
            let u_var1 = (u_var2 as i16).wrapping_add(1) as u16;
            u_var2 = u_var1 as u32;
            if 7 < u_var1 {
                return 0xffff;
            }
        }
    }
    u_var2
}
