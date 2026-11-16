// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994c4: *mut i32;
    static mut DAT_00999548: i32;
    static mut DAT_0052702c: i32;
    static mut DAT_00527024: i32;
    static mut DAT_005599a8: [u32; 0];
    static mut DAT_005599ac: [u32; 0];
    static mut DAT_005599b0: [u32; 0];
    static mut DAT_005599b4: [u8; 0];
    static mut DAT_005599b5: [u8; 0];
    static mut DAT_005599b6: [u8; 0];
    static mut DAT_005599b7: [u8; 0];
    static mut DAT_005599b8: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046fad0() -> i32;
    fn FUN_00470060();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046dd80(
    param_1: i32,
    param_2: u8,
    param_3: u8,
    param_4: *mut u32,
    param_5: u32,
) -> u8 {
    let mut param_4 = param_4;
    let mut local_404: i32 = 0;
    let mut local_400 = [0u8; 1021];
    // SAFETY: the globals and callees are the game's own; param_4 points at
    // the caller's data, read and copied where the original does; the local
    // buffer is the original's frame buffer.
    unsafe {
        if DAT_009994c4.is_null() {
            return 1;
        }
        let u_var3 = param_4.read_unaligned();
        let b_var1: u8 = if u_var3 < 0x3c || u_var3 > 0x3f { 1 } else { 0 };
        let base = local_400.as_mut_ptr();
        let mut p5: *mut u8 = base.add(3);
        let mut local_40c: i32 = 3;
        let tmp_w: u16 = ((u_var3 as u16) & 0x7f) | (((param_3 as u16) & 0x7f) << 8) | 0x80;
        base.cast::<u16>().write_unaligned(tmp_w);
        *base.add(2) = param_2;
        if b_var1 != 0 && FUN_0046fad0() == 0 {
            let tmp_s: i16 = ((param_5 as i16 as i32).wrapping_sub(4)) as i16;
            base.add(3).cast::<i16>().write_unaligned(tmp_s);
            p5 = base.add(5);
            local_40c = 5;
        }
        let mut p7: *mut u8 = p5;
        let mut p6: *const u32 = param_4;
        let mut cnt = param_5.wrapping_sub(4) >> 2;
        loop {
            p6 = p6.wrapping_add(1);
            if cnt == 0 {
                break;
            }
            p7.cast::<u32>().write_unaligned(p6.read_unaligned());
            p7 = p7.wrapping_add(4);
            cnt = cnt.wrapping_sub(1);
        }
        let mut cnt = param_5.wrapping_sub(4) & 3;
        while cnt != 0 {
            *p7 = p6.read_unaligned() as u8;
            p6 = p6.cast::<u8>().wrapping_add(1).cast::<u32>();
            p7 = p7.wrapping_add(1);
            cnt = cnt.wrapping_sub(1);
        }
        local_40c = local_40c.wrapping_add(param_5.wrapping_sub(4) as i32);
        if b_var1 != 0 {
            local_40c = local_40c.wrapping_add(1);
            *p5.wrapping_add(param_5.wrapping_sub(4) as usize) = 0x39;
        }
        if DAT_00999548 == param_1 {
            if 0x3ff < DAT_0052702c {
                FUN_00470060();
            }
            if DAT_0052702c < 0x400 {
                let mut d: *mut u32 = (&raw mut DAT_005599b8)
                    .cast::<u32>()
                    .wrapping_add((DAT_00527024 as u32).wrapping_mul(0x84) as usize);
                let mut cnt = param_5 >> 2;
                while cnt != 0 {
                    d.write_unaligned(param_4.read_unaligned());
                    param_4 = param_4.wrapping_add(1);
                    d = d.wrapping_add(1);
                    cnt = cnt.wrapping_sub(1);
                }
                let mut cnt = param_5 & 3;
                while cnt != 0 {
                    *d.cast::<u8>() = param_4.read_unaligned() as u8;
                    param_4 = param_4.cast::<u8>().wrapping_add(1).cast::<u32>();
                    d = d.cast::<u8>().wrapping_add(1).cast::<u32>();
                    cnt = cnt.wrapping_sub(1);
                }
                let i_var2 = DAT_00527024;
                let i_var4 = (DAT_00527024 as u32).wrapping_mul(0x210) as usize;
                let w1 = (DAT_00527024 as u32).wrapping_mul(0x84) as usize;
                (&raw mut DAT_005599a8)
                    .cast::<u32>()
                    .wrapping_add(w1)
                    .write(DAT_00999548 as u32);
                let w2 = (i_var2 as u32).wrapping_mul(0x84) as usize;
                (&raw mut DAT_005599ac)
                    .cast::<u32>()
                    .wrapping_add(w2)
                    .write(param_5);
                (&raw mut DAT_005599b5)
                    .cast::<u8>()
                    .wrapping_add(i_var4)
                    .write(param_2);
                DAT_0052702c = DAT_0052702c.wrapping_add(1);
                (&raw mut DAT_005599b6)
                    .cast::<u8>()
                    .wrapping_add(i_var4)
                    .write(param_3);
                (&raw mut DAT_005599b7)
                    .cast::<u8>()
                    .wrapping_add(i_var4)
                    .write(1);
                (&raw mut DAT_005599b0)
                    .cast::<u32>()
                    .wrapping_add((i_var2 as u32).wrapping_mul(0x84) as usize)
                    .write(0);
                (&raw mut DAT_005599b4)
                    .cast::<u8>()
                    .wrapping_add(i_var4)
                    .write(0);
                DAT_00527024 = DAT_00527024.wrapping_add(1);
                if 0x3ff < DAT_00527024 {
                    DAT_00527024 = 0;
                }
            }
        } else {
            let obj = DAT_009994c4;
            let vt = obj.read() as usize;
            let f = ((vt + 0x68) as *const *const c_void).read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, *mut c_void, i32) -> i32 =
                core::mem::transmute(f);
            local_404 = f(
                obj,
                DAT_00999548,
                param_1,
                0,
                base.cast::<c_void>(),
                local_40c,
            );
        }
        (local_404 == 0) as u8
    }
}
