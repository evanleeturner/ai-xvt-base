// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5c10: [u32; 1];
    static mut DAT_00aa5c2c: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbfa0(a: *mut i8, b: i32) -> i32;
    fn FUN_004db4e0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d67e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f8950(
    param_1: *mut i8,
    param_2: u32,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: i16,
) {
    let _ = param_2;
    if param_5 < 0 {
        return;
    }
    let mut local_50: [i32; 4] = [0; 4];
    let mut local_40: [i8; 64] = [0; 64];
    // SAFETY: param_1 is a NUL-terminated string from the caller; the copy
    // into the local buffer, the globals and the callees are the original's.
    unsafe {
        let mut len: usize = 0;
        while *param_1.add(len) != 0 {
            len += 1;
        }
        let s_var3 = len as i16;
        let lp = local_40.as_mut_ptr();
        let mut i = 0usize;
        while i <= len {
            *lp.add(i) = *param_1.add(i);
            i += 1;
        }
        let i_var4 = param_5 as i32;
        let mut s_var2 = param_6.wrapping_mul(8);
        if i_var4 < (s_var3 as i32) + 2 {
            let mut s_var8 = s_var3;
            if param_5 <= s_var3 {
                *lp.offset(i_var4 as isize) = 0;
                s_var8 = param_5;
            }
            let mut s_var10 = param_5;
            if 3 < param_5 {
                s_var10 = 3;
            }
            s_var10 = ((s_var2 as i32).wrapping_add((3 - s_var10 as i32).wrapping_mul(2))) as i16;
            let r = FUN_004dbfa0(lp, 10);
            while (s_var10 as i32) <= (s_var2 as i32) + 6 && 0 < s_var8 {
                let i_var7 = s_var10 as i32;
                s_var10 = s_var10.wrapping_add(1);
                let u = (&raw mut DAT_00aa5c10)
                    .cast::<u32>()
                    .offset(i_var7 as isize)
                    .read();
                *lp.offset(s_var8 as isize) = 0;
                FUN_004db4e0(10, lp, param_3 as i32, param_4 as i32, u);
                s_var8 = s_var8.wrapping_sub(1);
            }
            let i4 = ((r as i16) as i32).wrapping_add(param_3 as i32);
            FUN_004d6460(
                local_50.as_mut_ptr(),
                i4.wrapping_add(2),
                param_4 as i32,
                i4.wrapping_add(8),
                (param_4 as i32) + 6,
            );
            if param_5 < s_var3 {
                let v = (&raw mut DAT_00aa5c2c)
                    .cast::<u32>()
                    .offset(s_var2 as isize)
                    .read();
                FUN_004d67e0(local_50.as_mut_ptr(), 0, 0, v as i16, 1);
                return;
            }
        } else {
            if i_var4 < (s_var3 as i32) + 5 {
                s_var2 = ((s_var3 as i32) - (param_5 as i32) + (s_var2 as i32) + 9) as i16;
            } else {
                s_var2 = s_var2.wrapping_add(4);
            }
            let u = (&raw mut DAT_00aa5c10)
                .cast::<u32>()
                .offset(s_var2 as isize)
                .read();
            FUN_004db4e0(10, lp, param_3 as i32, param_4 as i32, u);
        }
    }
}
