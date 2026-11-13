// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fgets(s: *mut i8, n: i32, stream: *mut core::ffi::c_void) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00425a70(param_1: *mut core::ffi::c_void, param_2: *mut i8) -> i32 {
    // SAFETY: the stream and buffer belong to the caller; the buffer holds
    // 0x400 bytes as the original assumes, and fgets is the game's own.
    unsafe {
        loop {
            let pc_var2 = _fgets(param_2, 0x400, param_1);
            if pc_var2.is_null() {
                return -1;
            }
            param_2.add(0x3ff).write(0);
            if !(param_2.read() == b'/' as i8 && param_2.add(1).read() == b'/' as i8) {
                break;
            }
        }

        let mut u_var4: u32 = 0xffff_ffff;
        let mut pc_var2 = param_2;
        loop {
            if u_var4 == 0 {
                break;
            }
            u_var4 = u_var4.wrapping_sub(1);
            let c_var1 = pc_var2.read();
            pc_var2 = pc_var2.wrapping_add(1);
            if c_var1 == 0 {
                break;
            }
        }

        u_var4 = !u_var4;
        let mut i_var3 = u_var4.wrapping_sub(1) as i32;
        let idx = u_var4.wrapping_sub(2) as usize;
        let p = param_2.wrapping_add(idx);
        if p.read() == b'\n' as i8 {
            p.write(0);
            i_var3 = u_var4.wrapping_sub(2) as i32;
        }
        i_var3
    }
}
