// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8e3c: i32;
    static mut DAT_009a7b60: i32;
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
    static mut DAT_00a08100: u8;
    static mut DAT_009eca20: Option<unsafe extern "C" fn(i32, i32, i32, i32, i32)>;
    static mut DAT_0054f990: i32;
    static mut DAT_0054f994: i32;
    static mut DAT_0052f868: i32;
    static mut DAT_0054f9a0: i32;
    static mut DAT_0053f970: *mut f32;
    static mut DAT_0054f988: *mut i32;
    static mut _DAT_0053f974: f32;
    static mut _DAT_0054f9a4: f32;
    static mut _DAT_00518060: f32;
    static mut _DAT_00518080: f32;
    static mut DAT_00a90b20: i32;
    static mut DAT_009a7bc0: [u8; 0];
    static mut DAT_009a7bc1: [u8; 0];
    static mut DAT_009a7bc2: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aba50();
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004abe50();
    fn FUN_00408140();
    fn FUN_004b1c10();
    fn FUN_004b1c90();
    fn FUN_004b1d10(a: *mut f32, b: i32);
    fn FUN_004b1d70();
    fn FUN_004b1de0(a: *mut i32, b: i32);
    fn FUN_004b2020();
    fn FUN_004b1c50();
    fn FUN_00408110();
}

unsafe fn setf(idx: i32, v: f32) {
    // SAFETY: the vertex buffer is the game's own memory, written where the original does.
    unsafe {
        DAT_0053f970.offset(idx as isize).write(v);
    }
}

unsafe fn quad(c: [f32; 8], f1: f32, iv3: i32) {
    // SAFETY: the globals and the vertex/index buffers are the game's own memory.
    unsafe {
        let base = DAT_0054f994.wrapping_mul(8);
        setf(base.wrapping_add(0), c[0]);
        setf(base.wrapping_add(1), c[1]);
        setf(base.wrapping_add(8), c[2]);
        setf(base.wrapping_add(9), c[3]);
        setf(base.wrapping_add(0x10), c[4]);
        setf(base.wrapping_add(0x11), c[5]);
        setf(base.wrapping_add(0x18), c[6]);
        setf(base.wrapping_add(0x19), c[7]);
        let mut i = 0i32;
        loop {
            let b = DAT_0054f994.wrapping_add(i).wrapping_mul(8);
            setf(b.wrapping_add(2), f1);
            setf(b.wrapping_add(3), f1);
            setf(b.wrapping_add(6), 0.0);
            setf(b.wrapping_add(7), 0.0);
            DAT_0053f970
                .offset(b.wrapping_add(4) as isize)
                .cast::<i32>()
                .write(iv3);
            i = i.wrapping_add(1);
            setf(b.wrapping_add(5), 0.0);
            if i >= 4 {
                break;
            }
        }
        let p = DAT_0054f988;
        let k = DAT_0054f9a0.wrapping_mul(5);
        p.offset(k as isize).write(DAT_0054f994);
        p.offset(k.wrapping_add(1) as isize)
            .write(DAT_0054f994.wrapping_add(1));
        p.offset(k.wrapping_add(2) as isize)
            .write(DAT_0054f994.wrapping_add(2));
        p.offset(k.wrapping_add(4) as isize).write(0);
        p.offset(k.wrapping_add(3) as isize).write(0x9800);
        DAT_0054f9a0 = DAT_0054f9a0.wrapping_add(1);
        let k = DAT_0054f9a0.wrapping_mul(5);
        p.offset(k as isize).write(DAT_0054f994);
        p.offset(k.wrapping_add(1) as isize)
            .write(DAT_0054f994.wrapping_add(2));
        p.offset(k.wrapping_add(2) as isize)
            .write(DAT_0054f994.wrapping_add(3));
        p.offset(k.wrapping_add(4) as isize).write(0);
        p.offset(k.wrapping_add(3) as isize).write(0x9800);
        DAT_0054f994 = DAT_0054f994.wrapping_add(4);
        DAT_0054f9a0 = DAT_0054f9a0.wrapping_add(1);
    }
}

unsafe fn hq(a: i32, b: i32, y: i32, f1: f32, iv3: i32) {
    // SAFETY: reads the game's own float globals.
    unsafe {
        let c4 = _DAT_0054f9a4;
        let c3 = _DAT_0053f974;
        let y1 = y.wrapping_add(1) as f32;
        quad(
            [
                c4 + a as f32,
                c3 + y as f32,
                c4 + b as f32,
                y as f32 + c3,
                b as f32 + c4,
                c3 + y1,
                a as f32 + c4,
                y1 + c3,
            ],
            f1,
            iv3,
        );
    }
}

