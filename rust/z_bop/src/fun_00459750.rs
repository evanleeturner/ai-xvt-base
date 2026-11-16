// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a082c2: [u16; 1];
    static mut DAT_00a082c0: [u16; 1];
    static mut DAT_009a1fe8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041bb00(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00459750(param_1: u32) -> u16 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // table and record pointers are read and written at the offsets the
    // original uses.
    unsafe {
        let off = ((param_1 & 0xffff).wrapping_mul(4)) as usize;
        let u1: u16 = (&raw mut DAT_00a082c2)
            .cast::<u8>()
            .byte_add(off)
            .cast::<u16>()
            .read_unaligned();
        let mut u2: u16 = (&raw mut DAT_00a082c0)
            .cast::<u8>()
            .byte_add(off)
            .cast::<u16>()
            .read_unaligned();
        if u2 < u1 {
            loop {
                let base = DAT_009a1fe8.cast::<u8>();
                let rec = (u2 as u32).wrapping_mul(0x23) as usize;
                if base.byte_add(3 + rec).read() == 0 {
                    let p = base.byte_add(0x1f + rec).cast::<i32>().read_unaligned();
                    let p = (p as u32 as usize) as *mut u8;
                    p.byte_add(0x85).cast::<u16>().write_unaligned(0);
                    p.byte_add(1).write(0);
                    break;
                }
                u2 = u2.wrapping_add(1);
                if !(u2 < u1) {
                    break;
                }
            }
            if u2 < u1 {
                FUN_0041bb00(u2);
                return u2;
            }
        }
        0xffff
    }
}
