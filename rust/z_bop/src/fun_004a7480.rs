// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a08104: i32;
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [u32; 0];
    static mut DAT_00521f1c: [i32; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a082f4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425cb0() -> u32;
    fn FUN_00425d10() -> u32;
    fn FUN_00427a60(a: i16, b: i16, c: i16, d: i16, e: i32);
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae2a0(a: u32, b: u32) -> i32;
    fn FUN_004a76d0(a: i32, b: u32, c: i32, d: i32) -> u32;
    fn FUN_0042e4d0(a: u32, b: u32, c: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a7480(param_1: u32, param_2: i16) {
    // SAFETY: all globals, callees and the record pointers derived from them
    // are the game's own memory, accessed exactly where the original does.
    unsafe {
        let mut local_10 = [0u8; 16];
        let u_var2_first = FUN_00425cb0() as u16;
        if !(u_var2_first < 0x1fff || param_2 != 0) {
            return;
        }
        let i_var8: i32 = (param_1 & 0xffff)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8) as i32;
        let i_var4: i32 = ((i_var8 as u32).wrapping_add(0x1f) as *const i32).read_unaligned();
        DAT_00a08104 = ((i_var4 as u32).wrapping_add(0xa9) as *const i32).read_unaligned();
        let b_var1: u8 = ((i_var8 as u32).wrapping_add(3) as *const u8).read();
        if ((i_var4 as u32).wrapping_add(0x92) as *const u8).read() != 0 {
            FUN_00427a60(
                ((i_var8 as u32).wrapping_add(0x14) as *const i16).read_unaligned(),
                ((i_var8 as u32).wrapping_add(0x12) as *const i16).read_unaligned(),
                ((i_var8 as u32).wrapping_add(0x10) as *const i16).read_unaligned(),
                0,
                i_var8,
            );
        }
        let u_var6: u32 = b_var1 as u32;
        let tab70 = (&raw mut DAT_00a00870).cast::<u32>();
        let tab74 = (&raw mut DAT_00a00874).cast::<u32>();
        let tab1c = (&raw mut DAT_00521f1c).cast::<i32>();
        let base = u_var6.wrapping_mul(0x65) as usize;

        let (u_stack_18, u_stack_16): (u16, u16);
        if b_var1 < 0x49 {
            let v = tab70.add(base).read();
            u_stack_18 = v as u16;
            u_stack_16 = (v >> 0x10) as u16;
        } else {
            let v = FUN_004adc40(u_var6);
            u_stack_18 = v as u16;
            u_stack_16 = ((v as u32) >> 0x10) as u16;
        }
        let count: i32 = (((u_stack_16 as u32) << 16) | (u_stack_18 as u32)) as i32;

        let mut u_var5: u32 = 0;
        let mut u_var2: u16 = 0;
        if 0 < count {
            loop {
                let i_var4b: i32;
                if b_var1 < 0x49 {
                    let mut u_var7 = u_var5;
                    let d = tab70.add(base).read();
                    if (d as i32) <= (u_var5 as i32) {
                        u_var7 = d.wrapping_sub(1);
                    }
                    i_var4b = tab74.add((u_var7 as usize).wrapping_add(base)).read() as i32;
                } else {
                    i_var4b = FUN_004ae2a0(u_var6, u_var5);
                }
                if i_var4b == 1 {
                    local_10[u_var2 as usize] = u_var5 as u8;
                    u_var2 = u_var2.wrapping_add(1);
                }
                if u_var2 == 0x10 {
                    break;
                }
                u_var5 = (u_var5 as u16).wrapping_add(1) as u32;
                if !((u_var5 as i32) < count) {
                    break;
                }
            }
        }
        if u_var2 == 0 {
            local_10[0] = 0;
            u_var2 = 1;
        }
        let rec = tab1c.add(u_var6.wrapping_mul(0x1a) as usize).read();
        if param_2 != 0 {
            let mut s_var3: i16 = 1;
            u_var2 = DAT_00a082f4;
            loop {
                s_var3 = s_var3.wrapping_sub(1);
                ((u_var2 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8) as *mut u8)
                    .write(0);
                u_var2 = u_var2.wrapping_add(1);
                if s_var3 == 0 {
                    break;
                }
            }
            FUN_004a76d0(i_var8, local_10[0] as u32, rec, 0);
            FUN_0042e4d0(0x15, param_1 & 0xffff, DAT_009ecc34);
            return;
        }
        let u_var5b = FUN_00425cb0();
        let idx = ((u_var5b & 0xffff) % (u_var2 as u32)) as usize;
        let pick = local_10[idx];
        if ((pick as u32)
            .wrapping_add(0x33c)
            .wrapping_add(DAT_00a08104 as u32) as *const u8)
            .read()
            != 0
        {
            let r = FUN_004a76d0(i_var8, pick as u32, rec >> 4, 1);
            if (r as i16) != -1 {
                let u_var6b = r & 0xffff;
                let i_var8b = DAT_009ecc34;
                let u_var5c = FUN_00425d10();
                FUN_0042e4d0((u_var5c & 3).wrapping_add(0x16), u_var6b, i_var8b);
            }
        }
    }
}
