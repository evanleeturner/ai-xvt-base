// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0078073a: u32;
    static mut DAT_00780c5a: u32;
    static mut DAT_00758840: u32;
    static mut _DAT_0078109e: u32;
    static mut DAT_00780732: u32;
    static mut DAT_0078075a: u32;
    static mut DAT_007806fa: u32;
    static mut DAT_0078077a: u32;
    static mut DAT_00780726: u32;
    static mut DAT_00780c7a: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ca450();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5680() {
    let mut local_48: [u32; 10] = [0; 10];
    let mut local_20: [u32; 8] = [0; 8];
    // SAFETY: the globals are the game's own memory; the pointers walk the
    // same ranges, in the same order, as the original's loops.
    unsafe {
        let mut p4: *mut u32 = &raw mut DAT_0078073a;
        let mut p5: *mut u32 = local_20.as_mut_ptr();
        let mut i: i32 = 8;
        while i != 0 {
            p5.write_unaligned(p4.read_unaligned());
            p4 = p4.add(1);
            p5 = p5.add(1);
            i -= 1;
        }
        p4 = &raw mut DAT_00780c5a;
        p5 = local_48.as_mut_ptr();
        i = 10;
        while i != 0 {
            p5.write_unaligned(p4.read_unaligned());
            p4 = p4.add(1);
            p5 = p5.add(1);
            i -= 1;
        }
        p4 = &raw mut DAT_00758840;
        i = 0x30ba7;
        while i != 0 {
            p4.write_unaligned(0);
            p4 = p4.add(1);
            i -= 1;
        }
        p4 = local_20.as_mut_ptr();
        p5 = &raw mut DAT_0078073a;
        i = 8;
        while i != 0 {
            p5.write_unaligned(p4.read_unaligned());
            p4 = p4.add(1);
            p5 = p5.add(1);
            i -= 1;
        }
        p4 = local_48.as_mut_ptr();
        p5 = &raw mut DAT_00780c5a;
        i = 10;
        while i != 0 {
            p5.write_unaligned(p4.read_unaligned());
            p4 = p4.add(1);
            p5 = p5.add(1);
            i -= 1;
        }
        _DAT_0078109e = 1;
        DAT_00780732 = 1;
        p4 = local_48.as_mut_ptr();
        p5 = &raw mut DAT_0078075a;
        i = 10;
        while i != 0 {
            p5.write_unaligned(p4.read_unaligned());
            p4 = p4.add(1);
            p5 = p5.add(1);
            i -= 1;
        }
        if DAT_007806fa != 0 {
            FUN_004ca450();
            let mut i_var2: i32 = 0;
            let end = &raw mut DAT_00780c7a as usize;
            let mut p3: *mut u32 = &raw mut DAT_0078077a;
            loop {
                if p3.read_unaligned() == DAT_00780726 {
                    break;
                }
                p3 = p3.add(10);
                i_var2 += 1;
                if (p3 as usize) >= end {
                    break;
                }
            }
            if i_var2 == 0x20 {
                p3 = &raw mut DAT_0078077a;
                DAT_00780726 = DAT_00780c7a;
                loop {
                    let u_var1 = p3.read_unaligned();
                    if u_var1 != 0 && p3.add(1).read_unaligned() != 0 && u_var1 < DAT_00780726 {
                        DAT_00780726 = u_var1;
                    }
                    p3 = p3.add(10);
                    if (p3 as usize) >= end {
                        break;
                    }
                }
            }
        }
    }
}
