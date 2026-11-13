// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: [u8; 1];
    static mut DAT_009d8ce4: [u8; 1];
    static mut DAT_009a73c0: u32;
    static mut DAT_009a73c2: u32;
    static mut DAT_009a73b6: [u8; 1];
    static mut _DAT_009993f8: unsafe extern "stdcall" fn();
    static mut PTR_caseD_0_005278b0: [unsafe extern "stdcall" fn(); 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049f360() {
    // SAFETY: all globals are the game's own memory; offsets and accesses
    // follow the original exactly.
    unsafe {
        let base = (&raw mut DAT_00a08104).cast::<u8>();
        base.byte_add(0xb1).cast::<u32>().write_unaligned(0);

        let v = base.byte_add(0xb1).cast::<u32>().read_unaligned();
        base.byte_add(0xb5).cast::<u32>().write_unaligned(v);

        let v = base.byte_add(0xb5).cast::<u32>().read_unaligned();
        base.byte_add(0xb9).cast::<u32>().write_unaligned(v);

        base.byte_add(0x79).write(0);
        base.byte_add(0x78).write(0);

        base.byte_add(0x23b).cast::<u16>().write_unaligned(0);

        let idx = DAT_009a73c0
            .wrapping_mul(0x566)
            .wrapping_add(DAT_009a73c2.wrapping_mul(0x52));
        let b = (&raw mut DAT_009d8ce4)
            .cast::<u8>()
            .byte_add(idx as usize)
            .read();
        base.byte_add(0xc1).cast::<u16>().write_unaligned(b as u16);

        let v4 = base.byte_add(0xc1).cast::<i16>().read_unaligned();
        base.byte_add(0xc1)
            .cast::<i16>()
            .write_unaligned(v4.wrapping_mul(5));

        base.byte_add(0xa4).write(4);

        let p = (&raw mut DAT_009a73b6).cast::<u8>();
        p.byte_add(0x35).write(0);
        let sel = p.byte_add(0x34).read();
        _DAT_009993f8 = (&raw const PTR_caseD_0_005278b0)
            .cast::<unsafe extern "stdcall" fn()>()
            .add(sel as usize)
            .read();
        (_DAT_009993f8)();
    }
}