unsafe fn vq(x: i32, a: i32, b: i32, f1: f32, iv3: i32) {
    // SAFETY: reads the game's own float globals.
    unsafe {
        let c4 = _DAT_0054f9a4;
        let c3 = _DAT_0053f974;
        let x1 = x.wrapping_add(1) as f32;
        quad(
            [
                c4 + x as f32,
                c3 + a as f32,
                x as f32 + c4,
                c3 + b as f32,
                c4 + x1,
                b as f32 + c3,
                x1 + c4,
                a as f32 + c3,
            ],
            f1,
            iv3,
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c430(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let mut param_2 = param_2;
    let mut param_6 = param_6;
    // SAFETY: the callees and globals are the game's own memory and code.
    unsafe {
        if param_6 == 1 && param_3 == 4 && param_4 == 4 {
            FUN_004aba50();
            let u_var2 = DAT_00a08100;
            let i_var4 = param_1.wrapping_add(DAT_009a8e3c);
            DAT_00a08100 = param_5 as u8;
            let i_var5 = param_2.wrapping_add(DAT_009a7b60);
            FUN_004a9520(
                DAT_009a8e3c as i16,
                DAT_009a7b60 as i16,
                (DAT_009d682c as i16).wrapping_add(DAT_009a8e3c as i16),
                (DAT_009a1fe4 as i16).wrapping_add(DAT_009a7b60 as i16),
            );
            let f = DAT_009eca20;
            if let Some(f) = f {
                f(
                    i_var4 as i16 as i32,
                    i_var5,
                    (i_var4 as i16 as i32).wrapping_add(4),
                    (i_var5 as i16 as i32).wrapping_add(4),
                    1,
                );
            }
            DAT_00a08100 = u_var2;
            FUN_004abe50();
            return;
        }
        if DAT_0054f990 < DAT_0054f994.wrapping_add(0x20)
            || DAT_0052f868 < DAT_0054f9a0.wrapping_add(0x10)
        {
            FUN_00408140();
            FUN_004b1c10();
            FUN_004b1c90();
            FUN_004b1d10(DAT_0053f970, DAT_0054f994);
            FUN_004b1d70();
            FUN_004b1de0(DAT_0054f988, DAT_0054f9a0);
            FUN_004b2020();
            FUN_004b1c50();
            FUN_00408110();
            DAT_0054f9a0 = 0;
            DAT_0054f994 = 0;
        }
        let m = param_5.wrapping_mul(3);
        let b0 = (&raw mut DAT_009a7bc0)
            .cast::<u8>()
            .offset(m as isize)
            .read() as u32;
        let b1 = (&raw mut DAT_009a7bc1)
            .cast::<u8>()
            .offset(m as isize)
            .read() as u32;
        let b2 = (&raw mut DAT_009a7bc2)
            .cast::<u8>()
            .offset(m as isize)
            .read() as u32;
        let iv3 = (b0
            .wrapping_sub(0x40)
            .wrapping_mul(0x100)
            .wrapping_add(b1)
            .wrapping_mul(0x100)
            .wrapping_add(b2)
            .wrapping_mul(4)) as i32;
        let i_var5 = param_1.wrapping_add(param_3);
        let mut i_var4 = param_2.wrapping_add(param_4);
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
        let c60 = _DAT_00518060;
        let mut f1 = c60 / (param_6 as f32 * _DAT_00518080 + c60);
        if DAT_00a90b20 == 2 {
            f1 = c60 - f1;
        }
        let w = DAT_009d682c as i32;
        let h = DAT_009a1fe4 as i32;

        if -1 < param_2 && param_2 < h {
            let mut i6 = local_8.wrapping_add(param_1);
            let mut i7 = param_1;
            if param_1 < 0 {
                i7 = 0;
            }
            if w <= i6 {
                i6 = w - 1;
            }
            if i7 < i6 {
                hq(i7, i6, param_2, f1, iv3);
            }
            let mut i6 = i_var5.wrapping_sub(local_8);
            if i6 < 0 {
                i6 = 0;
            }
            let mut i7 = i_var5;
            if w <= i_var5 {
                i7 = w - 1;
            }
            if i6 < i7 {
                hq(i6, i7, param_2, f1, iv3);
            }
        }
        if -1 < i_var4 && i_var4 < h {
            let mut i6 = local_8.wrapping_add(param_1);
            let mut i7 = param_1;
            if param_1 < 0 {
                i7 = 0;
            }
            if w <= i6 {
                i6 = w - 1;
            }
            if i7 < i6 {
                hq(i7, i6, i_var4, f1, iv3);
            }
            local_8 = i_var5.wrapping_sub(local_8);
            let mut i6 = i_var5.wrapping_add(1);
            if local_8 < 0 {
                local_8 = 0;
            }
            if w <= i6 {
                i6 = w - 1;
            }
            if local_8 < i6 {
                hq(local_8, i6, i_var4, f1, iv3);
            }
        }
        if -1 < param_1 && param_1 < w {
            let mut i6 = local_4.wrapping_add(param_2);
            let mut i7 = param_2;
            if param_2 < 0 {
                i7 = 0;
            }
            if h <= i6 {
                i6 = h - 1;
            }
            if i7 < i6 {
                vq(param_1, i7, i6, f1, iv3);
            }
            let mut i6 = i_var4.wrapping_sub(local_4);
            if i6 < 0 {
                i6 = 0;
            }
            let mut i7 = i_var4;
            if h <= i_var4 {
                i7 = h - 1;
            }
            if i6 < i7 {
                vq(param_1, i6, i7, f1, iv3);
            }
        }
        if -1 < i_var5 && i_var5 < w {
            let mut i6 = local_4.wrapping_add(param_2);
            if param_2 < 0 {
                param_2 = 0;
            }
            if h <= i6 {
                i6 = h - 1;
            }
            if param_2 < i6 {
                vq(i_var5, param_2, i6, f1, iv3);
            }
            local_4 = i_var4.wrapping_sub(local_4);
            if local_4 < 0 {
                local_4 = 0;
            }
            if h <= i_var4 {
                i_var4 = h - 1;
            }
            if local_4 < i_var4 {
                vq(i_var5, local_4, i_var4, f1, iv3);
            }
        }
    }
}
