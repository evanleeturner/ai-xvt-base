// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db730: [i32; 0];
    static mut DAT_0071f720: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049b0d0(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut i32 {
    // SAFETY: the globals are the game's own tables; the record pointers stay
    // within the game's per-slot record array, accessed as the original does.
    unsafe {
        let counts = (&raw mut DAT_006db730).cast::<i32>();
        let base = (&raw mut DAT_0071f720).cast::<i32>();
        let i_var1: i32 = counts.offset(param_1 as isize).read_unaligned();
        let mut i_var4: i32 = 0;
        let mut pi_var2: *mut u8 = base
            .offset((param_1.wrapping_mul(0x7e9)) as isize)
            .cast::<u8>();
        let mut pi_var5: *mut u8 = pi_var2.offset(i_var1.wrapping_mul(0x12) as isize);
        let mut i_var3: i32;

        'lab: {
            if 0 < i_var1 {
                loop {
                    i_var3 = pi_var2.byte_add(8).cast::<i32>().read_unaligned();
                    if param_2 <= i_var3 {
                        break 'lab;
                    }
                    i_var4 = i_var4.wrapping_add(1);
                    pi_var2 = pi_var2.byte_add(0x12);
                    if !(i_var4 < i_var1) {
                        break;
                    }
                }
            }
            i_var3 = pi_var2.byte_add(8).cast::<i32>().read_unaligned();
        }

        if param_2 < i_var3 || i_var1 == i_var4 {
            if i_var1 == 0x1c2 {
                return core::ptr::null_mut();
            }
            counts
                .offset(param_1 as isize)
                .write_unaligned(i_var1.wrapping_add(1));
            while pi_var2 < pi_var5 {
                let v0 = pi_var5.byte_sub(0x12).cast::<i32>().read_unaligned();
                pi_var5.cast::<i32>().write_unaligned(v0);
                let v1 = pi_var5.byte_sub(0xe).cast::<i32>().read_unaligned();
                pi_var5.byte_add(4).cast::<i32>().write_unaligned(v1);
                let v2 = pi_var5.byte_sub(10).cast::<i32>().read_unaligned();
                pi_var5.byte_add(8).cast::<i32>().write_unaligned(v2);
                let v3 = pi_var5.byte_sub(6).cast::<i32>().read_unaligned();
                pi_var5.byte_add(12).cast::<i32>().write_unaligned(v3);
                let v4 = pi_var5.byte_sub(2).cast::<u16>().read_unaligned();
                pi_var5.byte_add(16).cast::<u16>().write_unaligned(v4);
                pi_var5 = pi_var5.byte_sub(0x12);
            }
        } else if param_2 == i_var3 {
            if pi_var2.byte_add(4).cast::<i32>().read_unaligned() == 0 {
                return core::ptr::null_mut();
            }
            if pi_var2.cast::<i32>().read_unaligned() == 1 {
                return core::ptr::null_mut();
            }
        }
        pi_var2.byte_add(8).cast::<i32>().write_unaligned(param_2);
        pi_var2.byte_add(4).cast::<i32>().write_unaligned(1);
        pi_var2.cast::<i32>().write_unaligned(0);
        let a = param_3.read_unaligned();
        pi_var2.byte_add(12).cast::<i32>().write_unaligned(a);
        let b = param_3.add(1).read_unaligned();
        pi_var2.byte_add(16).cast::<i16>().write_unaligned(b as i16);
        pi_var2.cast::<i32>()
    }
}
