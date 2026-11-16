// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b758: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_0071b756: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_00656c80: i8;
    static mut DAT_00655184: i16;
    static mut DAT_00656c90: [u16; 0];
    static mut DAT_00655ab0: [u8; 0];
    static mut DAT_00655ab2: [u8; 0];
    static mut DAT_00655ab4: [u8; 0];
    static mut DAT_00655ab6: [u8; 0];
    static mut DAT_0071f2f0: [u32; 0];
    static mut DAT_0070a184: *const core::ffi::c_void;
    static mut DAT_006d4988: i32;
    static mut DAT_00655c56: u16;
    static mut DAT_00655c54: u16;
    static mut DAT_00655c58: u16;
    static mut DAT_00655c5a: u16;
    static mut DAT_00656cd6: u16;
    static mut DAT_00655c0c: u16;
    static mut DAT_00655c0e: u16;
    static mut DAT_00655c10: u16;
    static mut DAT_00655c12: u16;
    static mut DAT_00656cca: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ee20(a: i8);
    fn FUN_004a98a0();
    fn FUN_004ac9c0();
    fn FUN_004a6f90();
    fn FUN_0040a840(a: u32);
    fn FUN_004a6a80();
    fn FUN_004aa0d0();
    fn FUN_004ad120();
    fn FUN_004ad970();
    fn FUN_004ae130();
    fn FUN_004ae560();
    fn FUN_0048eda0(a: u32);
    fn FUN_00478320(a: u32, b: u32) -> u32;
    fn FUN_004b15a0(a: i32, b: u16, c: i32);
    fn FUN_004ae700();
    fn FUN_004aeef0(a: u32);
    fn FUN_004af7c0();
    fn FUN_004b2880();
    fn FUN_004a2e70();
}

type CallFn = unsafe extern "C" fn(u32, u16, u16, u16, i32);

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn cur() -> u32 {
    // SAFETY: reads the game's global.
    unsafe { (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) }
}

#[inline(always)]
unsafe fn slot() -> u32 {
    // SAFETY: reads the game's global array.
    unsafe {
        let base = (&raw mut DAT_0071b730) as usize as u32;
        rd32(base.wrapping_add(cur()))
    }
}

#[inline(always)]
unsafe fn rec(i: u32) -> u32 {
    // SAFETY: reads the game's global.
    unsafe { i.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32) }
}

#[inline(always)]
unsafe fn slot_ok(v: u32) -> bool {
    if v == 0xffff_ffff {
        return false;
    }
    // SAFETY: record table is the game's own memory.
    let c = (unsafe { rd8(rec(v).wrapping_add(3)) }) as i8;
    c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4
}

