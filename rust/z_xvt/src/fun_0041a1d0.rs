// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007092c1: i8;
    static mut DAT_007092d0: i16;
    static mut DAT_007092c4: i32;
    static mut DAT_0070ac35: u8;
    static mut DAT_0070ac34: i8;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044a600(a: u32, b: u32) -> u32;
    fn FUN_00422710(a: i32) -> u16;
    fn FUN_00449b30(a: u32, b: u32) -> u32;
    fn FUN_00449bf0(a: u32, b: u32) -> i32;
    fn FUN_00449cb0(a: u32, b: u32) -> i32;
    fn FUN_00418450(a: i16) -> i64;
    fn FUN_004187b0(a: i16) -> i64;
    fn FUN_00413d50(a: i32, b: i32, c: u32, d: i32) -> u32;
    fn FUN_0042a230(a: i32, b: i32, c: i32, d: i16, e: i16, f: i16);
    fn FUN_00478470() -> u32;
    fn FUN_0040a1a0(a: u32, b: u32, c: i32);
    fn FUN_0044a510(a: u32, b: u32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> u32;
    fn FUN_0040b730(a: i32, b: i32, c: i32, d: i32, e: u32, f: i32);
}

#[inline(always)]
fn ad(b: i32, o: i32) -> *mut u8 {
    b.wrapping_add(o) as u32 as usize as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041a1d0(
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: u32,
) -> u16 {
    let mut param_3 = param_3;
    // SAFETY: all globals and callees are the game's own; the pointers
    // address the game's tables exactly as the original does.
    unsafe {
        let u_var13: u32 = param_2.wrapping_sub(1) as u16 as u32;
        let pc_var2 = ad(DAT_00732e54, (0x31au32.wrapping_add(u_var13)) as i32);
        let mut b_var3: u8 = pc_var2.read();
        if b_var3 == 0 {
            return param_3;
        }
        if b_var3 == 0xff {
            return param_3;
        }
        param_3 >>= 4;
        if param_3 == 0 {
            param_3 = 1;
        }
        if param_3 < b_var3 as u16 {
            let c_var6 = param_3 as u8;
            param_3 = 0;
            pc_var2.write(b_var3.wrapping_sub(c_var6));
        } else {
            pc_var2.write(0);
            param_3 = ((param_3 - b_var3 as u16) as u32).wrapping_mul(0x10) as u16;
            let p1 = param_1 & 0xffff;
            let i_var4: i32 = p1.wrapping_mul(8).wrapping_sub(p1).wrapping_mul(5) as i32;
            let mut u_var8 = FUN_0044a600(
                ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                u_var13,
            );
            if u_var8 != 0 {
                ad(DAT_00732e54, (0x2cau32.wrapping_add(u_var13)) as i32).write(2);
                b_var3 = DAT_0070ac35;
                if DAT_007092c1 != 0 {
                    DAT_007092d0 = DAT_007092d0.wrapping_add(1);
                    let mut i_var5 = DAT_007092c4.wrapping_add(0x32);
                    if ad(DAT_00732e54, (0x2f2u32.wrapping_add(u_var13)) as i32).read() != 0 {
                        i_var5 = DAT_007092c4.wrapping_add(100);
                    }
                    DAT_007092c4 = i_var5;
                    b_var3 = DAT_0070ac35.wrapping_add(2);
                    if b_var3 > 0x3b {
                        DAT_0070ac34 = DAT_0070ac34.wrapping_add(1);
                        b_var3 = DAT_0070ac35.wrapping_sub(0x3a);
                    }
                }
                DAT_0070ac35 = b_var3;
                let u_var7 = FUN_00422710(0xd);
                if u_var7 != 0xffff {
                    u_var8 = u_var7 as u32;
                    let i_var5: i32 = u_var8.wrapping_mul(0x23) as i32;

                    let mut o = 4;
                    while o <= 0xc {
                        let t = ad(DAT_006d4988, i_var4.wrapping_add(o))
                            .cast::<u32>()
                            .read_unaligned();
                        ad(DAT_006d4988, i_var5.wrapping_add(o))
                            .cast::<u32>()
                            .write_unaligned(t);
                        o += 4;
                    }

                    let mut local_14 = FUN_00449b30(
                        ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                        u_var13,
                    );
                    let mut i_var9 = FUN_00449bf0(
                        ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                        u_var13,
                    );
                    let mut local_10 = FUN_00449cb0(
                        ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                        u_var13,
                    );
                    if DAT_007092c1 != 0 {
                        b_var3 = ad(DAT_00732e54, (0x2f2u32.wrapping_add(u_var13)) as i32).read();
                        if b_var3 != 0 {
                            let s_var12 = (b_var3 as u32).wrapping_mul(0xffff_ff00u32) as i16;
                            let l_var14 = FUN_00418450(s_var12);
                            let l_var15 = FUN_004187b0(s_var12);
                            let u_var10 = FUN_00413d50(
                                l_var15 as i16 as i32,
                                0u32.wrapping_sub(l_var14 as i16 as i32 as u32) as i32,
                                local_14,
                                local_10,
                            );
                            local_10 = FUN_00413d50(
                                l_var14 as i16 as i32,
                                l_var15 as i16 as i32,
                                local_14,
                                local_10,
                            ) as i32;
                            local_14 = u_var10 & 0xffff;
                        }
                    }
                    DAT_0070ab18 = local_14 as i32;
                    DAT_0070ab14 = local_10;
                    DAT_0070ac20 = 0u32.wrapping_sub(i_var9 as u32) as i32;
                    i_var9 = DAT_006d4988.wrapping_add(i_var4);
                    if ad(i_var9, 3).read() == 0x35 {
                        DAT_0070ab18 = (local_14 as i32) >> 1;
                        DAT_0070ac20 >>= 1;
                        DAT_0070ab14 = local_10 >> 1;
                    }
                    FUN_0042a230(
                        i_var9,
                        DAT_0070ab14,
                        i_var9,
                        DAT_0070ab18 as i16,
                        DAT_0070ab14 as i16,
                        DAT_0070ac20 as i16,
                    );
                    if ad(DAT_006d4988, i_var4.wrapping_add(3)).read() == 0x35 {
                        DAT_0070ab18 = DAT_0070ab18.wrapping_mul(2);
                        DAT_0070ab14 = DAT_0070ab14.wrapping_mul(2);
                        DAT_0070ac20 = DAT_0070ac20.wrapping_mul(2);
                    }
                    let q = ad(DAT_006d4988, i_var5.wrapping_add(4)).cast::<u32>();
                    q.write_unaligned(q.read_unaligned().wrapping_add(DAT_0070ab18 as u32));
                    let q = ad(DAT_006d4988, i_var5.wrapping_add(8)).cast::<u32>();
                    q.write_unaligned(q.read_unaligned().wrapping_add(DAT_0070ab14 as u32));
                    let q = ad(DAT_006d4988, i_var5.wrapping_add(0xc)).cast::<u32>();
                    q.write_unaligned(q.read_unaligned().wrapping_add(DAT_0070ac20 as u32));

                    ad(DAT_006d4988, i_var5.wrapping_add(3)).write(0x81);
                    ad(DAT_006d4988, i_var5.wrapping_add(2)).write(0xd);
                    let mut pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.write(5);
                    ad(DAT_006d4988, i_var5.wrapping_add(0x19)).write(2);
                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(0x81).cast::<u16>().write_unaligned(0);
                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(0x7f).cast::<u16>().write_unaligned(0);
                    let pq = ad(DAT_006d4988, i_var4.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    let tmps = pq.byte_add(0x79).cast::<u16>().read_unaligned();
                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(0x79).cast::<u16>().write_unaligned(tmps);

                    let t = ad(DAT_006d4988, i_var4.wrapping_add(0x12))
                        .cast::<u16>()
                        .read_unaligned();
                    ad(DAT_006d4988, i_var5.wrapping_add(0x12))
                        .cast::<u16>()
                        .write_unaligned(t);
                    let t = ad(DAT_006d4988, i_var4.wrapping_add(0x10))
                        .cast::<u16>()
                        .read_unaligned();
                    ad(DAT_006d4988, i_var5.wrapping_add(0x10))
                        .cast::<u16>()
                        .write_unaligned(t);
                    ad(DAT_006d4988, i_var5.wrapping_add(0x14))
                        .cast::<u16>()
                        .write_unaligned(0);

                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(0x90).write(1);
                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(0x89).write(pp.byte_add(0x90).read());
                    let i_var9b = DAT_0071ecf4;
                    let u_var10 = FUN_00478470();
                    FUN_0040a1a0((u_var10 & 3).wrapping_add(0x16), u_var8, i_var9b);
                    let mut i_var9c = FUN_0044a510(
                        ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                        u_var13,
                    );
                    i_var9c >>= 9;
                    if 0xff < i_var9c {
                        i_var9c = 0xff;
                    }
                    pp = ad(DAT_006d4988, i_var5.wrapping_add(0x1f))
                        .cast::<u32>()
                        .read_unaligned() as usize as *mut u8;
                    pp.byte_add(1).write(i_var9c as u8);
                }
                let base = (&raw mut DAT_0071b730) as *mut u8;
                let tmpu = base
                    .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                    .cast::<u32>()
                    .read_unaligned();
                if tmpu == (param_4 & 0xffff) {
                    let u_var11 = FUN_00449790(
                        ad(DAT_006d4988, i_var4.wrapping_add(3)).read() as u32,
                        u_var13,
                    );
                    match u_var11 {
                        4 | 5 | 0x15 => {
                            FUN_0040b730(DAT_0071ecf4, -1, 0x17, 2, param_1 & 0xffff, 0x4000);
                        }
                        8 => {
                            FUN_0040b730(DAT_0071ecf4, -1, 0x17, 3, param_1 & 0xffff, 0xffff);
                        }
                        10 | 0x16 => {
                            FUN_0040b730(DAT_0071ecf4, -1, 0x17, 1, param_1 & 0xffff, 0xffff);
                        }
                        0xb | 0xc | 0x17 | 0x18 => {
                            FUN_0040b730(DAT_0071ecf4, -1, 0x17, 4, param_1 & 0xffff, 0xffff);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    param_3
}
