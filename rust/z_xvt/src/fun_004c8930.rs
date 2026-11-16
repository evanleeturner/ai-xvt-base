// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005322d4: u8;
    static mut DAT_005322dc: u8;
    static mut DAT_005322c4: u8;
    static mut DAT_005322cc: u8;
    static mut DAT_005322bc: u8;
    static mut DAT_005322b4: u8;
    static mut DAT_005322ac: u8;
    static mut DAT_005322a4: u8;
    static mut DAT_0053229c: u8;
    static mut DAT_00532294: u8;
    static mut DAT_0053228c: u8;
    static mut DAT_00532284: u8;
    static mut DAT_00531990: u8;
    static mut DAT_00531e7c: u8;
    static mut DAT_00531e70: u8;
    static mut DAT_00531358: u8;
    static mut DAT_00531eac: u8;
    static mut DAT_00531ebc: u8;
    static mut DAT_00531ea0: u8;
    static mut DAT_00531e94: u8;
    static mut DAT_005320b4: u8;
    static mut DAT_005320a8: u8;
    static mut DAT_0053209c: u8;
    static mut DAT_00532090: u8;
    static mut DAT_00532084: u8;
    static mut DAT_00532078: u8;
    static mut DAT_0053206c: u8;
    static mut DAT_00532060: u8;
    static mut DAT_00532054: u8;
    static mut DAT_00532040: u8;

    static mut DAT_0064ef9c: i32;
    static mut DAT_0081bc68: i32;
    static mut DAT_0081bcc0: i32;
    static mut DAT_0081fb96: i32;
    static mut DAT_0081bfb6: i32;
    static mut DAT_0081bfba: i32;
    static mut DAT_0082e3c2: i32;
    static mut DAT_0082e3c6: i32;
    static mut DAT_0081fb9e: i32;
    static mut DAT_0081fba2: i32;
    static mut DAT_0081c012: i32;
    static mut DAT_0081c016: i32;
    static mut DAT_0081fbda: i32;
    static mut DAT_0081fbde: i32;
    static mut DAT_0081bfbe: i32;
    static mut DAT_0082e3ca: i32;
    static mut DAT_0081fba6: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d9ea0(a: *mut i32);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004d9a80(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: u32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004d5420();
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004d9d20(a: *mut i32, b: *mut i8, c: *mut i8, d: i32);
    fn FUN_004dd030(a: *mut i32, b: i32, c: i32);
}

unsafe fn rd96() -> i32 {
    // SAFETY: the game's own unaligned global, read by value.
    unsafe { (&raw mut DAT_0081fb96).cast::<i32>().read_unaligned() }
}

unsafe fn base_draw() {
    // SAFETY: callees and globals belong to the game.
    unsafe {
        FUN_004d5420();
        FUN_004d52b0();
        let t96 = rd96();
        let p = if t96 == 0 {
            (&raw mut DAT_00531e7c).cast::<i8>()
        } else {
            (&raw mut DAT_00531e70).cast::<i8>()
        };
        FUN_004d1750(p, 0, 0);
        FUN_004d0e60((&raw mut DAT_00531358).cast::<i8>(), 0, 0);
        let p = if DAT_0081bc68 == 0 {
            (&raw mut DAT_00531eac).cast::<i8>()
        } else {
            (&raw mut DAT_00531ebc).cast::<i8>()
        };
        FUN_004d0e60(p, 0, 0);
        FUN_004d0580((&raw mut DAT_00531ea0).cast::<i8>(), 0, 0);
    }
}

unsafe fn award_block() {
    // SAFETY: callees and globals belong to the game.
    unsafe {
        FUN_004d0580((&raw mut DAT_005320b4).cast::<i8>(), 0, 0);
        let t96 = rd96();
        if t96 == 0 {
            FUN_004d0e60((&raw mut DAT_005320a8).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_0053209c).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00532090).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00532084).cast::<i8>(), 0, 0);
        } else {
            FUN_004d0e60((&raw mut DAT_00532078).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_0053206c).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00532060).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00532054).cast::<i8>(), 0, 0);
        }
    }
}

unsafe fn switch_overlay() {
    // SAFETY: callees and globals belong to the game.
    unsafe {
        match DAT_0081bcc0 {
            0 | 3 => FUN_004d0580((&raw mut DAT_00531e94).cast::<i8>(), 0, 0),
            1 => award_block(),
            2 => FUN_004d0580((&raw mut DAT_00532040).cast::<i8>(), 0, 0),
            _ => {}
        }
    }
}

unsafe fn copy_block(src: *mut i32, dst: *mut i32, tmpl_set: i32) {
    // SAFETY: the game's own unaligned globals, copied as the original does.
    unsafe {
        DAT_0064ef9c = 1;
        (&raw mut DAT_0081fb96)
            .cast::<i32>()
            .write_unaligned(tmpl_set);
        let _ = (src, dst);
    }
}

