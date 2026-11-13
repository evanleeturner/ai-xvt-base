// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006cca90: u16;
    static mut DAT_006cca92: u16;
    static mut DAT_006d4988: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f050(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00422710(param_1: u32) -> u16 {
    // SAFETY: the globals and callee are the game's own; the table and
    // record pointers are read and written where the original does.
    unsafe {
        let idx = (param_1 & 0xffff) as usize;
        let base2 = (&raw mut DAT_006cca92) as usize;
        let base0 = (&raw mut DAT_006cca90) as usize;
        let u_var1: u16 =
            ((base2.wrapping_add(idx.wrapping_mul(4))) as *const u16).read_unaligned();
        let mut u_var2: u16 =
            ((base0.wrapping_add(idx.wrapping_mul(4))) as *const u16).read_unaligned();
        if u_var2 < u_var1 {
            loop {
                let rec = DAT_006d4988 as u32;
                let off = (u_var2 as u32).wrapping_mul(0x23);
                let flag = (off.wrapping_add(3).wrapping_add(rec) as *const u8).read();
                if flag == 0 {
                    let local_ptr: i32 =
                        (rec.wrapping_add(0x1f).wrapping_add(off) as *const i32).read_unaligned();
                    ((local_ptr.wrapping_add(0x83)) as u32 as *mut u16).write_unaligned(0);
                    ((local_ptr.wrapping_add(1)) as u32 as *mut u8).write(0);
                    break;
                }
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < u_var1) {
                    break;
                }
            }
            if u_var2 < u_var1 {
                FUN_0040f050(u_var2);
                return u_var2;
            }
        }
        0xffff
    }
}
