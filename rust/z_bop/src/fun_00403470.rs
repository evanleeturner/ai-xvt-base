// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00403470(param_1: u32, param_2: u8, param_3: u8) {
    // SAFETY: the globals are the game's own memory; the entry table and the
    // pointers stored in it are read and written exactly where the original does.
    unsafe {
        if _DAT_00a07cec < (DAT_009a1fe0 as i32) {
            let mut i_var3: i32 = _DAT_00a07cec.wrapping_mul(0x23);
            let mut u_var2: u32 = _DAT_00a07cec as u32;
            loop {
                let base = DAT_009a1fe8;
                let flag = base
                    .wrapping_byte_offset(i_var3 as isize)
                    .wrapping_byte_add(3)
                    .cast::<i8>()
                    .read();
                let id = base
                    .wrapping_byte_offset(i_var3 as isize)
                    .wrapping_byte_add(0x16)
                    .cast::<u8>()
                    .read();
                if flag != 0 && (id as u32) == param_1 {
                    let local_ptr: i32 = base
                        .wrapping_byte_offset(i_var3 as isize)
                        .wrapping_byte_add(0x1f)
                        .cast::<i32>()
                        .read_unaligned();
                    let local_ptr2: i32 =
                        (local_ptr.wrapping_add(0xa9) as usize as *const i32).read_unaligned();
                    let i_var1 = local_ptr2;
                    (i_var1.wrapping_add(0xae) as usize as *mut u8).write(param_2);
                    (i_var1.wrapping_add(0xaf) as usize as *mut u8).write(param_3);
                }
                i_var3 = i_var3.wrapping_add(0x23);
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < DAT_009a1fe0) {
                    break;
                }
            }
        }
    }
}
