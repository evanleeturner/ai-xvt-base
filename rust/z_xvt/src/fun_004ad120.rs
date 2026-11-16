// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: u32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0052b0b4: [u8; 0];
    static mut DAT_0052b0b5: [u8; 0];
    static mut DAT_0052b0b6: [u8; 0];
    static mut DAT_0052b0b7: [u8; 0];
    static mut DAT_0052b0b8: [u8; 0];
    static mut DAT_0052b0b9: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7aa0(param_1: u32) -> u32;
    fn FUN_004ad3f0(param_1: u32, param_2: u16, param_3: u32);
}

/// SAFETY: caller must be in the game's context; reads the game's own memory.
unsafe fn cur_arg() -> u32 {
    unsafe {
        let tmp = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        let b = ((tmp
            .wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_006d4988)) as usize as *const u8)
            .read();
        ((DAT_006d4988 & 0xffffff00) | b as u32) & 0xffff00ff
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ad120() {
    // SAFETY: all globals, tables and pointers belong to the game's own
    // memory and are read exactly as the original does.
    unsafe {
        let tmp = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        let i_var4 = tmp.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let i_var2 = ((i_var4.wrapping_add(0x1f)) as usize as *const u32).read_unaligned();
        let flags_ptr = ((i_var2.wrapping_add(0xa7)) as usize as *const u32).read_unaligned();
        if (((flags_ptr.wrapping_add(0x1db)) as usize as *const u8).read() & 8) != 0 {
            let b3 = ((i_var4.wrapping_add(3)) as usize as *const u8).read();
            let u_var3 = FUN_004a7aa0(((i_var2 & 0xffffff00) | b3 as u32) & 0xffff00ff);
            let b_var1 = (&raw mut DAT_0052b0b8)
                .cast::<u8>()
                .add(((u_var3 & 0xffff).wrapping_mul(0xef)) as usize)
                .read();

            let u_var3 = FUN_004a7aa0(cur_arg());
            let b9 = (&raw mut DAT_0052b0b9)
                .cast::<u8>()
                .add(((u_var3 & 0xffff).wrapping_mul(0xef)) as usize)
                .read();
            if (b9 as u16).wrapping_add(b_var1 as u16) != 0 {
                let u_var3 = FUN_004a7aa0(cur_arg());
                let i2 = (u_var3 & 0xffff).wrapping_mul(0xef);
                let v = (&raw mut DAT_0052b0b4).cast::<u8>().add(i2 as usize).read();
                FUN_004ad3f0(((i2 & 0xffffff00) | v as u32) & 0xffff00ff, 0, 0);

                let u_var3 = FUN_004a7aa0(cur_arg());
                let i2 = (u_var3 & 0xffff).wrapping_mul(0xef);
                let v = (&raw mut DAT_0052b0b6).cast::<u8>().add(i2 as usize).read();
                FUN_004ad3f0(((i2 & 0xffffff00) | v as u32) & 0xffff00ff, 1, 0);

                let u_var3 = FUN_004a7aa0(cur_arg());
                let u_var6 = FUN_004a7aa0(0xc);
                if (u_var3 as i16) == (u_var6 as i16) {
                    let u_var3 = FUN_004a7aa0(cur_arg());
                    let i2 = (u_var3 & 0xffff).wrapping_mul(0xef);
                    let v = (&raw mut DAT_0052b0b5).cast::<u8>().add(i2 as usize).read();
                    FUN_004ad3f0(((i2 & 0xffffff00) | v as u32) & 0xffff00ff, 2, 1);

                    let u_var3 = FUN_004a7aa0(cur_arg());
                    let i2 = (u_var3 & 0xffff).wrapping_mul(0xef);
                    let v = (&raw mut DAT_0052b0b7).cast::<u8>().add(i2 as usize).read();
                    FUN_004ad3f0(((i2 & 0xffffff00) | v as u32) & 0xffff00ff, 3, 1);
                }
            }
        }
    }
}
