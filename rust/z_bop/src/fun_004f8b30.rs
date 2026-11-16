// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a91d12: [u8; 0];
    static mut DAT_00a91d17: [u8; 0];
    static mut _DAT_00669214: i32;
    static mut DAT_00a92142: [i16; 0];
    static mut DAT_00a9216e: [i16; 0];
    static mut DAT_0052c908: [i32; 0];
    static mut DAT_0052cab0: [u8; 0];
    static mut DAT_0052cab4: [u8; 0];
    static mut DAT_0052cab8: [u8; 0];
    static mut DAT_0052cabc: [u8; 0];
    static mut DAT_00aa5c18: [i32; 0];
    static mut DAT_00aa5bf8: [i32; 0];
    static mut DAT_00aa5c10: [u32; 0];
    static mut DAT_00aa5c28: [i32; 0];
    static mut DAT_0052d014: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f7b10(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: *mut i16,
        param_5: *mut i16,
    );
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d64c0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d67e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16, param_5: i32);
    fn FUN_004d6950(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16);
    fn FUN_004b52c0(param_1: *mut i8, param_2: *mut i32, param_3: i32, param_4: i32, param_5: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f8b30(param_1: *mut i32, param_2: u32, param_3: i16, param_4: i16) {
    let _ = param_2;
    let mut local_1a: i16;
    let mut local_18: i16 = 0;
    let mut local_16: i16 = 0;
    let mut local_10: [i32; 4] = [0; 4];
    let s_var5: i16;
    let s_var6: i16;
    // SAFETY: all globals and callees are the game's own memory and code;
    // indices and offsets are used exactly as the original does.
    unsafe {
        let i_var4 = param_3 as i32;
        let b_var2 = (&raw mut DAT_00a91d12)
            .cast::<u8>()
            .offset(i_var4.wrapping_mul(0x562) as isize)
            .read();
        let i_var3 = (i_var4 as u32)
            .wrapping_mul(0x2b1)
            .wrapping_add(_DAT_00669214 as u32) as i32;
        let kind = (&raw mut DAT_00a91d17)
            .cast::<u8>()
            .offset(i_var4.wrapping_mul(0x562) as isize)
            .read();
        let s_var7: i16 = match kind {
            0 => 0,
            1 | 4 => 8,
            2 => 0x18,
            3 => 0x10,
            5 => 0x20,
            _ => 0,
        };
        let a = (&raw mut DAT_00a92142)
            .cast::<i16>()
            .offset(i_var3 as isize)
            .read_unaligned();
        let b = (&raw mut DAT_00a9216e)
            .cast::<i16>()
            .offset(i_var3 as isize)
            .read_unaligned();
        FUN_004f7b10(
            param_1,
            a as i32,
            b as i32,
            &raw mut local_16,
            &raw mut local_18,
        );
        let i_var8 = (&raw mut DAT_0052c908)
            .cast::<i32>()
            .offset(b_var2 as isize)
            .read_unaligned()
            .wrapping_mul(0x10);
        let base = (&raw mut DAT_0052cab0).cast::<u8>();
        let rd = |p: *mut u8| -> i32 {
            p.byte_offset(i_var8 as isize)
                .cast::<i32>()
                .read_unaligned()
        };
        let i_var3 = rd((&raw mut DAT_0052cab8).cast::<u8>())
            .wrapping_sub(rd(base))
            .wrapping_add(1);
        let i_var4 = rd((&raw mut DAT_0052cabc).cast::<u8>())
            .wrapping_sub(rd((&raw mut DAT_0052cab4).cast::<u8>()))
            .wrapping_add(1);
        local_16 = (local_16 as i32).wrapping_sub((i_var3 >> 1) as i16 as i32) as i16;
        local_18 = (local_18 as i32).wrapping_sub((i_var4 >> 1) as i16 as i32) as i16;
        FUN_004d6460(
            local_10.as_mut_ptr(),
            local_16 as i32,
            local_18 as i32,
            i_var3.wrapping_add(local_16 as i32).wrapping_sub(1),
            i_var4.wrapping_add(local_18 as i32).wrapping_sub(1),
        );
        if param_4 < 0xc {
            let mut s5: i16;
            if param_4 < 4 {
                s_var6 = param_4.wrapping_add(1);
                local_1a = 0x10;
                s5 = (s_var7 as i32)
                    .wrapping_add((param_4 as i32).wrapping_mul(-2))
                    .wrapping_add(7) as i16;
            } else {
                s5 = s_var7.wrapping_add(1);
                if param_4 < 8 {
                    local_1a = ((0xb - param_4 as i32).wrapping_mul(2)) as i16;
                    s_var6 = 4;
                } else {
                    local_1a = ((0xb - param_4 as i32).wrapping_mul(2)) as i16;
                    s_var6 = (0xc - param_4 as i32) as i16;
                }
            }
            if 7 < param_4 {
                let mut local_14 = param_4 as i32;
                FUN_004d64c0(local_10.as_mut_ptr(), 0xb - local_14, 0xb - local_14);
                let v1 = (&raw mut DAT_00aa5c18)
                    .cast::<i32>()
                    .offset(s_var7 as isize)
                    .read_unaligned();
                FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, v1 as i16, 1);
                let v2 = (&raw mut DAT_00aa5bf8)
                    .cast::<i32>()
                    .offset((s_var7 as i32).wrapping_add(local_14) as isize)
                    .read_unaligned();
                FUN_004d6950(local_10.as_mut_ptr(), 0, 0, v2 as i16);
                let _ = &mut local_14;
            }
            if 0 < s_var6 {
                let mut local_14 = s_var6 as i32;
                loop {
                    let i3 = s5 as i32;
                    let i4 = local_1a as i32;
                    s5 = s5.wrapping_add(2);
                    let p1 = (&raw mut DAT_00aa5c10).cast::<u32>().offset(i3 as isize);
                    let ptr = base.byte_offset(i_var8 as isize).cast::<i32>();
                    let s = (&raw mut DAT_0052d014).cast::<i8>();
                    let l16 = local_16 as i32;
                    let l18 = local_18 as i32;
                    FUN_004b52c0(
                        s,
                        ptr,
                        l16.wrapping_sub(i4),
                        l18.wrapping_sub(i4),
                        p1.read_unaligned(),
                    );
                    FUN_004b52c0(
                        s,
                        ptr,
                        l16.wrapping_add(i4),
                        l18.wrapping_sub(i4),
                        p1.read_unaligned(),
                    );
                    FUN_004b52c0(
                        s,
                        ptr,
                        l16.wrapping_sub(i4),
                        l18.wrapping_add(i4),
                        p1.read_unaligned(),
                    );
                    FUN_004b52c0(
                        s,
                        ptr,
                        l16.wrapping_add(i4),
                        l18.wrapping_add(i4),
                        p1.read_unaligned(),
                    );
                    local_1a = local_1a.wrapping_sub(2);
                    local_14 = (local_14 as i16).wrapping_sub(1) as i32;
                    if (local_14 as i16) == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            FUN_004d64c0(local_10.as_mut_ptr(), -2, -2);
            let v1 = (&raw mut DAT_00aa5c18)
                .cast::<i32>()
                .offset(s_var7 as isize)
                .read_unaligned();
            FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, v1 as i16, 1);
            let v2 = (&raw mut DAT_00aa5c28)
                .cast::<i32>()
                .offset(s_var7 as isize)
                .read_unaligned();
            FUN_004d6950(local_10.as_mut_ptr(), 0, 0, v2 as i16);
        }
    }
}
