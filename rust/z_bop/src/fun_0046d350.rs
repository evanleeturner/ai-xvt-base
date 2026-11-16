// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009997d0: i32;
    static mut DAT_009997d4: [u8; 0x200];
    static mut DAT_009999d4: u32;
    static mut DAT_009999d8: i32;
    static mut DAT_009999dc: i32;
    static mut DAT_009999e0: [u8; 0x200];
    static mut DAT_00999be0: u32;
    static mut DAT_00999be4: i32;
    static mut DAT_009994f0: i32;
    static mut DAT_00999548: i32;
    static mut DAT_009994c4: *mut i32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_00bb736f: i8;

    static mut DAT_00999bf0: [u32; 0];
    static mut DAT_00999bf8: [u32; 0];
    static mut DAT_00999c00: [u32; 0];
    static mut DAT_00999c04: [i32; 0];
    static mut DAT_00999c08: [u8; 0];
    static mut DAT_00999e08: [u32; 0];

    static mut DAT_005ee1b8: [i32; 0];
    static mut DAT_005ee1bc: [i32; 0];
    static mut DAT_005ee1c0: [u8; 0];
    static mut DAT_005ee1c4: [u8; 0];
    static mut DAT_005ee1c5: [u8; 0];
    static mut DAT_005ee1c6: [u8; 0];
    static mut DAT_005ee1c8: [u8; 0];
    static mut DAT_00527020: i32;

    static mut DAT_005dd9b8: [i32; 0];
    static mut DAT_005dd9bc: [i32; 0];
    static mut DAT_005dd9c0: [u8; 0];
    static mut DAT_005dd9c4: [u8; 0];
    static mut DAT_005dd9c5: [u8; 0];
    static mut DAT_005dd9c6: [u8; 0];
    static mut DAT_005dd9c8: [u8; 0];
    static mut DAT_0052701c: i32;

    static mut DAT_005599a8: [i32; 0];
    static mut DAT_005599ac: [i32; 0];
    static mut DAT_005599b0: [i32; 0];
    static mut DAT_005599b4: [u8; 0];
    static mut DAT_005599b5: [u8; 0];
    static mut DAT_005599b6: [u8; 0];
    static mut DAT_005599b7: [u8; 0];
    static mut DAT_005599b8: [u8; 0];
    static mut DAT_00527024: i32;
    static mut DAT_0052702c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046fad0() -> i32;
    fn FUN_0046fee0(param_1: i32) -> u32;
    fn FUN_00470060();
}

macro_rules! at {
    ($sym:ident, $ty:ty, $idx:expr) => {
        (&raw mut $sym).cast::<$ty>().add(($idx) as usize)
    };
}

