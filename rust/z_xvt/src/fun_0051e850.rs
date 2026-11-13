// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051e850(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    let mut src = param_3 as *mut u8;
    let mut u_var9: u32 = (param_7.wrapping_neg() & 7) as u32;
    if (param_4 as i32) < (u_var9 as i32) {
        u_var9 = param_4;
    }
    let i_var4: i32 = (param_4.wrapping_sub(u_var9) as i32) >> 3;
    let u_var8: u32 = param_4
        .wrapping_add(i_var4.wrapping_mul(-8) as u32)
        .wrapping_sub(u_var9);
    let i_var5: i32 = param_6.wrapping_mul(2).wrapping_sub(param_4 as i32);
    let i_var6: i32 = param_2.wrapping_mul(2).wrapping_sub(param_4 as i32);
    let mut dst = param_1
        .wrapping_add(param_8.wrapping_mul(param_2))
        .wrapping_add(param_7) as usize as *mut u8;
    let mut i_var7: i32 = param_5 >> 1;

    if u_var9 == 0 && u_var8 == 0 {
        if i_var7 != 0 {
            // SAFETY: the caller supplies valid source and destination
            // surfaces; reads and writes happen in the original's order.
            unsafe {
                loop {
                    i_var7 = i_var7.wrapping_sub(1);
                    let mut i_var10 = i_var4;
                    while i_var10 != 0 {
                        let v = src.wrapping_byte_add(4).cast::<u32>().read_unaligned();
                        dst.wrapping_byte_add(4).cast::<u32>().write_unaligned(v);
                        let v = src.cast::<u32>().read_unaligned();
                        dst.cast::<u32>().write_unaligned(v);
                        src = src.wrapping_byte_add(8);
                        dst = dst.wrapping_byte_add(8);
                        i_var10 = i_var10.wrapping_sub(1);
                    }
                    src = src.wrapping_offset(i_var5 as isize);
                    dst = dst.wrapping_offset(i_var6 as isize);
                    if i_var7 == 0 {
                        break;
                    }
                }
            }
        }
    } else if i_var7 != 0 {
        // SAFETY: the caller supplies valid source and destination
        // surfaces; reads and writes happen in the original's order.
        unsafe {
            loop {
                i_var7 = i_var7.wrapping_sub(1);
                if u_var9 & 1 != 0 {
                    let v = src.read();
                    src = src.wrapping_byte_add(1);
                    dst.write(v);
                    dst = dst.wrapping_byte_add(1);
                }
                if u_var9 & 2 != 0 {
                    let v = src.cast::<u16>().read_unaligned();
                    src = src.wrapping_byte_add(2);
                    dst.cast::<u16>().write_unaligned(v);
                    dst = dst.wrapping_byte_add(2);
                }
                let mut i_var10 = i_var4;
                if u_var9 & 4 != 0 {
                    let v = src.cast::<u32>().read_unaligned();
                    src = src.wrapping_byte_add(4);
                    dst.cast::<u32>().write_unaligned(v);
                    dst = dst.wrapping_byte_add(4);
                }
                while i_var10 != 0 {
                    let v = src.wrapping_byte_add(4).cast::<u32>().read_unaligned();
                    dst.wrapping_byte_add(4).cast::<u32>().write_unaligned(v);
                    let v = src.cast::<u32>().read_unaligned();
                    dst.cast::<u32>().write_unaligned(v);
                    src = src.wrapping_byte_add(8);
                    dst = dst.wrapping_byte_add(8);
                    i_var10 = i_var10.wrapping_sub(1);
                }
                if u_var8 & 4 != 0 {
                    let v = src.cast::<u32>().read_unaligned();
                    src = src.wrapping_byte_add(4);
                    dst.cast::<u32>().write_unaligned(v);
                    dst = dst.wrapping_byte_add(4);
                }
                if u_var8 & 2 != 0 {
                    let v = src.cast::<u16>().read_unaligned();
                    src = src.wrapping_byte_add(2);
                    dst.cast::<u16>().write_unaligned(v);
                    dst = dst.wrapping_byte_add(2);
                }
                if u_var8 & 1 != 0 {
                    let v = src.read();
                    src = src.wrapping_byte_add(1);
                    dst.write(v);
                    dst = dst.wrapping_byte_add(1);
                }
                src = src.wrapping_offset(i_var5 as isize);
                dst = dst.wrapping_offset(i_var6 as isize);
                if i_var7 == 0 {
                    break;
                }
            }
        }
    }
}
