// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00732a3c: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042a0d0(param_1: u32, param_2: u8, param_3: u8) {
    // SAFETY: the globals are the game's own memory, and the table and
    // record pointers they lead to are read and written where the original does.
    unsafe {
        if _DAT_00732a3c < DAT_006d4980 {
            let mut i_var3: i32 = _DAT_00732a3c.wrapping_mul(0x23);
            let mut u_var2: i32 = _DAT_00732a3c;
            loop {
                let base = DAT_006d4988;
                let c = (base.wrapping_add(3).wrapping_add(i_var3) as *const i8).read();
                let b = (base.wrapping_add(i_var3).wrapping_add(0x16) as *const u8).read();
                if c != 0 && (b as u32) != param_1 {
                    let ptr_val = (DAT_006d4988.wrapping_add(i_var3).wrapping_add(0x1f)
                        as *const i32)
                        .read_unaligned();
                    let inner_ptr = (ptr_val.wrapping_add(0xa7) as *const i32).read_unaligned();
                    (inner_ptr.wrapping_add(0xae) as *mut u8).write(param_2);
                    (inner_ptr.wrapping_add(0xaf) as *mut u8).write(param_3);
                }
                i_var3 = i_var3.wrapping_add(0x23);
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < DAT_006d4980) {
                    break;
                }
            }
        }
    }
}
