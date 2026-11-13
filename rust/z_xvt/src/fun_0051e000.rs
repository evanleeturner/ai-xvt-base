// SPDX-License-Identifier: MIT

#[inline(always)]
fn expand_hi(x: u32) -> i32 {
    let u = x >> 16;
    (u & 0xff)
        .wrapping_add((u & 0xffff_ff00).wrapping_mul(0x10000))
        .wrapping_add(u.wrapping_mul(0x100)) as i32
}

#[inline(always)]
fn expand_lo(x: u32) -> i32 {
    (x & 0xff00)
        .wrapping_mul(0x10000)
        .wrapping_add((x & 0xffff).wrapping_mul(0x100))
        .wrapping_add(x & 0xff) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051e000(
    param_1: *mut u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut i32,
    param_6: i32,
) {
    let mut param_1 = param_1;
    let mut param_3 = param_3;
    let mut param_5 = param_5;
    let i_var4 = param_2 >> 2;
    let u_var12 = param_2.wrapping_sub(i_var4.wrapping_mul(4)) as u32;
    let i_var2 = param_6.wrapping_sub(param_2).wrapping_mul(2);
    let mut pi_var10 = param_5.wrapping_byte_offset(param_6 as isize);
    let src_adj = param_4.wrapping_sub(param_2) as isize;

    // SAFETY: the pointers are the caller's source and destination buffers,
    // read and written exactly where the original does.
    unsafe {
        if u_var12 == 0 {
            while param_3 != 0 {
                param_3 = param_3.wrapping_sub(1);
                let mut local_18 = i_var4;
                while local_18 != 0 {
                    local_18 = local_18.wrapping_sub(1);
                    let v = param_1.read_unaligned();
                    param_1 = param_1.wrapping_add(1);
                    let a = expand_hi(v);
                    param_5.write_unaligned(a);
                    pi_var10.write_unaligned(a);
                    let b = expand_lo(v);
                    param_5.wrapping_add(1).write_unaligned(b);
                    pi_var10.wrapping_add(1).write_unaligned(b);
                    param_5 = param_5.wrapping_add(2);
                    pi_var10 = pi_var10.wrapping_add(2);
                }
                param_1 = param_1.wrapping_byte_offset(src_adj);
                param_5 = param_5.wrapping_byte_offset(i_var2 as isize);
                pi_var10 = pi_var10.wrapping_byte_offset(i_var2 as isize);
            }
        } else if param_3 != 0 {
            loop {
                param_3 = param_3.wrapping_sub(1);
                let mut local_18 = i_var4;
                while local_18 != 0 {
                    local_18 = local_18.wrapping_sub(1);
                    let v = param_1.read_unaligned();
                    param_1 = param_1.wrapping_add(1);
                    let a = expand_hi(v);
                    param_5.write_unaligned(a);
                    pi_var10.write_unaligned(a);
                    let b = expand_lo(v);
                    param_5.wrapping_add(1).write_unaligned(b);
                    pi_var10.wrapping_add(1).write_unaligned(b);
                    param_5 = param_5.wrapping_add(2);
                    pi_var10 = pi_var10.wrapping_add(2);
                }
                let mut pi_var8 = param_5;
                let mut pi_var11 = pi_var10;
                let mut pu_var13 = param_1;
                if (u_var12 & 2) != 0 {
                    pu_var13 = param_1.wrapping_byte_offset(2);
                    let u1 = (param_1 as *const u16).read_unaligned() as u32;
                    pi_var8 = param_5.wrapping_add(1);
                    let i_var6 = (u1 & 0xff)
                        .wrapping_add((u1 & 0xffff_ff00).wrapping_mul(0x10000))
                        .wrapping_add(u1.wrapping_mul(0x100))
                        as i32;
                    pi_var11 = pi_var10.wrapping_add(1);
                    param_5.write_unaligned(i_var6);
                    pi_var10.write_unaligned(i_var6);
                }
                let mut pi_var9 = pi_var8;
                pi_var10 = pi_var11;
                let mut pu_var14 = pu_var13;
                if (u_var12 & 1) != 0 {
                    pu_var14 = pu_var13.wrapping_byte_offset(1);
                    pi_var9 = pi_var8.wrapping_byte_offset(2);
                    pi_var10 = pi_var11.wrapping_byte_offset(2);
                    let b = (pu_var13 as *const u8).read() as u16;
                    let s = b.wrapping_add(b.wrapping_mul(0x100)) as i16;
                    (pi_var8 as *mut i16).write_unaligned(s);
                    (pi_var11 as *mut i16).write_unaligned(s);
                }
                param_1 = pu_var14.wrapping_byte_offset(src_adj);
                param_5 = pi_var9.wrapping_byte_offset(i_var2 as isize);
                pi_var10 = pi_var10.wrapping_byte_offset(i_var2 as isize);
                if param_3 == 0 {
                    break;
                }
            }
        }
    }
}
