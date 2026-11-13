// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330ac: i32;
    static mut DAT_00733100: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431d70(param_1: *mut i32, param_2: *mut u32) -> *mut core::ffi::c_void;
    fn FUN_00430980(param_1: i32, param_2: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00432f60() -> u32 {
    let mut local_8: i32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: all callees and globals are the game's own; pointers returned by
    // FUN_00431d70 point at 10-word records read and copied as the original does.
    unsafe {
        let dvar1 = timeGetTime();
        let first: *mut i32 = 'a: loop {
            let dvar2 = timeGetTime();
            if dvar2.wrapping_sub(dvar1) > 60000 {
                break 'a core::ptr::null_mut();
            }
            loop {
                let p = FUN_00431d70(&raw mut local_8, &raw mut local_4) as *mut i32;
                if p.is_null() {
                    break;
                }
                if local_8 != 0 {
                    break 'a p;
                }
                FUN_00430980(p.read_unaligned(), p as usize as i32);
            }
        };
        if first.is_null() {
            return 0;
        }
        DAT_007330ac = first.read_unaligned();
        let mut i_var6: i32 = 0;
        if DAT_007330ac < 1 {
            return 1;
        }
        let mut dst: *mut i32 = &raw mut DAT_00733100;
        loop {
            let dvar1 = timeGetTime();
            let mut src: *mut i32 = 'b: loop {
                let dvar2 = timeGetTime();
                if dvar2.wrapping_sub(dvar1) >= 0xea61 {
                    break 'b core::ptr::null_mut();
                }
                loop {
                    let p = FUN_00431d70(&raw mut local_8, &raw mut local_4) as *mut i32;
                    if p.is_null() {
                        break;
                    }
                    if local_8 != 0 {
                        break 'b p;
                    }
                    FUN_00430980(p.read_unaligned(), p as usize as i32);
                }
            };
            if src.is_null() {
                return 0;
            }
            let mut d = dst;
            let mut n = 10;
            while n != 0 {
                d.write_unaligned(src.read_unaligned());
                src = src.add(1);
                d = d.add(1);
                n -= 1;
            }
            i_var6 = i_var6.wrapping_add(1);
            dst = dst.add(10);
            if DAT_007330ac <= i_var6 {
                return 1;
            }
        }
    }
}
