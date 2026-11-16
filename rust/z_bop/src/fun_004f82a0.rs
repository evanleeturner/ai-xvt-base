// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006696e4: i16;
    static mut DAT_006696f4: i16;
    static mut DAT_00669704: i16;
    static mut DAT_00a92142: i16;
    static mut DAT_00a9216e: i16;
    static mut DAT_00669716: i16;
    static mut DAT_00669726: i16;
    static mut DAT_00669736: i16;
    static mut DAT_00669746: i16;
    static mut DAT_00669756: i16;
    static mut DAT_00669766: i16;
    static mut DAT_00669582: i32;
    static mut _DAT_00669214: i32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a921c6: i16;
    static mut DAT_00a91d17: u8;
    static mut DAT_00a91d12: u8;
    static mut DAT_00a91d18: u8;
    static mut DAT_00a91d24: u8;
    static mut DAT_00b6a5a6: u32;
    static mut DAT_0052c908: i32;
    static mut DAT_0052cab0: i8;
    static mut DAT_0052cab4: i8;
    static mut DAT_0052cab8: i8;
    static mut DAT_0052cabc: i8;
    static mut DAT_00b69d20: i8;
    static mut s_mapicon_d_0052d008: i8;
    static mut DAT_0052d004: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004f7b10(
        param_1: *mut i32,
        param_2: i16,
        param_3: i16,
        param_4: *mut i16,
        param_5: *mut i16,
    );
    fn FUN_004f8b30(param_1: *mut i32, param_2: u32, param_3: i16, param_4: i16);
    fn FUN_004f8910(
        param_1: *mut i8,
        param_2: i32,
        param_3: i16,
        param_4: i16,
        param_5: i16,
        param_6: i16,
    );
    #[link_name = "sprintf"]
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004b50b0(param_1: *mut i8, param_2: *mut i32, param_3: i32, param_4: i32);
    fn FUN_004db4e0(
        param_1: i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f82a0(param_1: *mut i32, param_2: u32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let mut s_stack_40: i16 = 0;
        let mut local_42: i16 = 0;
        let mut local_44: i16 = 0;
        let mut local_38: [i32; 4] = [0; 4];
        let mut local_28: [i8; 40] = [0; 40];

        let mut s_var10: i16 = 0;
        FUN_004d6480(local_38.as_mut_ptr(), param_1);
        loop {
            if (&raw mut DAT_006696e4).offset(s_var10 as isize).read() != 0 {
                let s_var3 = (&raw mut DAT_006696f4).offset(s_var10 as isize).read();
                let idx = (s_var3 as i32).wrapping_mul(0x2b1) as isize;
                FUN_004f7b10(
                    local_38.as_mut_ptr(),
                    (&raw mut DAT_00a92142).offset(idx).read(),
                    (&raw mut DAT_00a9216e).offset(idx).read(),
                    &raw mut s_stack_40,
                    &raw mut local_42,
                );
                FUN_004f8b30(
                    param_1,
                    param_2,
                    s_var3,
                    (&raw mut DAT_00669704).offset(s_var10 as isize).read(),
                );
            }
            s_var10 = s_var10.wrapping_add(1);
            if s_var10 >= 8 {
                break;
            }
        }

        let mut local_3e: i16 = 0;
        loop {
            let i_var4 = local_3e as i32 as isize;
            if (&raw mut DAT_00669716).offset(i_var4).read() != 0 {
                let mut s10 = (&raw mut DAT_00669726).offset(i_var4).read();
                FUN_004f7b10(
                    local_38.as_mut_ptr(),
                    (&raw mut DAT_00669736).offset(i_var4).read(),
                    (&raw mut DAT_00669746).offset(i_var4).read(),
                    &raw mut s_stack_40,
                    &raw mut local_42,
                );
                let src = (&raw mut DAT_00669582).offset(s10 as isize).read() as usize as *const i8;
                let mut n: usize = 0;
                while src.add(n).read() != 0 {
                    n += 1;
                }
                n += 1;
                let dst = local_28.as_mut_ptr();
                let mut k: usize = 0;
                while k < n {
                    dst.add(k).write(src.add(k).read());
                    k += 1;
                }
                s10 = 0;
                let mut c_var2 = dst.read();
                while c_var2 != 0 {
                    let p = dst.offset(s10 as isize);
                    if p.read() == b'[' as i8 {
                        p.write(2);
                    }
                    if p.read() == b']' as i8 {
                        p.write(1);
                    }
                    s10 = s10.wrapping_add(1);
                    c_var2 = dst.offset(s10 as isize).read();
                }
                FUN_004f8910(
                    dst,
                    1,
                    s_stack_40,
                    local_42,
                    (&raw mut DAT_00669756).offset(i_var4).read(),
                    (&raw mut DAT_00669766).offset(i_var4).read(),
                );
            }
            local_3e = local_3e.wrapping_add(1);
            if local_3e > 7 {
                break;
            }
        }

        let mut local_3c: i32 = 1;
        local_3e = 0;
        if 0 < DAT_00aa5aac {
            loop {
                let i_var7 = local_3e as i32;
                let i_var1 = (i_var7 as u32).wrapping_mul(0x562) as i32;
                let i_var4 = (i_var7 as u32)
                    .wrapping_mul(0x2b1)
                    .wrapping_add(_DAT_00669214 as u32) as i32;
                if (&raw mut DAT_00a921c6).offset(i_var4 as isize).read() != 0 {
                    match (&raw mut DAT_00a91d17).offset(i_var1 as isize).read() {
                        0 => local_44 = 0,
                        1 => local_44 = 1,
                        2 => local_44 = 2,
                        3 => local_44 = 3,
                        4 => local_44 = 1,
                        5 => local_44 = 4,
                        _ => {}
                    }
                    let t = (&raw mut DAT_00a91d12).offset(i_var1 as isize).read() as u16 as i16;
                    let i_var8 = ((&raw mut DAT_0052c908).offset(t as isize).read() as u32)
                        .wrapping_mul(0x10) as i32;
                    let o = i_var8 as isize;
                    let a = (&raw mut DAT_0052cab8)
                        .byte_offset(o)
                        .cast::<i32>()
                        .read_unaligned();
                    let b = (&raw mut DAT_0052cab0)
                        .byte_offset(o)
                        .cast::<i32>()
                        .read_unaligned();
                    let i_var12 = (a as u32).wrapping_sub(b as u32).wrapping_add(1) as i32;
                    let c = (&raw mut DAT_0052cabc)
                        .byte_offset(o)
                        .cast::<i32>()
                        .read_unaligned();
                    let d = (&raw mut DAT_0052cab4)
                        .byte_offset(o)
                        .cast::<i32>()
                        .read_unaligned();
                    let i_var7b = (c as u32).wrapping_sub(d as u32).wrapping_add(1) as i32;
                    FUN_004f7b10(
                        local_38.as_mut_ptr(),
                        (&raw mut DAT_00a92142).offset(i_var4 as isize).read(),
                        (&raw mut DAT_00a9216e).offset(i_var4 as isize).read(),
                        &raw mut s_stack_40,
                        &raw mut local_42,
                    );
                    s_stack_40 = s_stack_40.wrapping_sub((i_var12 >> 1) as i16);
                    local_42 = local_42.wrapping_sub((i_var7b >> 1) as i16);
                    _sprintf(
                        &raw mut DAT_00b69d20,
                        (&raw mut s_mapicon_d_0052d008) as *const i8,
                        local_44 as i32,
                    );
                    FUN_004b50b0(
                        &raw mut DAT_00b69d20,
                        (&raw mut DAT_0052cab0).byte_offset(o).cast::<i32>(),
                        s_stack_40 as i32,
                        local_42 as i32,
                    );
                    let local_cmp = (&raw mut DAT_00b6a5a6).read_unaligned();
                    if (&raw mut DAT_00a91d24).offset(i_var1 as isize).read() != 0
                        && (&raw mut DAT_00a91d18).offset(i_var1 as isize).read() as u32
                            == local_cmp
                    {
                        s_stack_40 = s_stack_40.wrapping_add(i_var12 as i16);
                        local_42 = local_42.wrapping_add(i_var7b as i16);
                        _sprintf(
                            &raw mut DAT_00b69d20,
                            (&raw mut DAT_0052d004) as *const i8,
                            local_3c,
                        );
                        FUN_004db4e0(
                            10,
                            &raw mut DAT_00b69d20,
                            s_stack_40 as i32,
                            local_42 as i32,
                            0xffff,
                        );
                        local_3c = local_3c.wrapping_add(1);
                    }
                }
                local_3e = local_3e.wrapping_add(1);
                if local_3e >= DAT_00aa5aac {
                    break;
                }
            }
        }
    }
}
