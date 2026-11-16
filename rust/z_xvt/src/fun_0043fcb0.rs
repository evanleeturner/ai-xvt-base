// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b743: [i8; 1];
    static mut DAT_0071ecf4: u32;
    static mut DAT_006559b0: [i16; 1];
    static mut DAT_0052f0e4: u16;
    static mut DAT_0052f0e0: u16;
    static mut DAT_006559ba: i16;
    static mut DAT_006559b8: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043fcb0(param_1: u16) {
    // SAFETY: all globals are the game's own memory; the arrays are indexed
    // exactly as the original does.
    unsafe {
        let flag_base = (&raw mut DAT_0071b743).cast::<i8>();
        let arr = (&raw mut DAT_006559b0).cast::<i16>();
        let idx = DAT_0071ecf4.wrapping_mul(0x5bd) as usize;
        if flag_base.add(idx).read() == 0 {
            let psvar1 = arr.add(param_1 as usize);
            let mut local_2: u16 = 0;
            if psvar1.read() == 0 {
                psvar1.write(1);
                local_2 = DAT_0052f0e4;
                DAT_0052f0e4 = DAT_0052f0e0;
                DAT_0052f0e0 = param_1;
            } else {
                psvar1.write(-2);
            }
            if DAT_006559ba != 0 && DAT_006559b8 != 0 {
                if param_1 == 5 {
                    DAT_006559b8 = -2;
                } else {
                    DAT_006559ba = -2;
                }
                if local_2 != 5 && local_2 != 4 {
                    DAT_0052f0e4 = local_2;
                    return;
                }
            }
        } else {
            if param_1 != 2 {
                let mut u_var2: u16 = 0;
                loop {
                    if u_var2 != param_1
                        && u_var2 != 5
                        && u_var2 != 2
                        && arr.add(u_var2 as usize).read() != 0
                    {
                        arr.add(u_var2 as usize).write(-2);
                    }
                    u_var2 = u_var2.wrapping_add(1);
                    if u_var2 >= 8 {
                        break;
                    }
                }
            }
            let psvar1 = arr.add(param_1 as usize);
            if psvar1.read() != 0 {
                psvar1.write(-2);
                return;
            }
            psvar1.write(1);
            DAT_0052f0e4 = DAT_0052f0e0;
            DAT_0052f0e0 = param_1;
        }
    }
}
