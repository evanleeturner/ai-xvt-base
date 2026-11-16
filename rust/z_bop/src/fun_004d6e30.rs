// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b23: u8;
    static mut DAT_00aa7b9e: [u8; 0];
    static mut DAT_0052bb40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6e30(param_1: u8, param_2: u8, param_3: u8) -> i32 {
    // SAFETY: the globals are the game's own memory; the palette table and
    // distance table are read at the indices the original uses.
    unsafe {
        let local_mode: i32 = (&raw const DAT_00aa7b1f)
            .cast::<u8>()
            .cast::<i32>()
            .read_unaligned();

        if local_mode == 8 {
            let pal = (&raw const DAT_00aa7b9e).cast::<u8>();
            let tbl = (&raw const DAT_0052bb40).cast::<u32>();
            let mut u_var7: u32 = 0x7fffffff;
            let mut i_var6: i32 = 1;
            let mut i_var1: i32 = 1;
            loop {
                let base = (i_var6 as usize).wrapping_mul(4);
                let mut i_var4 = (pal.add(base).read() as i32).wrapping_sub(param_1 as i32);
                if i_var4 < 0 {
                    i_var4 = i_var4.wrapping_neg();
                }
                let mut i_var5 = (pal.add(base + 1).read() as i32).wrapping_sub(param_2 as i32);
                if i_var5 < 0 {
                    i_var5 = i_var5.wrapping_neg();
                }
                let mut i_var2 = (pal.add(base + 2).read() as i32).wrapping_sub(param_3 as i32);
                if i_var2 < 0 {
                    i_var2 = i_var2.wrapping_neg();
                }
                let u_var3 = tbl
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
        if local_mode != 0x10 {
            return local_mode;
        }
        if DAT_00aa7b23 != 0 {
            return (((((param_1 >> 3) as u32).wrapping_mul(0x20))
                .wrapping_add((param_2 >> 3) as u32))
            .wrapping_mul(0x20)
            .wrapping_add((param_3 >> 3) as u32)) as i32;
        }
        ((param_3 >> 3) as u32).wrapping_add(
            (((param_1 >> 3) as u32)
                .wrapping_mul(0x40)
                .wrapping_add((param_2 >> 2) as u32))
            .wrapping_mul(0x20),
        ) as i32
    }
}
