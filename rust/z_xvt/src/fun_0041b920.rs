// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0052686c: [u16; 0];
    static mut DAT_0052b14b: [i16; 0];
    static mut DAT_0052b14d: [i16; 0];
    static mut DAT_0052b14f: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00412810(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00483540(a: i16, b: i16, c: i32);
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
}

#[inline(always)]
unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the address is inside the game's own records.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the address is inside the game's own records.
    unsafe { (addr as usize as *const i16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(addr: i32) -> u8 {
    // SAFETY: the address is inside the game's own records.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041b920(param_1: i32, param_2: i32) {
    // SAFETY: all globals, callees and record addresses belong to the game;
    // reads and writes happen at the offsets the original uses.
    unsafe {
        let i_var7 = param_2.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let idx = rd8(i_var7.wrapping_add(3)) as usize * 0x16;
        let u_var1: u16 = (&raw mut DAT_0052686c)
            .cast::<u16>()
            .add(idx)
            .read_unaligned();
        let i_var2 = DAT_006d4988.wrapping_add(param_1.wrapping_mul(0x23));
        let i_var8 = rd32(i_var2.wrapping_add(4)).wrapping_sub(rd32(i_var7.wrapping_add(4)));
        let mut i_var5 = rd32(i_var2.wrapping_add(8)).wrapping_sub(rd32(i_var7.wrapping_add(8)));
        let i_var6 = rd32(i_var2.wrapping_add(0xc)).wrapping_sub(rd32(i_var7.wrapping_add(0xc)));
        let i_var2 = FUN_00412810(i_var8, i_var5, i_var6);
        if i_var2 <= ((u_var1 as u32).wrapping_mul(2) as i32)
            && rd32(i_var7.wrapping_add(0x1f)) != 0
        {
            if rd8(rd32(i_var7.wrapping_add(0x1f)).wrapping_add(0x90)) != 0 {
                FUN_00483410(
                    rd16(i_var7.wrapping_add(0x12)),
                    rd16(i_var7.wrapping_add(0x10)),
                    i_var7,
                );
                FUN_00483540(rd16(i_var7.wrapping_add(0x14)), 0, i_var7);
            }
            let mut i_var2 = rd32(i_var7.wrapping_add(0x1f));
            let i_var3 = FUN_00413cc0(
                rd16(i_var2.wrapping_add(0x97)) as i32,
                rd16(i_var2.wrapping_add(0x99)) as i32,
                rd16(i_var2.wrapping_add(0x9b)) as i32,
                i_var8,
                i_var5,
                i_var6,
            );
            i_var2 = rd32(i_var7.wrapping_add(0x1f));
            let i_var4 = FUN_00413cc0(
                rd16(i_var2.wrapping_add(0x91)) as i32,
                rd16(i_var2.wrapping_add(0x93)) as i32,
                rd16(i_var2.wrapping_add(0x95)) as i32,
                i_var8,
                i_var5,
                i_var6,
            );
            i_var2 = rd32(i_var7.wrapping_add(0x1f));
            i_var5 = FUN_00413cc0(
                rd16(i_var2.wrapping_add(0x9d)) as i32,
                rd16(i_var2.wrapping_add(0x9f)) as i32,
                rd16(i_var2.wrapping_add(0xa1)) as i32,
                i_var8,
                i_var5,
                i_var6,
            );
            i_var2 = rd32(rd32(i_var7.wrapping_add(0x1f)).wrapping_add(0xa7));
            if i_var2 != 0 {
                let off = (rd8(i_var2.wrapping_add(4)) as i32).wrapping_mul(0xef) as isize;
                let a = (&raw mut DAT_0052b14b)
                    .cast::<i16>()
                    .offset(off)
                    .read_unaligned() as i32;
                let b = (&raw mut DAT_0052b14f)
                    .cast::<i16>()
                    .offset(off)
                    .read_unaligned() as i32;
                let c = (&raw mut DAT_0052b14d)
                    .cast::<i16>()
                    .offset(off)
                    .read_unaligned() as i32;
                let i_var2 = FUN_00412810(
                    i_var3.wrapping_sub(a),
                    i_var4.wrapping_sub(b),
                    i_var5.wrapping_sub(c),
                );
                if i_var2 <= ((u_var1 / 6) as i32) {
                    let i_var2 = rd32(
                        DAT_006d4988
                            .wrapping_add(0x1f)
                            .wrapping_add(param_1.wrapping_mul(0x23)),
                    );
                    if i_var2 != 0 {
                        let i_var2 = rd32(i_var2.wrapping_add(0xa7));
                        if i_var2 != 0 {
                            ((i_var2.wrapping_add(0x1d)) as usize as *mut u32)
                                .write_unaligned(0x28000);
                            ((i_var2.wrapping_add(0x23e)) as usize as *mut u16).write_unaligned(0);
                        }
                    }
                }
            }
        }
    }
}