#[inline(always)]
unsafe fn call_ptr(a: u32, b: u16, c: u16, d: u16, e: i32) {
    // SAFETY: the game's function pointer, called with five arguments.
    unsafe {
        let f: CallFn = core::mem::transmute(DAT_0070a184);
        f(a, b, c, d, e);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a8cc0() {
    // SAFETY: all globals, callees and addresses are the game's own memory.
    unsafe {
        FUN_0048ee20(2);
        FUN_004a98a0();
        FUN_004ac9c0();
        let off = cur();
        let mut u_var4: u16;
        let p758 = (&raw mut DAT_0071b758) as usize as u32;
        let p74a = (&raw mut DAT_0071b74a) as usize as u32;
        let p756 = (&raw mut DAT_0071b756) as usize as u32;
        if rd8(p758.wrapping_add(off)) == 0 {
            u_var4 = ((((DAT_00656c80 == 0) as i32).wrapping_sub(1)) & 4) as u16;
        } else {
            let s = rd16(p74a.wrapping_add(off)) as i16;
            if s == -1 {
                u_var4 = 1;
            } else {
                u_var4 = (rd8(p756.wrapping_add(off)) as u16).wrapping_add(1);
            }
            DAT_00656c80 = 1;
            if rd8(p756.wrapping_add(off)) != 2 {
                DAT_00656c80 = 0;
            }
        }
        let mut i2: i32 = (DAT_00655184 as i32).wrapping_add(0x24);
        let p90 = (&raw mut DAT_00656c90).cast::<u16>();
        if p90.offset(i2 as isize).read_unaligned() != u_var4 {
            p90.offset(i2 as isize).write_unaligned(u_var4);
            i2 = (i2 as u32).wrapping_mul(0xc) as i32;
            let o = i2 as u32;
            let idx = (rd16(((&raw mut DAT_00655ab4) as usize as u32).wrapping_add(o)) as u32)
                .wrapping_add(u_var4 as u32);
            let v = (&raw mut DAT_0071f2f0)
                .cast::<u32>()
                .add(idx as usize)
                .read_unaligned();
            call_ptr(
                v,
                rd16(((&raw mut DAT_00655ab0) as usize as u32).wrapping_add(o)),
                rd16(((&raw mut DAT_00655ab2) as usize as u32).wrapping_add(o)),
                rd16(((&raw mut DAT_00655ab6) as usize as u32).wrapping_add(o)),
                0,
            );
        }
        FUN_004a6f90();
        FUN_0040a840(u_var4 as u32);
        FUN_004a6a80();
        FUN_004aa0d0();
        FUN_004ad120();
        FUN_004ad970();
        FUN_004ae130();
        FUN_004ae560();

        let s0 = slot();
        if slot_ok(s0) {
            let a = rd32(rec(s0).wrapping_add(0x1f));
            let b = rd32(a.wrapping_add(0xa7));
            if (rd8(b.wrapping_add(0x1db)) & 0x20) != 0
                && (DAT_00655c56 as u32).wrapping_add(DAT_00655c54 as u32) != 0
            {
                let b1 = rd8(b.wrapping_add(0x210));
                if DAT_00656cd6 as u32 != b1 as u32 {
                    DAT_00656cd6 = b1 as u16;
                    let idx = (DAT_00655c58 as u32).wrapping_add(b1 as u32);
                    let v = (&raw mut DAT_0071f2f0)
                        .cast::<u32>()
                        .add(idx as usize)
                        .read_unaligned();
                    call_ptr(v, DAT_00655c54, DAT_00655c56, DAT_00655c5a, 0);
                }
            }
        }

        let sa = slot();
        let ib = rd32(rec(sa).wrapping_add(0x1f));
        let ic = rd32(ib.wrapping_add(0xa7));
        if (rd8(ic.wrapping_add(0x1db)) & 0x40) != 0 {
            FUN_0048eda0(0x40);
            let ia = slot();
            let u_idx = ia.wrapping_mul(0x23);
            let ib = rd32(u_idx.wrapping_add(0x1f).wrapping_add(DAT_006d4988 as u32));
            let sv79 = rd16(ib.wrapping_add(0x79));
            let u_var3 = FUN_00478320(((u_idx >> 0x10) << 0x10) | sv79 as u32, 0x71c7);
            let c6: i8 = if DAT_00655184 == 0 { 2 } else { 0 };
            FUN_0048ee20(c6);
            FUN_004b15a0((DAT_00655184 as i32).wrapping_add(0x18), u_var3 as u16, 1);
        }

        let sa = slot();
        let ib = rd32(rec(sa).wrapping_add(0x1f));
        let ic = rd32(ib.wrapping_add(0xa7));
        if (rd8(ic.wrapping_add(0x1db)) & 0x40) != 0 {
            FUN_0048eda0(0x40);
            let ia = slot();
            let ib = rd32(rec(ia).wrapping_add(0x1f));
            let ic = rd32(ib.wrapping_add(0xa7));
            let s_bd = rd16(ic.wrapping_add(0xbd));
            let s_bf = rd16(ic.wrapping_add(0xbf));
            u_var4 = s_bd / 0x28f;
            if s_bf == 0 {
                u_var4 = u_var4.wrapping_mul(2);
            }
            let c6: i8 = if DAT_00655184 == 0 { 2 } else { 0 };
            FUN_0048ee20(c6);
            FUN_004b15a0((DAT_00655184 as i32).wrapping_add(0x19), u_var4, 1);
        }

        FUN_004ae700();
        FUN_004aeef0(0);
        FUN_004af7c0();

        let s0 = slot();
        if slot_ok(s0) && DAT_00655c0c != 0 {
            let a = rd32(rec(s0).wrapping_add(0x1f));
            let b = rd32(a.wrapping_add(0xa7));
            let b5: u32 = ((rd8(b.wrapping_add(0x29)) & 2) == 0) as u32;
            let u4 = b5 as u16;
            if DAT_00656cca != u4 {
                DAT_00656cca = u4;
                let idx = (DAT_00655c10 as u32).wrapping_add(b5);
                let v = (&raw mut DAT_0071f2f0)
                    .cast::<u32>()
                    .add(idx as usize)
                    .read_unaligned();
                call_ptr(v, DAT_00655c0c, DAT_00655c0e, DAT_00655c12, 0);
            }
        }
        FUN_004b2880();
        FUN_004a2e70();
    }
}
