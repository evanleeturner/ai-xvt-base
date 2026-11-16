// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d6930: u16;
    static mut DAT_009e9a27: [u8; 1];
    static mut DAT_009e9a59: [u8; 1];
    static mut DAT_009e9a5a: [u8; 1];
    static mut DAT_009e967c: [u8; 1];
    static mut DAT_009fd458: [u8; 1];
    static mut DAT_009d8b88: [u8; 1];
    static mut DAT_009d4d9e: [u8; 1];
    static mut DAT_00a08292: u16;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00451bf0(param_1: i32, param_2: *mut core::ffi::c_void);
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_0046f720(param_1: i32) -> *mut core::ffi::c_void;
}

#[inline(always)]
unsafe fn at(base: *mut u8, off: i32) -> *mut u8 {
    base.wrapping_offset(off as isize)
}

#[allow(non_snake_case)]
unsafe fn scan(i_var2: i32, param_1: i32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // offsets are those of the original.
    unsafe {
        let base = (&raw mut DAT_009e967c).cast::<u8>();
        let a5a = (&raw mut DAT_009e9a5a).cast::<u8>();
        let tbl = (&raw mut DAT_009fd458).cast::<u8>();
        let mut p = base;
        let mut local_8: i32 = 0;
        loop {
            if p.byte_add(4).read() != 0 {
                let mut b3 = false;
                DAT_00a08292 = 3;
                let t = at(a5a, i_var2).read();
                if t == 1 {
                    let a = at(base, i_var2).cast::<u16>().read_unaligned();
                    let b = p.cast::<u16>().read_unaligned();
                    if a == b {
                        b3 = true;
                    }
                } else if t == 2 {
                    let a = at(base, i_var2).cast::<u16>().read_unaligned();
                    let b = p.cast::<u16>().read_unaligned();
                    let b7 = if b == a {
                        false
                    } else {
                        let idx = (a as u32).wrapping_add((b as u32).wrapping_mul(0x1e5));
                        tbl.wrapping_add(idx as usize).read() == 0
                    };
                    b3 = b7;
                } else {
                    b3 = true;
                }
                if b3 {
                    FUN_00450650(0x176, local_8);
                }
            }
            p = p.wrapping_add(0x5bd);
            local_8 = local_8.wrapping_add(1);
            if base.wrapping_add(0x2de8) <= p {
                break;
            }
        }
        *at(a5a, i_var2) = 0;
        FUN_00450650(0x17a, param_1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481420(param_1: i32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // all accesses mirror the original's offsets.
    unsafe {
        let a27 = (&raw mut DAT_009e9a27).cast::<u8>();
        let a59 = (&raw mut DAT_009e9a59).cast::<u8>();
        let a5a = (&raw mut DAT_009e9a5a).cast::<u8>();
        let i_var2 = (param_1 as u32).wrapping_mul(0x5bd) as i32;
        let sel = DAT_009d6930;
        match sel {
            8 => {
                if at(a59, i_var2).read() != 0 {
                    let b = at(a59, i_var2).read().wrapping_sub(1);
                    *at(a59, i_var2) = b;
                    *at(a27, i_var2.wrapping_add(b as i32)) = 0x5f;
                    let c = at(a59, i_var2).read();
                    *at(a27, i_var2.wrapping_add(1).wrapping_add(c as i32)) = 0;
                }
                FUN_00451bf0(0, at(a27, i_var2).cast());
                FUN_00450650((at(a5a, i_var2).read() as i32) + 0x176, param_1);
            }
            9 => {
                let c = at(a5a, i_var2).read();
                let n = c.wrapping_add(1);
                *at(a5a, i_var2) = n;
                if 3 < n {
                    *at(a5a, i_var2) = 1;
                }
                FUN_00451bf0(0, at(a27, i_var2).cast());
                FUN_00450650((at(a5a, i_var2).read() as i32) + 0x176, param_1);
            }
            0xd => {
                let c = at(a59, i_var2).read();
                *at(a27, i_var2.wrapping_add(c as i32)) = 0;
                let p5 = FUN_0046f720(param_1);
                FUN_00451bf0(0, p5);
                FUN_00451bf0(1, at(a27, i_var2).cast());
                scan(i_var2, param_1);
            }
            0x1b => {
                *at(a5a, i_var2) = 0;
                FUN_00450650(0x17b, param_1);
            }
            0x9b | 0x9c | 0x9d | 0x9e => {
                let c = at(a59, i_var2).read();
                *at(a27, i_var2.wrapping_add(c as i32)) = 0;
                let p5 = FUN_0046f720(param_1);
                FUN_00451bf0(0, p5);
                let idx = (sel as u32)
                    .wrapping_add((param_1 as u32).wrapping_mul(4))
                    .wrapping_mul(0x46);
                let d = (&raw mut DAT_009d4d9e)
                    .cast::<u8>()
                    .wrapping_add(idx as usize);
                FUN_00451bf0(1, d.cast());
                scan(i_var2, param_1);
            }
            0 => {
                let off = (param_1 as u32).wrapping_mul(0x10);
                let v = (&raw mut DAT_009d8b88)
                    .cast::<u8>()
                    .wrapping_add(off as usize)
                    .cast::<i16>()
                    .read_unaligned();
                if param_1 == DAT_009ecc34 && v < 0xec {
                    FUN_00451bf0(0, at(a27, i_var2).cast());
                    FUN_00450650((at(a5a, i_var2).read() as i32) + 0x176, param_1);
                }
            }
            _ => {
                if at(a59, i_var2).read() < 0x30 {
                    let c = at(a59, i_var2).read() as i32;
                    *at(a27, i_var2.wrapping_add(c)) = sel as u8;
                    let c = at(a59, i_var2).read() as i32;
                    *at(a27, i_var2.wrapping_add(1).wrapping_add(c)) = 0x5f;
                    let c = at(a59, i_var2).read() as i32;
                    *at(a27, i_var2.wrapping_add(2).wrapping_add(c)) = 0;
                    let c = at(a59, i_var2).read();
                    *at(a59, i_var2) = c.wrapping_add(1);
                }
                FUN_00451bf0(0, at(a27, i_var2).cast());
                FUN_00450650((at(a5a, i_var2).read() as i32) + 0x176, param_1);
            }
        }
    }
}
