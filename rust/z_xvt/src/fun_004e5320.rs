// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758620: [u32; 1];
    static mut DAT_0081bfd2: [u8; 32];
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfbe: [u8; 1];
    static mut DAT_0081bcb8: i32;
    static mut DAT_00757cee: [u8; 1];
    static mut DAT_00757e3e: [u8; 1];
    static mut DAT_00757cf6: [u8; 1];
    static mut DAT_00757e44: [u8; 1];
    static mut DAT_0085e7b4: u8;
    static mut DAT_0085e7b5: u8;
    static mut DAT_0085e7b6: u8;
    static mut DAT_0085e7b7: u8;
    static mut DAT_0085e7b8: u8;
    static mut DAT_0085e7b9: u8;
    static mut DAT_0085e7ba: u8;
    static mut DAT_0085e7bb: u8;
    static mut DAT_0085e7bc: u8;
    static mut DAT_0085e7bd: u8;
    static mut DAT_0085e7be: u8;
    static mut DAT_0085e7bf: u8;
    static mut DAT_0085e78f: u8;
    static mut DAT_0085e7c4: u8;
    static mut DAT_0085e790: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc7b0(param_1: *mut i32) -> *mut u32;
    fn FUN_004ced20(param_1: i32) -> u32;
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32);
    fn FUN_004cde00() -> u32;
    #[link_name = "rand"]
    fn _rand() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e5320(param_1: i32) {
    // SAFETY: all globals and callees belong to the game; pointers are used
    // at the offsets the original uses, with unaligned access where packed.
    unsafe {
        let buf = (&raw mut DAT_00758620) as *mut u32;
        let w = |i: i32, v: u32| {
            buf.offset(i as isize).write(v);
        };
        let mut a8: [i32; 2] = [0, 0];
        let a8p = a8.as_mut_ptr();
        let u_size: u32;

        w(0, 0x41);
        core::ptr::copy_nonoverlapping(
            (&raw const DAT_0081bfd2) as *const u8,
            buf.add(1) as *mut u8,
            32,
        );
        let u_idx = ((&raw const DAT_0081bfba) as *const u32).read_unaligned();
        w(10, u_idx);
        let u_elem = ((&raw const DAT_0081bfbe) as *const u8)
            .byte_add(u_idx.wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned();
        w(11, u_elem);
        if DAT_0081bcb8 == 0 {
            w(12, 8);
            let mut iv10: i32 = 0xe;
            w(13, FUN_004cde00());
            let mut iv6: i32 = 0;
            let p4 = FUN_004cc7b0(a8p);
            if 0 < a8p.read() {
                let mut p7 = p4.add(8) as *mut i32;
                loop {
                    if p7.add(1).read() == 1 {
                        w(iv10, p7.read() as u32);
                        iv10 = iv10.wrapping_add(3);
                        let b = (p7.offset(-8) as *const u8).read();
                        w(iv10.wrapping_sub(2), (b as u32).wrapping_sub(1));
                        let v5 = FUN_004ced20(p7.read());
                        w(iv10.wrapping_sub(1), v5);
                    }
                    p7 = p7.add(10);
                    iv6 = iv6.wrapping_add(1);
                    if !(iv6 < a8p.read()) {
                        break;
                    }
                }
            }
            u_size = (iv10 as u32).wrapping_shl(2);
        } else {
            w(12, 8);
            w(13, 8);
            let p4 = FUN_004cc7b0(a8p);
            a8[1] = 0xe;
            let mut pb7 = (&raw mut DAT_00757cee) as *mut u8;
            let end1 = (&raw mut DAT_00757e3e) as *mut u8 as usize;
            loop {
                let mut iv6: i32 = 0;
                let mut b11 = a8p.read() == 0;
                if 0 < a8p.read() {
                    let mut p8 = p4.add(8) as *mut i32;
                    loop {
                        let t0 = pb7.cast::<i32>().read_unaligned();
                        if p8.read() != 0 && p8.add(1).read() != 0 && t0 == p8.read() {
                            let b =
                                (p4.offset((iv6.wrapping_mul(10)) as isize) as *const u8).read();
                            let t1 = (b as i32).wrapping_sub(1);
                            pb7.add(4).cast::<i32>().write_unaligned(t1);
                            break;
                        }
                        p8 = p8.add(10);
                        iv6 = iv6.wrapping_add(1);
                        if !(iv6 < a8p.read()) {
                            break;
                        }
                    }
                    b11 = iv6 == a8p.read();
                }
                if b11 {
                    pb7.cast::<i32>().write_unaligned(0);
                }
                pb7 = pb7.add(0x2a);
                if !((pb7 as usize) < end1) {
                    break;
                }
            }
            let mut iv6: i32 = 0xe;
            pb7 = (&raw mut DAT_00757cee) as *mut u8;
            loop {
                let iv10 = pb7.cast::<i32>().read_unaligned();
                let iv1 = pb7.add(4).cast::<i32>().read_unaligned();
                w(iv6, iv10 as u32);
                w(iv6.wrapping_add(1), iv1 as u32);
                iv6 = iv6.wrapping_add(3);
                pb7 = pb7.add(0x2a);
                let v5 = FUN_004ced20(iv10);
                w(iv6.wrapping_sub(1), v5);
                if !((pb7 as usize) < end1) {
                    break;
                }
            }
            FUN_004cb830(param_1, buf, (iv6 as u32).wrapping_mul(4));
            iv6 = 1;
            w(0, 0x71);
            pb7 = (&raw mut DAT_00757cf6) as *mut u8;
            let end2 = ((&raw mut DAT_00757e44) as *mut u8 as usize).wrapping_add(2);
            loop {
                let u2 = pb7.add(4).cast::<u32>().read_unaligned();
                w(iv6, pb7.cast::<u32>().read_unaligned());
                let u3 = pb7.add(8).cast::<u32>().read_unaligned();
                let pb9 = pb7.add(0x2a);
                w(iv6.wrapping_add(1), u2);
                iv6 = iv6.wrapping_add(5);
                let u2 = pb7.add(12).cast::<u32>().read_unaligned();
                w(iv6.wrapping_sub(3), u3);
                let u3 = pb7.add(16).cast::<u32>().read_unaligned();
                w(iv6.wrapping_sub(2), u2);
                w(iv6.wrapping_sub(1), u3);
                pb7 = pb9;
                if !((pb9 as usize) < end2) {
                    break;
                }
            }
            u_size = (iv6 as u32).wrapping_mul(4);
        }
        FUN_004cb830(param_1, buf, u_size);
        w(0, 0x59);
        w(1, DAT_0085e7b4 as u32);
        w(2, DAT_0085e7b5 as u32);
        w(3, DAT_0085e7b6 as u32);
        w(4, DAT_0085e7b7 as u32);
        w(5, DAT_0085e7b8 as u32);
        w(6, DAT_0085e7b9 as u32);
        w(7, DAT_0085e7ba as u32);
        w(8, DAT_0085e7bb as u32);
        w(9, DAT_0085e7bc as u32);
        w(10, DAT_0085e7bd as u32);
        w(11, DAT_0085e7be as u32);
        w(12, DAT_0085e7bf as u32);
        w(13, _rand() as u32);
        w(14, DAT_0085e78f as u32);
        w(15, DAT_0085e7c4 as u32);
        w(16, DAT_0085e790 as u32);
        FUN_004cb830(param_1, buf, 0x44);
    }
}
