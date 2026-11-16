// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec608: i8;
    static mut DAT_00999548: i32;
    static mut DAT_009994fc: i32;
    static mut DAT_0099f6e0: i32;
    static mut DAT_005597a0: [u32; 4];
    static mut DAT_005597b0: [u32; 0];
    static mut DAT_00999be8: [u32; 0];
    static mut DAT_00999550: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046e040() -> i32;
    fn FUN_00448d70() -> u32;
    fn FUN_0046c730();
    fn FUN_0046fa10(param_1: i32);
    fn FUN_0046fa50(param_1: u32);
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn fill_list() {
    // SAFETY: the globals are the game's own tables, read and written where the original does.
    unsafe {
        let mut i_var6: i32 = 0;
        if 0 < DAT_0099f6e0 {
            let mut src = (&raw mut DAT_00999be8).cast::<u32>();
            let mut dst = (&raw mut DAT_005597b0).cast::<u32>();
            loop {
                i_var6 = i_var6.wrapping_add(1);
                dst.write_unaligned(src.read_unaligned());
                dst.byte_add(4)
                    .cast::<u8>()
                    .write(src.byte_add(12).cast::<u8>().read());
                dst.byte_add(5)
                    .cast::<u8>()
                    .write(src.byte_add(20).cast::<u8>().read());
                dst.byte_add(6)
                    .cast::<u8>()
                    .write(src.byte_add(16).cast::<u8>().read());
                dst.byte_add(7)
                    .cast::<u8>()
                    .write(src.byte_add(24).cast::<u8>().read());
                src = src.wrapping_add(0x8e);
                dst = dst.wrapping_add(2);
                if !(i_var6 < DAT_0099f6e0) {
                    break;
                }
            }
        }
    }
}

unsafe fn remove_entry(idx: i32) {
    // SAFETY: the globals are the game's own tables, read and written where the original does.
    unsafe {
        DAT_0099f6e0 = DAT_0099f6e0.wrapping_add(-1);
        let base = (&raw mut DAT_00999be8).cast::<u32>();
        let last = DAT_0099f6e0;
        let src = base.offset((last as isize).wrapping_mul(0x8e));
        let dst = base.offset((idx as isize).wrapping_mul(0x8e));
        let mut k = 0isize;
        while k < 0x8e {
            dst.offset(k)
                .write_unaligned(src.offset(k).read_unaligned());
            k += 1;
        }
        let e = base.offset((last as isize).wrapping_mul(0x8e));
        e.write_unaligned(0);
        e.add(1).write_unaligned(0x7f);
        e.add(3).write_unaligned(0x7f);
        e.add(5).write_unaligned(0x7f);
        e.add(2).write_unaligned(0x7f);
        e.add(4).write_unaligned(0x7f);
        e.add(6).write_unaligned(0x7f);
        e.add(7).write_unaligned(0);
        e.byte_add(0x20).cast::<u8>().write(0x39);
        e.add(0x88).write_unaligned(1);
        e.add(0x89).write_unaligned(0);
        e.add(0x8a).write_unaligned(0);
        e.add(0x8b).write_unaligned(0);
    }
}

