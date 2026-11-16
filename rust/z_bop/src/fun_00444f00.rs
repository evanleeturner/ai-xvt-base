// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556928: u32;
    static mut DAT_00556930: u32;
    static mut DAT_00556949: u32;
    static mut DAT_0055694d: u32;
    static mut DAT_00556951: f64;
    static mut DAT_00556955: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444f90(param_1: u32);
    fn FUN_00443890(param_1: u32, param_2: u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444f00(param_1: u32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the double at DAT_00556951 overlaps DAT_00556955 as in the original,
    // and the pointer read is at the game's own object address.
    unsafe {
        DAT_00556951 = 1.0;
        let mut d_var1: f64 = DAT_00556951;
        while param_1 < DAT_00556930 {
            let bits: u64 = d_var1.to_bits();
            DAT_00556955 = (bits >> 32) as u32;
            (&raw mut DAT_00556951)
                .cast::<u32>()
                .write_unaligned(bits as u32);
            DAT_0055694d = DAT_00556955;
            DAT_00556949 = (&raw mut DAT_00556951).cast::<u32>().read_unaligned();
            let p = DAT_00556928.wrapping_add(0xe) as *const f64;
            DAT_00556951 = p.read_unaligned() - 1.0;
            DAT_00556930 = 0;
            FUN_00444f90(DAT_00556928);
            FUN_00443890(DAT_00556928, DAT_00556949, DAT_0055694d);
            d_var1 = DAT_00556951;
        }
        DAT_00556951 = d_var1;
    }
}
