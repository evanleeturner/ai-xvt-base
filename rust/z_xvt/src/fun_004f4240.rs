// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007577d0: [u32; 0];
    static mut DAT_007577ec: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dd3e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16, param_5: i32);
}

unsafe fn copy_str(src: *const i8, dst: *mut i8) -> i32 {
    // SAFETY: src is a NUL-terminated string; dst is the caller's buffer.
    unsafe {
        let mut n: usize = 0;
        loop {
            let c = *src.add(n);
            *dst.add(n) = c;
            n += 1;
            if c == 0 {
                break;
            }
        }
        (n - 1) as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f4240(
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
    let l40 = local_40.as_mut_ptr();
    // SAFETY: the game's own globals and functions; param_1 is a NUL-terminated
    // string copied into the local buffer as the original does.
    unsafe {
        let len = copy_str(param_1, l40);
        let s_var2 = len as i16;
        let mut s_var7 = (param_6 as i32).wrapping_mul(8) as i16;
        let i_var3 = param_5 as i32;
        if i_var3 < (s_var2 as i32) + 2 {
            let mut s_var10 = s_var2;
            if param_5 <= s_var2 {
                *l40.offset(i_var3 as isize) = 0;
                s_var10 = param_5;
            }
            let mut s_var8 = param_5;
            if 3 < param_5 {
                s_var8 = 3;
            }
            s_var8 = (s_var7 as i32).wrapping_add((3 - s_var8 as i32).wrapping_mul(2)) as i16;
            let r = FUN_004db2f0(l40, 10);
            while (s_var8 as i32) <= (s_var7 as i32) + 6 && 0 < s_var10 {
                let i_var6 = s_var8 as i32;
                s_var8 = s_var8.wrapping_add(1);
                let u = (&raw mut DAT_007577d0)
                    .cast::<u32>()
                    .offset(i_var6 as isize)
                    .read_unaligned();
                *l40.offset(s_var10 as isize) = 0;
                FUN_004da7c0(10, l40, param_3 as i32, param_4 as i32, u);
                s_var10 = s_var10.wrapping_sub(1);
            }
            let i3 = (r as i16 as i32).wrapping_add(param_3 as i32);
            FUN_004dcfd0(
                local_50.as_mut_ptr(),
                i3.wrapping_add(2),
                param_4 as i32,
                i3.wrapping_add(8),
                (param_4 as i32).wrapping_add(6),
            );
            if param_5 < s_var2 {
                let v = (&raw mut DAT_007577ec)
                    .cast::<u32>()
                    .offset(s_var7 as isize)
                    .read_unaligned();
                FUN_004dd3e0(local_50.as_mut_ptr(), 0, 0, v as i16, 1);
                return;
            }
        } else {
            copy_str(param_1, l40);
            if i_var3 < (s_var2 as i32) + 5 {
                s_var7 = ((s_var7 as i32).wrapping_sub(param_5 as i32).wrapping_add(9))
                    .wrapping_add(s_var2 as i32) as i16;
            } else {
                s_var7 = (s_var7 as i32).wrapping_add(4) as i16;
            }
            let u = (&raw mut DAT_007577d0)
                .cast::<u32>()
                .offset(s_var7 as isize)
                .read_unaligned();
            FUN_004da7c0(10, l40, param_3 as i32, param_4 as i32, u);
        }
    }
}
