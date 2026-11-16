// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb7375: i8;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00521080: [u8; 0];
    static mut DAT_009d7188: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_00a0a7f0: [i32; 0];
    static mut DAT_009ed228: [u8; 0];
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425d10() -> u16;
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> i32;
    fn FUN_00430610(a: u32, b: i8, c: i8, d: i8, e: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00430200(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed and read exactly where the original does.
    unsafe {
        let mut u_stack_8: u32 = 0;

        if DAT_00bb7375 == 0 {
            return 0;
        }
        let base = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
        let tmp_int = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .add(base)
            .cast::<i32>()
            .read_unaligned();
        if tmp_int == -1 {
            return 0;
        }
        if param_4 != 0xffff {
            let u_var1 = FUN_00425d10();
            if param_4 <= u_var1 {
                return 0;
            }
        }
        let mut s_var4: i16 = -1;
        if param_1 == 1 {
            if param_3 == -1 || DAT_009a1fe0 <= param_3 {
                return 0;
            }
            let entry_addr =
                (DAT_009a1fe8 as u32).wrapping_add((param_3 as u32).wrapping_mul(0x23));
            let entry = entry_addr as *const u8;
            let tmp_ushort = (&raw mut DAT_009e967c)
                .cast::<u8>()
                .add(base)
                .cast::<u16>()
                .read_unaligned();
            let idx1 = (entry.byte_add(0x16).read() as u32)
                .wrapping_add((tmp_ushort as u32).wrapping_mul(0x30));
            let b1 = (&raw mut DAT_009d7188)
                .cast::<u8>()
                .add(idx1 as usize)
                .read();
            u_stack_8 = (&raw mut DAT_00521080).cast::<u8>().add(b1 as usize).read() as u32;
            if u_stack_8 == 0xff {
                return 0;
            }
            s_var4 = entry.cast::<i16>().read_unaligned();
            let i_var2 = FUN_00435a10(
                (&raw mut DAT_009ed228).cast::<u8>().add(3).read(),
                DAT_009ed22c,
                DAT_009ed22d,
            );
            if param_2 != 0x1b && param_2 != 0x1c {
                let slot = (&raw mut DAT_00a0a7f0)
                    .cast::<i32>()
                    .offset(param_3 as isize);
                let old = slot.read();
                if old != 0 && (i_var2 as u32).wrapping_sub(old as u32) < 0xb {
                    return 0;
                }
                slot.write(i_var2);
            }
            if u_stack_8 == 4 || u_stack_8 == 5 {
                let tmp_ushort = (&raw mut DAT_009e967c)
                    .cast::<u8>()
                    .add(base)
                    .cast::<u16>()
                    .read_unaligned();
                let a = (DAT_009a1fe8 as u32)
                    .wrapping_add(0x16)
                    .wrapping_add((param_3 as u32).wrapping_mul(0x23))
                    as *const u8;
                let idx2 = (a.read() as u32).wrapping_mul(0x566);
                let v = (&raw mut DAT_009d8c88)
                    .cast::<u8>()
                    .add(idx2 as usize)
                    .read();
                if (v as u16) != tmp_ushort {
                    u_stack_8 = 0xb_u32.wrapping_sub((u_stack_8 == 4) as u32);
                }
            }
        }
        let c_var5: i8;
        if param_1 == 1 {
            FUN_00430610(u_stack_8.wrapping_add(0x2b8), 2, 1, 0, s_var4);
            c_var5 = 1;
        } else {
            if param_1 < 2 {
                return 1;
            }
            if 6 < param_1 {
                return 1;
            }
            c_var5 = 0;
        }
        FUN_00430610(
            (param_2 as u32).wrapping_add(0x2b8),
            2,
            param_1 as i8,
            c_var5,
            s_var4,
        );
        1
    }
}
