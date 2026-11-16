// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527e94: i32;
    static mut DAT_00527e90: i32;
    static mut DAT_009a1fe8: *mut i32;
    static mut DAT_0051c4a8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_0051c4ac: i32;
    static mut DAT_009e967a: i8;
    static mut DAT_009e9685: i8;
    static mut DAT_009e9a5b: i8;
    static mut DAT_009e9a5f: i8;
    static mut DAT_009e9a63: i8;
    static mut DAT_00552c80: [i32; 0x400];
    static mut DAT_00553c80: [i32; 0x400];
    static mut DAT_00551c80: [i32; 0x400];
    static mut DAT_00550c80: [i32; 0x400];
    static mut DAT_00554c80: [i32; 0x400];
    static mut _DAT_0051c340: f32;
    static mut _DAT_0051c34c: f32;
    static mut _DAT_0051c350: f32;
    static mut _DAT_0051c358: f32;
    static mut _DAT_0051c35c: f32;
    static mut _DAT_0051c364: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042e4d0(a: u32, b: u32, c: i32);
    fn _rand() -> i32;
    fn FUN_0046abf0(a: u32, b: u32) -> u32;
    fn FUN_00424410();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004244e0() {
    // SAFETY: all globals and callees are the game's own memory; the pointer
    // DAT_009a1fe8 and the tables are accessed at the offsets the original uses.
    unsafe {
        let u_var1 = DAT_00527e94;
        let mut local_e4: i32 = 0x400;
        if DAT_00527e90 == 0 {
            local_e4 = 0x200;
        }
        DAT_00527e94 = 0;

        let mut local_d8 = [0u32; 8];
        let mut local_b4 = [0u32; 0x2c];

        let base = DAT_009a1fe8 as *mut u8;
        for i in 0..8usize {
            local_d8[i] = base.add(i * 4).cast::<u32>().read_unaligned();
        }
        let local_d8s: u16 = base.add(32).cast::<u16>().read_unaligned();
        let local_d8c: u8 = base.add(34).read();

        let tmpp = (DAT_009a1fe8 as *mut u8)
            .add(0x1f)
            .cast::<*mut u8>()
            .read_unaligned();
        for i in 0..0x2cusize {
            local_b4[i] = tmpp.add(i * 4).cast::<u32>().read_unaligned();
        }
        let local_b4c: u8 = tmpp.add(0x2c * 4).read();

        let t0 = (&raw mut DAT_00552c80).cast::<u8>();
        let t1 = (&raw mut DAT_00553c80).cast::<u8>();
        let t2 = (&raw mut DAT_00551c80).cast::<u8>();
        let t3 = (&raw mut DAT_00550c80).cast::<u8>();
        let t4 = (&raw mut DAT_00554c80).cast::<u8>();

        if DAT_0051c4a8 != 0 {
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp16 = (&raw mut DAT_009e967a)
                .cast::<u8>()
                .byte_offset(off)
                .cast::<i16>()
                .read_unaligned();
            let a: u32 = if tmp16 == 1 { 0x50 } else { 0x53 };
            FUN_0042e4d0(a, 0xffff_ffff, DAT_009ecc34);
            let mut local_e8: i32 = 0;
            while local_e8 < 0x1000 {
                let mut u_var9: u32;
                let mut u_var2: u32;
                loop {
                    u_var9 = _rand() as u32;
                    u_var2 = _rand() as u32;
                    let u_var3 = _rand() as u32;
                    let i_var4 = (u_var3 & 3).wrapping_add(2) as i32;
                    u_var9 = ((i_var4 as u32).wrapping_mul(u_var9 & 0x3fff) as i32 >> 3) as u32;
                    u_var2 = ((i_var4 as u32).wrapping_mul(u_var2 & 0x3fff) as i32 >> 3) as u32;
                    if ((u_var9.wrapping_add(u_var2) as i32 >> 1) as u32) >= 8 {
                        break;
                    }
                }
                let o = local_e8 as isize;
                t0.byte_offset(o).cast::<i32>().write_unaligned(
                    ((u_var9.wrapping_add(u_var2) as i32 >> 8) as u32).wrapping_add(1) as i32,
                );
                let u_var3 = _rand() as u32;
                if (u_var3 & 0x1000) == 0 {
                    u_var9 = 0u32.wrapping_sub(u_var9);
                }
                t1.byte_offset(o).cast::<u32>().write_unaligned(u_var9);
                let v = _rand() as u32;
                if (v & 0x1000) == 0 {
                    u_var2 = 0u32.wrapping_sub(u_var2);
                }
                t2.byte_offset(o).cast::<u32>().write_unaligned(u_var2);
                t3.byte_offset(o).cast::<i32>().write_unaligned(0x4000);
                let r = FUN_0046abf0(
                    t2.byte_offset(o).cast::<u32>().read_unaligned(),
                    t1.byte_offset(o).cast::<u32>().read_unaligned(),
                );
                t4.byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned((r & 0xffff).wrapping_add(0x4000));
                local_e8 = local_e8.wrapping_add(4);
            }
            DAT_0051c4a8 = 0;
        }

        let mut i_var4: i32 = 0;
        if local_e4 != 0 {
            let mut local_e8 = local_e4;
            loop {
                let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
                let o = i_var4 as isize;
                let p = DAT_009a1fe8;

                let tmp32 = (&raw mut DAT_009e9a5b)
                    .cast::<u8>()
                    .byte_offset(off)
                    .cast::<u32>()
                    .read_unaligned();
                p.add(1).write_unaligned(
                    tmp32.wrapping_add(t1.byte_offset(o).cast::<u32>().read_unaligned()) as i32,
                );
                let p = DAT_009a1fe8;
                let tmp32 = (&raw mut DAT_009e9a5f)
                    .cast::<u8>()
                    .byte_offset(off)
                    .cast::<u32>()
                    .read_unaligned();
                p.add(2).write_unaligned(
                    tmp32.wrapping_add(t3.byte_offset(o).cast::<u32>().read_unaligned()) as i32,
                );
                let p = DAT_009a1fe8;
                let tmp32 = (&raw mut DAT_009e9a63)
                    .cast::<u8>()
                    .byte_offset(off)
                    .cast::<u32>()
                    .read_unaligned();
                p.add(3).write_unaligned(
                    tmp32.wrapping_add(t2.byte_offset(o).cast::<u32>().read_unaligned()) as i32,
                );
                let pb = DAT_009a1fe8 as *mut u8;
                pb.add(3).write(0x89);
                let pb = DAT_009a1fe8 as *mut u8;
                pb.add(2).write(7);
                let pb = DAT_009a1fe8 as *mut u8;
                pb.add(20)
                    .cast::<i16>()
                    .write_unaligned(t4.byte_offset(o).cast::<i32>().read_unaligned() as i16);
                let pb = DAT_009a1fe8 as *mut u8;
                pb.add(16).cast::<i16>().write_unaligned(0);
                let pb = DAT_009a1fe8 as *mut u8;
                pb.add(0x12).cast::<i16>().write_unaligned(0x4000);

                let v0 = t0.byte_offset(o).cast::<i32>().read_unaligned();
                _DAT_0051c340 = v0 as f32;
                _DAT_0051c358 = (0u32.wrapping_sub(v0 as u32) as i32) as f32;
                let mut u_var9 = (&raw mut DAT_009e9685)
                    .cast::<u8>()
                    .byte_offset(off)
                    .cast::<u32>()
                    .read_unaligned();
                _DAT_0051c34c = _DAT_0051c340;
                _DAT_0051c364 = _DAT_0051c358;
                if u_var9 < 0x1d8 {
                    let q = (u_var9 >> 2) as f64;
                    let f_var7 = q * q;
                    _DAT_0051c350 = f_var7 as f32;
                    _DAT_0051c35c = f_var7 as f32;
                    let p = DAT_009a1fe8;
                    let cur = p.add(2).read_unaligned() as u32;
                    p.add(2)
                        .write_unaligned(cur.wrapping_add(u_var9.wrapping_mul(0xffff_fff0)) as i32);
                    DAT_0051c4ac = 1;
                } else {
                    if DAT_0051c4ac != 0 {
                        let tmp16 = (&raw mut DAT_009e967a)
                            .cast::<u8>()
                            .byte_offset(off)
                            .cast::<i16>()
                            .read_unaligned();
                        u_var9 = if tmp16 == 1 { 0x51 } else { 0x54 };
                        FUN_0042e4d0(u_var9, 0xffff_ffff, DAT_009ecc34);
                        DAT_0051c4ac = 0;
                    }
                    _DAT_0051c350 = 16000.0;
                    _DAT_0051c35c = 16000.0;
                    let p = DAT_009a1fe8;
                    let cur = p.add(2).read_unaligned() as u32;
                    p.add(2)
                        .write_unaligned(cur.wrapping_add(0xffff_e280) as i32);
                    let pi = DAT_009a1fe8;
                    let l_var8: i64 = 16000.0f32 as i64;
                    let cur = pi.add(2).read_unaligned() as u32;
                    pi.add(2)
                        .write_unaligned(cur.wrapping_sub(l_var8 as i32 as u32) as i32);
                }
                i_var4 = i_var4.wrapping_add(4);
                FUN_00424410();
                local_e8 = local_e8.wrapping_sub(1);
                if local_e8 == 0 {
                    break;
                }
            }
        }

        let base = DAT_009a1fe8 as *mut u8;
        for i in 0..8usize {
            base.add(i * 4).cast::<u32>().write_unaligned(local_d8[i]);
        }
        base.add(32).cast::<u16>().write_unaligned(local_d8s);
        base.add(34).write(local_d8c);

        let tmpp = (DAT_009a1fe8 as *mut u8)
            .add(0x1f)
            .cast::<*mut u8>()
            .read_unaligned();
        for i in 0..0x2cusize {
            tmpp.add(i * 4).cast::<u32>().write_unaligned(local_b4[i]);
        }
        tmpp.add(0x2c * 4).write(local_b4c);
        DAT_00527e94 = u_var1;
    }
}