unsafe fn switch_side(side: i32, a: *mut i32, b: *mut i32, rest: *mut i32) {
    // SAFETY: the game's own unaligned globals, copied as the original does.
    unsafe {
        copy_block(a, b, side);
        (&raw mut DAT_0081bfb6)
            .cast::<i32>()
            .write_unaligned(a.read_unaligned());
        (&raw mut DAT_0081bfba)
            .cast::<i32>()
            .write_unaligned(b.read_unaligned());
        let mut s = rest.cast::<u8>();
        let mut d = (&raw mut DAT_0081bfbe).cast::<u8>();
        let mut n = 5;
        while n != 0 {
            let v = s.cast::<i32>().read_unaligned();
            d.cast::<i32>().write_unaligned(v);
            s = s.wrapping_add(4);
            d = d.wrapping_add(4);
            n -= 1;
        }
        let t96 = rd96();
        let off = (t96 as u32).wrapping_mul(0x3a09).wrapping_mul(4) as usize;
        let v = (&raw mut DAT_0081fbda)
            .cast::<u8>()
            .wrapping_add(off)
            .cast::<i32>()
            .read_unaligned();
        (&raw mut DAT_0081c012).cast::<i32>().write_unaligned(v);
        let t96 = rd96();
        let off = (t96 as u32).wrapping_mul(0x3a09).wrapping_mul(4) as usize;
        let v = (&raw mut DAT_0081fbde)
            .cast::<u8>()
            .wrapping_add(off)
            .cast::<i32>()
            .read_unaligned();
        (&raw mut DAT_0081c016).cast::<i32>().write_unaligned(v);
        base_draw();
        switch_overlay();
        FUN_004d5350(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c8930() -> u32 {
    let mut local_1c: [i32; 7] = [0; 7];
    let mut local_2c: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; locals are valid
    // for the pointers handed out.
    unsafe {
        local_1c[4] = 0;
        local_1c[0] = 1;
        local_1c[1] = 1;
        local_1c[2] = 1;
        local_1c[3] = 1;
        local_1c[5] = 1;
        local_1c[6] = 1;
        let p1c = local_1c.as_mut_ptr();
        p1c.wrapping_offset(DAT_0081bcc0 as isize).write(2);
        let t96 = rd96();
        p1c.wrapping_offset(t96.wrapping_add(5) as isize).write(2);
        FUN_004d9ea0(p1c);
        let p2c = local_2c.as_mut_ptr();
        FUN_004dcfd0(p2c, 0x16, 0x14e, 0x2a, 0x166);

        let jewel = (&raw mut DAT_00531990).cast::<i8>();

        // reg7
        let t96 = rd96();
        if t96 == 1 {
            let p = FUN_004dcfa0(0x275);
            FUN_004d9d20(p2c, (&raw mut DAT_005322d4).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004dcfa0(0x275);
            let s = (&raw mut DAT_005322dc).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0x11, jewel);
            if r != 0 {
                switch_side(
                    1,
                    (&raw mut DAT_0082e3c2).cast::<i32>(),
                    (&raw mut DAT_0082e3c6).cast::<i32>(),
                    (&raw mut DAT_0082e3ca).cast::<i32>(),
                );
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);

        // reg6
        let t96 = rd96();
        if t96 == 0 {
            let p = FUN_004dcfa0(0x274);
            FUN_004d9d20(p2c, (&raw mut DAT_005322c4).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004dcfa0(0x274);
            let s = (&raw mut DAT_005322cc).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0x10, jewel);
            if r != 0 {
                switch_side(
                    0,
                    (&raw mut DAT_0081fb9e).cast::<i32>(),
                    (&raw mut DAT_0081fba2).cast::<i32>(),
                    (&raw mut DAT_0081fba6).cast::<i32>(),
                );
            }
        }
        FUN_004dcfd0(p2c, 0x16, 0xc6, 0x2a, 0xde);

        // reg4
        if DAT_0081bcc0 == 3 {
            let p = FUN_004dcfa0(10);
            FUN_004d9d20(p2c, (&raw mut DAT_005322bc).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004dcfa0(10);
            let s = (&raw mut DAT_005322b4).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0xe, jewel);
            if r != 0 {
                DAT_0064ef9c = 1;
                DAT_0081bcc0 = 3;
                base_draw();
                FUN_004d0580((&raw mut DAT_00531e94).cast::<i8>(), 0, 0);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);

        // reg3
        if DAT_0081bcc0 == 2 {
            let p = FUN_004dcfa0(8);
            FUN_004d9d20(p2c, (&raw mut DAT_005322ac).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004dcfa0(8);
            let s = (&raw mut DAT_005322a4).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0xd, jewel);
            if r != 0 {
                DAT_0064ef9c = 1;
                DAT_0081bcc0 = 2;
                base_draw();
                FUN_004d0580((&raw mut DAT_00532040).cast::<i8>(), 0, 0);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);

        // reg2
        if DAT_0081bcc0 == 1 {
            let p = FUN_004dcfa0(9);
            FUN_004d9d20(p2c, (&raw mut DAT_0053229c).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004dcfa0(9);
            let s = (&raw mut DAT_00532294).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0xc, jewel);
            if r != 0 {
                DAT_0064ef9c = 1;
                DAT_0081bcc0 = 1;
                base_draw();
                award_block();
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);

        // reg1
        if DAT_0081bcc0 != 0 {
            let p = FUN_004dcfa0(7);
            let s = (&raw mut DAT_00532284).cast::<i8>();
            let r = FUN_004d9a80(p2c, s, s, p, 0xc, 0, 0xb, jewel);
            if r != 0 {
                DAT_0064ef9c = 1;
                DAT_0081bcc0 = 0;
                base_draw();
                FUN_004d0580((&raw mut DAT_00531e94).cast::<i8>(), 0, 0);
                FUN_004d5350(1);
            }
            return 1;
        }
        let p = FUN_004dcfa0(7);
        FUN_004d9d20(p2c, (&raw mut DAT_0053228c).cast::<i8>(), p, 0xc);
        1
    }
}
