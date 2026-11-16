// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7b0e: i8;
    static mut DAT_00aa7b24: i8;
    static mut DAT_00aa7f9e: i8;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
    static mut DAT_00aa7b1b: [u8; 4];
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b2e: [u8; 4];
    static mut DAT_00aa7b32: [u8; 4];
    static mut DAT_00aa7b36: [u8; 4];
    static mut DAT_00aa7b3a: [u8; 4];
    static mut DAT_00aa7b92: [u8; 4];
    static mut DAT_00aa7b96: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d37e0() -> i32;
    fn FUN_004dca20();
    fn FUN_004d4c10();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004d6ca0(extraout_ECX: i32);
}

unsafe fn slot(obj: u32, off: u32) -> usize {
    // SAFETY: obj is a COM-style object pointer owned by the game.
    unsafe {
        let vt = (obj as usize as *const u32).read_unaligned();
        (vt.wrapping_add(off) as usize as *const u32).read_unaligned() as usize
    }
}

macro_rules! rd {
    ($g:ident) => {
        (&raw mut $g).cast::<u32>().read_unaligned()
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d49a0() {
    let mut local_80: [i32; 32] = [0; 32];
    // SAFETY: all globals, callees and COM objects are the game's own memory;
    // pointers are read and written where the original does.
    unsafe {
        let mut tmp_a: u32 = rd!(DAT_00aa7b2e);
        if tmp_a != 0 {
            if DAT_00aa7b0d != 0 {
                FUN_004d4970();
            }
            local_80[3] = 0x280;
            local_80[4] = 0x1e0;
            local_80[1] = 0;
            local_80[2] = 0;
            if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
                loop {
                    tmp_a = rd!(DAT_00aa7b2e);
                    let f: unsafe extern "stdcall" fn(u32, *mut i32) -> i32 =
                        core::mem::transmute(slot(tmp_a, 0x44));
                    let r = f(tmp_a, local_80.as_mut_ptr());
                    if r != 0 {
                        break;
                    }
                    if local_80[0] != 0 {
                        break;
                    }
                }
                let v = rd!(DAT_00aa7b1f) as i32;
                if DAT_00aa7f9e == 1 && v == 8 {
                    FUN_004d6ca0(0);
                    DAT_00aa7f9e = 0;
                }
                loop {
                    tmp_a = rd!(DAT_00aa7b36);
                    let tmp_b = rd!(DAT_00aa7b3a);
                    let f: unsafe extern "stdcall" fn(u32, u32, i32) -> i32 =
                        core::mem::transmute(slot(tmp_a, 0x2c));
                    let r = f(tmp_a, tmp_b, 1);
                    if r == 0 {
                        break;
                    }
                    if r == -0x7789fe3e {
                        if FUN_004d37e0() != 0 {
                            break;
                        }
                        continue;
                    }
                    if r == -0x7789fde4 {
                        continue;
                    }
                    break;
                }
            } else {
                loop {
                    tmp_a = rd!(DAT_00aa7b36);
                    let tmp_b = rd!(DAT_00aa7b3a);
                    let f: unsafe extern "stdcall" fn(u32, i32, i32, u32, *mut i32, i32) -> i32 =
                        core::mem::transmute(slot(tmp_a, 0x1c));
                    let r = f(tmp_a, 0, 0, tmp_b, local_80.as_mut_ptr().add(1), 0);
                    if r == 0 {
                        break;
                    }
                    if r == -0x7789fe3e {
                        if FUN_004d37e0() != 0 {
                            return;
                        }
                    } else if r != -0x7789fde4 {
                        return;
                    }
                }
            }
            if DAT_00aa7b24 != 0 {
                let v = rd!(DAT_00aa7b96);
                if v != 0 {
                    (&raw mut DAT_00aa7b96).cast::<u32>().write_unaligned(0);
                    let mut src = rd!(DAT_00aa7b92);
                    if src != 0 {
                        tmp_a = rd!(DAT_00aa7b32);
                        if tmp_a != 0 {
                            for i in 5..32 {
                                local_80[i] = 0;
                            }
                            local_80[5] = 0x6c;
                            loop {
                                tmp_a = rd!(DAT_00aa7b32);
                                let f: unsafe extern "stdcall" fn(
                                    u32,
                                    i32,
                                    *mut i32,
                                    i32,
                                    i32,
                                )
                                    -> i32 = core::mem::transmute(slot(tmp_a, 100));
                                let r = f(tmp_a, 0, local_80.as_mut_ptr().add(5), 0, 0);
                                if r == 0 {
                                    break;
                                }
                                if r == -0x7789fe3e {
                                    tmp_a = rd!(DAT_00aa7b32);
                                    let g: unsafe extern "stdcall" fn(u32) -> i32 =
                                        core::mem::transmute(slot(tmp_a, 0x6c));
                                    g(tmp_a);
                                    continue;
                                }
                                if r == -0x7789fde4 {
                                    continue;
                                }
                                break;
                            }
                            if local_80[14] != 0 {
                                src = rd!(DAT_00aa7b92);
                                let w = rd!(DAT_00aa7b1b);
                                let mut n =
                                    (w.wrapping_mul(0xf) & 0x7ffffff).wrapping_shl(3) as i32;
                                while n != 0 {
                                    let dst = local_80[14] as u32;
                                    let val = (src as usize as *const u32).read_unaligned();
                                    (dst as usize as *mut u32).write_unaligned(val);
                                    src = src.wrapping_add(4);
                                    local_80[14] = dst.wrapping_add(4) as i32;
                                    n = n.wrapping_sub(1);
                                }
                                tmp_a = rd!(DAT_00aa7b32);
                                let h: unsafe extern "stdcall" fn(u32, i32) -> i32 =
                                    core::mem::transmute(slot(tmp_a, 0x80));
                                h(tmp_a, 0);
                            }
                        }
                    }
                }
                if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
                    loop {
                        tmp_a = rd!(DAT_00aa7b3a);
                        let tmp_b = rd!(DAT_00aa7b32);
                        let f: unsafe extern "stdcall" fn(
                            u32,
                            i32,
                            i32,
                            u32,
                            *mut i32,
                            i32,
                        ) -> i32 = core::mem::transmute(slot(tmp_a, 0x1c));
                        let r = f(tmp_a, 0, 0, tmp_b, local_80.as_mut_ptr().add(1), 0);
                        if r == 0 {
                            break;
                        }
                        if r == -0x7789fe3e {
                            if FUN_004d37e0() != 0 {
                                return;
                            }
                        } else if r != -0x7789fde4 {
                            return;
                        }
                    }
                } else {
                    FUN_004dca20();
                }
            }
            if DAT_00aa7b0e != 0 {
                FUN_004d4c10();
            }
        }
    }
}
