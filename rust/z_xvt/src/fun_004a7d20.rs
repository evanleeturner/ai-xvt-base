// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ac28: u8;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0052552c: u32;
    static mut DAT_00525550: u32;
    static mut DAT_00655a94: u16;
    static mut DAT_00655a9c: u16;
    static mut DAT_00655aa0: u16;
    static mut DAT_00655a9e: u16;
    static mut DAT_00655a9a: u16;
    static mut DAT_00655aa2: u16;
    static mut DAT_00655414: u16;
    static mut DAT_0065517e: u16;
    static mut DAT_0065517a: u16;
    static mut DAT_00655a90: u16;
    static mut DAT_0065517c: u16;
    static mut DAT_006551cc: u16;
    static mut DAT_00655186: u16;
    static mut DAT_00655188: u16;
    static mut DAT_006551c8: u16;
    static mut DAT_00655a92: u16;
    static mut DAT_00655416: u16;
    static mut DAT_00655418: u16;
    static mut DAT_00655810: u16;
    static mut DAT_00655176: u16;
    static mut DAT_0071b743: [i8; 0];
    static mut DAT_00655184: u16;
    static mut DAT_00655fb0: [u8; 0];
    static mut DAT_00655fb2: [u8; 0];
    static mut DAT_00656c90: [u16; 0];
    static mut DAT_00655f8e: u16;
    static mut DAT_00655a84: u16;
    static mut DAT_00655812: u16;
    static mut DAT_0065518a: u16;
    static mut DAT_00655a8e: u16;
    static mut DAT_00655174: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004af9c0();
    fn FUN_004afb90();
    fn FUN_004a80d0(param_1: u32);
    fn FUN_004a85e0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a7d20(param_1: u32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // array accesses follow the original's indexing.
    unsafe {
        DAT_0070ac28 = 1;
        if param_1 == DAT_0071ecf4 {
            if DAT_0052552c == 0x13 {
                DAT_00655a94 = 6;
                DAT_00655a9c = 0x8d;
                DAT_00655aa0 = 0x67;
                DAT_00655a9e = 100;
                DAT_00655a9a = 0x50;
                DAT_00655aa2 = 0x35;
                DAT_00655414 = 0x2f;
                DAT_0065517e = 2;
                DAT_0065517a = 0x6d;
                DAT_00655a90 = 0x35;
                DAT_0065517c = 0x46;
                DAT_006551cc = 0x79;
                DAT_00655186 = 0xba;
                DAT_00655188 = 0x6f;
                DAT_006551c8 = 0x34;
                DAT_00655a92 = 0x99;
                DAT_00655416 = 0x99;
                DAT_00655418 = 0x35;
                DAT_00655810 = 0x70;
                DAT_00655176 = 0x70;
            } else if DAT_0052552c == 0x101 {
                DAT_00655a94 = 0xc;
                DAT_00655a9c = 0x11a;
                DAT_00655aa0 = 0xce;
                DAT_00655a9e = 200;
                DAT_00655a9a = 0xa0;
                DAT_00655aa2 = 0x6b;
                DAT_00655810 = 0xe1;
                DAT_00655414 = 0x5e;
                DAT_0065517e = 4;
                DAT_0065517a = 0xdb;
                DAT_00655176 = 0xe0;
                DAT_0065517c = 0x8c;
                DAT_006551cc = 0xf2;
                DAT_00655186 = 0x175;
                DAT_00655188 = 0xdf;
                DAT_006551c8 = 0x68;
                DAT_00655a90 = 0x6b;
                DAT_00655a92 = 0x132;
                DAT_00655416 = 0x132;
                DAT_00655418 = 0x6b;
            } else if DAT_0052552c == 0x112 {
                DAT_00655a94 = 9;
                DAT_00655a9c = 0xd3;
                DAT_00655aa0 = 0x9a;
                DAT_00655a9e = 0x96;
                DAT_00655a9a = 0x78;
                DAT_00655aa2 = 0x50;
                DAT_00655414 = 0x46;
                DAT_0065517e = 3;
                DAT_0065517a = 0xa4;
                DAT_00655a90 = 0x50;
                DAT_0065517c = 0x69;
                DAT_006551cc = 0xb5;
                DAT_00655186 = 0x117;
                DAT_00655188 = 0xa7;
                DAT_006551c8 = 0x4e;
                DAT_00655a92 = 0xe5;
                DAT_00655416 = 0xe5;
                DAT_00655418 = 0x50;
                DAT_00655810 = 0xa8;
                DAT_00655176 = 0xa8;
            }
            let idx = DAT_0071ecf4.wrapping_mul(0x5bd) as usize;
            if *(&raw mut DAT_0071b743).cast::<i8>().add(idx) != 0 {
                let off = (DAT_00655184 as u32).wrapping_mul(0xc) as usize;
                DAT_00655810 = (&raw mut DAT_00655fb0)
                    .cast::<u8>()
                    .add(off)
                    .cast::<u16>()
                    .read_unaligned();
                DAT_00655414 = (&raw mut DAT_00655fb2)
                    .cast::<u8>()
                    .add(off)
                    .cast::<u16>()
                    .read_unaligned();
            }
            let arr = (&raw mut DAT_00656c90).cast::<u16>();
            let mut u_var2: u32 = 0;
            loop {
                let mut u_var1: u16 = 0;
                loop {
                    let base = u_var2.wrapping_mul(0x78) as usize;
                    if u_var1 < 0x6a || 0x6e < u_var1 {
                        arr.add(base + u_var1 as usize).write(0xfffe);
                    } else {
                        arr.add(base + u_var1 as usize).write(0xfffd);
                    }
                    if u_var1 == 0x5d {
                        arr.add(base + 0x5d).write(0xfffd);
                    }
                    u_var1 = u_var1.wrapping_add(1);
                    if u_var1 >= 0x78 {
                        break;
                    }
                }
                let v = (u_var2 as u16).wrapping_add(1);
                u_var2 = v as u32;
                if v >= 3 {
                    break;
                }
            }
            DAT_00655f8e = 1;
            DAT_00655a84 = 0xffff;
            DAT_00655812 = 0;
            DAT_0065518a = 0;
            DAT_00655a8e = 0;
            DAT_00655174 = 0;
            FUN_004af9c0();
            if DAT_00525550 == 0 {
                FUN_004afb90();
            }
            FUN_004a80d0(DAT_0071ecf4);
            FUN_004a85e0(DAT_0071ecf4);
        }
        DAT_0070ac28 = 0;
    }
}
