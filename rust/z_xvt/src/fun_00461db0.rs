// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db594: u32;
    static mut DAT_006fede0: u32;
    static mut DAT_0070acc8: u32;
    static mut DAT_00525534: *mut u32;
    static mut DAT_0052550c: u32;
    static mut DAT_0052552c: u32;
    static mut DAT_00527d18: *mut core::ffi::c_void;
    static mut _DAT_00527d1c: u32;
    static mut DAT_005f1d24: *mut core::ffi::c_void;
    static mut DAT_005f1d20: *mut core::ffi::c_void;
    static mut DAT_00527ce8: u32;
    static mut DAT_00527d00: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f570() -> *mut core::ffi::c_void;
}

unsafe fn clear(mut p: *mut u32, n: u32) {
    let mut c = n >> 2;
    // SAFETY: writes zeros into the game's own buffer, as the original does.
    unsafe {
        while c != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            c -= 1;
        }
        let mut b = p as *mut u8;
        let mut r = n & 3;
        while r != 0 {
            b.write(0);
            b = b.add(1);
            r -= 1;
        }
    }
}

unsafe fn udiv(a: u32, b: u32) -> (u32, u32) {
    let q: u32;
    let r: u32;
    // SAFETY: the machine's own div, faulting on zero as the original does.
    unsafe {
        core::arch::asm!("div {d:e}", d = in(reg) b, inout("eax") a => q, inout("edx") 0u32 => r, options(nomem, nostack));
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00461db0() {
    // SAFETY: all globals and the callee are the game's own memory.
    unsafe {
        let mut u_var2 = DAT_006db594;
        let mut u_var1: u32 = 0;
        if DAT_006db594 != 0 {
            loop {
                (&raw mut DAT_006fede0)
                    .add(u_var1 as usize)
                    .write(DAT_0070acc8.wrapping_mul(u_var1));
                u_var1 = u_var1.wrapping_add(1);
                if u_var1 >= u_var2 {
                    break;
                }
            }
        }
        DAT_00525534 = FUN_0048f570() as *mut u32;
        u_var1 = DAT_006db594;
        u_var2 = DAT_0052550c;
        'end: {
            'mode_b: {
                'mode_c: {
                    if DAT_0052552c == 0x13 {
                        break 'mode_b;
                    }
                    if DAT_0052552c == 0x101 {
                        let mut u_var4: u32 = 0;
                        let (q0, _) = udiv(DAT_0070acc8.wrapping_mul(DAT_006db594), DAT_0052550c);
                        if q0 != 0 {
                            loop {
                                u_var4 = u_var4.wrapping_add(1);
                                clear(DAT_00525534, u_var2);
                                let (q, _) = udiv(DAT_0070acc8.wrapping_mul(u_var1), u_var2);
                                if u_var4 >= q {
                                    break;
                                }
                            }
                        }
                        let (_, r) = udiv(DAT_0070acc8.wrapping_mul(u_var1), u_var2);
                        u_var2 = r;
                        if u_var2 != 0 {
                            break 'mode_c;
                        }
                    } else {
                        if DAT_0052552c != 0x112 {
                            break 'mode_b;
                        }
                        u_var2 = DAT_0070acc8.wrapping_mul(DAT_006db594);
                        break 'mode_c;
                    }
                    DAT_00527d18 = (&raw mut DAT_00527d00).cast();
                    _DAT_00527d1c = 0xc;
                    break 'end;
                }
                clear(DAT_00525534, u_var2);
                DAT_00527d18 = (&raw mut DAT_00527d00).cast();
                _DAT_00527d1c = 0xc;
                break 'end;
            }
            u_var1 = DAT_0070acc8.wrapping_mul(DAT_006db594);
            clear(DAT_00525534, u_var1);
            DAT_00527d18 = (&raw mut DAT_00527ce8).cast();
            _DAT_00527d1c = 10;
        }
        DAT_005f1d24 = (&raw mut DAT_0070acc8).cast();
        DAT_005f1d20 = (&raw mut DAT_006fede0).cast();
    }
}
