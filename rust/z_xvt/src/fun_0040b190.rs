// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_0085e792: i8;
    static mut DAT_0085e79b: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_006d4988: i32;
    static mut DAT_0073c018: [u8; 0];
    static mut DAT_0073c060: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(param_1: i32, param_2: i32) -> u32;
    fn FUN_004398b0(param_1: i32);
    fn FUN_00490a50(
        param_1: *mut u8,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: u32,
        param_6: i32,
    );
    fn FUN_004784d0() -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b190() {
    // SAFETY: globals and callees are the game's own; the pointers read
    // are the game's own structures, read where the original reads them.
    unsafe {
        if !(DAT_00525550 == 0 && DAT_0070a189 != 0 && DAT_0085e792 != 0 && DAT_0085e79b != 0) {
            return;
        }
        let mut u_var5: u32 = 0x7f;
        if DAT_0085e79b < 10 {
            u_var5 = (DAT_0085e79b as u32).wrapping_mul(0xd);
        }
        let base = (&raw mut DAT_0071b730).cast::<u8>();
        let mut i_var1: i32 = base
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned();
        if i_var1 == -1 {
            if FUN_00439830(0x3f, 0x100) != 0 {
                FUN_004398b0(0x3f);
            }
            if FUN_00439830(0x40, 0x100) != 0 {
                FUN_004398b0(0x40);
            }
            if FUN_00439830(0x41, 0x100) != 0 {
                FUN_004398b0(0x41);
            }
            if FUN_00439830(0x42, 0x100) != 0 {
                FUN_004398b0(0x42);
                return;
            }
        } else {
            let mut u_addr: u32 = (i_var1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988 as u32);
            let i_tmp: i32 = (u_addr as usize as *const i32).read_unaligned();
            u_addr = (i_tmp as u32).wrapping_add(0xa7);
            i_var1 = (u_addr as usize as *const i32).read_unaligned();

            u_addr = (i_var1 as u32).wrapping_add(0x19);
            let mut i_field: i32 = (u_addr as usize as *const i32).read_unaligned();
            if i_field == 0 {
                if FUN_00439830(0x3f, 0x100) != 0 {
                    FUN_004398b0(0x3f);
                }
                if FUN_00439830(0x40, 0x100) != 0 {
                    FUN_004398b0(0x40);
                }
                if FUN_00439830(0x41, 0x100) != 0 {
                    FUN_004398b0(0x41);
                }
            } else {
                let p18 = (&raw mut DAT_0073c018).cast::<u8>();
                if FUN_00439830(0x3f, 0x100) == 0 {
                    FUN_00490a50(p18, 1, 1, 0x7d, u_var5, 0x40);
                }
                if FUN_00439830(0x40, 0x100) == 0 && FUN_00439830(0x41, 0x100) == 0 {
                    let u_var2 = FUN_004784d0();
                    if u_var2 < 0x1000 {
                        let u_var3 = u_var5;
                        let u_var4 = FUN_004784d0() as u32;
                        FUN_00490a50(
                            p18.byte_add(((u_var4 & 1).wrapping_mul(0x18)) as usize),
                            1,
                            1,
                            0x7d,
                            u_var3,
                            0x40,
                        );
                    }
                }
            }
            u_addr = (i_var1 as u32).wrapping_add(0x1d);
            i_field = (u_addr as usize as *const i32).read_unaligned();
            if i_field == 0 {
                if FUN_00439830(0x42, 0x100) != 0 {
                    FUN_004398b0(0x42);
                }
            } else if FUN_00439830(0x42, 0x100) == 0 {
                FUN_00490a50(
                    (&raw mut DAT_0073c060).cast::<u8>(),
                    1,
                    1,
                    0x7d,
                    u_var5 / 2,
                    0x40,
                );
                return;
            }
        }
    }
}
