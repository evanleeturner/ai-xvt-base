// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c818: i32;
    static mut DAT_0052c81c: i32;
    static mut DAT_0052c808: u32;
    static mut DAT_0052c80c: u32;
    static mut DAT_0052c82c: u32;
    static mut DAT_0052c828: i32;
    static mut DAT_0052c830: u32;
    static mut DAT_0052c834: *mut u8;
    static mut DAT_0052c838: *mut u8;
    static mut DAT_00aa6080: u32;
    static mut DAT_00aa6084: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetFocus() -> *mut c_void;
    fn _SmackToBuffer_28(a: i32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> i32;
    fn _SmackDoFrame_4(a: i32);
    fn _SmackToBufferRect_8(a: i32, b: i32) -> i32;
    fn _SmackNextFrame_4(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004eee70();
    fn FUN_004f0a50(a: i32);
    fn FUN_004d5b70(a: u8);
    fn FUN_004ef8e0(a: *mut i32, b: u32, c: i32, d: u32, e: *mut i32, f: *mut u32);
    fn FUN_004eecb0(a: i32, b: i32, c: i32, d: i32);
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game object field, read as the original does.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ef600() {
    // SAFETY: globals, callees and the pointed-to game objects belong to the
    // game; accesses mirror the original's order and offsets.
    unsafe {
        let unaff_esi: u32 = 0;
        let u_stack_a8: u32 = 0;
        let mut u_stack_b4: i32 = 0;
        let mut u_stack_b8: u32 = 0;
        let mut local_6c: [u32; 27] = [0; 27];

        let focus = GetFocus();
        if focus as usize as i32 != rd(DAT_0052c818.wrapping_add(0x20)) {
            return;
        }
        if rd(DAT_0052c81c.wrapping_add(0x68)) != 0 {
            FUN_004eee70();
        }
        local_6c[0] = 0x6c;
        loop {
            let surf = rd(DAT_0052c818.wrapping_add(0x10));
            let f = rd(rd(surf).wrapping_add(100)) as u32 as usize as *const c_void;
            let f: unsafe extern "stdcall" fn(i32, i32, *mut u32, i32, i32) -> i32 =
                core::mem::transmute(f);
            let r = f(surf, 0, local_6c.as_mut_ptr(), 1, 0);
            if r != -0x7789fe3e {
                break;
            }
            let surf = rd(DAT_0052c818.wrapping_add(0x10));
            let g = rd(rd(surf).wrapping_add(0x6c)) as u32 as usize as *const c_void;
            let g: unsafe extern "stdcall" fn(i32) -> i32 = core::mem::transmute(g);
            if g(surf) != 0 {
                return;
            }
        }
        _SmackToBuffer_28(
            DAT_0052c81c,
            DAT_0052c808,
            DAT_0052c80c,
            u_stack_a8,
            rd(DAT_0052c81c.wrapping_add(8)) as u32,
            local_6c[4],
            DAT_0052c82c,
        );
        _SmackDoFrame_4(DAT_0052c81c);
        DAT_0052c828 = 1;
        {
            let surf = rd(DAT_0052c818.wrapping_add(0x10));
            let h = rd(rd(surf).wrapping_add(0x80)) as u32 as usize as *const c_void;
            let h: unsafe extern "stdcall" fn(i32, u32) -> i32 = core::mem::transmute(h);
            h(surf, unaff_esi);
        }
        FUN_004f0a50(rd(DAT_0052c81c.wrapping_add(0x374)));
        let cb = rd(DAT_0052c818.wrapping_add(0x2c));
        if cb != 0 {
            let cbf: unsafe extern "C" fn(i32, i32, i32) -> i32 =
                core::mem::transmute(cb as u32 as usize as *const c_void);
            let r = cbf(
                rd(DAT_0052c81c.wrapping_add(0x374)),
                rd(DAT_0052c81c.wrapping_add(0xc)).wrapping_sub(1),
                rd(DAT_0052c818.wrapping_add(0x30)),
            );
            if r == 1 {
                FUN_004d5b70(DAT_00aa6080 as u8);
            }
        }
        if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
            let mut u_var8: u32 = 0;
            let mut i_var4: i32 = 0;
            if _SmackToBufferRect_8(DAT_0052c81c, 0) != 0 {
                loop {
                    let mut i_var7 = i_var4;
                    if rd(DAT_0052c81c.wrapping_add(0x388)) != 0 {
                        let base = DAT_0052c838.byte_offset(i_var4 as isize);
                        base.cast::<i32>()
                            .write_unaligned(rd(DAT_0052c81c.wrapping_add(0x380)));
                        i_var7 = i_var4.wrapping_add(0x10);
                        u_var8 = u_var8.wrapping_add(1);
                        base.byte_add(4)
                            .cast::<i32>()
                            .write_unaligned(rd(DAT_0052c81c.wrapping_add(900)));
                        base.byte_add(8)
                            .cast::<i32>()
                            .write_unaligned(rd(DAT_0052c81c.wrapping_add(0x388)));
                        base.byte_add(0xc)
                            .cast::<i32>()
                            .write_unaligned(rd(DAT_0052c81c.wrapping_add(0x38c)));
                    }
                    let i_var5 = _SmackToBufferRect_8(DAT_0052c81c, 0);
                    i_var4 = i_var7;
                    if i_var5 == 0 {
                        break;
                    }
                }
            }
            FUN_004ef8e0(
                DAT_0052c838.cast::<i32>(),
                u_var8,
                DAT_0052c834 as usize as i32,
                DAT_0052c830,
                &raw mut u_stack_b4,
                &raw mut u_stack_b8,
            );
            while u_stack_b8 != 0 {
                u_stack_b8 = u_stack_b8.wrapping_sub(1);
                let p = u_stack_b8
                    .wrapping_mul(0x10)
                    .wrapping_add(u_stack_b4 as u32) as usize as *const i32;
                FUN_004eecb0(
                    p.read_unaligned(),
                    p.byte_add(4).read_unaligned(),
                    p.byte_add(8).read_unaligned(),
                    p.byte_add(0xc).read_unaligned(),
                );
            }
            {
                let back = rd(DAT_0052c818.wrapping_add(0xc));
                let k = rd(rd(back).wrapping_add(0x2c)) as u32 as usize as *const c_void;
                let k: unsafe extern "stdcall" fn(i32, i32, i32) -> i32 = core::mem::transmute(k);
                k(back, 0, 1);
            }
            let p2 = DAT_0052c834;
            DAT_0052c834 = DAT_0052c838;
            DAT_0052c838 = p2;
            DAT_0052c830 = u_var8;
        } else {
            let mut i_var4 = _SmackToBufferRect_8(DAT_0052c81c, 0);
            while i_var4 != 0 {
                FUN_004eecb0(
                    rd(DAT_0052c81c.wrapping_add(0x380)),
                    rd(DAT_0052c81c.wrapping_add(900)),
                    rd(DAT_0052c81c.wrapping_add(0x388)),
                    rd(DAT_0052c81c.wrapping_add(0x38c)),
                );
                i_var4 = _SmackToBufferRect_8(DAT_0052c81c, 0);
            }
        }
        if rd(DAT_0052c81c.wrapping_add(0xc)).wrapping_sub(rd(DAT_0052c81c.wrapping_add(0x374)))
            == 1
        {
            DAT_00aa6084 = 1;
            return;
        }
        _SmackNextFrame_4(DAT_0052c81c);
    }
}