unsafe fn len_with_nul(p: *const i8) -> u32 {
    // SAFETY: p points at a NUL-terminated string in the game's memory.
    unsafe {
        let mut n: u32 = 0;
        loop {
            let c = p.byte_add(n as usize).read();
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        n
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046e080(param_1: i32, param_2: i32) {
    // SAFETY: the callees and globals are the game's own; param_2 is an address
    // of the caller's message block, read where the original does.
    unsafe {
        let p4 = param_2.wrapping_add(4) as *const i32;
        let p8 = param_2.wrapping_add(8) as *const i32;
        let a = &raw mut DAT_005597a0;
        let a0 = a.cast::<u32>();
        let tbl = (&raw mut DAT_00999550).cast::<u32>();
        let ents = (&raw mut DAT_00999be8).cast::<u32>();
        if param_1 == 3 {
            if FUN_0046e040() != 0 {
                if DAT_009ec608 == 0 {
                    if p4.read_unaligned() == 1 {
                        if p8.read_unaligned() != DAT_00999548 {
                            a0.add(1).write_unaligned(0);
                            a0.add(2).write_unaligned(DAT_0099f6e0 as u32);
                            a0.write_unaligned(0x3b);
                            a0.add(3).write_unaligned(timeGetTime());
                            fill_list();
                            FUN_0046d350(
                                p8.read_unaligned(),
                                a0,
                                (DAT_0099f6e0 as u32).wrapping_mul(8).wrapping_add(0x10),
                            );
                            a0.write_unaligned(0x3a);
                            a0.add(1).write_unaligned(0);
                            FUN_0046d350(p8.read_unaligned(), a0, 8);
                        }
                        let mut i_var6: i32 = 0;
                        DAT_009994fc = 0;
                        FUN_0046c730();
                        if 0 < DAT_009994fc {
                            loop {
                                let e = tbl.offset((i_var6 as isize).wrapping_mul(10));
                                if e.add(8).read_unaligned() as i32 != DAT_00999548 {
                                    FUN_0046fa10(e as usize as i32);
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                if !(i_var6 < DAT_009994fc) {
                                    break;
                                }
                            }
                            return;
                        }
                    }
                } else if p4.read_unaligned() == 1 && p8.read_unaligned() != DAT_00999548 {
                    a0.add(1).write_unaligned(0);
                    a0.write_unaligned(0x3b);
                    a0.add(2).write_unaligned(DAT_0099f6e0 as u32);
                    a0.add(3).write_unaligned(timeGetTime());
                    fill_list();
                    FUN_0046d350(
                        p8.read_unaligned(),
                        a0,
                        (DAT_0099f6e0 as u32).wrapping_mul(8).wrapping_add(0x10),
                    );
                    a0.write_unaligned(0x3a);
                    a0.add(1).write_unaligned(FUN_00448d70());
                    a0.add(2).write_unaligned(0x65);
                    a0.add(3).write_unaligned(0);
                    FUN_0046d350(p8.read_unaligned(), a0, 0x10);
                    return;
                }
            }
        } else if param_1 == 5 {
            if FUN_0046e040() != 0 {
                if DAT_009ec608 == 0 {
                    if p4.read_unaligned() == 1 {
                        let mut i_var6: i32 = 0;
                        DAT_009994fc = 0;
                        FUN_0046c730();
                        FUN_0046fa50(p8.read_unaligned() as u32);
                        if 0 < DAT_0099f6e0 {
                            loop {
                                let e = ents.offset((i_var6 as isize).wrapping_mul(0x8e));
                                if e.read_unaligned() as i32 == p8.read_unaligned() {
                                    remove_entry(i_var6);
                                    return;
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                if !(i_var6 < DAT_0099f6e0) {
                                    break;
                                }
                            }
                            return;
                        }
                    }
                } else if p4.read_unaligned() == 1 {
                    let mut i_var6: i32 = 0;
                    if 0 < DAT_009994fc {
                        loop {
                            let e = tbl.offset((i_var6 as isize).wrapping_mul(10));
                            if e.add(8).read_unaligned() as i32 == p8.read_unaligned() {
                                if e.add(9).read_unaligned() != 0 {
                                    a0.write_unaligned(0xd);
                                    FUN_0046d350(DAT_00999548, a0, 4);
                                }
                                FUN_0046fa50(p8.read_unaligned() as u32);
                                tbl.offset((i_var6 as isize).wrapping_mul(10))
                                    .add(9)
                                    .write_unaligned(0);
                                break;
                            }
                            i_var6 = i_var6.wrapping_add(1);
                            if !(i_var6 < DAT_009994fc) {
                                break;
                            }
                        }
                    }
                    i_var6 = 0;
                    if 0 < DAT_0099f6e0 {
                        loop {
                            let e = ents.offset((i_var6 as isize).wrapping_mul(0x8e));
                            if e.read_unaligned() as i32 == p8.read_unaligned() {
                                remove_entry(i_var6);
                                return;
                            }
                            i_var6 = i_var6.wrapping_add(1);
                            if !(i_var6 < DAT_0099f6e0) {
                                break;
                            }
                        }
                        return;
                    }
                }
            }
        } else {
            if param_1 != 0x103 {
                return;
            }
            if p4.read_unaligned() == 1 {
                let mut i_var6: i32 = 0;
                if 0 < DAT_009994fc {
                    loop {
                        let e = tbl.offset((i_var6 as isize).wrapping_mul(10));
                        if e.add(8).read_unaligned() as i32 == p8.read_unaligned() {
                            break;
                        }
                        i_var6 = i_var6.wrapping_add(1);
                        if DAT_009994fc <= i_var6 {
                            return;
                        }
                    }
                    let src1 = param_2.wrapping_add(0x1c) as *const i8;
                    let n1 = len_with_nul(src1);
                    let dst1 = tbl
                        .cast::<i8>()
                        .byte_add(0x10)
                        .offset((i_var6 as isize).wrapping_mul(0x28));
                    let mut k: u32 = 0;
                    while k < n1 {
                        dst1.byte_add(k as usize)
                            .write(src1.byte_add(k as usize).read());
                        k += 1;
                    }
                    let n2 = len_with_nul(dst1);
                    let src2 =
                        (n2.wrapping_add(0x1c).wrapping_add(param_2 as u32)) as usize as *const i8;
                    let n3 = len_with_nul(src2);
                    let dst2 = tbl.offset((i_var6 as isize).wrapping_mul(10)).cast::<i8>();
                    k = 0;
                    while k < n3 {
                        dst2.byte_add(k as usize)
                            .write(src2.byte_add(k as usize).read());
                        k += 1;
                    }
                    tbl.cast::<u8>()
                        .byte_offset((i_var6 as isize).wrapping_mul(0x28))
                        .byte_add(0x1c)
                        .write(0);
                    tbl.cast::<u8>()
                        .byte_offset((i_var6 as isize).wrapping_mul(0x28))
                        .byte_add(0xc)
                        .write(0);
                }
            }
        }
    }
}
