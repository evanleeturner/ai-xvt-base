// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_0085e793: i8;
    static mut DAT_0085e79c: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b741: [i8; 0];
    static mut DAT_0071b743: [i8; 0];
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00536928: u8;
    static mut DAT_0073ba30: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(a: i32, b: i32) -> u32;
    fn FUN_004398b0(a: i32);
    fn FUN_004396b0(a: i32, b: i32, c: i32);
    fn FUN_004783e0(a: u16, b: u32) -> u32;
    fn FUN_00478320(a: u32, b: u16) -> u32;
    fn FUN_00490a50(a: *mut i8, b: i32, c: i32, d: i32, e: u32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040ae30() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers read from game structures are dereferenced where the original does.
    unsafe {
        if !(DAT_00525550 == 0 && DAT_0070a189 != 0 && DAT_0085e793 != 0 && DAT_0085e79c != 0) {
            return;
        }
        let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let mut i_stack_4: i32 = 0;
        let mut i_var7: i32 = -1;
        let mut i_var8: i32 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .add(idx)
            .cast::<i32>()
            .read_unaligned();
        let mut u_var3: u8 = i_stack_4 as u8;
        if i_var8 != -1 {
            let off = (i_var8 as u32).wrapping_mul(0x23).wrapping_add(3) as usize;
            u_var3 = DAT_006d4988.add(off).read();
            match u_var3 {
                1 | 4 | 0xe | 0xf => {
                    i_var7 = 0x43;
                    i_stack_4 = 11000;
                }
                2 => {
                    i_var7 = 0x44;
                    i_stack_4 = 0x157c;
                }
                3 | 0xd => {
                    i_var7 = 0x45;
                    i_stack_4 = 0x157c;
                }
                5 | 6 | 7 | 8 | 9 => {
                    i_var7 = 0x46;
                    i_stack_4 = 0x157c;
                }
                0xc | 0x10 => {
                    i_var7 = 0x47;
                    i_stack_4 = 11000;
                }
                _ => {}
            }
        }
        if i_var7 == -1 {
            if (&raw mut DAT_0071b743).cast::<i8>().add(idx).read() != 0 {
                i_var8 = -1;
                match DAT_00536928 {
                    1 | 4 | 0xe | 0xf => i_var8 = 0x43,
                    2 => i_var8 = 0x44,
                    3 | 0xd => i_var8 = 0x45,
                    5 | 6 | 7 | 8 | 9 => i_var8 = 0x46,
                    0xc | 0x10 => i_var8 = 0x47,
                    _ => {}
                }
                if FUN_00439830(i_var8, 0x100) != 0 {
                    FUN_004398b0(i_var8);
                }
                if FUN_00439830(0x4e, 0x100) != 0 {
                    FUN_004398b0(0x4e);
                }
                if FUN_00439830(0x4f, 0x100) != 0 {
                    FUN_004398b0(0x4f);
                }
            }
        } else {
            DAT_00536928 = u_var3;
            if (&raw mut DAT_0071b741).cast::<i8>().add(idx).read() != 1 {
                let off = (i_var8 as u32).wrapping_mul(0x23).wrapping_add(0x1f) as usize;
                let i_ptr: i32 = DAT_006d4988.add(off).cast::<i32>().read_unaligned();
                let i_var1: i32 = ((i_ptr as u32 as usize) as *const u8)
                    .byte_add(0xa7)
                    .cast::<i32>()
                    .read_unaligned();
                let base = (i_var1 as u32 as usize) as *const u8;
                if (base.byte_add(0x1df).read() & 0x40) != 0 {
                    let u_var4 = DAT_0085e79c as u16;
                    let u_var6 = ((i_var8 as u32) >> 16) as u16;
                    let us_val: u16 = base.byte_add(0xbd).cast::<u16>().read_unaligned();
                    let u_var5: u32 = if u_var4 < 10 {
                        ((u_var6 as u32) << 16) | (u_var4.wrapping_mul(0xd) as u32)
                    } else {
                        ((u_var6 as u32) << 16) | 0x7f
                    };
                    let u_var2 = FUN_004783e0(us_val, 0xffff);
                    let i_var8b: i32 = (i_stack_4 as u32)
                        .wrapping_add(((u_var2 & 0xffff) / 0x28f).wrapping_mul(0x37))
                        as i32;
                    let us_val2: u16 = base.byte_add(0xbd).cast::<u16>().read_unaligned();
                    let u_var5 = FUN_00478320(u_var5, us_val2);
                    let u_var2 = FUN_00439830(i_var7, 0x100);
                    if u_var2 == 0 {
                        FUN_004396b0(i_var7, 0x777, i_var8b);
                        FUN_00490a50(
                            (&raw mut DAT_0073ba30)
                                .cast::<i8>()
                                .offset(i_var7.wrapping_mul(6) as isize),
                            1,
                            1,
                            0x7d,
                            (u_var5 >> 1) & 0x7fff,
                            0x40,
                        );
                        return;
                    }
                    FUN_004396b0(i_var7, 0x777, i_var8b);
                    return;
                }
            }
            if FUN_00439830(i_var7, 0x100) != 0 {
                FUN_004398b0(i_var7);
            }
            if FUN_00439830(0x4e, 0x100) != 0 {
                FUN_004398b0(0x4e);
            }
            if FUN_00439830(0x4f, 0x100) != 0 {
                FUN_004398b0(0x4f);
                return;
            }
        }
    }
}
