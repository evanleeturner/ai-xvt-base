// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ee60: *mut u32;
    static mut DAT_0052f20c: i32;
    static mut DAT_0052f208: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b821c(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5d64(param_1: *mut u32, param_2: u32) -> u32 {
    let mut local_18: *mut u32 = param_1;
    let mut local_c: u32 = 0;
    // SAFETY: the globals and callee are the game's own; param_1 points at an
    // array of param_2 five-word records and DAT_0064ee60 at a writable output
    // buffer, accessed exactly where the original does.
    unsafe {
        while local_c < param_2 {
            let mut local_24: u32 = 0;
            if local_18.add(3).read_unaligned() == 0 {
                let mut local_14 = local_18;
                let mut local_1c = local_c;
                while local_1c < param_2
                    && local_14.add(3).read_unaligned() == 0
                    && local_14.add(4).read_unaligned() == local_18.add(4).read_unaligned()
                {
                    local_24 = local_24.wrapping_add(1);
                    local_1c = local_1c.wrapping_add(1);
                    local_14 = local_14.wrapping_add(5);
                }
                FUN_004b821c(local_18.add(4).read_unaligned());
                if DAT_0052f20c != 0 {
                    let p = DAT_0064ee60;
                    p.cast::<u8>().write_unaligned(8);
                    p.cast::<u8>().byte_add(1).write_unaligned(8);
                    p.cast::<u16>().byte_add(2).write_unaligned(1);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(1);
                    DAT_0064ee60.write_unaligned(1);
                    DAT_0064ee60.wrapping_add(1).write_unaligned(0);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(2);
                    DAT_0052f20c = 0;
                }
                let p = DAT_0064ee60;
                p.cast::<u8>().write_unaligned(3);
                p.cast::<u8>().byte_add(1).write_unaligned(8);
                p.cast::<i16>().byte_add(2).write_unaligned(local_24 as i16);
                DAT_0064ee60 = DAT_0064ee60.wrapping_add(1);
                let mut local_10: u32 = 0;
                while local_10 < local_24 {
                    let p = DAT_0064ee60;
                    p.cast::<i16>()
                        .write_unaligned(local_18.read_unaligned() as i16);
                    p.cast::<i16>()
                        .byte_add(2)
                        .write_unaligned(local_18.add(1).read_unaligned() as i16);
                    p.wrapping_add(1)
                        .cast::<i16>()
                        .write_unaligned(local_18.add(2).read_unaligned() as i16);
                    p.cast::<u16>().byte_add(6).write_unaligned(0x700);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(2);
                    local_18 = local_18.wrapping_add(5);
                    local_10 = local_10.wrapping_add(1);
                }
            } else {
                let mut local_14 = local_18;
                let mut local_1c = local_c;
                let i_var1: i32 = local_18.add(3).read_unaligned() as i32;
                while local_1c < param_2
                    && local_14.add(3).read_unaligned() as i32 == i_var1
                    && local_14.add(4).read_unaligned() == local_18.add(4).read_unaligned()
                {
                    local_24 = local_24.wrapping_add(1);
                    local_1c = local_1c.wrapping_add(1);
                    local_14 = local_14.wrapping_add(5);
                }
                FUN_004b821c(local_18.add(4).read_unaligned());
                if local_18.add(3).read_unaligned() as i32 != DAT_0052f20c {
                    let p = DAT_0064ee60;
                    p.cast::<u8>().write_unaligned(8);
                    p.cast::<u8>().byte_add(1).write_unaligned(8);
                    p.cast::<u16>().byte_add(2).write_unaligned(1);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(1);
                    DAT_0064ee60.write_unaligned(1);
                    let v = (i_var1.wrapping_add(0x88) as usize as *const u32).read_unaligned();
                    DAT_0064ee60.wrapping_add(1).write_unaligned(v);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(2);
                    DAT_0052f20c = i_var1;
                }
                let p = DAT_0064ee60;
                p.cast::<u8>().write_unaligned(3);
                p.cast::<u8>().byte_add(1).write_unaligned(8);
                p.cast::<i16>().byte_add(2).write_unaligned(local_24 as i16);
                DAT_0064ee60 = DAT_0064ee60.wrapping_add(1);
                let mut local_10: u32 = 0;
                while local_10 < local_24 {
                    let p = DAT_0064ee60;
                    p.cast::<i16>()
                        .write_unaligned(local_18.read_unaligned() as i16);
                    p.cast::<i16>()
                        .byte_add(2)
                        .write_unaligned(local_18.add(1).read_unaligned() as i16);
                    p.wrapping_add(1)
                        .cast::<i16>()
                        .write_unaligned(local_18.add(2).read_unaligned() as i16);
                    p.cast::<u16>().byte_add(6).write_unaligned(0x700);
                    DAT_0064ee60 = DAT_0064ee60.wrapping_add(2);
                    local_18 = local_18.wrapping_add(5);
                    local_10 = local_10.wrapping_add(1);
                }
            }
            local_c = local_c.wrapping_add(local_24);
        }
        DAT_0052f208 = DAT_0052f208.wrapping_add(param_2);
    }
    1
}
