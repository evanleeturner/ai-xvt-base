// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556930: i32;
    static mut DAT_00556949: f64;
    static mut DAT_00556951: u32;
    static mut DAT_00556955: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444eb0(param_1: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444f90(param_1: *mut u8) {
    // SAFETY: param_1 points at the caller's record; the globals are the
    // game's own memory, read and written unaligned where the original does.
    unsafe {
        let d_0e: f64 = param_1.byte_add(0xe).cast::<f64>().read_unaligned();
        let i_0a: i32 = param_1.byte_add(10).cast::<i32>().read_unaligned();
        let u_12: u32 = param_1.byte_add(0x12).cast::<u32>().read_unaligned();
        let u_0e_low: u32 = param_1.byte_add(0xe).cast::<u32>().read_unaligned();

        let g_556949: f64 = (&raw mut DAT_00556949).read_unaligned();
        let g_556951: u32 = (&raw mut DAT_00556951).read_unaligned();
        let g_556955: u32 = (&raw mut DAT_00556955).read_unaligned();

        let thresh: f64 = f64::from_bits((g_556951 as u64) | ((g_556955 as u64) << 32));

        if param_1.byte_add(2).read() != 0 {
            let mut u_var1: u32 = 0;
            loop {
                let flags = param_1.byte_add(2).read() as u32;
                if (1u32.wrapping_shl(u_var1 & 0x1f) & flags) != 0 {
                    let ptr_arg: *mut u8 = param_1
                        .byte_add((u_var1.wrapping_mul(4).wrapping_add(0x32)) as usize)
                        .cast::<*mut u8>()
                        .read_unaligned();
                    FUN_00444f90(ptr_arg);
                }
                u_var1 = u_var1.wrapping_add(1);
                if !(u_var1 < 8) {
                    break;
                }
            }
        }
        if d_0e <= g_556949 {
            FUN_00444eb0(param_1);
            return;
        }
        if i_0a != 0 {
            DAT_00556930 = DAT_00556930.wrapping_add(1);
        }
        if d_0e < thresh {
            (&raw mut DAT_00556955).write_unaligned(u_12);
            (&raw mut DAT_00556951).write_unaligned(u_0e_low);
        }
    }
}
