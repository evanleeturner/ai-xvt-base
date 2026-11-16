// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e50: u8;
    static mut _DAT_006db7bc: i32;
    static mut _DAT_006da4fc: i32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_0071eae0: *const core::ffi::c_void;
    static mut DAT_005f10d0: i32;
    static mut DAT_005f10d4: i32;
    static mut DAT_005f0fa8: i32;
    static mut DAT_005f10e0: u32;
    static mut DAT_005f10b0: *mut i8;
    static mut DAT_005f10c8: *mut i32;
    static mut _DAT_005f10e4: f32;
    static mut _DAT_005f10b4: f32;
    static mut _DAT_005201b8: f32;
    static mut DAT_006da540: [u8; 1];
    static mut DAT_006da541: [u8; 1];
    static mut DAT_006da542: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6a80();
    fn FUN_004a6f90();
    fn FUN_0048ed00(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004b5bc2();
    fn FUN_004b5c5c(vb: *mut i8, n: i32);
    fn FUN_004b5ce0();
    fn FUN_004b5d64(ib: *mut i32, n: u32);
    fn FUN_004b6054();
}

unsafe fn vbase() -> *mut i8 {
    // SAFETY: game globals; the vertex buffer is the game's own memory.
    unsafe { DAT_005f10b0.wrapping_add((DAT_005f10d4 as u32).wrapping_mul(0x20) as usize) }
}

unsafe fn wf(vp: *mut i8, off: usize, v: f32) {
    // SAFETY: vp points into the game's vertex buffer.
    unsafe { vp.wrapping_add(off).cast::<f32>().write_unaligned(v) }
}

unsafe fn tail(f1: f32, c: i32) {
    // SAFETY: the globals and buffers are the game's own memory.
    unsafe {
        let mut i: i32 = 0;
        loop {
            let vp = DAT_005f10b0.wrapping_add(
                ((DAT_005f10d4 as u32)
                    .wrapping_add(i as u32)
                    .wrapping_mul(0x20)) as usize,
            );
            wf(vp, 8, f1);
            wf(vp, 0xc, f1);
            vp.wrapping_add(0x18).cast::<i32>().write_unaligned(0);
            vp.wrapping_add(0x1c).cast::<i32>().write_unaligned(0);
            vp.wrapping_add(0x10).cast::<i32>().write_unaligned(c);
            vp.wrapping_add(0x14).cast::<i32>().write_unaligned(0);
            i = i.wrapping_add(1);
            if i >= 4 {
                break;
            }
        }
        let d4 = DAT_005f10d4;
        let ib = DAT_005f10c8;
        let e = DAT_005f10e0.wrapping_mul(5) as usize;
        ib.wrapping_add(e).write_unaligned(d4);
        ib.wrapping_add(e + 1).write_unaligned(d4.wrapping_add(1));
        ib.wrapping_add(e + 2).write_unaligned(d4.wrapping_add(2));
        ib.wrapping_add(e + 3).write_unaligned(0);
        ib.wrapping_add(e + 4).write_unaligned(0x3000);
        DAT_005f10e0 = DAT_005f10e0.wrapping_add(1);
        let e = DAT_005f10e0.wrapping_mul(5) as usize;
        let d4 = DAT_005f10d4;
        let ib = DAT_005f10c8;
        ib.wrapping_add(e).write_unaligned(d4);
        ib.wrapping_add(e + 1).write_unaligned(d4.wrapping_add(2));
        ib.wrapping_add(e + 2).write_unaligned(d4.wrapping_add(3));
        ib.wrapping_add(e + 3).write_unaligned(0);
        ib.wrapping_add(e + 4).write_unaligned(0x3000);
        DAT_005f10d4 = DAT_005f10d4.wrapping_add(4);
        DAT_005f10e0 = DAT_005f10e0.wrapping_add(1);
    }
}

unsafe fn quad_h(xa: i32, xb: i32, y: i32, f1: f32, c: i32) {
    // SAFETY: the globals and buffers are the game's own memory.
    unsafe {
        let vp = vbase();
        let e4 = _DAT_005f10e4;
        let b4 = _DAT_005f10b4;
        let y1 = y.wrapping_add(1);
        wf(vp, 0x00, e4 + xa as f32);
        wf(vp, 0x04, b4 + y as f32);
        wf(vp, 0x20, e4 + xb as f32);
        wf(vp, 0x24, y as f32 + b4);
        wf(vp, 0x40, xb as f32 + e4);
        wf(vp, 0x44, b4 + y1 as f32);
        wf(vp, 0x60, xa as f32 + e4);
        wf(vp, 0x64, y1 as f32 + b4);
        tail(f1, c);
    }
}

