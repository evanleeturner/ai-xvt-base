// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0052686c: [u8; 0];
    static mut DAT_0071b75d: u8;
    static mut DAT_0071b75f: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00412810(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00483410(a: i16, b: i16, c: u32);
    fn FUN_00483540(a: i16, b: i32, c: u32);
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_004496d0(a: u32, b: i32) -> *mut i32;
    fn __ftol() -> i64;
    fn FUN_00449770();
    fn FUN_004110e0(a: u32, b: i32, c: i32, d: u32, e: u32, f: u16, g: u32, h: u16, i: u32);
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) b, inout("eax") a => q, out("edx") _);
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041b530(param_1: u32, param_2: i32) {
    let unaff_ebx: u32 = 0;
    let unaff_ebp: u32 = 0;
    let unaff_esi: u32 = 0;
    let unaff_edi: u32 = 0;
    // SAFETY: all memory touched is the game's own; callees and globals are the game's.
    unsafe {
        let u_var1: u32 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988 as u32);
        let tbl = (&raw mut DAT_0052686c).cast::<u8>();
        let idx = (r8(u_var1.wrapping_add(3)) as usize).wrapping_mul(0x16);
        let u_var2: u32 = (tbl.add(idx).cast::<u16>().read_unaligned()) as u32;
        let i_var3: u32 = (DAT_006d4988 as u32).wrapping_add(param_1.wrapping_mul(0x23));
        let mut i_var11 =
            r32(i_var3.wrapping_add(4)).wrapping_sub(r32(u_var1.wrapping_add(4))) as i32;
        let mut i_var13 =
            r32(i_var3.wrapping_add(8)).wrapping_sub(r32(u_var1.wrapping_add(8))) as i32;
        let mut i_var9 =
            r32(i_var3.wrapping_add(0xc)).wrapping_sub(r32(u_var1.wrapping_add(0xc))) as i32;
        let i_var3b = FUN_00412810(i_var11, i_var13, i_var9);
        let t0 = r32(u_var1.wrapping_add(0x1f));
        if !(i_var3b <= u_var2.wrapping_mul(3) as i32 && t0 != 0) {
            return;
        }
        if r8(t0.wrapping_add(0x90)) != 0 {
            let s0 = r16(u_var1.wrapping_add(0x12)) as i16;
            let s1 = r16(u_var1.wrapping_add(0x10)) as i16;
            FUN_00483410(s0, s1, u_var1);
            let s0 = r16(u_var1.wrapping_add(0x14)) as i16;
            FUN_00483540(s0, 0, u_var1);
        }
        let t0 = r32(u_var1.wrapping_add(0x1f));
        let s0 = r16(t0.wrapping_add(0x97)) as i16 as i32;
        let s1 = r16(t0.wrapping_add(0x99)) as i16 as i32;
        let s2 = r16(t0.wrapping_add(0x9b)) as i16 as i32;
        let i_var4 = FUN_00413cc0(s0, s1, s2, i_var11, i_var13, i_var9);
        let t0 = r32(u_var1.wrapping_add(0x1f));
        let s0 = r16(t0.wrapping_add(0x91)) as i16 as i32;
        let s1 = r16(t0.wrapping_add(0x93)) as i16 as i32;
        let s2 = r16(t0.wrapping_add(0x95)) as i16 as i32;
        let u_var5 = FUN_00413cc0(s0, s1, s2, i_var11, i_var13, i_var9) as u32;
        if !(u_var5 == u_var2
            || (0u32.wrapping_sub(u_var2) as i32) < (0u32.wrapping_sub(u_var5) as i32))
        {
            return;
        }
        let t0 = r32(u_var1.wrapping_add(0x1f));
        let s0 = r16(t0.wrapping_add(0x9d)) as i16 as i32;
        let s1 = r16(t0.wrapping_add(0x9f)) as i16 as i32;
        let s2 = r16(t0.wrapping_add(0xa1)) as i16 as i32;
        let i_var3c = FUN_00413cc0(s0, s1, s2, i_var11, i_var13, i_var9);
        i_var9 = FUN_004490f0(r8(u_var1.wrapping_add(3)) as u32);
        let mut local_20: i32 = 0;
        if 0 < i_var9 {
            loop {
                'next: {
                    let pi = FUN_004496d0(r8(u_var1.wrapping_add(3)) as u32, local_20);
                    if !pi.is_null() {
                        if pi.read() == 6 {
                            let l15 = __ftol();
                            let l16 = __ftol();
                            i_var13 = l16 as i32;
                            let l16 = __ftol();
                            let mut i_var8 = l16 as i32;
                            i_var11 = l15 as i32;
                            if i_var11 < i_var13 {
                                i_var11 = i_var13;
                            }
                            if i_var11 < i_var8 {
                                i_var11 = i_var8;
                            }
                            let mut i_var14 = i_var11.wrapping_mul(8);
                            if (u_var2.wrapping_mul(2) as i32) < i_var11.wrapping_mul(8) {
                                i_var14 = u_var2.wrapping_mul(2) as i32;
                            }
                            let l15 = __ftol();
                            i_var11 =
                                0u32.wrapping_sub(u_var5).wrapping_sub(l15 as i32 as u32) as i32;
                            if 0 < i_var11 && i_var11 <= i_var14 {
                                let l15 = __ftol();
                                let mut i_var12 =
                                    (i_var4 as u32).wrapping_sub(l15 as i32 as u32) as i32;
                                let l15 = __ftol();
                                let mut i_var10 =
                                    (i_var3c as u32).wrapping_sub(l15 as i32 as u32) as i32;
                                i_var13 = i_var13.wrapping_add(i_var13 >> 1);
                                i_var13 = i_var13
                                    .wrapping_add(idiv(i_var13.wrapping_mul(i_var11), i_var14));
                                i_var8 = i_var8.wrapping_add(i_var8 >> 1);
                                i_var8 = i_var8
                                    .wrapping_add(idiv(i_var8.wrapping_mul(i_var11), i_var14));
                                if i_var12 < 0 {
                                    i_var12 = i_var12.wrapping_neg();
                                }
                                if i_var10 < 0 {
                                    i_var10 = i_var10.wrapping_neg();
                                }
                                if i_var12 <= i_var13 && i_var10 <= i_var8 {
                                    FUN_00449770();
                                    let local_24 = i_var8
                                        .wrapping_sub(i_var10)
                                        .wrapping_sub(i_var12)
                                        .wrapping_add(i_var13)
                                        .wrapping_mul(100);
                                    let local_28 = i_var8.wrapping_add(i_var13);
                                    if local_28 == 0 || (local_28 == -1 && local_24 == i32::MIN) {
                                        RaiseException(0xC0000094, 0, 0, core::ptr::null());
                                    }
                                    let a = idiv(local_24, local_28);
                                    let b = idiv(
                                        i_var14.wrapping_sub(i_var11).wrapping_mul(100),
                                        i_var14,
                                    );
                                    let t = a.wrapping_mul(b) / 100;
                                    let t2 = t.wrapping_mul(i_var14 >> 8);
                                    let mut u_var7: i32 = t2 / 100;
                                    if 0x40 < u_var7 {
                                        u_var7 = 0x40;
                                    }
                                    let base = (DAT_006d4988 as u32)
                                        .wrapping_add(param_1.wrapping_mul(0x23));
                                    let mut v = r32(base.wrapping_add(0x1b)) as i32;
                                    if v != -1 {
                                        v = v.wrapping_mul(0x5bd);
                                        let p5f = (&raw mut DAT_0071b75f).byte_offset(v as isize);
                                        let w = p5f.cast::<u16>().read_unaligned();
                                        if (w as i32) < u_var7 {
                                            let p5d =
                                                (&raw mut DAT_0071b75d).byte_offset(v as isize);
                                            p5d.cast::<u16>().write_unaligned(param_2 as u16);
                                            p5f.cast::<u16>().write_unaligned(u_var7 as u16);
                                        }
                                    }
                                    let v = r32(base.wrapping_add(0x1f));
                                    if v != 0 {
                                        let v = r32(v.wrapping_add(0xa7));
                                        if v != 0 {
                                            let s0 = r16(v.wrapping_add(0x20b)) as i16 as i32;
                                            let s1 = r16(v.wrapping_add(0x20d)) as i16 as i32;
                                            if s0.wrapping_add(s1) == 0 {
                                                u_var7 >>= 3;
                                            }
                                        }
                                    }
                                    if u_var7 < 1 {
                                        u_var7 = 1;
                                    }
                                    FUN_004110e0(
                                        param_1,
                                        0xffff,
                                        0xfffe,
                                        u_var7 as u32,
                                        unaff_ebp,
                                        unaff_edi as u16,
                                        unaff_esi,
                                        unaff_ebx as u16,
                                        u_var1,
                                    );
                                    break 'next;
                                }
                            }
                        }
                        FUN_00449770();
                    }
                }
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < i_var9) {
                    break;
                }
            }
        }
    }
}
