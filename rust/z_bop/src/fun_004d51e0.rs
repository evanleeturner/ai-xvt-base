// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acec05: u32;
    static mut DAT_00acf125: u32;
    static mut DAT_00aa6d00: u32;
    static mut _DAT_00acf569: u32;
    static mut DAT_00acebfd: u32;
    static mut DAT_00acec25: u32;
    static mut DAT_00acebc5: u32;
    static mut DAT_00acec45: u32;
    static mut DAT_00acebf1: u32;
    static mut DAT_00acf145: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cdc20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d51e0() {
    let mut local_48: [u32; 10] = [0; 10];
    let mut local_20: [u32; 8] = [0; 8];
    // SAFETY: the globals are the game's own memory; all accesses are
    // unaligned-safe and stay within the ranges the original touches.
    unsafe {
        let mut src = &raw mut DAT_00acec05 as *const u32;
        let mut dst = local_20.as_mut_ptr();
        let mut n = 8;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        let mut src = &raw mut DAT_00acf125 as *const u32;
        let mut dst = local_48.as_mut_ptr();
        let mut n = 10;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        let mut p = &raw mut DAT_00aa6d00;
        let mut n = 0x30be9;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        p.cast::<u16>().write_unaligned(0);
        p.cast::<u8>().add(2).write(0);

        let mut src = local_20.as_ptr();
        let mut dst = &raw mut DAT_00acec05;
        let mut n = 8;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        let mut src = local_48.as_ptr();
        let mut dst = &raw mut DAT_00acf125;
        let mut n = 10;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        _DAT_00acf569 = 1;
        DAT_00acebfd = 1;
        let mut src = local_48.as_ptr();
        let mut dst = &raw mut DAT_00acec25;
        let mut n = 10;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        if DAT_00acebc5 != 0 {
            FUN_004cdc20();
            let limit = &raw mut DAT_00acf145 as usize;
            let mut i: i32 = 0;
            let mut p = &raw mut DAT_00acec45;
            loop {
                if p.read_unaligned() == DAT_00acebf1 {
                    break;
                }
                p = p.add(10);
                i = i.wrapping_add(1);
                if !((p as usize) < limit) {
                    break;
                }
            }
            if i == 0x20 {
                let mut p = &raw mut DAT_00acec45;
                DAT_00acebf1 = DAT_00acf145;
                loop {
                    let u = p.read_unaligned();
                    if u != 0 && p.add(1).read_unaligned() != 0 && u < DAT_00acebf1 {
                        DAT_00acebf1 = u;
                    }
                    p = p.add(10);
                    if !((p as usize) < limit) {
                        break;
                    }
                }
            }
        }
    }
}
