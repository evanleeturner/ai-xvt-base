// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: [u8; 0];
    static mut DAT_0070ada4: [u8; 0];
    static mut DAT_006d9d60: u32;
    static mut DAT_006d9d62: u32;
    static mut DAT_006d9d56: [u8; 0];
    static mut PTR_caseD_0_0052a6f0: [*const core::ffi::c_void; 0];
    static mut _DAT_0065864c: *const core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00491ad0() {
    // SAFETY: all globals are the game's own memory; offsets are accessed
    // exactly as the original does, and the stored pointer is the game's
    // void(void) handler.
    unsafe {
        let base = (&raw mut DAT_00732e54).cast::<u8>();

        let _ = base.byte_add(0xb1).cast::<u32>().read_unaligned();
        base.byte_add(0xb1).cast::<u32>().write_unaligned(0);

        let v_b5 = base.byte_add(0xb1).cast::<u32>().read_unaligned();
        base.byte_add(0xb5).cast::<u32>().write_unaligned(v_b5);

        let v_b9 = base.byte_add(0xb5).cast::<u32>().read_unaligned();
        base.byte_add(0xb9).cast::<u32>().write_unaligned(v_b9);

        base.byte_add(0x79).write(0);
        base.byte_add(0x78).write(0);

        let _ = base.byte_add(0x231).cast::<u16>().read_unaligned();
        base.byte_add(0x231).cast::<u16>().write_unaligned(0);

        let idx = DAT_006d9d60
            .wrapping_mul(0x566)
            .wrapping_add(DAT_006d9d62.wrapping_mul(0x52));
        let v_c1 = u16::from(
            (&raw mut DAT_0070ada4)
                .cast::<u8>()
                .add(idx as usize)
                .read(),
        );
        base.byte_add(0xc1).cast::<u16>().write_unaligned(v_c1);

        let v_c1_s = base.byte_add(0xc1).cast::<i16>().read_unaligned();
        base.byte_add(0xc1)
            .cast::<i16>()
            .write_unaligned(v_c1_s.wrapping_mul(5));

        base.byte_add(0xa4).write(4);
        let tbl = (&raw mut DAT_006d9d56).cast::<u8>();
        tbl.byte_add(0x35).write(0);
        let sel = tbl.byte_add(0x34).read();
        _DAT_0065864c = (&raw mut PTR_caseD_0_0052a6f0)
            .cast::<*const core::ffi::c_void>()
            .add(sel as usize)
            .read();
        let f: unsafe extern "C" fn() = core::mem::transmute(_DAT_0065864c);
        f();
    }
}
