// SPDX-License-Identifier: MIT

#[inline(always)]
unsafe fn c1(s: &mut *mut u8, d: &mut *mut u8) {
    // SAFETY: caller-provided buffers, accessed as the original does.
    unsafe {
        d.write(s.read());
        *s = s.wrapping_byte_add(1);
        *d = d.wrapping_byte_add(1);
    }
}

#[inline(always)]
unsafe fn c2(s: &mut *mut u8, d: &mut *mut u8) {
    // SAFETY: caller-provided buffers, accessed as the original does.
    unsafe {
        d.cast::<u16>()
            .write_unaligned(s.cast::<u16>().read_unaligned());
        *s = s.wrapping_byte_add(2);
        *d = d.wrapping_byte_add(2);
    }
}

#[inline(always)]
unsafe fn c4(s: &mut *mut u8, d: &mut *mut u8) {
    // SAFETY: caller-provided buffers, accessed as the original does.
    unsafe {
        d.cast::<u32>()
            .write_unaligned(s.cast::<u32>().read_unaligned());
        *s = s.wrapping_byte_add(4);
        *d = d.wrapping_byte_add(4);
    }
}

#[inline(always)]
unsafe fn c8(s: &mut *mut u8, d: &mut *mut u8) {
    // SAFETY: caller-provided buffers, accessed as the original does.
    unsafe {
        d.wrapping_byte_add(4)
            .cast::<u32>()
            .write_unaligned(s.wrapping_byte_add(4).cast::<u32>().read_unaligned());
        d.cast::<u32>()
            .write_unaligned(s.cast::<u32>().read_unaligned());
        *s = s.wrapping_byte_add(8);
        *d = d.wrapping_byte_add(8);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051d3b0(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    let mut param_5 = param_5;
    let mut s = param_3 as *mut u8;
    let mut u_var6: u32 = (param_7.wrapping_neg() as u32) & 7;
    if (param_4 as i32) < (u_var6 as i32) {
        u_var6 = param_4;
    }
    let i_var4: i32 = (param_4.wrapping_sub(u_var6) as i32) >> 3;
    let u_var5: u32 = param_4
        .wrapping_add(i_var4.wrapping_mul(-8) as u32)
        .wrapping_sub(u_var6);
    let mut d = param_8
        .wrapping_mul(param_2)
        .wrapping_add(param_7)
        .wrapping_add(param_1) as u32 as usize as *mut u8;
    let src_adv = (param_6 as u32).wrapping_sub(param_4) as i32 as isize;
    let dst_adv = (param_2 as u32).wrapping_sub(param_4) as i32 as isize;

    // SAFETY: the game's own buffers, copied exactly as the original does.
    unsafe {
        if u_var6 == 0 && u_var5 == 0 {
            if param_5 != 0 {
                loop {
                    param_5 = param_5.wrapping_sub(1);
                    let mut n = i_var4;
                    while n != 0 {
                        c8(&mut s, &mut d);
                        n = n.wrapping_sub(1);
                    }
                    s = s.wrapping_byte_offset(src_adv);
                    d = d.wrapping_byte_offset(dst_adv);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
        } else if param_5 != 0 {
            loop {
                param_5 = param_5.wrapping_sub(1);
                if u_var6 & 1 != 0 {
                    c1(&mut s, &mut d);
                }
                if u_var6 & 2 != 0 {
                    c2(&mut s, &mut d);
                }
                let mut n = i_var4;
                if u_var6 & 4 != 0 {
                    c4(&mut s, &mut d);
                }
                while n != 0 {
                    c8(&mut s, &mut d);
                    n = n.wrapping_sub(1);
                }
                if u_var5 & 4 != 0 {
                    c4(&mut s, &mut d);
                }
                if u_var5 & 2 != 0 {
                    c2(&mut s, &mut d);
                }
                if u_var5 & 1 != 0 {
                    c1(&mut s, &mut d);
                }
                s = s.wrapping_byte_offset(src_adv);
                d = d.wrapping_byte_offset(dst_adv);
                if param_5 == 0 {
                    break;
                }
            }
        }
    }
}
