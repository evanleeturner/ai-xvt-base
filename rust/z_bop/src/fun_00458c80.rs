// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099f970: u32;
    static mut DAT_0099f974: u32;
    static mut DAT_0099f978: u32;
    static mut DAT_0099f97c: u16;
    static mut DAT_0099f97e: u16;
    static mut DAT_0099f980: u16;
    static mut DAT_009a8d40: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a20cc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004597f0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00458c80(param_1: u16, param_2: i16, param_3: u8) -> u32 {
    // SAFETY: the globals and the callee are the game's own; the record table
    // pointer and the per-index array are the game's memory, accessed at the
    // same offsets as the original.
    unsafe {
        let mut u_var4: u32 = FUN_004597f0();
        let s_var3 = u_var4 as i16;
        if s_var3 != -1 {
            let i_var2: i32 = ((u_var4 & 0xffff) as i32).wrapping_mul(0x23);
            let p = DAT_009a1fe8.wrapping_add(i_var2 as u32) as usize as *mut u8;

            p.byte_add(4).cast::<u32>().write_unaligned(DAT_0099f970);
            p.byte_add(8).cast::<u32>().write_unaligned(DAT_0099f974);
            p.byte_add(0xc).cast::<u32>().write_unaligned(DAT_0099f978);
            p.byte_add(0x10).cast::<u16>().write_unaligned(DAT_0099f97c);
            p.byte_add(0x12).cast::<u16>().write_unaligned(DAT_0099f97e);
            p.byte_add(0x14).cast::<u16>().write_unaligned(DAT_0099f980);

            let v = p.byte_add(4).cast::<u32>().read_unaligned();
            p.byte_add(4)
                .cast::<u32>()
                .write_unaligned(v.wrapping_shl(8));
            let v = p.byte_add(8).cast::<u32>().read_unaligned();
            p.byte_add(8)
                .cast::<u32>()
                .write_unaligned(v.wrapping_shl(8));
            let v = p.byte_add(0xc).cast::<u32>().read_unaligned();
            p.byte_add(0xc)
                .cast::<u32>()
                .write_unaligned(v.wrapping_shl(8));
            let v = p.byte_add(0x10).cast::<u16>().read_unaligned();
            p.byte_add(0x10)
                .cast::<u16>()
                .write_unaligned(v.wrapping_shl(8));
            let v = p.byte_add(0x12).cast::<u16>().read_unaligned();
            p.byte_add(0x12)
                .cast::<u16>()
                .write_unaligned(v.wrapping_shl(8));
            let v = p.byte_add(0x14).cast::<u16>().read_unaligned();
            p.byte_add(0x14)
                .cast::<u16>()
                .write_unaligned(v.wrapping_shl(8));

            p.cast::<u16>().write_unaligned(DAT_009a8d40);
            DAT_009a8d40 = DAT_009a8d40.wrapping_add(1);
            p.byte_add(0x16).write(param_1 as u8);
            p.byte_add(2).write(param_2 as u8);
            p.byte_add(3).write(param_3);
            p.byte_add(0x17).cast::<u16>().write_unaligned(0x3ff);
            p.byte_add(0x19).write(0);

            let q = (&raw mut DAT_009a20cc as usize as u32)
                .wrapping_add((param_1 as u32).wrapping_mul(0x126)) as usize
                as *mut u16;
            if param_2 == 8 {
                let w = q.read_unaligned();
                p.byte_add(0x1a).write(((w as u8) & 7).wrapping_mul(0x1d));
            } else {
                p.byte_add(0x1a).write(0);
            }
            u_var4 = (param_1 as u32).wrapping_mul(0x93);
            let w = q.read_unaligned();
            q.write_unaligned(w.wrapping_add(1));
        }
        (u_var4 & 0xFFFF0000) | (s_var3 as u16 as u32)
    }
}
