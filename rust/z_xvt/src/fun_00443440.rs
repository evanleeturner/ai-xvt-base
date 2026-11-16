// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_00732a3c: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00732e54: i32;

    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070ad4b: [u8; 0];
    static mut DAT_0070ad54: [u8; 0];
    static mut DAT_0070b216: [i8; 0];
    static mut DAT_0072f508: [i8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_0071b73e: [u8; 0];
    static mut DAT_00703ca0: [i8; 0];

    static mut DAT_00524948: [i8; 0];
    static mut s_stationaryldrpln_00524934: [i8; 0];
    static mut s_stationaryflwpln_00524920: [i8; 0];
    static mut DAT_005250b4: [i8; 0];
    static mut DAT_00525b80: [i8; 0];
    static mut s_formevadeldr1pln_005250a0: [i8; 0];
    static mut s_formevadeflw1pln_00525b6c: [i8; 0];
    static mut s_flyhomeevadepln_00525b50: [i8; 0];
    static mut s_intohyperspacepln_00525b10: [i8; 0];
    static mut s_enterhangarpln_00523be4: [i8; 0];
    static mut s_craftwaitforgopln_00525b24: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_004296b0(param_1: u32);
    fn FUN_00429310(param_1: u32);
    fn FUN_004a35b0(
        param_1: u32,
        param_2: *mut u32,
        param_3: i16,
        param_4: u32,
        param_5: i32,
        param_6: u32,
    );
}

