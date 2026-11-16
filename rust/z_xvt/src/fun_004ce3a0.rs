// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806fa: u32;
    static mut DAT_00780706: u32;
    static mut DAT_0078070a: u32;
    static mut DAT_0078070e: u32;
    static mut DAT_00780712: u32;
    static mut DAT_00780716: u32;
    static mut DAT_0078071a: u32;
    static mut DAT_0078071e: u32;
    static mut DAT_00780722: u32;
    static mut DAT_00780726: u32;
    static mut DAT_0078072a: u32;
    static mut DAT_00780c5a: u32;
    static mut DAT_00780c82: u32;
    static mut DAT_00780c86: u32;
    static mut DAT_00780e86: u32;
    static mut DAT_00780e8a: u32;
    static mut DAT_00780e8e: u32;
    static mut DAT_00780e92: u32;
    static mut DAT_00781092: u32;
    static mut DAT_00781096: u32;
    static mut DAT_007810a6: u32;
    static mut DAT_008050a6: u32;
    static mut DAT_008050aa: i32;
    static mut DAT_008050ae: i32;
    static mut DAT_008050b2: u32;
    static mut DAT_008158b2: u32;
    static mut DAT_00815ac6: u32;
    static mut DAT_0081b5be: i32;
}

unsafe fn copy_dwords(src: *const u32, dst: *mut u32, n: usize) {
    let mut i = 0usize;
    while i < n {
        // SAFETY: caller guarantees both ranges are valid for n dwords.
        unsafe {
            dst.add(i).write_unaligned(src.add(i).read_unaligned());
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ce3a0(
    param_1: *mut u32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
    param_6: *mut u32,
    param_7: i32,
    param_8: *mut i32,
    param_9: *mut i32,
    param_10: *mut u32,
    param_11: *mut u32,
    param_12: *mut i32,
    param_13: *mut u32,
    param_14: *mut u32,
    param_15: *mut u32,
    param_16: *mut u32,
    param_17: *mut u32,
    param_18: *mut u32,
    param_19: *mut u32,
    param_20: *mut u32,
    param_21: *mut u32,
    param_22: *mut u32,
) -> u32 {
    // SAFETY: the globals are the game's own memory; the parameters point at
    // caller buffers of the sizes the original copies into and out of.
    unsafe {
        param_1.write_unaligned((&raw const DAT_007806fa).read_unaligned());
        param_2.write_unaligned((&raw const DAT_00780706).read_unaligned());
        param_2
            .add(1)
            .write_unaligned((&raw const DAT_0078070a).read_unaligned());
        param_2
            .add(2)
            .write_unaligned((&raw const DAT_0078070e).read_unaligned());
        param_2
            .add(3)
            .write_unaligned((&raw const DAT_00780712).read_unaligned());
        param_3.write_unaligned((&raw const DAT_00780716).read_unaligned());
        param_3
            .add(1)
            .write_unaligned((&raw const DAT_0078071a).read_unaligned());
        param_3
            .add(2)
            .write_unaligned((&raw const DAT_0078071e).read_unaligned());
        param_3
            .add(3)
            .write_unaligned((&raw const DAT_00780722).read_unaligned());
        param_4.write_unaligned((&raw const DAT_0078072a).read_unaligned());
        param_5.write_unaligned((&raw const DAT_00780726).read_unaligned());
        copy_dwords(&raw const DAT_00780c5a, param_6, 10);

        param_9.write_unaligned((&raw const DAT_008050ae).read_unaligned());
        param_8.write_unaligned((&raw const DAT_008050aa).read_unaligned());
        param_10.write_unaligned((&raw const DAT_008050a6).read_unaligned());

        let mut i_var3: i32 = 0;
        let mut i_var1: i32 = (&raw const DAT_008050aa).read_unaligned();
        let mut i_var9: i32 = (&raw const DAT_008050ae).read_unaligned();
        if 0 < i_var9 {
            loop {
                let mut i_var5 = i_var1.wrapping_add(1);
                let src = (&raw const DAT_007810a6)
                    .wrapping_add((i_var1 as u32).wrapping_mul(0x84) as usize);
                let dst = ((i_var1 as u32)
                    .wrapping_mul(0x210)
                    .wrapping_add(param_7 as u32)) as usize as *mut u32;
                copy_dwords(src, dst, 0x84);
                if 0x3ff < i_var5 {
                    i_var5 = 0;
                }
                i_var3 = i_var3.wrapping_add(1);
                i_var1 = i_var5;
                i_var9 = (&raw const DAT_008050ae).read_unaligned();
                if !(i_var3 < i_var9) {
                    break;
                }
            }
        }

        let mut i_var1: i32 = 0;
        param_12.write_unaligned((&raw const DAT_0081b5be).read_unaligned());
        let mut i_var9: i32 = (&raw const DAT_0081b5be).read_unaligned();
        if 0 < i_var9 {
            let mut p6: *const u32 = &raw const DAT_00815ac6;
            let mut p11 = param_11;
            loop {
                copy_dwords(p6, p11, 0x8e);
                p11 = p11.wrapping_add(0x8e);
                i_var1 = i_var1.wrapping_add(1);
                p6 = p6.wrapping_add(0x8e);
                i_var9 = (&raw const DAT_0081b5be).read_unaligned();
                if !(i_var1 < i_var9) {
                    break;
                }
            }
        }

        param_13.write_unaligned((&raw const DAT_00780c82).read_unaligned());
        copy_dwords(&raw const DAT_00780c86, param_14, 0x80);
        param_15.write_unaligned((&raw const DAT_00780e86).read_unaligned());
        param_16.write_unaligned((&raw const DAT_00780e8a).read_unaligned());
        param_17.write_unaligned((&raw const DAT_00780e8e).read_unaligned());
        copy_dwords(&raw const DAT_00780e92, param_18, 0x80);
        param_19.write_unaligned((&raw const DAT_00781092).read_unaligned());
        param_20.write_unaligned((&raw const DAT_00781096).read_unaligned());

        let mut i = 0usize;
        while i < 0x4200 {
            param_21.add(i).write_unaligned(0);
            i += 1;
        }

        let end = (&raw const DAT_008158b2) as usize;
        let mut p4: *const u32 = &raw const DAT_008050b2;
        let mut dst = param_21;
        loop {
            let p7 = p4.wrapping_add(0x84);
            copy_dwords(p4, dst, 0x84);
            dst = dst.wrapping_add(0x84);
            p4 = p7;
            if !((p7 as usize) < end) {
                break;
            }
        }

        param_22.write_unaligned((&raw const DAT_008158b2).read_unaligned());
    }
    1
}
