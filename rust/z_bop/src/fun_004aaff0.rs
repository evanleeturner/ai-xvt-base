// SPDX-License-Identifier: MIT

use core::ffi::c_void;
use core::mem::transmute;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066ddd4: *mut i32;
    static mut DAT_0066ddd0: u32;
    static mut DAT_00527ea8: i32;
    static mut DAT_00527e90: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_0066ddc4: *mut i32;
    static mut DAT_0066dde0: i32;
    static mut _DAT_00803b74: i32;
    static mut DAT_00803b70: u32;
    static mut DAT_00527eac: i32;
    static mut DAT_00527eb8: i32;
    static mut DAT_0066e700: *mut i32;
    static mut DAT_009d8c08: u32;
    static mut DAT_00803f80: *mut i32;
    static mut _DAT_0066ddec: *mut i32;
    static mut DAT_0066ddcc: *mut i32;
    static mut DAT_005233e0: *mut u32;
    static mut DAT_005233e8: *mut u32;
    static mut DAT_0066ddd8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d56a0() -> i32;
    fn FUN_004ab970(param_1: i32) -> u32;
    fn FUN_004ac400(param_1: *mut i32);
    fn FUN_004aafe0() -> i32;
    fn FUN_00408170();
}

unsafe fn vt(obj: *mut i32, off: usize) -> *const c_void {
    // SAFETY: obj is a COM-style object whose first dword is a vtable pointer.
    unsafe {
        let table = (obj as *const *const u8).read_unaligned();
        let f = table.byte_add(off).cast::<u32>().read_unaligned();
        f as usize as *const c_void
    }
}

