// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: [i8; 1];
    static mut DAT_0071bb37: [i8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00655c48: u16;
    static mut DAT_00655c4a: u16;
    static mut DAT_00655c4c: u16;
    static mut DAT_00655c4e: u16;
    static mut DAT_0071f2de: u16;
    static mut DAT_00655f8e: u16;
    static mut DAT_00655f84: u16;
    static mut DAT_00655f86: u16;
    static mut DAT_00655f88: u16;
    static mut DAT_00655f8a: u16;
    static mut DAT_00655f8c: u16;
    static mut DAT_006d4a4e: u16;
    static mut DAT_00656cd4: u16;
    static mut DAT_0071f2f0: [u32; 1];
    static mut DAT_00656c44: [*mut u8; 1];
    static mut DAT_0070a184: unsafe extern "C" fn(u32, u32, u32, u32, u32);
    static mut DAT_00703c80: unsafe extern "C" fn();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6f90();
    fn FUN_0040a1a0(param_1: u32, param_2: u32, param_3: i32) -> u32;
    fn FUN_004a6a80();
    fn FUN_0048ee20(param_1: i8);
    fn FUN_0048ed00(a: u32, b: u32, c: u32, d: u32);
    fn FUN_0048ece0(a: u32, b: u32);
    fn FUN_0048ed70(a: u32);
    fn FUN_0048eda0(a: u32);
    fn FUN_0048f270(param_3: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004af530() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers read from game structures are dereferenced where the original does.
    unsafe {
        let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
        let iv2: i32 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .add(idx as usize)
            .cast::<i32>()
            .read_unaligned();

        let b3: bool = if iv2 == -1 {
            false
        } else {
            let a = (iv2 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32);
            let c = (a as *const i8).read();
            !(((c != 1 && c != 2) && c != 3) && (c != 0x0e && c != 4))
        };

        if b3 && (DAT_00655c48 as u32).wrapping_add(DAT_00655c4a as u32) != 0 {
            let qa = (iv2 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988 as u32);
            let q = (qa as *const u32).read_unaligned();
            let base = ((q.wrapping_add(0xa7)) as *const i32).read_unaligned();
            let p = base as u32 as *const u8;

            let s0 = p.byte_add(199).cast::<u16>().read_unaligned();
            let mut u5: u16 = s0 / 3;
            let mut u4: u16 = 2;
            if u5 != 0 {
                let s1 = p.byte_add(0xc3).cast::<u16>().read_unaligned();
                u4 = s1 / u5;
            }

            let t0 = p.byte_add(0x20b).cast::<u16>().read_unaligned();
            let t1 = p.byte_add(0x20d).cast::<u16>().read_unaligned();
            if t0.wrapping_add(t1) < 100 && u4 == 2 {
                u5 = (DAT_0071f2de / 0x3b) & 1;
                if u5 != 0 {
                    FUN_004a6f90();
                    FUN_0040a1a0(0x22, 0xffff_ffff, DAT_0071ecf4);
                    FUN_004a6a80();
                    DAT_00655f8e = DAT_00655f8e.wrapping_add(1);
                    if 0x2ef < DAT_00655f8e {
                        DAT_00655f8e = 1;
                    }
                }
            } else {
                u5 = 0;
            }

            let flag = (&raw mut DAT_0071bb37)
                .cast::<i8>()
                .byte_add(idx as usize)
                .read();
            if flag == 0 {
                if DAT_00656cd4 != u5 {
                    DAT_00656cd4 = u5;
                    let i = (u5 as u32).wrapping_add(DAT_00655c4c as u32) as usize;
                    let v = (&raw mut DAT_0071f2f0).cast::<u32>().add(i).read();
                    (DAT_0070a184)(
                        v,
                        DAT_00655c48 as u32,
                        DAT_00655c4a as u32,
                        DAT_00655c4e as u32,
                        0,
                    );
                }
                if DAT_00655f8e != 0 {
                    FUN_0048ee20(0);
                    FUN_0048ed00(
                        DAT_00655f84 as u32,
                        DAT_00655f86 as u32,
                        (DAT_00655f84 as u32).wrapping_add(DAT_00655f8c as u32),
                        (DAT_006d4a4e as u32).wrapping_add(DAT_00655f86 as u32),
                    );
                    FUN_0048ece0(DAT_00655f84 as u32, DAT_00655f86 as u32);
                    FUN_0048ed70((u5 as u32).wrapping_add(DAT_00655f8a as u32));
                    let mut u8v: u32 = 0;
                    FUN_0048eda0(
                        (((u5 == 0) as u32).wrapping_sub(1) & 2).wrapping_add(DAT_00655f88 as u32),
                    );
                    if 0x200 < DAT_00655f8e {
                        u5 = DAT_00655f8e.wrapping_sub(0x200);
                        (DAT_00703c80)();
                        u8v = (u5 >> 4) as u32;
                    }
                    let ptr = (&raw mut DAT_00656c44)
                        .cast::<*mut u8>()
                        .add(u8v as usize)
                        .read();
                    FUN_0048f270(ptr);
                }
            }
        }
    }
}
