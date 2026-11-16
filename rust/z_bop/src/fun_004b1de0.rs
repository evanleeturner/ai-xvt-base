// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664c80: *mut u32;
    static mut DAT_00528ca8: i32;
    static mut _DAT_00528ca4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b2130(param_1: u32);
}

#[allow(non_snake_case)]
unsafe fn emit_records(param_1: &mut *mut u32, count: i32) {
    // SAFETY: the output buffer and the record array are the game's own memory;
    // writes and reads happen where the original does them.
    unsafe {
        DAT_00664c80.cast::<u8>().write(3);
        DAT_00664c80.cast::<u8>().byte_add(1).write(8);
        DAT_00664c80
            .cast::<u8>()
            .byte_add(2)
            .cast::<i16>()
            .write_unaligned(count as i16);
        DAT_00664c80 = DAT_00664c80.wrapping_add(1);
        let mut n = count;
        while n != 0 {
            let p = *param_1;
            let d = DAT_00664c80;
            d.cast::<i16>().write_unaligned(p.read_unaligned() as i16);
            d.cast::<u8>()
                .byte_add(2)
                .cast::<i16>()
                .write_unaligned(p.wrapping_add(1).read_unaligned() as i16);
            d.wrapping_add(1)
                .cast::<i16>()
                .write_unaligned(p.wrapping_add(2).read_unaligned() as i16);
            d.cast::<u8>()
                .byte_add(6)
                .cast::<u16>()
                .write_unaligned(0x700);
            DAT_00664c80 = DAT_00664c80.wrapping_add(2);
            *param_1 = p.wrapping_add(5);
            n = n.wrapping_sub(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b1de0(param_1: *mut u32, param_2: u32) -> u32 {
    let mut param_1 = param_1;
    let mut local_c: u32 = 0;
    // SAFETY: the globals and callee are the game's own; param_1 points at
    // an array of 5-word records of at least param_2 entries.
    unsafe {
        if param_2 != 0 {
            loop {
                let mut i_var4: i32 = 0;
                let i_var2 = param_1.wrapping_add(4).read_unaligned() as i32;
                if i_var2 == 0 {
                    let mut u_var1 = local_c;
                    let mut p = param_1;
                    while u_var1 < param_2
                        && p.wrapping_add(4).read_unaligned() == 0
                        && p.wrapping_add(3).read_unaligned()
                            == param_1.wrapping_add(3).read_unaligned()
                    {
                        i_var4 = i_var4.wrapping_add(1);
                        u_var1 = u_var1.wrapping_add(1);
                        p = p.wrapping_add(5);
                    }
                    FUN_004b2130(param_1.wrapping_add(3).read_unaligned());
                    if DAT_00528ca8 != 0 {
                        let d = DAT_00664c80;
                        d.cast::<u8>().write(8);
                        d.cast::<u8>().byte_add(1).write(8);
                        d.cast::<u8>().byte_add(2).cast::<u16>().write_unaligned(1);
                        DAT_00664c80 = DAT_00664c80.wrapping_add(1);
                        DAT_00664c80.write_unaligned(1);
                        DAT_00664c80.wrapping_add(1).write_unaligned(0);
                        DAT_00528ca8 = 0;
                        DAT_00664c80 = DAT_00664c80.wrapping_add(2);
                    }
                    emit_records(&mut param_1, i_var4);
                } else {
                    let mut u_var1 = local_c;
                    let mut p = param_1;
                    while u_var1 < param_2
                        && p.wrapping_add(4).read_unaligned() as i32 == i_var2
                        && p.wrapping_add(3).read_unaligned()
                            == param_1.wrapping_add(3).read_unaligned()
                    {
                        i_var4 = i_var4.wrapping_add(1);
                        u_var1 = u_var1.wrapping_add(1);
                        p = p.wrapping_add(5);
                    }
                    FUN_004b2130(param_1.wrapping_add(3).read_unaligned());
                    if param_1.wrapping_add(4).read_unaligned() as i32 != DAT_00528ca8 {
                        let d = DAT_00664c80;
                        d.cast::<u8>().write(8);
                        d.cast::<u8>().byte_add(1).write(8);
                        d.cast::<u8>().byte_add(2).cast::<u16>().write_unaligned(1);
                        DAT_00664c80 = DAT_00664c80.wrapping_add(1);
                        DAT_00664c80.write_unaligned(1);
                        DAT_00664c80.wrapping_add(1).write_unaligned(
                            (i_var2.wrapping_add(0x74) as usize as *const u32).read_unaligned(),
                        );
                        DAT_00664c80 = DAT_00664c80.wrapping_add(2);
                        DAT_00528ca8 = i_var2;
                    }
                    emit_records(&mut param_1, i_var4);
                }
                local_c = local_c.wrapping_add(i_var4 as u32);
                if !(local_c < param_2) {
                    break;
                }
            }
        }
        _DAT_00528ca4 = _DAT_00528ca4.wrapping_add(param_2);
    }
    1
}