unsafe fn str_eq(a: *const i8, b: *const i8, n: i32) -> bool {
    let mut n = n;
    let mut i: usize = 0;
    let mut eq;
    loop {
        if n == 0 {
            // count is never zero at the first iteration
            return true;
        }
        n -= 1;
        // SAFETY: the caller passes game string buffers at least n bytes long
        // (or ending in a mismatch), read as the original's repe cmpsb does.
        eq = unsafe { a.add(i).read() == b.add(i).read() };
        i += 1;
        if !eq {
            return false;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00443440(param_1: u16, param_2: i16, param_3: u32, param_4: i32) {
    // SAFETY: all globals and the pointers derived from them belong to the
    // game; they are read and written at the offsets the original uses.
    unsafe {
        let a_ad48 = (&raw mut DAT_0070ad48).cast::<u8>();
        let a_ad4b = (&raw mut DAT_0070ad4b).cast::<u8>();
        let a_ad54 = (&raw mut DAT_0070ad54).cast::<u8>();
        let a_b216 = (&raw mut DAT_0070b216).cast::<i8>();
        let a_f508 = (&raw mut DAT_0072f508).cast::<i8>();
        let a_b730 = (&raw mut DAT_0071b730).cast::<u8>();
        let a_b73c = (&raw mut DAT_0071b73c).cast::<u8>();
        let a_b73e = (&raw mut DAT_0071b73e).cast::<u8>();
        let a_3ca0 = (&raw mut DAT_00703ca0).cast::<i8>();

        let unaff_ebp: u32 = 0;
        let pk = param_4 as u32;

        if param_1 != 0xffff {
            let t16 = a_b73c
                .add(pk.wrapping_mul(0x5bd) as usize)
                .cast::<u16>()
                .read_unaligned();
            let u_var7 = t16 as u32;
            let mut b = false;
            let k = DAT_006d4988
                .add((param_1 as u32).wrapping_mul(0x23).wrapping_add(0x16) as usize)
                .read() as u32;
            let first = a_ad48.add(k.wrapping_mul(0x566) as usize).read() as u32;
            if first != u_var7 {
                b = a_f508
                    .add(first.wrapping_add(u_var7.wrapping_mul(0x1e5)) as usize)
                    .read()
                    == 0;
            }
            if !b {
                return;
            }
        }

        let mut local_e: i16 = 0;
        let mut local_c: u16 = 0xffff;
        let mut u_stack_a: u16 = DAT_00732a3c;
        if (u_stack_a as i32) < DAT_006d4980 {
            let i_var4 = pk.wrapping_mul(0x5bd) as usize;
            loop {
                let u_var7 = u_stack_a as u32;
                'next: {
                    let t32 = a_b730.add(i_var4).cast::<u32>().read_unaligned();
                    if t32 == u_var7 {
                        break 'next;
                    }
                    let i_var6 = DAT_006d4988.add(u_var7.wrapping_mul(0x23) as usize);
                    let t32 = i_var6.add(0x1b).cast::<u32>().read_unaligned();
                    if !(t32 == 0xffff_ffff && i_var6.add(3).read() != 0) {
                        break 'next;
                    }
                    let i_var5 = (i_var6.add(0x16).read() as u32).wrapping_mul(0x566) as usize;
                    let t16 = a_b73c.add(i_var4).cast::<u16>().read_unaligned();
                    if a_ad48.add(i_var5).read() as u16 != t16 {
                        break 'next;
                    }
                    let q = i_var6.add(0x1f).cast::<u32>().read_unaligned() as usize as *mut u8;
                    let i_var2 = q.add(0xa7).cast::<u32>().read_unaligned() as usize as *mut u8;
                    if i_var2.add(7).read() != 0 {
                        break 'next;
                    }
                    let u16v = a_b73e.add(i_var4).cast::<u16>().read_unaligned();
                    let i_var1 = (u16v as u32).wrapping_mul(0x566) as usize;
                    let cond = (i_var6.add(0x16).read() as u32 == u16v as u32)
                        || (a_b216.add(i_var5).read() != 0
                            && a_b216.add(i_var1).read() == a_b216.add(i_var5).read())
                        || (a_ad4b.add(i_var5).read() as u32)
                            .wrapping_sub((a_ad54.add(i_var1).read() as u32).wrapping_sub(1))
                            == 9;
                    if !cond {
                        break 'next;
                    }
                    let r6 = FUN_0042b150(i_var2 as usize as i32) as usize as *mut u8;
                    if param_2 == 0x9a {
                        i_var2.add(0x342).cast::<u16>().write_unaligned(param_1);
                        let t = r6.add(0x2b).cast::<u16>().read_unaligned();
                        if t == param_1 {
                            r6.add(0x2b).cast::<u16>().write_unaligned(0xffff);
                        }
                        local_e = local_e.wrapping_add(1);
                        local_c = u_stack_a;
                        break 'next;
                    }
                    let p9 = a_3ca0.add(r6.add(10).read() as usize * 0x55) as *const i8;
                    let list: [(*const i8, i32); 9] = [
                        ((&raw mut DAT_00524948).cast::<i8>(), 8),
                        ((&raw mut s_stationaryldrpln_00524934).cast::<i8>(), 0x11),
                        ((&raw mut s_stationaryflwpln_00524920).cast::<i8>(), 0x11),
                        ((&raw mut DAT_005250b4).cast::<i8>(), 0xc),
                        ((&raw mut DAT_00525b80).cast::<i8>(), 0xc),
                        ((&raw mut s_formevadeldr1pln_005250a0).cast::<i8>(), 0x11),
                        ((&raw mut s_formevadeflw1pln_00525b6c).cast::<i8>(), 0x11),
                        ((&raw mut s_flyhomeevadepln_00525b50).cast::<i8>(), 0x10),
                        ((&raw mut s_intohyperspacepln_00525b10).cast::<i8>(), 0x12),
                    ];
                    let mut k = 0;
                    while k < 9 {
                        if str_eq(p9, list[k].0, list[k].1) {
                            break 'next;
                        }
                        k += 1;
                    }
                    if str_eq(p9, (&raw mut s_enterhangarpln_00523be4).cast::<i8>(), 0xf) {
                        break 'next;
                    }
                    if str_eq(
                        p9,
                        (&raw mut s_craftwaitforgopln_00525b24).cast::<i8>(),
                        0x12,
                    ) {
                        r6.add(10).write(r6.add(0xd).read());
                        DAT_00732e54 = i_var2 as usize as i32;
                        FUN_004296b0(u_stack_a as u32);
                        FUN_00429310(u_var7);
                    }
                    r6.add(0x2b).cast::<u16>().write_unaligned(param_1);
                    if i_var2.add(0x342).cast::<u16>().read_unaligned() == param_1 {
                        i_var2.add(0x342).cast::<u16>().write_unaligned(0xffff);
                    }
                    local_e = local_e.wrapping_add(1);
                    local_c = u_stack_a;
                }
                u_stack_a = u_stack_a.wrapping_add(1);
                if !((u_stack_a as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        if DAT_0071ecf4 == param_4 && local_c != 0xffff {
            let q = DAT_006d4988
                .add((local_c as u32).wrapping_mul(0x23).wrapping_add(0x1f) as usize)
                .cast::<u32>()
                .read_unaligned() as usize as *mut u8;
            let puvar3 = q.add(0xa7).cast::<u32>().read_unaligned() as usize as *mut u32;
            let packed = ((u_stack_a as u32) << 16) | local_c as u32;
            if local_e == 1 {
                FUN_004a35b0(packed, puvar3, param_2, param_3, 0, unaff_ebp);
                return;
            }
            FUN_004a35b0(packed, puvar3, param_2, param_3, 1, unaff_ebp);
        }
    }
}
