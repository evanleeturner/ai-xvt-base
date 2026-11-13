// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn _free(p: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db420() {
    let mut p: u32 = 0x00aa80a3;
    // SAFETY: the addresses are the game's own fixed memory tables, accessed
    // exactly as the original does; _free is the game's allocator release.
    unsafe {
        loop {
            let flag = (p.wrapping_add(0x608) as *mut u8).read();
            if flag == 1 {
                let tmp = (p as *const u32).read_unaligned();
                if tmp != 0 {
                    _free(tmp as *mut core::ffi::c_void);
                    (p as *mut u32).write_unaligned(0);
                }
                (p.wrapping_add(0x608) as *mut u8).write(0);
            }
            p = p.wrapping_add(0x60b);
            if p >= 0x00aabd11 {
                break;
            }
        }

        let mut q: u32 = 0x00aabd11;
        let mut n: i32 = 0x100;
        while n != 0 {
            (q as *mut u32).write_unaligned(0);
            q = q.wrapping_add(4);
            n -= 1;
        }
    }
}
