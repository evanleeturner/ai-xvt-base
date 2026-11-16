// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6d00: u32;
    static mut DAT_00aa6d04: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn _free(p: *mut core::ffi::c_void);
    fn _realloc(p: *mut core::ffi::c_void, size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b75b0(param_1: *mut i32) -> u32 {
    // SAFETY: param_1 points at the caller's image descriptor; the globals,
    // allocator and row buffers are the game's own memory, accessed as the original does.
    unsafe {
        let i_var2 = param_1.read();
        let i_var3 = param_1.add(1).read();
        let memory = _malloc((i_var2 as u32).wrapping_mul(i_var3 as u32)) as *mut u8;
        if memory.is_null() {
            param_1.add(2).write(0);
            return 0;
        }
        let mut local_1c: u32 = 0;
        let mut local_18: *mut u8 = param_1.add(4).read() as usize as *mut u8;
        let mut i_var7: i32 = 0x280;
        if param_1.read() < 0x281 {
            i_var7 = param_1.read();
        }
        let mut local_4: i32 = 0;
        let mut local_8: *mut u8 = memory;
        let tbl: *mut u8 = &raw mut DAT_00aa6d04 as *mut u8;

        'fin: {
            if param_1.add(1).read() < 1 {
                break 'fin;
            }
            loop {
                let mut i_var14: i32 = 0;
                let mut i_var15: i32 = 0;
                let mut b_var13: u8 = local_18.read();
                let mut local_24: *mut u8 = tbl;
                tbl.write(0);
                let mut pb5: *mut u8 = local_18;
                let mut pb8: *mut u8 = tbl;
                if 0 < i_var7 {
                    loop {
                        let mut i_var12: i32 = 0;
                        loop {
                            if i_var7 <= i_var15 || pb5.offset(i_var12 as isize).read() != b_var13 {
                                break;
                            }
                            i_var15 = i_var15.wrapping_add(1);
                            i_var12 = i_var12.wrapping_add(1);
                            if i_var12 >= 0x3f {
                                break;
                            }
                        }
                        let b_var11 = i_var12 as u8;
                        let mut pb9: *mut u8;
                        if b_var13 == 0 {
                            i_var14 = i_var14.wrapping_add(1);
                            pb8.write(b_var11 | 0x40);
                            pb9 = pb8.add(1);
                            local_24 = pb8;
                        } else if i_var12 < 3 {
                            if local_24.read() & 0x80 == 0 {
                                i_var14 = i_var14.wrapping_add(1);
                                pb8.write(b_var11 | 0x80);
                                pb9 = pb8.add(1);
                                local_24 = pb8;
                                let mut i_var17: i32 = 0;
                                if 0 < i_var12 {
                                    loop {
                                        let v = pb5.offset(i_var17 as isize).read();
                                        i_var17 = i_var17.wrapping_add(1);
                                        pb8.offset(i_var17 as isize).write(v);
                                        if i_var17 >= i_var12 {
                                            break;
                                        }
                                    }
                                }
                            } else {
                                b_var13 = (local_24.read() & 0x7f).wrapping_add(b_var11);
                                if b_var13 < 0x80 {
                                    local_24.write(b_var13 | 0x80);
                                    pb9 = pb8;
                                    let mut i_var17: i32 = 0;
                                    if 0 < i_var12 {
                                        loop {
                                            let i_var18 = i_var17.wrapping_add(1);
                                            pb8.offset(i_var17 as isize)
                                                .write(pb5.offset(i_var17 as isize).read());
                                            i_var17 = i_var18;
                                            if i_var18 >= i_var12 {
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    i_var14 = i_var14.wrapping_add(1);
                                    pb8.write(b_var11 | 0x80);
                                    pb9 = pb8.add(1);
                                    local_24 = pb8;
                                    let mut i_var17: i32 = 0;
                                    if 0 < i_var12 {
                                        loop {
                                            let v = pb5.offset(i_var17 as isize).read();
                                            i_var17 = i_var17.wrapping_add(1);
                                            pb8.offset(i_var17 as isize).write(v);
                                            if i_var17 >= i_var12 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            pb9 = pb9.offset(i_var12 as isize);
                            i_var14 = i_var14.wrapping_add(i_var12);
                        } else {
                            pb8.write(b_var11);
                            i_var14 = i_var14.wrapping_add(2);
                            pb8.add(1).write(b_var13);
                            pb9 = pb8.add(2);
                            local_24 = pb8;
                        }
                        if i_var7 <= i_var15 {
                            pb9.write(0x80);
                            DAT_00aa6d00 = i_var14.wrapping_add(5) as u32;
                            break;
                        }
                        pb5 = pb5.offset(i_var12 as isize);
                        b_var13 = pb5.read();
                        pb8 = pb9;
                    }
                } else {
                    tbl.write(0x80);
                    DAT_00aa6d00 = 5;
                }
                let u_var4 = DAT_00aa6d00;
                local_1c = local_1c.wrapping_add(DAT_00aa6d00);
                if ((i_var2 as u32).wrapping_mul(i_var3 as u32) as i32) < (local_1c as i32) {
                    _free(memory as *mut core::ffi::c_void);
                    local_1c = (param_1.read() as u32)
                        .wrapping_mul(param_1.add(1).read() as u32)
                        .wrapping_add(1);
                    break 'fin;
                }
                let mut src: *const u8 = (&raw mut DAT_00aa6d00) as *const u8;
                let mut dst: *mut u8 = local_8;
                let mut n = u_var4;
                while n != 0 {
                    dst.write(src.read());
                    src = src.add(1);
                    dst = dst.add(1);
                    n -= 1;
                }
                local_8 = local_8.add(u_var4 as usize);
                local_18 = local_18.offset(param_1.read() as isize);
                local_4 = local_4.wrapping_add(1);
                if param_1.add(1).read() <= local_4 {
                    break 'fin;
                }
            }
        }

        // LAB_004b77a9
        if ((param_1.read() as u32).wrapping_mul(param_1.add(1).read() as u32) as i32)
            < (local_1c as i32)
        {
            return 0;
        }
        let pvvar6 = _realloc(memory as *mut core::ffi::c_void, local_1c);
        if !memory.is_null() {
            _free(param_1.add(4).read() as usize as *mut core::ffi::c_void);
            param_1.add(4).write(pvvar6 as usize as i32);
            param_1.add(2).write(1);
            param_1.add(3).write(local_1c as i32);
            return 1;
        }
        _free(core::ptr::null_mut());
        param_1.add(2).write(0);
        0
    }
}
