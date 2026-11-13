// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a38: u32;
    static mut DAT_00703c88: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007394b0: u32;
    static mut DAT_007394b4: u32;
    static mut DAT_007394b8: u32;
    static mut DAT_007394bc: u16;
    static mut DAT_007394be: u16;
    static mut DAT_007394c0: u16;
    static mut DAT_006db6c0: i16;
    static mut DAT_006d4a6c: i16;
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_00421a80(param_1: u16, param_2: i16, param_3: u8) -> u32 {
    // SAFETY: all globals are the game's own memory; the record table pointer
    // in DAT_006d4988 addresses the game's entries, accessed where the original does.
    unsafe {
        let mut u_var2: u32 = DAT_00732a38 & 0xFFFF;
        let mut u_var5: u16 = DAT_00732a38 as u16;
        let mut i_var3: i32;
        let mut u_var4: u16;
        'found: {
            while (u_var2 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38 as i32) {
                i_var3 = DAT_006d4988;
                let addr = (u_var5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988 as u32);
                if (addr as *const u8).read() == 0 {
                    break 'found;
                }
                u_var5 = u_var5.wrapping_add(1);
                u_var2 = u_var5 as u32;
            }
            i_var3 = 0;
            u_var5 = 0xFFFF;
        }
        u_var4 = ((i_var3 as u32) >> 16) as u16;
        if u_var5 != 0xFFFF {
            let off = (u_var5 as i32).wrapping_mul(0x23);
            let at = |o: i32| -> *mut u8 {
                (DAT_006d4988.wrapping_add(o).wrapping_add(off) as u32) as *mut u8
            };
            at(4).cast::<u32>().write_unaligned(DAT_007394b0);
            at(8).cast::<u32>().write_unaligned(DAT_007394b4);
            at(0xC).cast::<u32>().write_unaligned(DAT_007394b8);
            at(0x10).cast::<u16>().write_unaligned(DAT_007394bc);
            at(0x12).cast::<u16>().write_unaligned(DAT_007394be);
            at(0x14).cast::<u16>().write_unaligned(DAT_007394c0);

            let t = at(4).cast::<u32>().read_unaligned();
            at(4).cast::<u32>().write_unaligned(t.wrapping_shl(8));
            let t = at(8).cast::<u32>().read_unaligned();
            at(8).cast::<u32>().write_unaligned(t.wrapping_shl(8));
            let t = at(0xC).cast::<u32>().read_unaligned();
            at(0xC).cast::<u32>().write_unaligned(t.wrapping_shl(8));
            let t = at(0x10).cast::<u16>().read_unaligned();
            at(0x10).cast::<u16>().write_unaligned(t.wrapping_shl(8));
            let t = at(0x12).cast::<u16>().read_unaligned();
            at(0x12).cast::<u16>().write_unaligned(t.wrapping_shl(8));
            let t = at(0x14).cast::<u16>().read_unaligned();
            at(0x14).cast::<u16>().write_unaligned(t.wrapping_shl(8));

            at(0).cast::<i16>().write_unaligned(DAT_006db6c0);
            DAT_006db6c0 = DAT_006db6c0.wrapping_add(1);

            at(0x16).write(param_1 as u8);
            at(2).write(param_2 as u8);
            at(3).write(param_3);
            at(0x17).cast::<u16>().write_unaligned(0x3FF);
            at(0x19).write(0);

            let rec = (&raw mut DAT_006d4a6c)
                .cast::<u8>()
                .wrapping_add((param_1 as usize).wrapping_mul(0x126))
                .cast::<u16>();
            if param_2 == 8 {
                let t = rec.read_unaligned();
                at(0x1A).write(((t as u8) & 7).wrapping_mul(0x1D));
            } else {
                at(0x1A).write(0);
            }

            u_var4 = ((param_1 as u32).wrapping_mul(0x93) >> 16) as u16;

            let t = rec.read_unaligned();
            rec.write_unaligned(t.wrapping_add(1));
        }
        (u32::from(u_var4) << 16) | u32::from(u_var5)
    }
}