unsafe fn quad_v(x: i32, ya: i32, yb: i32, f1: f32, c: i32) {
    // SAFETY: the globals and buffers are the game's own memory.
    unsafe {
        let vp = vbase();
        let e4 = _DAT_005f10e4;
        let b4 = _DAT_005f10b4;
        let x1 = x.wrapping_add(1);
        wf(vp, 0x00, e4 + x as f32);
        wf(vp, 0x04, b4 + ya as f32);
        wf(vp, 0x20, x as f32 + e4);
        wf(vp, 0x24, b4 + yb as f32);
        wf(vp, 0x40, e4 + x1 as f32);
        wf(vp, 0x44, yb as f32 + b4);
        wf(vp, 0x60, x1 as f32 + e4);
        wf(vp, 0x64, ya as f32 + b4);
        tail(f1, c);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004604c0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let mut param_2 = param_2;
    let mut param_6 = param_6;
    // SAFETY: all globals and callees are the game's own; buffers are the
    // game's vertex/index buffers used as the original does.
    unsafe {
        if param_6 == 1 && param_3 == 4 && param_4 == 4 {
            FUN_004a6a80();
            let u_var2 = DAT_00732e50;
            let i4 = (param_1 as u32).wrapping_add(_DAT_006db7bc as u32) as i32;
            DAT_00732e50 = param_5 as u8;
            let i5 = (param_2 as u32).wrapping_add(_DAT_006da4fc as u32) as i32;
            FUN_0048ed00(
                _DAT_006db7bc as i16,
                _DAT_006da4fc as i16,
                (DAT_007091ae as u32).wrapping_add(_DAT_006db7bc as i16 as i32 as u32) as i16,
                (DAT_006d4984 as u32).wrapping_add(_DAT_006da4fc as i16 as i32 as u32) as i16,
            );
            let f: unsafe extern "C" fn(i32, i32, i32, i32, i32) =
                core::mem::transmute(DAT_0071eae0);
            f(
                (0u32 | (i4 as i16 as u16 as u32)) as i32,
                i5,
                (i4 as i16 as i32).wrapping_add(4),
                (i5 as i16 as i32).wrapping_add(4),
                1,
            );
            DAT_00732e50 = u_var2;
            FUN_004a6f90();
            return;
        }
        if DAT_005f10d0 < (DAT_005f10d4 as u32).wrapping_add(0x20) as i32
            || DAT_005f0fa8 < DAT_005f10e0.wrapping_add(0x10) as i32
        {
            FUN_004b5bc2();
            FUN_004b5c5c(DAT_005f10b0, DAT_005f10d4);
            FUN_004b5ce0();
            FUN_004b5d64(DAT_005f10c8, DAT_005f10e0);
            FUN_004b6054();
            DAT_005f10e0 = 0;
            DAT_005f10d4 = 0;
        }
        let idx = (param_5 as u32).wrapping_mul(3) as usize;
        let p0 = (&raw mut DAT_006da540)
            .cast::<u8>()
            .wrapping_add(idx)
            .read() as u32;
        let p1 = (&raw mut DAT_006da541)
            .cast::<u8>()
            .wrapping_add(idx)
            .read() as u32;
        let p2 = (&raw mut DAT_006da542)
            .cast::<u8>()
            .wrapping_add(idx)
            .read() as u32;
        let i3 = (p0
            .wrapping_sub(0x40)
            .wrapping_mul(0x100)
            .wrapping_add(p1)
            .wrapping_mul(0x100)
            .wrapping_add(p2)
            .wrapping_mul(4)) as i32;
        let i5 = param_1.wrapping_add(param_3);
        let i4 = param_2.wrapping_add(param_4);
        let mut local_8 = param_3 >> 3;
        let mut local_4 = param_4 >> 3;
        if local_8 < 3 {
            local_8 = 3;
        }
        if local_4 < 3 {
            local_4 = 3;
        }
        if param_3 < local_8 {
            local_8 = param_3;
        }
        if param_4 < local_4 {
            local_4 = param_4;
        }
        if param_6 < 1 {
            param_6 = 1;
        }
        let f1 = param_6 as f32 * _DAT_005201b8;
        let w = DAT_007091ae as i32;
        let h = DAT_006d4984 as i32;

        if -1 < param_2 && param_2 < h {
            let mut a = local_8.wrapping_add(param_1);
            let mut b = param_1;
            if param_1 < 0 {
                b = 0;
            }
            if w <= a {
                a = w.wrapping_sub(1);
            }
            if b < a {
                quad_h(b, a, param_2, f1, i3);
            }
            let mut a = i5.wrapping_sub(local_8);
            if a < 0 {
                a = 0;
            }
            let mut b = i5;
            if w <= i5 {
                b = w.wrapping_sub(1);
            }
            if a < b {
                quad_h(a, b, param_2, f1, i3);
            }
        }
        if -1 < i4 && i4 < h {
            let mut a = local_8.wrapping_add(param_1);
            let mut b = param_1;
            if param_1 < 0 {
                b = 0;
            }
            if w <= a {
                a = w.wrapping_sub(1);
            }
            if b < a {
                quad_h(b, a, i4, f1, i3);
            }
            let l8 = i5.wrapping_sub(local_8);
            let mut a = i5.wrapping_add(1);
            let mut l8 = l8;
            if l8 < 0 {
                l8 = 0;
            }
            if w <= a {
                a = w.wrapping_sub(1);
            }
            if l8 < a {
                quad_h(l8, a, i4, f1, i3);
            }
        }
        if -1 < param_1 && param_1 < w {
            let mut a = local_4.wrapping_add(param_2);
            let mut b = param_2;
            if param_2 < 0 {
                b = 0;
            }
            if h <= a {
                a = h.wrapping_sub(1);
            }
            if b < a {
                quad_v(param_1, b, a, f1, i3);
            }
            let mut a = i4.wrapping_sub(local_4);
            if a < 0 {
                a = 0;
            }
            let mut b = i4;
            if h <= i4 {
                b = h.wrapping_sub(1);
            }
            if a < b {
                quad_v(param_1, a, b, f1, i3);
            }
        }
        if -1 < i5 && i5 < w {
            let mut a = local_4.wrapping_add(param_2);
            if param_2 < 0 {
                param_2 = 0;
            }
            if h <= a {
                a = h.wrapping_sub(1);
            }
            if param_2 < a {
                quad_v(i5, param_2, a, f1, i3);
            }
            local_4 = i4.wrapping_sub(local_4);
            if local_4 < 0 {
                local_4 = 0;
            }
            let mut b = i4;
            if h <= i4 {
                b = h.wrapping_sub(1);
            }
            if local_4 < b {
                quad_v(i5, local_4, b, f1, i3);
            }
        }
    }
}
