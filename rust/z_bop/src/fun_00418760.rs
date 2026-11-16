// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8db0: [i32; 1];
    static mut DAT_009ed670: [i32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418760(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut i32 {
    // SAFETY: the globals are the game's own tables; param_3 points at the
    // caller's data. All reads and writes follow the original's offsets.
    unsafe {
        let counts = (&raw mut DAT_009a8db0).cast::<i32>();
        let i_var1 = counts.offset(param_1 as isize).read();
        let mut p3: *mut u8 = (&raw mut DAT_009ed670)
            .cast::<i32>()
            .offset(param_1.wrapping_mul(0x7e9) as isize)
            .cast::<u8>();
        let mut p5: *mut u8 = p3.offset(i_var1.wrapping_mul(0x12) as isize);
        let mut i_var6: i32 = 0;
        let mut i_var4: i32;
        let mut found = false;
        i_var4 = 0;
        if i_var1 > 0 {
            loop {
                i_var4 = p3.byte_add(8).cast::<i32>().read_unaligned();
                if param_2 <= i_var4 {
                    found = true;
                    break;
                }
                i_var6 = i_var6.wrapping_add(1);
                p3 = p3.offset(0x12);
                if !(i_var6 < i_var1) {
                    break;
                }
            }
        }
        if !found {
            i_var4 = p3.byte_add(8).cast::<i32>().read_unaligned();
        }
        if i_var4 >= param_2 && i_var6 != i_var1 {
            if i_var4 == param_2 {
                if p3.byte_add(4).cast::<i32>().read_unaligned() == 0 {
                    return core::ptr::null_mut();
                }
                if p3.cast::<i32>().read_unaligned() == 1 {
                    return core::ptr::null_mut();
                }
            }
        } else {
            if i_var1 == 0x1c2 {
                return core::ptr::null_mut();
            }
            counts
                .offset(param_1 as isize)
                .write(i_var1.wrapping_add(1));
            while (p3 as usize) < (p5 as usize) {
                let s = p5.offset(-0x12);
                p5.cast::<i32>()
                    .write_unaligned(s.cast::<i32>().read_unaligned());
                p5.byte_add(4)
                    .cast::<i32>()
                    .write_unaligned(s.byte_add(4).cast::<i32>().read_unaligned());
                p5.byte_add(8)
                    .cast::<i32>()
                    .write_unaligned(s.byte_add(8).cast::<i32>().read_unaligned());
                p5.byte_add(12)
                    .cast::<i32>()
                    .write_unaligned(s.byte_add(12).cast::<i32>().read_unaligned());
                p5.byte_add(16)
                    .cast::<u16>()
                    .write_unaligned(s.byte_add(16).cast::<u16>().read_unaligned());
                p5 = p5.offset(-0x12);
            }
        }
        p3.byte_add(8).cast::<i32>().write_unaligned(param_2);
        p3.byte_add(4).cast::<i32>().write_unaligned(1);
        p3.cast::<i32>().write_unaligned(0);
        p3.byte_add(12)
            .cast::<i32>()
            .write_unaligned(param_3.read_unaligned());
        p3.byte_add(16)
            .cast::<i16>()
            .write_unaligned(param_3.add(1).read_unaligned() as i16);
        p3.cast::<i32>()
    }
}
