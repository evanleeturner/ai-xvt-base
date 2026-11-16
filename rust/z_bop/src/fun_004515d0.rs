// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9a77: [i8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_005233ec: i32;
    static mut DAT_00a09288: [i16; 0];
    static mut DAT_00a0928a: [i16; 0];
    static mut DAT_00a09290: [i16; 0];
    static mut DAT_00a09292: [i16; 0];
    static mut DAT_00a08374: u16;
    static mut DAT_009d1300: unsafe extern "C" fn();
    static mut DAT_005235e0: i8;
    static mut DAT_009a8d90: i32;
    static mut DAT_009a7ba0: i32;
    static mut DAT_00a0829c: i32;
    static mut DAT_009a1ed0: [i8; 0];
    static mut s__d__d_005240f4: [i8; 0];
    static mut DAT_00a080f8: i32;
    static mut DAT_009ec464: u8;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_005235dc: i32;
    static mut DAT_009ed232: u8;
    static mut DAT_009d1148: i32;
    static mut DAT_009a7ec0: i32;
    static mut DAT_009a7b44: i32;
    static mut DAT_009a8bfc: i32;
    static mut DAT_005233c8: u8;
    static mut DAT_00a0a120: i32;
    static mut DAT_009ec45c: i32;
    static mut DAT_00a0a124: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004aba50();
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9640(a: i8);
    fn FUN_0043c290(a: u16, b: u8);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9590(a: u32);
    fn FUN_004a9a50(a: *mut i8);
    fn FUN_004abe50();
    fn FUN_004a95f0(a: u32);
    fn FUN_004a9960(a: i32);
    fn FUN_004a9ac0(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; faults on zero as the original does.
    unsafe {
        asm!("cdq", "idiv {b:e}", b = in(reg) b, inout("eax") a => q, out("edx") _);
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004515d0() {
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        let i_var1 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32;
        let p = (&raw mut DAT_009e9a77).cast::<i8>().offset(i_var1 as isize);
        if p.read() != 0 && p.read() != 0x13 {
            return;
        }
        FUN_004aba50();
        FUN_0040e0c0(0, 0x140, 200, 0);
        FUN_004a95c0(0x2c);
        FUN_004a9640(0);
        let b_var4: u8 = if DAT_005233ec == 0x13 { 7 } else { 3 };
        let idx = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32;
        let u_var6 = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .offset(idx as isize)
            .cast::<u32>()
            .read_unaligned();
        FUN_0043c290(u_var6 as u16, b_var4);
        let k = (DAT_00a08374 as u32).wrapping_mul(6) as usize;
        let a88 = (&raw mut DAT_00a09288).cast::<i16>();
        let a8a = (&raw mut DAT_00a0928a).cast::<i16>();
        let a90 = (&raw mut DAT_00a09290).cast::<i16>();
        let a92 = (&raw mut DAT_00a09292).cast::<i16>();
        FUN_004a9500(a88.add(k).read(), a8a.add(k).read());
        let k = (DAT_00a08374 as u32).wrapping_mul(6) as usize;
        FUN_004a9520(
            a88.add(k).read(),
            a8a.add(k).read(),
            a90.add(k).read().wrapping_add(a88.add(k).read()),
            a92.add(k).read().wrapping_add(a8a.add(k).read()),
        );
        (DAT_009d1300)();
        if DAT_005235e0 != 0 && DAT_009a8d90 != 0 && DAT_009a7ba0 != 0 && DAT_00a0829c != 0 {
            FUN_004a9590(0x4e);
            let i_var7 = idiv(0xec, DAT_009a8d90);
            if DAT_009a7ba0 == i32::MIN && DAT_00a0829c == -1 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            let i_var8 = idiv(DAT_009a7ba0, DAT_00a0829c);
            if i_var8 == 0 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            let r = idiv(0xec, i_var8);
            sprintf(
                (&raw mut DAT_009a1ed0).cast::<i8>(),
                (&raw mut s__d__d_005240f4).cast::<i8>(),
                i_var7,
                r,
            );
        }
        FUN_004a9a50((&raw mut DAT_009a1ed0).cast::<i8>());
        FUN_004abe50();
        if DAT_00a080f8 < 2 {
            return;
        }
        DAT_009ec464 = 0;
        FUN_004a9640(0);
        FUN_0040e0c0(
            DAT_009a806c,
            DAT_009a7bac,
            DAT_009a8c14,
            (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
        );
        let b = DAT_009ed232 as i16;
        let left = (DAT_005235dc as i16).wrapping_add(b.wrapping_mul(-2));
        FUN_004a9520(
            left,
            DAT_009d1148 as i16,
            (DAT_009a7ec0 as i16)
                .wrapping_add(b.wrapping_mul(2))
                .wrapping_add(1),
            DAT_009a7b44 as i16,
        );
        FUN_004a9500(left, DAT_009d1148 as i16);
        let mut u_var5: Option<u32> = None;
        match DAT_009a8bfc {
            0 => {
                FUN_004a95c0(DAT_005233c8 as u32);
                FUN_004a9590(DAT_005233c8 as u32);
                FUN_004a95f0(DAT_005233c8 as u32);
            }
            1 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4e);
            }
            2 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4b);
            }
            3 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4a);
            }
            _ => {}
        }
        if let Some(v) = u_var5 {
            FUN_004a9590(v);
        }
        FUN_004a9960(DAT_00a0a120);
        let mut u_var5: Option<u32> = None;
        match DAT_009ec45c {
            0 => {
                FUN_004a95c0(DAT_005233c8 as u32);
                FUN_004a9590(DAT_005233c8 as u32);
                FUN_004a95f0(DAT_005233c8 as u32);
            }
            1 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4e);
            }
            2 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4a);
            }
            3 => {
                FUN_004a95c0(0x41);
                u_var5 = Some(0x4b);
            }
            _ => {}
        }
        if let Some(v) = u_var5 {
            FUN_004a9590(v);
        }
        FUN_004a9ac0(DAT_00a0a124);
        FUN_004a95f0(0x40);
        FUN_0040e0c0(0, 0x140, 200, 0);
    }
}
