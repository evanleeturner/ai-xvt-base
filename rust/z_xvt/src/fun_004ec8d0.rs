// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757c84: i32;
    static mut DAT_00533540: i32;
    static mut DAT_00743daf: i32;
    static mut DAT_00757cb0: i32;
    static mut DAT_007438d5: i32;
    static mut DAT_007579d0: i32;
    static mut DAT_00757870: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_00757680: i32;
    static mut DAT_00757cfe: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ec8d0(param_1: i32) -> i32 {
    // SAFETY: all globals are the game's own memory; offsets are computed
    // exactly as the original does and read where the original reads.
    unsafe {
        let tbl = (&raw mut DAT_00533540).cast::<u8>();
        let b_daf = (&raw mut DAT_00743daf).cast::<u8>();
        let b_8d5 = (&raw mut DAT_007438d5).cast::<u8>();

        if param_1 == -1 {
            if DAT_00757c84 != 0 {
                let idx = DAT_00757c84.wrapping_add(DAT_00757cb0.wrapping_mul(0x562));
                let b = b_daf.wrapping_offset(idx as isize).read();
                return tbl
                    .wrapping_offset((b as u32).wrapping_mul(4) as i32 as isize)
                    .cast::<i32>()
                    .read_unaligned();
            }
            let idx = DAT_00757cb0.wrapping_mul(0x562);
            let b = b_8d5.wrapping_offset(idx as isize).read();
            if b != 0 {
                return tbl
                    .wrapping_offset((b as u32).wrapping_mul(4) as i32 as isize)
                    .cast::<i32>()
                    .read_unaligned();
            }
            return 0;
        }

        let counts = (&raw mut DAT_007579d0).cast::<i32>();
        let list = (&raw mut DAT_00757870).cast::<i32>();
        let b_680 = (&raw mut DAT_00757680).cast::<u8>();
        let key_p = (&raw mut DAT_00757cee)
            .cast::<u8>()
            .wrapping_offset(param_1.wrapping_mul(0x2a) as isize);

        let mut local_4: i32 = 0;
        let mut iv2: i32 = 0;
        let mut k: usize = 0;
        while k < 10 {
            let cnt = counts.add(k).read();
            let mut iv3: i32 = 0;
            if 0 < cnt {
                let mut p1 = list.add(k * 8);
                loop {
                    if p1.read() == key_p.cast::<i32>().read_unaligned() {
                        let off = iv3.wrapping_add(iv2).wrapping_mul(4);
                        local_4 = b_680
                            .wrapping_offset(off as isize)
                            .cast::<i32>()
                            .read_unaligned();
                        break;
                    }
                    p1 = p1.add(1);
                    iv3 = iv3.wrapping_add(1);
                    if !(iv3 < counts.add(k).read()) {
                        break;
                    }
                }
            }
            iv2 = iv2.wrapping_add(8);
            k += 1;
        }

        let sel = (&raw mut DAT_00757cfe)
            .cast::<u8>()
            .wrapping_offset(param_1.wrapping_mul(0x2a) as isize)
            .cast::<i32>()
            .read_unaligned();
        if sel != 0 {
            let idx = sel.wrapping_add(local_4.wrapping_mul(0x562));
            let b = b_daf.wrapping_offset(idx as isize).read();
            return tbl
                .wrapping_offset((b as u32).wrapping_mul(4) as i32 as isize)
                .cast::<i32>()
                .read_unaligned();
        }
        let b = b_8d5
            .wrapping_offset(local_4.wrapping_mul(0x562) as isize)
            .read();
        if b != 0 {
            return tbl
                .wrapping_offset((b as u32).wrapping_mul(4) as i32 as isize)
                .cast::<i32>()
                .read_unaligned();
        }
        0
    }
}
