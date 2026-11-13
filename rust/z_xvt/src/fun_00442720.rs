// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732910: u32;
    static mut DAT_0071b749: i8;
    static mut DAT_0071ecf4: u32;
    static mut DAT_00732a2c: i32;
    static mut DAT_0071bb1b: i8;
    static mut DAT_00732a28: i32;
    static mut DAT_0071bb1f: i8;
    static mut DAT_00732a24: i32;
    static mut DAT_0071bb23: i8;
    static mut DAT_00732f3c: u32;
    static mut DAT_0052552c: u32;
    static mut DAT_006da538: u32;
    static mut DAT_0071b743: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_0040ca60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_0040ca00(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_0040ca30(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_0040ca90(param_1: u32, param_2: u32) -> i32;
    fn FUN_0040cad0(param_1: u32, param_2: u32) -> i32;
    fn FUN_004428e0(param_1: u32) -> u32;
    fn FUN_004354e0(param_1: i32, param_2: i32, param_3: u32, param_4: u32, param_5: u8);
    fn FUN_00477c90(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442720(param_1: u16, param_2: u8) {
    // SAFETY: the globals and callees are the game's own; the indexed
    // addresses are the game's per-entry records read where the original does.
    unsafe {
        if param_1 == 0xffff || DAT_00732910 != 0 {
            return;
        }
        let off = DAT_0071ecf4.wrapping_mul(0x5bd) as usize;
        if (&raw mut DAT_0071b749).cast::<u8>().add(off).read() == 0 {
            return;
        }
        FUN_00421c70(param_1 as u32, 0);
        let local_bb1b: i32 = (&raw mut DAT_0071bb1b)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let local_bb1f: i32 = (&raw mut DAT_0071bb1f)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let local_bb23: i32 = (&raw mut DAT_0071bb23)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let mut i_var6 = DAT_00732a2c.wrapping_sub(local_bb1b);
        let i_var7 = DAT_00732a28.wrapping_sub(local_bb1f);
        let mut i_var4 = DAT_00732a24.wrapping_sub(local_bb23);
        let u_var1 = FUN_0040ca60(i_var6, i_var7, i_var4);
        if 0 < (u_var1 as i32) {
            let u_var2 = FUN_0040ca00(i_var6, i_var7, i_var4);
            let u_var3 = FUN_0040ca30(i_var6, i_var7, i_var4);
            i_var4 = FUN_0040ca90(u_var2, u_var1);
            i_var6 = FUN_0040cad0(u_var3, u_var1);
            let u_var2 = FUN_004428e0(param_1 as u32);
            let mut u_var2 = DAT_00732f3c.wrapping_mul(u_var2) / u_var1;
            let u_var3: u32 = if DAT_0052552c == 0x13 { 4 } else { 8 };
            let u_var5 = (DAT_006da538 >> 1).wrapping_add(DAT_006da538 >> 2);
            if (u_var2 as i32) < (u_var3 as i32) {
                u_var2 = u_var3;
            }
            if (u_var5 as i32) < (u_var2 as i32) {
                u_var2 = u_var5;
            }
            u_var2 = u_var2.wrapping_add(4);
            let half = (u_var2 as i32) / 2;
            if (&raw mut DAT_0071b743).cast::<u8>().add(off).read() != 0 {
                FUN_004354e0(
                    i_var4.wrapping_sub(half),
                    i_var6.wrapping_sub(half),
                    u_var2,
                    u_var2,
                    param_2,
                );
                return;
            }
            FUN_00477c90(
                i_var4.wrapping_sub(half),
                i_var6.wrapping_sub(half),
                u_var2,
                u_var2,
                param_2 as u32,
                u_var1,
            );
        }
    }
}
