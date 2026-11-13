// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00536f11: f64;
    static mut DAT_00536f25: i32;
    static mut DAT_00536ef8: i32;
    static mut DAT_00536f1d: u32;
    static mut DAT_00536f19: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00428a10(param_1: *mut u8) {
    // SAFETY: param_1 and the pointers stored in it are the game's own
    // structures, read and written at the offsets the original uses; the
    // globals are the game's own memory.
    unsafe {
        if param_1.byte_add(2).read() != 0 {
            let mut u_var2: u32 = 0;
            loop {
                let p_tmp = param_1
                    .byte_add((u_var2.wrapping_mul(4)).wrapping_add(0x32) as usize)
                    .cast::<u32>()
                    .read_unaligned() as usize as *mut u8;
                let mask = 1i32.wrapping_shl(u_var2 & 0x1f) as u32;
                if (mask & (param_1.byte_add(2).read() as u32)) != 0 {
                    FUN_00428a10(p_tmp);
                }
                u_var2 = u_var2.wrapping_add(1);
                if u_var2 >= 8 {
                    break;
                }
            }
        }

        let d_tmp = param_1.byte_add(0xe).cast::<f64>().read_unaligned();
        if d_tmp <= _DAT_00536f11 {
            let i_var1 = param_1.byte_add(0x2e).cast::<i32>().read_unaligned();
            let q = i_var1 as u32 as usize as *mut u8;

            let mut b_tmp = q.byte_add(2).read();
            let sh = (param_1.read() & 0x1f) as u32;
            b_tmp = b_tmp & (!(1i32.wrapping_shl(sh))) as u8;
            q.byte_add(2).write(b_tmp);

            let mut i_tmp = q.byte_add(10).cast::<i32>().read_unaligned();
            let i_param_10 = param_1.byte_add(10).cast::<i32>().read_unaligned();
            i_tmp = i_tmp.wrapping_add(i_param_10);
            q.byte_add(10).cast::<i32>().write_unaligned(i_tmp);

            let a = param_1.byte_add(0x16).cast::<f64>().read_unaligned();
            let b = q.byte_add(0x16).cast::<f64>().read_unaligned();
            q.byte_add(0x16).cast::<f64>().write_unaligned(a + b);

            let a = param_1.byte_add(0x1e).cast::<f64>().read_unaligned();
            let b = q.byte_add(0x1e).cast::<f64>().read_unaligned();
            q.byte_add(0x1e).cast::<f64>().write_unaligned(a + b);

            let a = param_1.byte_add(0x26).cast::<f64>().read_unaligned();
            let b = q.byte_add(0x26).cast::<f64>().read_unaligned();
            q.byte_add(0x26).cast::<f64>().write_unaligned(a + b);

            DAT_00536f25 = DAT_00536f25.wrapping_add(-1);
            return;
        }

        let i_tmp = param_1.byte_add(10).cast::<i32>().read_unaligned();
        if i_tmp != 0 {
            DAT_00536ef8 = DAT_00536ef8.wrapping_add(1);
        }

        let d_tmp = param_1.byte_add(0xe).cast::<f64>().read_unaligned();
        let d_concat = (&raw mut DAT_00536f19).cast::<f64>().read_unaligned();
        if d_tmp < d_concat {
            let ui_tmp = param_1.byte_add(0x12).cast::<u32>().read_unaligned();
            DAT_00536f1d = ui_tmp;
            let ui_tmp = param_1.byte_add(0xe).cast::<u32>().read_unaligned();
            DAT_00536f19 = ui_tmp;
        }
    }
}
