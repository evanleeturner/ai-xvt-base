// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6090: i32;
    static mut DAT_00aa6094: i32;
    static mut DAT_00aa6150: [i8; 0];
    static mut DAT_00aa6cfc: i32;
    static mut DAT_0052c818: *mut i8;
    static mut DAT_0052c80c: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d11e0() -> i32;
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004d6e30(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_004d1400() -> u32;
    fn FUN_004d48e0() -> i32;
    fn FUN_004d67e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16, param_5: i32);
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004d4970();
    fn FUN_004eecb0(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f02f0() {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are walked exactly as the original does.
    unsafe {
        let mut p7: *mut i32 = &raw mut DAT_00aa6090;
        let mut i9: i32 = 0;
        loop {
            let r = FUN_004d11e0();
            if r == p7.read() {
                break;
            }
            p7 = p7.add(2);
            i9 = i9.wrapping_add(1);
            if (p7 as usize) >= 0xaa60d0 {
                break;
            }
        }
        if (&raw mut DAT_00aa6094).offset((i9 as isize) * 2).read() != 1 {
            return;
        }
        let mut rect = [0i32; 4];
        let width: u32 = DAT_0052c818.byte_add(4).cast::<u32>().read_unaligned();
        rect[0] = 0;
        rect[1] = 0;
        rect[2] = width.wrapping_sub(1) as i32;
        rect[3] = (DAT_0052c80c as u32).wrapping_sub(1) as i32;
        let mut local_6c: [*mut i8; 2] = [core::ptr::null_mut(); 2];
        local_6c[0] = FUN_004ddc10(0x324);
        local_6c[1] = FUN_004ddc10(0x325);
        let local_70 = FUN_004d6e30(0xff, 0xff, 0xff);
        let local_78: u32 = DAT_0052c818.byte_add(4).cast::<u32>().read_unaligned() / 0x14;
        let local_74: u32 = (DAT_0052c818
            .byte_add(4)
            .cast::<u32>()
            .read_unaligned()
            .wrapping_add(local_78.wrapping_mul(0xFFFF_FFFE)))
            >> 2;
        let u_var3 = FUN_004d1400();
        DAT_00aa6cfc = FUN_004d48e0();
        let i9 = FUN_004d6e30(0, 0, 0) as i32;
        p7 = &raw mut DAT_00aa6090;
        FUN_004d67e0(rect.as_mut_ptr(), 0, 0, i9 as i16, -1);
        let mut local_8c: u32 = 0;
        let mut local_64 = [0i8; 100];
        loop {
            if p7.read() != 0 {
                rect[0] = (local_8c & 3).wrapping_mul(local_74).wrapping_add(local_78) as i32;
                rect[1] = (local_8c >> 2).wrapping_mul((DAT_0052c80c >> 1) as u32) as i32;
                rect[2] = (rect[0] as u32).wrapping_add(local_74) as i32;
                rect[3] = (rect[1] as u32).wrapping_add((DAT_0052c80c >> 1) as u32) as i32;
                let mut u6: u32 = 0;
                if u_var3 != 0 {
                    let base = (&raw mut DAT_00aa6150).cast::<i8>();
                    let mut p4 = base.byte_add(0xe);
                    loop {
                        let v = p4.cast::<i32>().read_unaligned();
                        if v == p7.read() {
                            let src = base.byte_add((u6 as usize).wrapping_mul(0x2a));
                            let mut n = 0usize;
                            while src.add(n).read() != 0 {
                                n += 1;
                            }
                            n += 1;
                            let mut k = 0usize;
                            while k < n {
                                local_64.as_mut_ptr().add(k).write(src.add(k).read());
                                k += 1;
                            }
                            break;
                        }
                        p4 = p4.byte_add(0x2a);
                        u6 = u6.wrapping_add(1);
                        if u6 >= u_var3 {
                            break;
                        }
                    }
                }
                let idx = p7.add(1).read();
                let src = local_6c.as_ptr().offset(idx as isize).read();
                let mut n = 0usize;
                while src.add(n).read() != 0 {
                    n += 1;
                }
                n += 1;
                let mut e = 0usize;
                while local_64.as_ptr().add(e).read() != 0 {
                    e += 1;
                }
                let dst = local_64.as_mut_ptr().add(e);
                let mut k = 0usize;
                while k < n {
                    dst.add(k).write(src.add(k).read());
                    k += 1;
                }
                FUN_004db640(0xc, local_64.as_mut_ptr(), rect.as_mut_ptr(), local_70);
            }
            p7 = p7.add(2);
            local_8c = local_8c.wrapping_add(1);
            if (p7 as usize) > 0xaa60cf {
                FUN_004d4970();
                if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
                    return;
                }
                FUN_004eecb0(
                    0,
                    0,
                    DAT_0052c818.byte_add(4).cast::<i32>().read_unaligned(),
                    DAT_0052c80c,
                );
                return;
            }
        }
    }
}
