// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00556941: u32;
    static mut _DAT_00556945: u32;
    static mut _DAT_00556965: u32;
    static mut DAT_00556934: u32;
    static mut DAT_00556935: u32;
    static mut DAT_00556936: u32;
    static mut DAT_0055693d: u32;
    static mut DAT_00556959: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004444f0(param_1: i32) {
    let base = param_1 as u32;
    // SAFETY: param_1 is the address of the game's node structure and the
    // globals are the game's own memory; reads and writes follow the original.
    unsafe {
        let field_2: u8 = (base.wrapping_add(2) as *const u8).read();
        if field_2 != 0 {
            let mut u_var3: u32 = 0;
            loop {
                if (1u32.wrapping_shl(u_var3 & 0x1f) & (field_2 as u32)) != 0 {
                    let child_ptr: u32 =
                        (base.wrapping_add(0x32).wrapping_add(u_var3.wrapping_mul(4))
                            as *const u32)
                            .read_unaligned();
                    FUN_004444f0(child_ptr as i32);
                }
                u_var3 = u_var3.wrapping_add(1);
                if u_var3 >= 8 {
                    break;
                }
            }
        }

        let mut d_var2: f64 = (&raw mut _DAT_00556941 as *const f64).read_unaligned();

        let field_10: u32 = (base.wrapping_add(10) as *const u32).read_unaligned();
        if field_10 != 0 {
            let field_6: u32 = (base.wrapping_add(6) as *const u32).read_unaligned();
            let pbv = field_6.wrapping_mul(9).wrapping_add(DAT_0055693d) as *const u8;
            let tbl = &raw mut DAT_00556959 as *const u32;

            let i1 = (pbv.add(1).read() as u32).wrapping_sub(DAT_00556935);
            let a2 = tbl.add(i1 as usize).read_unaligned() as u64 as f64;

            let i0 = (pbv.read() as u32).wrapping_sub(DAT_00556934);
            let a1 = tbl.add(i0 as usize).read_unaligned() as u64 as f64;

            let i2 = (pbv.add(2).read() as u32).wrapping_sub(DAT_00556936);
            let a3 = tbl.add(i2 as usize).read_unaligned() as u64 as f64;

            let d_var1: f64 = a3 + a1 + a2;

            d_var2 = (&raw mut _DAT_00556941 as *const f64).read_unaligned();
            if d_var1 < d_var2 {
                let f6: u16 = (base.wrapping_add(6) as *const u16).read_unaligned();
                _DAT_00556965 = f6 as u32;
                d_var2 = d_var1;
            }
        }

        let bits: u64 = d_var2.to_bits();
        _DAT_00556945 = (bits >> 32) as u32;
        _DAT_00556941 = bits as u32;
    }
}