unsafe fn sdm(w: *mut i32, h: i32) -> i32 {
    // SAFETY: calls the game's DirectDraw SetDisplayMode slot.
    unsafe {
        let o = DAT_0066ddd4;
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i32, i32) -> i32 = transmute(vt(o, 0x54));
        f(o, w, h)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004aaff0() -> u32 {
    const S_1A3: u8 = 0;
    const S_2D8: u8 = 1;
    const S_JOIN: u8 = 2;
    const S_40D: u8 = 3;
    const S_425: u8 = 4;

    let mut frame: [u32; 392] = [0; 392];
    let fp: *mut u32 = frame.as_mut_ptr();

    // SAFETY: all globals and callees are the game's own; fp points at a
    // local frame large enough for every offset used.
    unsafe {
        DAT_0066ddd4 = FUN_004d56a0() as usize as *mut i32;
        if DAT_00527ea8 == 0 {
            let o = DAT_0066ddd4;
            let f: unsafe extern "stdcall" fn(*mut i32, u32, i32) -> i32 = transmute(vt(o, 0x50));
            let r = f(o, DAT_0066ddd0, 8);
            if r != 0 {
                return FUN_004ab970(2);
            }
        } else {
            let o = DAT_0066ddd4;
            let f: unsafe extern "stdcall" fn(*mut i32, u32, i32) -> i32 = transmute(vt(o, 0x50));
            let r = f(o, DAT_0066ddd0, 0x51);
            if r != 0 {
                return FUN_004ab970(2);
            }
        }
        if DAT_00527e90 != 0 {
            DAT_005233d8 = 2;
        }
        let mut iv1: i32;
        'main: {
            if DAT_00527ea8 == 0 {
                break 'main;
            }
            iv1 = sdm(DAT_0066ddc4, DAT_0066dde0);
            if iv1 == 0 {
                break 'main;
            }
            let mut st: u8;
            if DAT_0066ddc4 as usize == 0x140 {
                DAT_0066ddc4 = 0x200usize as *mut i32;
                DAT_0066dde0 = 0x180;
                let iv3 = sdm(0x200usize as *mut i32, 0x180);
                iv1 = 0;
                st = S_2D8;
                if iv3 != 0 {
                    DAT_0066ddc4 = 0x280usize as *mut i32;
                    DAT_0066dde0 = 0x1e0;
                    iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                    if iv1 != 0 {
                        DAT_0066dde0 = 0xf0;
                        DAT_0066ddc4 = 0x140usize as *mut i32;
                        st = S_1A3;
                    }
                }
            } else {
                st = S_1A3;
                if DAT_0066ddc4 as usize == 0x200 {
                    DAT_0066ddc4 = 0x280usize as *mut i32;
                    DAT_0066dde0 = 0x1e0;
                    iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                    if iv1 == 0 {
                        st = S_2D8;
                    } else {
                        DAT_0066dde0 = 0x180;
                        DAT_0066ddc4 = 0x200usize as *mut i32;
                    }
                }
            }
            loop {
                match st {
                    S_1A3 => {
                        if iv1 == 0 {
                            st = S_40D;
                            continue;
                        }
                        if DAT_005233d8 != 2 {
                            st = S_2D8;
                            continue;
                        }
                        DAT_005233d8 = 1;
                        iv1 = sdm(DAT_0066ddc4, DAT_0066dde0);
                        if iv1 == 0 {
                            break 'main;
                        }
                        if DAT_0066ddc4 as usize == 0x140 {
                            DAT_0066ddc4 = 0x200usize as *mut i32;
                            DAT_0066dde0 = 0x180;
                            iv1 = sdm(0x200usize as *mut i32, 0x180);
                            if iv1 == 0 {
                                break 'main;
                            }
                            DAT_0066ddc4 = 0x280usize as *mut i32;
                            DAT_0066dde0 = 0x1e0;
                            iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                            if iv1 == 0 {
                                break 'main;
                            }
                            DAT_0066dde0 = 0xf0;
                            DAT_0066ddc4 = 0x140usize as *mut i32;
                            st = S_40D;
                        } else if DAT_0066ddc4 as usize == 0x200 {
                            DAT_0066ddc4 = 0x280usize as *mut i32;
                            DAT_0066dde0 = 0x1e0;
                            iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                            st = S_JOIN;
                        } else {
                            st = S_40D;
                        }
                    }
                    S_2D8 => {
                        if iv1 == 0 {
                            break 'main;
                        }
                        if DAT_005233d8 == 1 {
                            DAT_005233d8 = 2;
                            iv1 = sdm(DAT_0066ddc4, DAT_0066dde0);
                            if iv1 == 0 {
                                break 'main;
                            }
                            if DAT_0066ddc4 as usize == 0x140 {
                                DAT_0066ddc4 = 0x200usize as *mut i32;
                                DAT_0066dde0 = 0x180;
                                iv1 = sdm(0x200usize as *mut i32, 0x180);
                                if iv1 == 0 {
                                    break 'main;
                                }
                                DAT_0066ddc4 = 0x280usize as *mut i32;
                                DAT_0066dde0 = 0x1e0;
                                iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                                if iv1 == 0 {
                                    break 'main;
                                }
                                DAT_0066dde0 = 0xf0;
                                DAT_0066ddc4 = 0x140usize as *mut i32;
                                st = S_40D;
                            } else if DAT_0066ddc4 as usize == 0x200 {
                                DAT_0066ddc4 = 0x280usize as *mut i32;
                                DAT_0066dde0 = 0x1e0;
                                iv1 = sdm(0x280usize as *mut i32, 0x1e0);
                                st = S_JOIN;
                            } else {
                                st = S_40D;
                            }
                        } else {
                            st = S_40D;
                        }
                    }
                    S_JOIN => {
                        if iv1 == 0 {
                            break 'main;
                        }
                        DAT_0066dde0 = 0x180;
                        DAT_0066ddc4 = 0x200usize as *mut i32;
                        st = S_40D;
                    }
                    S_40D => {
                        if iv1 != 0 {
                            return FUN_004ab970(3);
                        }
                        st = S_425;
                    }
                    _ => break 'main,
                }
            }
        }
        if DAT_005233d8 != 2 {
            DAT_00527e90 = 0;
        }
        let u_var2: u32 = 0;
        _DAT_00803b74 = 1;
        let caps = fp.add(39);
        for i in 0..0x5b {
            caps.add(i).write(0);
        }
        caps.write(0x16c);
        {
            let o = DAT_0066ddd4;
            let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 = transmute(vt(o, 0x2c));
            iv1 = f(o, caps);
        }
        if iv1 == 0
            && (fp.add(37).read() & 0x400000) != 0
            && (caps.read() & 1) != 0
            && (caps.read() & 0x200) == 0
        {
            _DAT_00803b74 = 2;
        }
        let desc1 = fp.add(9);
        let desc0 = fp.add(5);
        if DAT_00527ea8 == 0 {
            for i in 0..0x1b {
                desc1.add(i).write(0);
            }
            desc1.write(0x6c);
            desc1.add(1).write(1);
            {
                let o = DAT_0066ddd4;
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32, *mut *mut i32, i32) -> i32 =
                    transmute(vt(o, 0x18));
                iv1 = f(o, desc1, &raw mut DAT_0066e700, 0);
            }
            if iv1 != 0 {
                return FUN_004ab970(4);
            }
            iv1 = FUN_004aafe0();
            if iv1 == 0 {
                return FUN_004ab970(0xc);
            }
            for i in 0..0x1b {
                desc0.add(i).write(0);
            }
            desc0.write(0x6c);
            {
                let o = DAT_0066e700;
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 =
                    transmute(vt(o, 0x58));
                f(o, desc0);
            }
            DAT_00803b70 = u_var2;
            if (fp.add(22).read() & 0x20) == 0 && (fp.add(22).read() & 0x40) != 0 {
                DAT_005233d8 = 2;
                if (fp.add(26).read() & 0x400) == 0 {
                    DAT_00527eb8 = 0x22b;
                } else {
                    DAT_00527eb8 = 0x235;
                }
            } else {
                DAT_005233d8 = 1;
                DAT_00527eb8 = 8;
            }
            if !DAT_0066e700.is_null() {
                let o = DAT_0066e700;
                let f: unsafe extern "stdcall" fn(*mut i32) -> i32 = transmute(vt(o, 8));
                f(o);
                DAT_0066e700 = core::ptr::null_mut();
            }
        } else {
            for i in 0..0x1b {
                desc1.add(i).write(0);
            }
            desc1.write(0x6c);
            if DAT_00527eac == 0 {
                desc1.add(1).write(1);
            } else {
                desc1.add(1).write(0x21);
            }
            {
                let o = DAT_0066ddd4;
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32, *mut *mut i32, i32) -> i32 =
                    transmute(vt(o, 0x18));
                iv1 = f(o, desc1, &raw mut DAT_0066e700, 0);
            }
            if iv1 != 0 {
                return FUN_004ab970(4);
            }
            DAT_00527eb8 = 0x235;
            if DAT_005233d8 != 2 {
                DAT_00527eb8 = 8;
            }
            for i in 0..0x1b {
                desc0.add(i).write(0);
            }
            desc0.write(0x6c);
            {
                let o = DAT_0066e700;
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 =
                    transmute(vt(o, 0x58));
                f(o, desc0);
            }
            DAT_00803b70 = u_var2;
            if (fp.add(22).read() & 0x20) == 0 {
                if (fp.add(22).read() & 0x40) != 0 {
                    DAT_005233d8 = 2;
                    DAT_00527eb8 = 0x235;
                    if (fp.add(26).read() & 0x400) == 0 {
                        DAT_00527eb8 = 0x22b;
                    }
                }
            } else {
                DAT_005233d8 = 1;
                DAT_00527eb8 = 8;
            }
            DAT_009d8c08 = DAT_00803b70;
            FUN_004ac400(DAT_0066e700);
            if DAT_00527eac == 0 {
                _DAT_0066ddec = DAT_0066e700;
                DAT_00803f80 = DAT_0066e700;
            } else {
                fp.add(2).write(4);
                {
                    let o = DAT_0066e700;
                    let f: unsafe extern "stdcall" fn(*mut i32, *mut u32, *mut *mut i32) -> i32 =
                        transmute(vt(o, 0x30));
                    iv1 = f(o, fp.add(2), &raw mut DAT_00803f80);
                }
                if iv1 != 0 {
                    return FUN_004ab970(5);
                }
                fp.add(3).write(DAT_005233e0 as usize as u32);
                fp.add(2).write(DAT_005233e8 as usize as u32);
                {
                    let o = DAT_0066ddd4;
                    let f: unsafe extern "stdcall" fn(
                        *mut i32,
                        *mut u32,
                        *mut *mut i32,
                        i32,
                    ) -> i32 = transmute(vt(o, 0x18));
                    iv1 = f(o, fp, &raw mut DAT_0066ddcc, 0);
                }
                if iv1 != 0 {
                    return FUN_004ab970(6);
                }
                FUN_004ac400(DAT_00803f80);
                _DAT_0066ddec = DAT_00803f80;
                FUN_004ac400(DAT_0066ddcc);
            }
            iv1 = FUN_004aafe0();
            if iv1 == 0 {
                return FUN_004ab970(0xc);
            }
        }
        if DAT_00527ea8 != 0 {
            if DAT_005233d8 == 1 {
                let pal = fp.add(121) as *mut i8;
                let mut c6: i8 = 0;
                let mut p = pal;
                loop {
                    p.write(c6);
                    let next = p.add(4);
                    p.add(1).write(c6);
                    p.add(2).write(c6);
                    c6 = c6.wrapping_add(1);
                    p = next;
                    if !(p < pal.add(0x400)) {
                        break;
                    }
                }
                {
                    let o = DAT_0066ddd4;
                    let f: unsafe extern "stdcall" fn(
                        *mut i32,
                        i32,
                        *mut i8,
                        *mut i32,
                        i32,
                    ) -> i32 = transmute(vt(o, 0x14));
                    f(o, 0x4c, pal, &raw mut DAT_0066ddd8, 0);
                }
                if DAT_0066ddd8 != 0 {
                    let o = DAT_0066e700;
                    let f: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 =
                        transmute(vt(o, 0x7c));
                    f(o, DAT_0066ddd8);
                }
            } else {
                DAT_0066ddd8 = 0;
            }
        }
        if DAT_00527e90 != 0 {
            FUN_00408170();
        }
        1
    }
}
