// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d9192: i32;
    static mut DAT_009e99e9: i32;
    static mut DAT_009e9678: i32;
    static mut DAT_009e9a1b: i32;
    static mut DAT_009d8c89: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425c60(a: u32, b: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435530(param_1: i32, param_2: i32, param_3: i32) {
    if param_3 != 2 && param_3 != 3 {
        return;
    }
    // SAFETY: all addresses are the game's own memory, accessed at the same
    // offsets and in the same order as the original.
    unsafe {
        let mut i_var2: i32 = param_2.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        let mut i_var3: i32 =
            ((i_var2.wrapping_add(0x1b)) as u32 as usize as *const i32).read_unaligned();
        if i_var3 == -1 {
            let b = ((i_var2.wrapping_add(0x16)) as u32 as usize as *const u8).read() as u32;
            let base = (&raw mut DAT_009d9192) as usize as u32;
            i_var3 =
                (base.wrapping_add(b.wrapping_mul(0x566)) as usize as *const i32).read_unaligned();
            if i_var3 == -1 {
                return;
            }
        }
        let p = ((i_var2.wrapping_add(0x1f)) as u32 as usize as *const i32).read_unaligned();
        i_var2 = ((p.wrapping_add(0xa9)) as u32 as usize as *const i32).read_unaligned();
        let a = ((i_var2.wrapping_add(0xd7)) as u32 as usize as *const u32).read_unaligned();
        let b = ((i_var2.wrapping_add(0xd3)) as u32 as usize as *const u32).read_unaligned();
        let u_var1 = FUN_00425c60(a, b);
        if (u_var1 as u16) <= 0x7fff {
            return;
        }
        let base = (&raw mut DAT_009d9192) as usize as u32;
        i_var2 = (base.wrapping_add((param_1 as u32).wrapping_mul(0x566)) as usize as *const i32)
            .read_unaligned();
        if i_var2 != -1 {
            if param_3 == 3 {
                let idx = ((&raw mut DAT_009e9678) as usize as u32)
                    .wrapping_add((i_var2 as u32).wrapping_mul(0x5bd))
                    as usize as *const u16;
                let idx = idx.read_unaligned() as u32;
                let addr = ((&raw mut DAT_009e99e9) as usize as u32)
                    .wrapping_add(idx.wrapping_mul(2))
                    .wrapping_add((i_var3 as u32).wrapping_mul(0x5bd))
                    as usize as *mut i16;
                addr.write_unaligned(addr.read_unaligned().wrapping_add(1));
            }
            return;
        }
        if param_3 == 3 {
            let idx = ((&raw mut DAT_009d8c89) as usize as u32)
                .wrapping_add((param_1 as u32).wrapping_mul(0x566)) as usize
                as *const u8;
            let idx = idx.read() as u32;
            let addr = ((&raw mut DAT_009e9a1b) as usize as u32)
                .wrapping_add(idx.wrapping_mul(2))
                .wrapping_add((i_var3 as u32).wrapping_mul(0x5bd)) as usize
                as *mut i16;
            addr.write_unaligned(addr.read_unaligned().wrapping_add(1));
        }
    }
}
