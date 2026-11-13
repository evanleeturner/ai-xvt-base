// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9683: [u8; 0];
    static mut DAT_009ecc34: u32;
    static mut DAT_00a08ba0: [u16; 0];
    static mut DAT_0052154c: u16;
    static mut DAT_00521548: u16;
    static mut DAT_00a08baa: u16;
    static mut DAT_00a08ba8: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004803f0(param_1: u16) {
    // SAFETY: all globals are the game's own memory; the arrays are indexed
    // exactly as the original does.
    unsafe {
        let arr = (&raw mut DAT_00a08ba0).cast::<u16>();
        let flags = (&raw mut DAT_009e9683).cast::<u8>();
        let mut local_2: u16 = 0;
        if flags.add(DAT_009ecc34.wrapping_mul(0x5bd) as usize).read() == 0 {
            let psvar1 = arr.add(param_1 as usize);
            if psvar1.read() == 0 {
                psvar1.write(1);
                local_2 = DAT_0052154c;
                DAT_0052154c = DAT_00521548;
                DAT_00521548 = param_1;
            } else {
                psvar1.write(0xfffe);
            }
            if DAT_00a08baa != 0 && DAT_00a08ba8 != 0 {
                if param_1 == 5 {
                    DAT_00a08ba8 = 0xfffe;
                } else {
                    DAT_00a08baa = 0xfffe;
                }
                if local_2 != 5 && local_2 != 4 {
                    DAT_0052154c = local_2;
                    return;
                }
            }
        } else {
            if param_1 != 2 {
                let mut u_var2: u16 = 0;
                loop {
                    if param_1 != u_var2
                        && u_var2 != 5
                        && u_var2 != 2
                        && arr.add(u_var2 as usize).read() != 0
                    {
                        arr.add(u_var2 as usize).write(0xfffe);
                    }
                    u_var2 = u_var2.wrapping_add(1);
                    if u_var2 >= 8 {
                        break;
                    }
                }
            }
            let psvar1 = arr.add(param_1 as usize);
            if psvar1.read() != 0 {
                psvar1.write(0xfffe);
                return;
            }
            psvar1.write(1);
            DAT_0052154c = DAT_00521548;
            DAT_00521548 = param_1;
        }
    }
}