unsafe fn cp(dst: *mut u8, src: *const u8, n: u32) {
    // SAFETY: caller guarantees both regions are valid for n bytes.
    unsafe { core::ptr::copy_nonoverlapping(src, dst, n as usize) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: u32) -> u8 {
    let mut local_404: i32 = 0;
    if (param_3 as i32) < 4 {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // the caller's packet of param_3 bytes; all accesses mirror the original.
    unsafe {
        let mut local_400: [u16; 0x200] = [0; 0x200];
        let uvar1 = param_2.read_unaligned();
        let bvar3 = (uvar1 as u8) & 0x7f;
        let mut uvar10: u16 = bvar3 as u16;
        let mut bvar15: u8 = if uvar1 < 0x3c || 0x3f < uvar1 { 1 } else { 0 };
        let svar2 = param_3 as i16;
        let mut local_41c: *mut u8;
        let mut local_420: i32;
        let src = (param_2 as *const u8).add(4);

        let mode: u8 = if uvar1 == 1 && DAT_00bb736f == 1 {
            1
        } else if param_1 == 0 {
            2
        } else if DAT_009994f0 == param_1 {
            3
        } else {
            4
        };

        if mode != 4 {
            let cnt: *mut i32;
            let buf: *mut u8;
            let lenp: *mut u32;
            let flagp: *mut i32;
            if mode == 1 {
                uvar10 = (((DAT_009999dc as u16) & 0x7f) << 8) | 0x8081;
                cnt = &raw mut DAT_009999dc;
                buf = (&raw mut DAT_009999e0).cast::<u8>();
                lenp = &raw mut DAT_00999be0;
                flagp = &raw mut DAT_00999be4;
            } else if mode == 2 {
                uvar10 = uvar10 | (((DAT_009997d0 as u16) & 0x7f) << 8);
                cnt = &raw mut DAT_009997d0;
                buf = (&raw mut DAT_009997d4).cast::<u8>();
                lenp = &raw mut DAT_009999d4;
                flagp = &raw mut DAT_009999d8;
            } else {
                uvar10 = uvar10 | (((DAT_009999dc as u16) & 0x7f) << 8) | 0x8080;
                cnt = &raw mut DAT_009999dc;
                buf = (&raw mut DAT_009999e0).cast::<u8>();
                lenp = &raw mut DAT_00999be0;
                flagp = &raw mut DAT_00999be4;
            }
            *cnt = (*cnt).wrapping_add(1);
            if 0x7f < *cnt {
                *cnt = 0;
            }
            local_41c = local_400.as_mut_ptr().add(1).cast::<u8>();
            local_420 = 2;
            local_400[0] = uvar10;
            if bvar15 != 0 && FUN_0046fad0() == 0 {
                local_400[1] = (svar2 as i32).wrapping_sub(4) as u16;
                local_41c = local_400.as_mut_ptr().add(2).cast::<u8>();
                local_420 = 4;
            }
            let n = param_3.wrapping_sub(4);
            cp(local_41c, src, n);
            let tail = local_41c.add(n as usize);
            local_420 = (n as i32).wrapping_add(local_420);
            if bvar15 != 0 {
                if *flagp == 0 {
                    let l = *lenp;
                    cp(tail, buf, l);
                    local_420 = local_420.wrapping_add(*lenp as i32);
                } else {
                    *tail = 0x39;
                    *flagp = 0;
                    local_420 = local_420.wrapping_add(1);
                }
            }
            *buf = bvar3;
            cp(buf.add(1), src, n);
            *lenp = param_3.wrapping_sub(3);
        } else {
            let uvar8 = FUN_0046fee0(param_1);
            if uvar8 < DAT_0099f6e0 && uvar8 < 0x28 {
                let p = at!(DAT_00999c04, i32, uvar8.wrapping_mul(0x8e));
                let ivar4 = p.read();
                let ivar5 = ivar4.wrapping_add(1);
                p.write(ivar5);
                uvar10 = uvar10 | (((ivar4 as u16) & 0x7f) << 8);
                if 0x7f < ivar5 {
                    p.write(0);
                }
            }
            uvar10 |= 0x8000;
            local_41c = local_400.as_mut_ptr().add(1).cast::<u8>();
            local_420 = 2;
            local_400[0] = uvar10;
            if bvar15 != 0 && FUN_0046fad0() == 0 {
                local_400[1] = (svar2 as i32).wrapping_sub(4) as u16;
                local_41c = local_400.as_mut_ptr().add(2).cast::<u8>();
                local_420 = 4;
            }
            let n = param_3.wrapping_sub(4);
            cp(local_41c, src, n);
            local_420 = (n as i32).wrapping_add(local_420);
            let slot = at!(DAT_00999c08, u8, uvar8.wrapping_mul(0x238));
            if bvar15 != 0 {
                let l = at!(DAT_00999e08, u32, uvar8.wrapping_mul(0x8e)).read();
                cp(local_41c.add(n as usize), slot, l);
                local_420 = local_420
                    .wrapping_add(at!(DAT_00999e08, u32, uvar8.wrapping_mul(0x8e)).read() as i32);
            }
            slot.write(bvar3);
            cp(slot.add(1), src, n);
            at!(DAT_00999e08, u32, uvar8.wrapping_mul(0x8e)).write(param_3.wrapping_sub(3));
        }

        if (uvar1 != 1 || DAT_00bb736f != 1) && DAT_00999548 != param_1 {
            let b3 = (local_400[0] >> 8) as u8;
            if uvar1 == 2 {
                let idx = DAT_00527020 as u32;
                cp(
                    at!(DAT_005ee1c8, u8, idx.wrapping_mul(0x210)),
                    param_2 as *const u8,
                    param_3,
                );
                let ivar4 = DAT_00527020 as u32;
                let ivar5 = (DAT_00527020 as u32).wrapping_mul(0x210);
                at!(DAT_005ee1b8, i32, (DAT_00527020 as u32).wrapping_mul(0x84)).write(param_1);
                at!(DAT_005ee1bc, i32, ivar4.wrapping_mul(0x84)).write(param_3 as i32);
                at!(DAT_005ee1c0, u8, ivar5)
                    .cast::<i32>()
                    .write_unaligned(0);
                at!(DAT_005ee1c4, u8, ivar5).write(0);
                at!(DAT_005ee1c5, u8, ivar5).write(0);
                at!(DAT_005ee1c6, u8, ivar5).write(b3 & 0x7f);
                DAT_00527020 = DAT_00527020.wrapping_add(1);
                if 0xff < DAT_00527020 {
                    DAT_00527020 = 0;
                }
            }
            let idx = DAT_0052701c as u32;
            cp(
                at!(DAT_005dd9c8, u8, idx.wrapping_mul(0x210)),
                param_2 as *const u8,
                param_3,
            );
            let ivar4 = DAT_0052701c as u32;
            let ivar5 = (DAT_0052701c as u32).wrapping_mul(0x210);
            at!(DAT_005dd9b8, i32, (DAT_0052701c as u32).wrapping_mul(0x84)).write(param_1);
            at!(DAT_005dd9bc, i32, ivar4.wrapping_mul(0x84)).write(param_3 as i32);
            at!(DAT_005dd9c0, u8, ivar5)
                .cast::<i32>()
                .write_unaligned(0);
            at!(DAT_005dd9c4, u8, ivar5).write(0);
            if param_1 == 0 {
                at!(DAT_005dd9c5, u8, ivar5).write(0);
            } else {
                at!(DAT_005dd9c5, u8, ivar5).write(2);
                if DAT_009994f0 != param_1 {
                    at!(DAT_005dd9c5, u8, ivar5).write(1);
                }
            }
            let n4 = DAT_0052701c.wrapping_add(1);
            DAT_0052701c = n4;
            at!(DAT_005dd9c6, u8, ivar5).write(b3 & 0x7f);
            uvar10 = local_400[0];
            if 0x7f < n4 {
                DAT_0052701c = 0;
            }
        }

        if (DAT_00999548 == param_1
            || param_1 == 0
            || DAT_009994c4.is_null()
            || DAT_009994f0 == param_1)
            && (DAT_0052702c < 0x400 || {
                FUN_00470060();
                DAT_0052702c < 0x400
            })
        {
            let idx = DAT_00527024 as u32;
            cp(
                at!(DAT_005599b8, u8, idx.wrapping_mul(0x210)),
                param_2 as *const u8,
                param_3,
            );
            let ivar4 = DAT_00527024 as u32;
            let ivar5 = (DAT_00527024 as u32).wrapping_mul(0x210);
            at!(DAT_005599a8, i32, (DAT_00527024 as u32).wrapping_mul(0x84)).write(DAT_00999548);
            at!(DAT_005599ac, i32, ivar4.wrapping_mul(0x84)).write(param_3 as i32);
            at!(DAT_005599b0, i32, ivar4.wrapping_mul(0x84)).write(0);
            at!(DAT_005599b4, u8, ivar5).write(0);
            at!(DAT_005599b7, u8, ivar5).write(0);
            let uvar8 = FUN_0046fee0(DAT_00999548);
            let val = ((uvar10 & 0x7f00) >> 8) as u32;
            let slot = (DAT_00527024 as u32).wrapping_mul(0x210);
            if uvar1 == 1 && DAT_00bb736f == 1 {
                bvar15 = (uvar8 < DAT_0099f6e0) as u8;
                at!(DAT_005599b5, u8, slot).write(2);
                if bvar15 < 0x28 {
                    at!(DAT_00999c00, u32, uvar8.wrapping_mul(0x8e)).write(val);
                }
            } else if param_1 == 0 {
                bvar15 = (uvar8 < DAT_0099f6e0) as u8;
                at!(DAT_005599b5, u8, slot).write(0);
                if bvar15 != 0 && uvar8 < 0x28 {
                    at!(DAT_00999bf8, u32, uvar8.wrapping_mul(0x8e)).write(val);
                }
            } else if DAT_009994f0 == param_1 {
                bvar15 = (uvar8 < DAT_0099f6e0) as u8;
                at!(DAT_005599b5, u8, slot).write(2);
                if bvar15 < 0x28 {
                    at!(DAT_00999c00, u32, uvar8.wrapping_mul(0x8e)).write(val);
                }
            } else {
                bvar15 = (uvar8 < DAT_0099f6e0) as u8;
                at!(DAT_005599b5, u8, slot).write(1);
                if bvar15 < 0x28 {
                    at!(DAT_00999bf0, u32, uvar8.wrapping_mul(0x8e)).write(val);
                }
            }
            DAT_0052702c = DAT_0052702c.wrapping_add(1);
            at!(DAT_005599b6, u8, (DAT_00527024 as u32).wrapping_mul(0x210))
                .write(((local_400[0] >> 8) as u8) & 0x7f);
            DAT_00527024 = DAT_00527024.wrapping_add(1);
            if 0x3ff < DAT_00527024 {
                DAT_00527024 = 0;
            }
        }

        if !DAT_009994c4.is_null() {
            if DAT_00999548 != param_1 {
                let vt = DAT_009994c4.read_unaligned() as u32 as usize;
                let fp = ((vt + 0x68) as *const u32).read_unaligned() as usize;
                let f: unsafe extern "stdcall" fn(
                    *mut i32,
                    i32,
                    i32,
                    i32,
                    *mut core::ffi::c_void,
                    i32,
                ) -> i32 = core::mem::transmute(fp);
                local_404 = f(
                    DAT_009994c4,
                    DAT_00999548,
                    param_1,
                    0,
                    local_400.as_mut_ptr().cast::<core::ffi::c_void>(),
                    local_420,
                );
            }
            return (local_404 == 0) as u8;
        }
        1
    }
}
