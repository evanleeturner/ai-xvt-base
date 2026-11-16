// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759657: [u8; 4];
    static mut DAT_0075965b: u8;
    static mut DAT_007596d3: [u8; 0];
    static mut DAT_00532c68: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df760(param_1: u8, param_2: u8, param_3: u8) -> i32 {
    // SAFETY: the globals are the game's own memory; the mode is a packed
    // int at an odd address, read unaligned, and the palette and table
    // indices stay within the game's 256-entry tables as in the original.
    unsafe {
        let mode: i32 = (&raw mut DAT_00759657)
            .cast::<u8>()
            .cast::<i32>()
            .read_unaligned();

        if mode == 8 {
            let pal = (&raw mut DAT_007596d3).cast::<u8>();
            let tbl = (&raw mut DAT_00532c68).cast::<u32>();
            let mut u_var7: u32 = 0x7fffffff;
            let mut i_var6: i32 = 1;
            let mut i_var1: i32 = 1;
            loop {
                let base = (i_var6.wrapping_mul(4)) as usize;
                let mut i_var4 =
                    (u32::from(pal.add(base).read())).wrapping_sub(u32::from(param_1)) as i32;
                if i_var4 < 0 {
                    i_var4 = 0u32.wrapping_sub(i_var4 as u32) as i32;
                }
                let mut i_var5 =
                    (u32::from(pal.add(base + 1).read())).wrapping_sub(u32::from(param_2)) as i32;
                if i_var5 < 0 {
                    i_var5 = 0u32.wrapping_sub(i_var5 as u32) as i32;
                }
                let mut i_var2 =
                    (u32::from(pal.add(base + 2).read())).wrapping_sub(u32::from(param_3)) as i32;
                if i_var2 < 0 {
                    i_var2 = 0u32.wrapping_sub(i_var2 as u32) as i32;
                }
                let u_var3: u32 = tbl
                    .add(i_var2 as usize)
                    .read()
                    .wrapping_add(tbl.add(i_var5 as usize).read())
                    .wrapping_add(tbl.add(i_var4 as usize).read());
                if u_var3 == 0 {
                    break;
                }
                if u_var3 < u_var7 {
                    i_var1 = i_var6;
                    u_var7 = u_var3;
                }
                i_var6 = i_var6.wrapping_add(1);
                if 0xff < i_var6 {
                    return i_var1;
                }
            }
            return i_var6;
        }
        if mode != 0x10 {
            return mode;
        }
        if DAT_0075965b != 0 {
            return ((u32::from(param_1 >> 3).wrapping_mul(0x20))
                .wrapping_add(u32::from(param_2 >> 3))
                .wrapping_mul(0x20)
                .wrapping_add(u32::from(param_3 >> 3))) as i32;
        }
        u32::from(param_3 >> 3).wrapping_add(
            (u32::from(param_1 >> 3).wrapping_mul(0x40))
                .wrapping_add(u32::from(param_2 >> 2))
                .wrapping_mul(0x20),
        ) as i32
    }
}
