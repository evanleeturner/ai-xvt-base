// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [i16; 0];
    static mut DAT_009a73b0: u16;
    static mut extraout_ECX: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adc40(param_1: i32) -> i32;
    fn FUN_004ae2a0(param_1: i32, param_2: i32) -> u32;
    fn FUN_004836e0(param_1: u32, param_2: u32, param_3: i32) -> u32;
    fn FUN_00459b10(param_1: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045f1e0(param_1: u32) -> u32 {
    let mut local_44: u16 = 0;
    let mut local_3c: u32 = 0x1000000;
    let u_var10: u32 = param_1 & 0xffff;
    let mut local_42: u16 = 0;
    let mut local_34: [u8; 52] = [0; 52];
    // SAFETY: all globals and callees are the game's own memory and code;
    // the table address is computed and read where the original does.
    unsafe {
        let b_var1: u8 = ((DAT_009a1fe8
            .wrapping_add(3)
            .wrapping_add(u_var10.wrapping_mul(0x23))) as *const u8)
            .read();
        if DAT_009a1fe0 <= u_var10 as i32 {
            return u_var10.wrapping_mul(7) & 0xffff0000;
        }
        let tab32 = (&raw mut DAT_00a00870).cast::<u32>();
        let tab16 = (&raw mut DAT_00a00874).cast::<i16>();
        let count: i32;
        if b_var1 < 0x49 {
            count = tab32.add(b_var1 as usize * 0x65).read() as i32;
        } else {
            count = FUN_004adc40(b_var1 as i32);
        }
        let mut u_var9: u32 = 0;
        if 0 < count {
            loop {
                let b_var2 = ((DAT_009a1fe8
                    .wrapping_add(3)
                    .wrapping_add(u_var10.wrapping_mul(0x23)))
                    as *const u8)
                    .read();
                let u_var6: u32 = b_var2 as u32;
                let s_var3: i16;
                let i_var7: i32;
                if b_var2 < 0x49 {
                    let mut u_var8 = u_var9;
                    let n = tab32.add(u_var6 as usize * 0x65).read();
                    if (n as i32) <= (u_var9 as i32) {
                        u_var8 = n.wrapping_sub(1);
                    }
                    i_var7 = u_var6.wrapping_mul(0x65).wrapping_add(u_var8) as i32;
                    s_var3 = tab16.offset(i_var7 as isize).read();
                } else {
                    let u_var5 = FUN_004ae2a0(u_var6 as i32, u_var9 as i32);
                    s_var3 = u_var5 as i16;
                    i_var7 = extraout_ECX;
                }
                if s_var3 == 1 || s_var3 == 3 {
                    if b_var1 != 0x36 {
                        local_34
                            .as_mut_ptr()
                            .add(local_42 as usize)
                            .write(u_var9 as u8);
                        local_42 = local_42.wrapping_add(1);
                    } else {
                        let u_var6b = FUN_004836e0(
                            ((i_var7 as u32) & 0xffff0000) | (DAT_009a73b0 as u32),
                            param_1,
                            u_var9 as i32,
                        );
                        if u_var6b < local_3c {
                            local_44 = u_var9 as u16;
                            local_3c = u_var6b;
                        }
                    }
                }
                u_var9 = (u_var9 as u16).wrapping_add(1) as u32;
                if !((u_var9 as i32) < count) {
                    break;
                }
            }
        }
        if b_var1 != 0x36 {
            let u_var4 = FUN_00459b10(local_42);
            return local_34.as_ptr().add(u_var4 as usize).read() as u32;
        }
        local_44 as u32
    }
}
