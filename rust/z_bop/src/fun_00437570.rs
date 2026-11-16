// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8c87: [u8; 1];
    static mut DAT_009ec478: *mut i8;
    static mut DAT_005235b8: [i8; 1];
    static mut DAT_00523588: [i8; 1];
    static mut DAT_005235a0: [i8; 1];
    static mut DAT_00521240: [u8; 1];
    static mut DAT_005212f8: [u8; 1];
    static mut DAT_005212b0: [u8; 1];
    static mut DAT_00521440: [u8; 1];
    static mut DAT_005214f8: [u8; 1];
    static mut DAT_005214b0: [u8; 1];
    static mut DAT_00521340: [u8; 1];
    static mut DAT_005213f8: [u8; 1];
    static mut DAT_005213b0: [u8; 1];
    static mut DAT_009a8e38: i32;
    static mut DAT_009a6fe0: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_009d8c00: i16;
    static mut DAT_00a07ce4: i16;
    static mut DAT_009ecc4c: i32;
    static mut DAT_009d80c4: unsafe extern "C" fn();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427570(param_1: u32, param_2: u32) -> i16;
    fn FUN_00427590(param_1: u32, param_2: u32) -> i16;
    fn FUN_004aba50();
    fn FUN_0040ece0(param_1: *mut u8, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00437570(param_1: i32, param_2: u32, param_3: u32, param_4: u32) {
    // SAFETY: all globals, tables and callees are the game's own memory; the
    // entry pointers are read exactly where the original reads them.
    unsafe {
        let p_entry = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32) as *const u8;
        let mut local_8: u32 = p_entry.byte_add(3).read() as u32;
        let p_extra: u32 = p_entry.byte_add(0x1f).cast::<u32>().read_unaligned();
        let u_var1: u8 = if p_extra == 0 {
            let idx = (p_entry.byte_add(0x16).read() as u32).wrapping_mul(0x566);
            (&raw mut DAT_009d8c87)
                .cast::<u8>()
                .byte_add(idx as usize)
                .read()
        } else {
            (p_extra.wrapping_add(0x88) as *const u8).read()
        };
        let mut local_4: u32 = 0;
        let mut i_var4: i32 = match u_var1 {
            0 => 0,
            1 | 4 => 2,
            2 => 3,
            3 => {
                local_4 = 1;
                2
            }
            _ => 1,
        };
        let s_var2 = FUN_00427570(param_2, param_4);
        let s_var3 = FUN_00427590(param_3, param_4);
        let cur = DAT_009ec478 as *const i8;
        let u_var5: u32;
        let u_var6: u32;
        if cur == (&raw const DAT_005235b8).cast::<i8>() {
            let mut v6: u32 = 0x13;
            if local_8 < 0x6a {
                v6 = (&raw mut DAT_00521240)
                    .cast::<u8>()
                    .add(local_8 as usize)
                    .read() as u32;
            }
            local_8 = (&raw mut DAT_005212f8).cast::<u8>().add(v6 as usize).read() as u32;
            u_var5 = (&raw mut DAT_005212b0).cast::<u8>().add(v6 as usize).read() as u32;
            u_var6 = v6;
        } else if cur == (&raw const DAT_00523588).cast::<i8>() {
            let mut v6: u32 = 0x13;
            if local_8 < 0x6a {
                v6 = (&raw mut DAT_00521440)
                    .cast::<u8>()
                    .add(local_8 as usize)
                    .read() as u32;
            }
            local_8 = (&raw mut DAT_005214f8).cast::<u8>().add(v6 as usize).read() as u32;
            u_var5 = (&raw mut DAT_005214b0).cast::<u8>().add(v6 as usize).read() as u32;
            u_var6 = v6;
        } else {
            let mut v5 = local_4;
            let mut v6 = local_4;
            if cur == (&raw const DAT_005235a0).cast::<i8>() {
                v6 = 0x13;
                if local_8 < 0x6a {
                    v6 = (&raw mut DAT_00521340)
                        .cast::<u8>()
                        .add(local_8 as usize)
                        .read() as u32;
                }
                local_8 = (&raw mut DAT_005213f8).cast::<u8>().add(v6 as usize).read() as u32;
                v5 = (&raw mut DAT_005213b0).cast::<u8>().add(v6 as usize).read() as u32;
            }
            u_var5 = v5;
            u_var6 = v6;
        }
        i_var4 = i_var4.wrapping_mul(DAT_009a8e38);
        let u_var8: u16 = ((s_var2 as i32).wrapping_add(DAT_009a6fe0 as i32) as u32)
            .wrapping_sub((u_var5 / 2) as i16 as i32 as u32) as u16;
        let u_var7: u16 = ((DAT_00a0813c as i32).wrapping_add(s_var3 as i32) as u32)
            .wrapping_sub((local_8 / 2) as i16 as i32 as u32) as u16;
        if (u_var8 as i16) >= DAT_009a6fe0
            && ((u_var5 as u16).wrapping_add(u_var8) as i16) < DAT_009d8c00
            && (u_var7 as i16) >= DAT_00a0813c
            && (u_var7.wrapping_add(local_8 as u16) as i16) < DAT_00a07ce4
        {
            FUN_004aba50();
            if local_4 == 0 {
                (DAT_009d80c4)();
            } else {
                let adj = (i_var4 as u32).wrapping_add(((i_var4 >> 31) as u32) & 3) as i32 >> 2;
                let idx = u_var6.wrapping_add(adj as u32).wrapping_mul(4);
                let addr = (DAT_009ecc4c as u32).wrapping_add(idx) as *const *mut u8;
                let p = addr.read_unaligned();
                FUN_0040ece0(p, u_var8 as u32, u_var7 as u32, 0, 0);
            }
            FUN_004abe50();
        }
    }
}
