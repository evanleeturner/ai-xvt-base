// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0071ecf4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u32;
    fn FUN_0040a1a0(param_1: u32, param_2: u32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00412700(param_1: u32, param_2: u8) {
    // SAFETY: the globals and callees are the game's own; the table base and
    // the per-entry pointer address the game's memory, accessed where the
    // original does.
    unsafe {
        let idx = param_1.wrapping_mul(0x23) as usize;
        DAT_006d4988.byte_add(3).byte_add(idx).write(param_2);
        DAT_006d4988.byte_add(2).byte_add(idx).write(0xd);
        let ptr: *mut u8 = DAT_006d4988
            .byte_add(0x1f)
            .byte_add(idx)
            .cast::<*mut u8>()
            .read_unaligned();
        ptr.write(5);
        DAT_006d4988.byte_add(0x19).byte_add(idx).write(2);
        ptr.byte_add(1).write(0);
        ptr.byte_add(0x79).cast::<u16>().write_unaligned(0);
        ptr.byte_add(0x81).cast::<u16>().write_unaligned(0);
        ptr.byte_add(0x7f).cast::<u16>().write_unaligned(0);
        DAT_006d4988
            .byte_add(0x14)
            .byte_add(idx)
            .cast::<u16>()
            .write_unaligned(0);
        ptr.byte_add(0x77).cast::<u16>().write_unaligned(0);
        ptr.byte_add(0x90).write(1);
        let i_var2 = DAT_0071ecf4;
        let u_var1 = FUN_004784d0();
        FUN_0040a1a0((u_var1 & 3).wrapping_add(0x16), param_1, i_var2);
    }
}
