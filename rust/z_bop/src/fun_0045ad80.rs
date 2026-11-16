// SPDX-License-Identifier: MIT

use core::ffi::c_void;

macro_rules! ext_u8 {
    ($($n:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        unsafe extern "C" {
            $(static mut $n: u8;)*
        }
    };
}

macro_rules! ext_i8 {
    ($($n:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        unsafe extern "C" {
            $(static mut $n: i8;)*
        }
    };
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523434: [i8; 0];
    static mut DAT_00a07bd8: *mut c_void;
    static mut DAT_00a08294: i16;
    static mut DAT_009a2000: i16;
    static mut _DAT_009a2002: i16;
    static mut DAT_009a2004: [u8; 2];
    static mut DAT_009d8120: [i16; 0];
    static mut DAT_009e8f60: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(param_1: *mut i8, param_2: *const i8, param_3: i32, param_4: i32) -> i32;
    fn FUN_0049ea10(param_1: *mut c_void, param_2: u32, param_3: i32, param_4: *mut c_void) -> i32;
    fn _strncpy(param_1: *mut i8, param_2: *const i8, param_3: u32) -> *mut i8;
    fn _fseek(param_1: *mut c_void, param_2: i32, param_3: i32) -> i32;
    fn FUN_004ac5d0(param_1: i32, param_2: i32) -> i16;
    fn FUN_004ac720(param_1: u32) -> *mut i8;
    fn FUN_004ac740();
    fn FUN_0049e9c0(param_1: i32) -> u32;
}

ext_u8!(
    DAT_009a2006,
    DAT_009a2007,
    DAT_009a2008,
    DAT_009a2009,
    DAT_009a200a,
    DAT_009a200e,
    DAT_009a2016,
    DAT_009a201a,
    DAT_009a202a,
    DAT_009a202e,
    DAT_009a203e,
    DAT_009a2042,
    DAT_009a2052,
    DAT_009a2056,
    DAT_009a2062,
    DAT_009fd5e3,
    DAT_009fd440,
    DAT_009fd458,
    DAT_009fe800,
    DAT_009fe840,
    DAT_009fe84a,
    DAT_009fe84b,
    DAT_009fe84c,
    DAT_009fe84d,
    DAT_009fe84e,
    DAT_009fe84f,
    DAT_009fe850,
    DAT_009fe851,
    DAT_009fe854,
    DAT_009fe860,
    DAT_009fe870,
    DAT_009a8080,
    DAT_009a8081,
    DAT_009a8082,
    DAT_009a8083,
    DAT_009a8084,
    DAT_009a8085,
    DAT_009a8086,
    DAT_009a8087,
    DAT_009a808a,
    DAT_009a8096,
    DAT_009a80a6,
    DAT_009a80a8,
    DAT_009a80a9,
    DAT_00556aa8,
    DAT_00556aab,
    DAT_00556aac,
    DAT_00556aad,
    DAT_00556aae,
    DAT_00556ab2,
    DAT_00556c38,
    DAT_00556c22,
    DAT_00556ca0,
    DAT_00556ca1,
    DAT_00556ca2,
    DAT_00556ca3,
    DAT_00556ca4,
    DAT_00556ca5,
    DAT_00556ca6,
    DAT_00556ca7,
    DAT_00556ca8,
    DAT_00556ca9,
    DAT_00556caa,
    DAT_00556cab,
    DAT_00556cac,
    DAT_00556cad,
    DAT_00556cae,
    DAT_00556caf,
    DAT_00556cb0,
    DAT_00556cb1,
    DAT_00556cb3,
    DAT_00556cb4,
    DAT_00556cb5,
    DAT_00556cb6,
    DAT_00556cb7,
    DAT_00556cb8,
    DAT_00556cb9,
    DAT_00556cba,
    DAT_00556cbb,
    DAT_00556cbc,
    DAT_00556cbd,
    DAT_00556cbe,
    DAT_00556cbf,
    DAT_00556cc0,
    DAT_00556cc1,
    DAT_00556cc2,
    DAT_00556cc4,
    DAT_00556cc5,
    DAT_00556cc6,
    DAT_00556cc7,
    DAT_00556cc8,
    DAT_00556cc9,
    DAT_00556cca,
    DAT_00556ccb,
    DAT_00556ccc,
    DAT_00556cce,
    DAT_00556cd0,
    DAT_00556cd1,
    DAT_00556cd2,
    DAT_00556cd3,
    DAT_00556cd4,
    DAT_00556cd5,
    DAT_00556cd6,
    DAT_00556cd7,
    DAT_00556cd8,
    DAT_00556cdc,
    DAT_00556ce0,
    DAT_00556ce4,
    DAT_00556ce5,
    DAT_00556ce6,
    DAT_00556ce7,
    DAT_00556ce8,
    DAT_00556cea,
    DAT_00556cee,
    DAT_00556cf2,
    DAT_00556cf6,
    DAT_00556cf7,
    DAT_00556cf8,
    DAT_00556cf9,
    DAT_00556cfa,
    DAT_00556cfc,
    DAT_00556d00,
    DAT_00556d04,
    DAT_00556d08,
    DAT_00556d09,
    DAT_00556d0a,
    DAT_00556d0b,
    DAT_00556d0c,
    DAT_00556d0e,
    DAT_00556d0f,
    DAT_00556d10,
    DAT_00556d11,
    DAT_00556d12,
    DAT_00556d13,
    DAT_00556d14,
    DAT_00556d15,
    DAT_00556d16,
    DAT_00556d18,
    DAT_00556d36,
    DAT_00556d54,
    DAT_00556d72,
    DAT_00556d90,
    DAT_00556d91,
    DAT_00556d92,
    DAT_00556d93,
    DAT_00556d98,
    DAT_00556d99,
    DAT_00556d9a,
    DAT_00556d9b,
    DAT_00556d9c,
    DAT_00556d9d,
    DAT_00556d9e,
    DAT_00556d9f,
    DAT_00556da0,
    DAT_00556db0,
    DAT_00556db1,
    DAT_00556db2,
    DAT_00556db8,
    DAT_00556dbc,
    DAT_00556dc0,
    DAT_00556dc4,
    DAT_00556dc8,
    DAT_00556dcc,
    DAT_00556dd0,
    DAT_00556dd4,
    DAT_00556dd8,
    DAT_00556ddc,
    DAT_00556de0,
    DAT_00556de4,
    DAT_00556de8,
    DAT_00556e28,
    DAT_00556e29,
    DAT_00556e2a,
    DAT_00556e2b,
    DAT_00556e2c,
    DAT_00556e2d,
    DAT_00556e2e,
    DAT_00556e2f,
    DAT_00556e30,
    DAT_00556e40,
    DAT_00556e41,
    DAT_00556e48,
    DAT_00556e4a,
    DAT_00556e4c,
    DAT_00556e4d,
    DAT_00556e4e,
    DAT_00556e4f,
    DAT_00556e50,
    DAT_00556e51,
    DAT_00556e52,
    DAT_00556e56,
    DAT_00556e5a,
    DAT_00556e5e,
    DAT_00556e62,
    DAT_00556e66,
    DAT_00556e6a,
    DAT_00556e6e,
    DAT_00556e72,
    DAT_00556e76,
    DAT_00556e7a,
    DAT_00556e7e,
    DAT_00556e82,
    DAT_00556e86,
    DAT_00556e8a,
    DAT_009d8c30,
    DAT_009d8c34,
    DAT_009d8c38,
    DAT_009d8c48,
    DAT_009d8c4c,
    DAT_009d8c58,
    DAT_009d8c5c,
    DAT_009d8c60,
    DAT_009d8c6c,
    DAT_009d8c70,
    DAT_009d8c74,
    DAT_009d8c80,
    DAT_009d8c81,
    DAT_009d8c82,
    DAT_009d8c83,
    DAT_009d8c84,
    DAT_009d8c85,
    DAT_009d8c86,
    DAT_009d8c87,
    DAT_009d8c88,
    DAT_009d8c89,
    DAT_009d8c8a,
    DAT_009d8c8b,
    DAT_009d8c8c,
    DAT_009d8c8d,
    DAT_009d8c8e,
    DAT_009d8c8f,
    DAT_009d8c90,
    DAT_009d8c91,
    DAT_009d8c92,
    DAT_009d8c93,
    DAT_009d8c94,
    DAT_009d8c95,
    DAT_009d8c96,
    DAT_009d8c97,
    DAT_009d8c98,
    DAT_009d8c99,
    DAT_009d8c9a,
    DAT_009d8c9b,
    DAT_009d8c9c,
    DAT_009d8c9d,
    DAT_009d8c9e,
    DAT_009d8c9f,
    DAT_009d8ca0,
    DAT_009d8ca1,
    DAT_009d8ca2,
    DAT_009d8ca3,
    DAT_009d8ca4,
    DAT_009d8ca5,
    DAT_009d8ca8,
    DAT_009d8cb5,
    DAT_009d8cb6,
    DAT_009d8cb7,
    DAT_009d8cb8,
    DAT_009d8cb9,
    DAT_009d8cba,
    DAT_009d8cbb,
    DAT_009d8cc3,
    DAT_009d8cc4,
    DAT_009d8cc5,
    DAT_009d8cc6,
    DAT_009d8cc8,
    DAT_009d8cca,
    DAT_009d8ccb,
    DAT_009d8ccc,
    DAT_009d8ccd,
    DAT_009d8cce,
    DAT_009d8ccf,
    DAT_009d8cd0,
    DAT_009d8cd1,
    DAT_009d8cd2,
    DAT_009d8cd6,
    DAT_009d8cda,
    DAT_009d8cde,
    DAT_009d8cdf,
    DAT_009d8ce0,
    DAT_009d8ce1,
    DAT_009d8ce2,
    DAT_009d8d24,
    DAT_009d8d28,
    DAT_009d8d2c,
    DAT_009d8d30,
    DAT_009d8d31,
    DAT_009d8d32,
    DAT_009d8d33,
    DAT_009d8d34,
    DAT_009d8d76,
    DAT_009d8d7a,
    DAT_009d8d7e,
    DAT_009d8d82,
    DAT_009d8d83,
    DAT_009d8d84,
    DAT_009d8d85,
    DAT_009d8d86,
    DAT_009d8e25,
    DAT_009d8e26,
    DAT_009d8e27,
    DAT_009d8e29,
    DAT_009d8e73,
    DAT_009d8e74,
    DAT_009d8e75,
    DAT_009d8e77,
    DAT_009d8ec1,
    DAT_009d8ec2,
    DAT_009d8ec3,
    DAT_009d8ec5,
    DAT_009d8f0f,
    DAT_009d8f10,
    DAT_009d8f11,
    DAT_009d8f12,
    DAT_009d8f13,
    DAT_009d9096,
    DAT_009d90c2,
    DAT_009d90ee,
    DAT_009d911a,
    DAT_009d9146,
    DAT_009d9147,
    DAT_009d9148,
    DAT_009d9157,
    DAT_009d9192,
);

ext_i8!(
    DAT_009a2012,
    DAT_009a2026,
    DAT_009a203a,
    DAT_009a204e,
    DAT_009fd462,
    DAT_009fd4a2,
    DAT_009fd4e2,
    DAT_009fd522,
    DAT_009fd562,
    DAT_009fd5a2,
    DAT_009a808f,
    DAT_00556ab8,
    DAT_00556af8,
    DAT_00556bb8,
    DAT_00556bf8,
    DAT_00556b38,
    DAT_00556b78,
    DAT_00556c3a,
    DAT_00556c70,
    DAT_00556c7c,
    DAT_00556c88,
    DAT_00556c94,
    DAT_00556cb2,
    DAT_009d8c44,
);

macro_rules! ap {
    ($b:ident, $o:expr) => {
        (&raw mut $b).cast::<u8>().offset(($o) as isize)
    };
}

macro_rules! sp {
    ($b:ident) => {
        (&raw const $b).cast::<u8>()
    };
}

macro_rules! st {
    ($b:ident, $o:expr, $v:expr) => {
        ap!($b, $o).write($v)
    };
}

unsafe fn mc(d: *mut u8, s: *const u8, n: usize) {
    // SAFETY: caller passes valid, non-overlapping game memory of n bytes.
    unsafe { core::ptr::copy_nonoverlapping(s, d, n) }
}

unsafe fn cps(d: *mut u8, s: *const u8) {
    // SAFETY: the source is a NUL-terminated string; the destination is the game's buffer.
    unsafe {
        let mut i = 0usize;
        loop {
            let c = s.add(i).read();
            d.add(i).write(c);
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn skip_blocks(fp: *mut c_void) {
    // SAFETY: file calls on the game's own file handle.
    unsafe {
        let mut l: i16 = 0;
        let mut n: i16 = 0x20;
        loop {
            FUN_0049ea10((&raw mut l).cast::<c_void>(), 2, 1, fp);
            if l != 0 {
                _fseek(fp, l as i32, 1);
            }
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
    }
}

unsafe fn rd_name(buf: *mut i8, fp: *mut c_void, unaff: &mut u32) {
    // SAFETY: buf holds at least 0x40 bytes; callees are the game's own.
    unsafe {
        FUN_0049ea10(buf.cast::<c_void>(), 0x40, 1, fp);
        let mut iv: i32 = -1;
        let mut p = buf as *const i8;
        loop {
            if iv == 0 {
                break;
            }
            iv = iv.wrapping_sub(1);
            let c = p.read();
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
        let l = (!(iv as u16)).wrapping_sub(1) as i16;
        let hi = ((*unaff >> 16) as u16) as u32;
        if l < 1 {
            *unaff = hi << 16;
        } else {
            let s = FUN_004ac5d0(l as i32 + 1, 0);
            *unaff = (hi << 16) | (s as u16 as u32);
            if s != 0 {
                let p16 = FUN_004ac720(*unaff);
                mc(p16.cast::<u8>(), buf.cast::<u8>(), l as usize);
                p16.offset(l as isize).write(0);
            }
            FUN_004ac740();
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045ad80(param_1: *mut i8) -> u8 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed and used exactly where the original does.
    unsafe {
        let mut unaff_ebp: u32 = 0;
        let mut local_160: i16 = 0;
        let mut local_15e: i16 = 0;
        let mut local_15c: i16 = 0;
        let mut local_15a: i16 = 0;
        let mut local_158: i16;
        let mut local_156: i16;
        let mut local_154: i16 = 0;
        let mut local_152: i16;
        let local_150: *mut c_void;
        let mut local_14c: i32;
        let mut local_148: i32;
        let mut local_144: [i8; 160] = [0; 160];
        let mut local_a4: [u32; 41] = [0; 41];
        let mut sva: i16;

        let r = FUN_0049e720(param_1, (&raw const DAT_00523434).cast::<i8>(), 1, 0);
        if r == 0 {
            return 0;
        }
        local_150 = DAT_00a07bd8;
        FUN_0049ea10((&raw mut DAT_00a08294).cast::<c_void>(), 2, 1, local_150);
        'mid: {
            if DAT_00a08294 != 0xe {
                if DAT_00a08294 != -1 && DAT_00a08294 != 0xc && DAT_00a08294 != 0xd {
                    return 0;
                }
                if DAT_00a08294 != 0xe && DAT_00a08294 != 0xc && DAT_00a08294 != 0xd {
                    FUN_0049ea10((&raw mut local_15e).cast::<c_void>(), 2, 1, local_150);
                    FUN_0049ea10((&raw mut local_15c).cast::<c_void>(), 2, 1, local_150);
                    FUN_0049ea10((&raw mut local_160).cast::<c_void>(), 2, 1, local_150);
                    DAT_009a2000 = local_15e;
                    _DAT_009a2002 = local_15c;
                    FUN_0049ea10(
                        (&raw mut DAT_00556aa8).cast::<c_void>(),
                        0x1c2,
                        1,
                        local_150,
                    );
                    DAT_009a2004[0] = DAT_00556aab;
                    DAT_009a2008 = DAT_00556aac;
                    DAT_009a2009 = DAT_00556aad;
                    mc(ap!(DAT_009a200a, 0), sp!(DAT_00556aae), 4);
                    mc(ap!(DAT_009a200e, 0), sp!(DAT_00556ab2), 4);
                    cps(ap!(DAT_009fd462, 0), sp!(DAT_00556ab8));
                    cps(ap!(DAT_009fd4a2, 0), sp!(DAT_00556af8));
                    cps(ap!(DAT_009fd4e2, 0), sp!(DAT_00556bb8));
                    cps(ap!(DAT_009fd522, 0), sp!(DAT_00556bf8));
                    cps(ap!(DAT_009fd562, 0), sp!(DAT_00556b38));
                    cps(ap!(DAT_009fd5a2, 0), sp!(DAT_00556b78));
                    DAT_009fd5e3 = DAT_00556c38;
                    cps(ap!(DAT_009a2012, 0), sp!(DAT_00556c3a));
                    cps(ap!(DAT_009a2026, 0), sp!(DAT_00556c3a));
                    cps(ap!(DAT_009a203a, 0), sp!(DAT_00556c3a));
                    cps(ap!(DAT_009a204e, 0), sp!(DAT_00556c3a));
                    local_156 = 0;
                    if 0 < local_15e {
                        loop {
                            FUN_0049ea10(
                                (&raw mut DAT_00556c70).cast::<c_void>(),
                                0x124,
                                1,
                                local_150,
                            );
                            let i: i32 = (local_156 as i32).wrapping_mul(0x566);
                            cps(ap!(DAT_009d8c30, i), sp!(DAT_00556c70));
                            cps(ap!(DAT_009d8c44, i), sp!(DAT_00556c7c));
                            cps(ap!(DAT_009d8c58, i), sp!(DAT_00556c88));
                            cps(ap!(DAT_009d8c6c, i), sp!(DAT_00556c94));
                            st!(DAT_009d8c80, i, DAT_00556ca0);
                            st!(DAT_009d8c81, i, DAT_00556ca1);
                            st!(DAT_009d8c82, i, DAT_00556ca2);
                            st!(DAT_009d8c83, i, DAT_00556ca3);
                            st!(DAT_009d8c84, i, DAT_00556ca4);
                            st!(DAT_009d8c85, i, DAT_00556ca5);
                            st!(DAT_009d8c86, i, DAT_00556ca6);
                            st!(DAT_009d8c87, i, DAT_00556ca7);
                            st!(DAT_009d8c88, i, 0);
                            st!(DAT_009d8c89, i, DAT_00556ca8);
                            st!(DAT_009d8c8a, i, DAT_00556ca9);
                            st!(DAT_009d8c8b, i, DAT_00556caa);
                            st!(DAT_009d8c8c, i, DAT_00556cab);
                            st!(DAT_009d8c8d, i, DAT_00556cac);
                            st!(DAT_009d8c8e, i, DAT_00556cad);
                            st!(DAT_009d8c8f, i, DAT_00556cae);
                            st!(DAT_009d8c90, i, DAT_00556caf);
                            st!(DAT_009d8c91, i, DAT_00556cb0);
                            st!(DAT_009d8c92, i, DAT_00556cb1);
                            let c1 = DAT_00556cb2;
                            st!(DAT_009d8c93, i, 0);
                            if c1 == 0 {
                                st!(DAT_009d8c94, i, 0);
                            } else {
                                st!(DAT_009d8c94, i, 1);
                                st!(DAT_009d8c96, i, c1.wrapping_sub(1) as u8);
                            }
                            st!(DAT_009d8c95, i, 0);
                            st!(DAT_009d8c97, i, DAT_00556cb3);
                            st!(DAT_009d8c98, i, DAT_00556cb4);
                            st!(DAT_009d8c99, i, DAT_00556cb5);
                            st!(DAT_009d8c9a, i, DAT_00556cb6);
                            st!(DAT_009d8c9b, i, DAT_00556cb7);
                            st!(DAT_009d8c9c, i, DAT_00556cb8);
                            st!(DAT_009d8c9d, i, DAT_00556cb9);
                            st!(DAT_009d8c9e, i, DAT_00556cba);
                            st!(DAT_009d8c9f, i, DAT_00556cbb);
                            st!(DAT_009d8ca2, i, DAT_00556cbe);
                            st!(DAT_009d8ca3, i, DAT_00556cbf);
                            st!(DAT_009d8ca1, i, DAT_00556cbd);
                            st!(DAT_009d8ca5, i, DAT_00556cc1);
                            st!(DAT_009d8ca0, i, DAT_00556cbc);
                            st!(DAT_009d8ca4, i, DAT_00556cc0);
                            st!(DAT_009d8ca8, i, DAT_00556cc2);
                            st!(DAT_009d8cb5, i, 0);
                            st!(DAT_009d8cc6, i, 0);
                            st!(DAT_009d8cb6, i, DAT_00556cc4);
                            st!(DAT_009d8cb7, i, DAT_00556cc5);
                            st!(DAT_009d8cb8, i, DAT_00556cc6);
                            st!(DAT_009d8cb9, i, DAT_00556cc7);
                            st!(DAT_009d8cbb, i, DAT_00556cc9);
                            st!(DAT_009d8cba, i, DAT_00556cc8);
                            st!(DAT_009d8cc3, i, DAT_00556cca);
                            st!(DAT_009d8cc4, i, DAT_00556ccb);
                            st!(DAT_009d8cc5, i, DAT_00556ccc);
                            mc(ap!(DAT_009d8cc8, i), sp!(DAT_00556cce), 2);
                            st!(DAT_009d8cca, i, DAT_00556cd0);
                            st!(DAT_009d8ccb, i, DAT_00556cd1);
                            st!(DAT_009d8ccc, i, DAT_00556cd2);
                            st!(DAT_009d8ccd, i, DAT_00556cd3);
                            st!(DAT_009d8cce, i, DAT_00556cd4);
                            st!(DAT_009d8ccf, i, DAT_00556cd5);
                            st!(DAT_009d8cd0, i, DAT_00556cd6);
                            st!(DAT_009d8cd1, i, DAT_00556cd7);
                            mc(ap!(DAT_009d8cd2, i), sp!(DAT_00556cd8), 4);
                            mc(ap!(DAT_009d8cd6, i), sp!(DAT_00556cdc), 4);
                            mc(ap!(DAT_009d8cda, i), sp!(DAT_00556ce0), 4);
                            mc(ap!(DAT_009d8cde, i), sp!(DAT_00556ce4), 4);
                            mc(ap!(DAT_009d8ce2, i), sp!(DAT_00556ce8), 2);
                            mc(ap!(DAT_009d8d24, i), sp!(DAT_00556cea), 4);
                            mc(ap!(DAT_009d8d28, i), sp!(DAT_00556cee), 4);
                            mc(ap!(DAT_009d8d2c, i), sp!(DAT_00556cf2), 4);
                            mc(ap!(DAT_009d8d30, i), sp!(DAT_00556cf6), 4);
                            mc(ap!(DAT_009d8d34, i), sp!(DAT_00556cfa), 2);
                            mc(ap!(DAT_009d8d76, i), sp!(DAT_00556cfc), 4);
                            mc(ap!(DAT_009d8d7a, i), sp!(DAT_00556d00), 4);
                            mc(ap!(DAT_009d8d7e, i), sp!(DAT_00556d04), 4);
                            mc(ap!(DAT_009d8d82, i), sp!(DAT_00556d08), 4);
                            mc(ap!(DAT_009d8d86, i), sp!(DAT_00556d0c), 2);
                            st!(DAT_009d8cde, i, DAT_00556ce4);
                            st!(DAT_009d8ce0, i, DAT_00556ce6);
                            st!(DAT_009d8d30, i, DAT_00556cf6);
                            st!(DAT_009d8d32, i, DAT_00556cf8);
                            st!(DAT_009d8d82, i, DAT_00556d08);
                            st!(DAT_009d8d84, i, DAT_00556d0a);
                            st!(DAT_009d8cd2, i, DAT_00556cd8);
                            let u8v = DAT_00556cea;
                            st!(DAT_009d8d24, i, DAT_00556cea);
                            st!(DAT_009d8d76, i, u8v);
                            st!(DAT_009d8cdf, i, DAT_00556ce5);
                            st!(DAT_009d8ce1, i, DAT_00556ce7);
                            st!(DAT_009d8d31, i, DAT_00556cf7);
                            st!(DAT_009d8d33, i, DAT_00556cf9);
                            st!(DAT_009d8d83, i, DAT_00556d09);
                            st!(DAT_009d8d85, i, DAT_00556d0b);
                            st!(DAT_009d8e25, i, 0);
                            st!(DAT_009d8e26, i, DAT_00556d0e);
                            st!(DAT_009d8e27, i, DAT_00556d0f);
                            st!(DAT_009d8e29, i, 1);
                            st!(DAT_009d8e73, i, 2);
                            st!(DAT_009d8e74, i, DAT_00556d10);
                            st!(DAT_009d8e75, i, DAT_00556d11);
                            st!(DAT_009d8e77, i, 1);
                            st!(DAT_009d8ec1, i, 1);
                            st!(DAT_009d8ec2, i, DAT_00556d12);
                            st!(DAT_009d8ec3, i, DAT_00556d13);
                            st!(DAT_009d8ec5, i, 1);
                            st!(DAT_009d8f0f, i, 2);
                            st!(DAT_009d8f10, i, DAT_00556d14);
                            st!(DAT_009d8f11, i, DAT_00556d15);
                            st!(DAT_009d8f12, i, DAT_00556d16);
                            st!(DAT_009d8f13, i, 1);
                            local_152 = 0;
                            let mut uv8: u8;
                            let mut uv9: u8;
                            loop {
                                let iv27 = local_152 as i32;
                                let iv20 =
                                    (local_156 as i32).wrapping_mul(0x2b3).wrapping_add(iv27);
                                local_152 = local_152.wrapping_add(1);
                                let so = (iv27 * 2) as isize;
                                mc(
                                    ap!(DAT_009d9096, iv20.wrapping_mul(2)),
                                    sp!(DAT_00556d18).offset(so),
                                    2,
                                );
                                let iv20 = iv20.wrapping_mul(2);
                                mc(ap!(DAT_009d90c2, iv20), sp!(DAT_00556d36).offset(so), 2);
                                mc(ap!(DAT_009d90ee, iv20), sp!(DAT_00556d54).offset(so), 2);
                                mc(ap!(DAT_009d911a, iv20), sp!(DAT_00556d72).offset(so), 2);
                                uv9 = DAT_00556d92;
                                uv8 = DAT_00556d91;
                                if !(local_152 < 0xf) {
                                    break;
                                }
                            }
                            st!(DAT_009d9146, i, DAT_00556d90);
                            st!(DAT_009d9147, i, uv8);
                            st!(DAT_009d9148, i, uv9);
                            st!(DAT_009d9157, i, DAT_00556d93);
                            let b2 = ap!(DAT_009d8c87, i).read();
                            let set1 = if b2 == 0 || b2 == 4 {
                                true
                            } else if b2 == 2 || b2 == 3 || b2 == 5 {
                                (&raw const DAT_00556c22)
                                    .offset(((b2 as u32).wrapping_mul(0xc)) as isize)
                                    .read()
                                    == 0x31
                            } else {
                                false
                            };
                            st!(DAT_009d8c88, i, set1 as u8);
                            ap!(DAT_009d9192, i)
                                .cast::<u32>()
                                .write_unaligned(0xffff_ffff);
                            local_156 = local_156.wrapping_add(1);
                            if !(local_156 < local_15e) {
                                break;
                            }
                        }
                    }
                    sva = 0;
                    if 0 < local_15c {
                        loop {
                            FUN_0049ea10(
                                (&raw mut DAT_00556de8).cast::<c_void>(),
                                0x5a,
                                1,
                                local_150,
                            );
                            let iv20 = sva as i32;
                            sva = sva.wrapping_add(1);
                            let i = iv20.wrapping_mul(0x72);
                            _strncpy(
                                ap!(DAT_009fe800, i).cast::<i8>(),
                                (&raw const DAT_00556de8).cast::<i8>(),
                                0x40,
                            );
                            st!(DAT_009fe84a, i, DAT_00556e28);
                            st!(DAT_009fe84b, i, DAT_00556e29);
                            st!(DAT_009fe84c, i, DAT_00556e2a);
                            st!(DAT_009fe84d, i, DAT_00556e2b);
                            st!(DAT_009fe84e, i, DAT_00556e2c);
                            st!(DAT_009fe84f, i, DAT_00556e2d);
                            st!(DAT_009fe850, i, DAT_00556e2e);
                            st!(DAT_009fe851, i, DAT_00556e2f);
                            st!(DAT_009fe854, i, DAT_00556e41);
                            _strncpy(
                                ap!(DAT_009fe860, i).cast::<i8>(),
                                (&raw const DAT_00556e30).cast::<i8>(),
                                0x10,
                            );
                            st!(DAT_009fe870, i, DAT_00556e40);
                            st!(DAT_009fe840, i, 1);
                            if !(sva < local_15c) {
                                break;
                            }
                        }
                    }
                    sva = 0;
                    if 0 < local_160 {
                        loop {
                            FUN_0049ea10(
                                (&raw mut DAT_00556d98).cast::<c_void>(),
                                0x1c,
                                1,
                                local_150,
                            );
                            let iv = sva as i32;
                            sva = sva.wrapping_add(1);
                            st!(DAT_009a8080, iv.wrapping_mul(0x2a), DAT_00556d98);
                            let i = iv.wrapping_mul(0x2a);
                            st!(DAT_009a8084, i, DAT_00556d9c);
                            st!(DAT_009a8081, i, DAT_00556d99);
                            st!(DAT_009a8085, i, DAT_00556d9d);
                            st!(DAT_009a8083, i, DAT_00556d9b);
                            st!(DAT_009a8087, i, DAT_00556d9f);
                            st!(DAT_009a808a, i, DAT_00556db1);
                            st!(DAT_009a8082, i, DAT_00556d9a);
                            st!(DAT_009a8086, i, DAT_00556d9e);
                            _strncpy(
                                ap!(DAT_009a8096, i).cast::<i8>(),
                                (&raw const DAT_00556da0).cast::<i8>(),
                                0x10,
                            );
                            st!(DAT_009a80a6, i, DAT_00556db0);
                            st!(DAT_009a80a8, i, DAT_00556db2);
                            st!(DAT_009a80a9, i, 0);
                            if !(sva < local_160) {
                                break;
                            }
                        }
                    }
                    _fseek(local_150, 0x32a, 1);
                    skip_blocks(local_150);
                    skip_blocks(local_150);
                    break 'mid;
                }
            }
            if DAT_00a08294 == 10 {
                FUN_0049ea10((&raw mut DAT_00556e48).cast::<c_void>(), 0x82, 1, local_150);
                mc(ap!(DAT_009a2000, 0), sp!(DAT_00556e48), 2);
                DAT_009a2004[0] = DAT_00556e4c;
                mc(ap!(_DAT_009a2002, 0), sp!(DAT_00556e4a), 2);
                DAT_009a2004[1] = DAT_00556e4d;
                DAT_009a2006 = DAT_00556e4e;
                DAT_009a2007 = DAT_00556e4f;
                DAT_009a2008 = DAT_00556e50;
                DAT_009a2009 = DAT_00556e51;
                mc(ap!(DAT_009a200a, 0), sp!(DAT_00556e52), 4);
                mc(ap!(DAT_009a200e, 0), sp!(DAT_00556e56), 4);
                mc(ap!(DAT_009a2012, 0), sp!(DAT_00556e5a), 4);
                mc(ap!(DAT_009a2016, 0), sp!(DAT_00556e5e), 4);
                mc(ap!(DAT_009a201a, 0), sp!(DAT_00556e62), 4);
                mc(ap!(DAT_009a2026, 0), sp!(DAT_00556e66), 4);
                mc(ap!(DAT_009a202a, 0), sp!(DAT_00556e6a), 4);
                mc(ap!(DAT_009a202e, 0), sp!(DAT_00556e6e), 4);
                mc(ap!(DAT_009a203a, 0), sp!(DAT_00556e72), 4);
                mc(ap!(DAT_009a203e, 0), sp!(DAT_00556e76), 4);
                mc(ap!(DAT_009a2042, 0), sp!(DAT_00556e7a), 4);
                mc(ap!(DAT_009a204e, 0), sp!(DAT_00556e7e), 4);
                mc(ap!(DAT_009a2052, 0), sp!(DAT_00556e82), 4);
                mc(ap!(DAT_009a2056, 0), sp!(DAT_00556e86), 4);
                mc(ap!(DAT_009a2062, 0), sp!(DAT_00556e8a), 4);
            } else {
                FUN_0049ea10(local_a4.as_mut_ptr().cast::<c_void>(), 0xa2, 1, local_150);
                core::ptr::write_bytes(ap!(DAT_009a2000, 0), 0, 0xa2);
                mc(ap!(DAT_009a2000, 0), local_a4.as_ptr().cast::<u8>(), 0xa2);
            }
            sva = 0;
            if 0 < DAT_009a2000 {
                loop {
                    let puv21: *mut u8;
                    let sv29: u32;
                    if DAT_00a08294 == 10 {
                        FUN_0049ea10((&raw mut DAT_00556db8).cast::<c_void>(), 0x30, 1, local_150);
                        let i = (sva as i32).wrapping_mul(0x566);
                        mc(ap!(DAT_009d8c30, i), sp!(DAT_00556db8), 4);
                        mc(ap!(DAT_009d8c34, i), sp!(DAT_00556dbc), 4);
                        mc(ap!(DAT_009d8c38, i), sp!(DAT_00556dc0), 4);
                        mc(ap!(DAT_009d8c44, i), sp!(DAT_00556dc4), 4);
                        mc(ap!(DAT_009d8c48, i), sp!(DAT_00556dc8), 4);
                        mc(ap!(DAT_009d8c4c, i), sp!(DAT_00556dcc), 4);
                        mc(ap!(DAT_009d8c58, i), sp!(DAT_00556dd0), 4);
                        mc(ap!(DAT_009d8c5c, i), sp!(DAT_00556dd4), 4);
                        mc(ap!(DAT_009d8c60, i), sp!(DAT_00556dd8), 4);
                        puv21 = ap!(DAT_009d8c80, i);
                        mc(ap!(DAT_009d8c6c, i), sp!(DAT_00556ddc), 4);
                        mc(ap!(DAT_009d8c70, i), sp!(DAT_00556de0), 4);
                        mc(ap!(DAT_009d8c74, i), sp!(DAT_00556de4), 4);
                        sv29 = 0x512;
                    } else {
                        sv29 = 0x562;
                        puv21 = ap!(DAT_009d8c30, (sva as i32).wrapping_mul(0x566));
                    }
                    FUN_0049ea10(puv21.cast::<c_void>(), sv29, 1, local_150);
                    let sv14 = DAT_009a2000;
                    let iv = sva as i32;
                    sva = sva.wrapping_add(1);
                    unaff_ebp = (iv as u32).wrapping_mul(0x159);
                    ap!(DAT_009d9192, iv.wrapping_mul(0x566))
                        .cast::<u32>()
                        .write_unaligned(0xffff_ffff);
                    if !(sva < sv14) {
                        break;
                    }
                }
            }
            sva = 0;
            if 0 < _DAT_009a2002 {
                loop {
                    let mut sv14 = sva;
                    if DAT_00a08294 != 10 {
                        local_152 = 0;
                        FUN_0049ea10((&raw mut local_152).cast::<c_void>(), 2, 1, local_150);
                        sv14 = local_152;
                    }
                    sva = sva.wrapping_add(1);
                    FUN_0049ea10(
                        ap!(DAT_009fe800, (sv14 as i32).wrapping_mul(0x72)).cast::<c_void>(),
                        0x72,
                        1,
                        local_150,
                    );
                    if !(sva < _DAT_009a2002) {
                        break;
                    }
                }
            }
            sva = 0;
            loop {
                FUN_0049ea10((&raw mut local_15a).cast::<c_void>(), 2, 1, local_150);
                let iv = sva as i32;
                sva = sva.wrapping_add(1);
                FUN_0049ea10(
                    ap!(DAT_009a8080, iv.wrapping_mul(0x126)).cast::<c_void>(),
                    0x2a,
                    local_15a as i32,
                    local_150,
                );
                if !(sva < 10) {
                    break;
                }
            }
            sva = 0;
            loop {
                FUN_0049ea10((&raw mut local_15a).cast::<c_void>(), 2, 1, local_150);
                if local_15a != 0 {
                    FUN_0049ea10(
                        ap!(DAT_009fd440, (sva as i32).wrapping_mul(0x1e5)).cast::<c_void>(),
                        0x1e5,
                        1,
                        local_150,
                    );
                }
                sva = sva.wrapping_add(1);
                if !(sva < 10) {
                    break;
                }
            }
            local_152 = 0;
            loop {
                sva = 10;
                _fseek(local_150, 0x32a, 1);
                loop {
                    _fseek(local_150, 1, 1);
                    sva = sva.wrapping_sub(1);
                    if sva == 0 {
                        break;
                    }
                }
                skip_blocks(local_150);
                skip_blocks(local_150);
                local_152 = local_152.wrapping_add(1);
                if !(local_152 < 8) {
                    break;
                }
            }
            local_156 = 0;
            if 0 < DAT_009a2000 {
                loop {
                    local_152 = 0;
                    local_14c = (local_156 as i32).wrapping_shl(3);
                    loop {
                        sva = 0;
                        loop {
                            rd_name(local_144.as_mut_ptr(), local_150, &mut unaff_ebp);
                            let iv = sva as i32;
                            sva = sva.wrapping_add(1);
                            let idx = ((local_152 as i32).wrapping_add(local_14c))
                                .wrapping_mul(3)
                                .wrapping_add(iv);
                            (&raw mut DAT_009d8120)
                                .cast::<i16>()
                                .offset(idx as isize)
                                .write(unaff_ebp as i16);
                            if !(sva < 3) {
                                break;
                            }
                        }
                        local_152 = local_152.wrapping_add(1);
                        if !(local_152 < 8) {
                            break;
                        }
                    }
                    local_156 = local_156.wrapping_add(1);
                    if !(local_156 < DAT_009a2000) {
                        break;
                    }
                }
            }
            local_156 = 0;
            loop {
                local_152 = 0;
                local_14c = (local_156 as i32).wrapping_mul(7);
                loop {
                    local_158 = 0;
                    loop {
                        local_148 = local_158 as i32;
                        sva = 0;
                        loop {
                            rd_name(local_144.as_mut_ptr(), local_150, &mut unaff_ebp);
                            let iv = sva as i32;
                            sva = sva.wrapping_add(1);
                            let idx = local_148
                                .wrapping_add(
                                    ((local_152 as i32).wrapping_add(local_14c)).wrapping_mul(4),
                                )
                                .wrapping_mul(3)
                                .wrapping_add(iv);
                            (&raw mut DAT_009e8f60)
                                .cast::<i16>()
                                .offset(idx as isize)
                                .write(unaff_ebp as i16);
                            if !(sva < 3) {
                                break;
                            }
                        }
                        local_158 = local_158.wrapping_add(1);
                        if !(local_158 < 4) {
                            break;
                        }
                    }
                    local_152 = local_152.wrapping_add(1);
                    if !(local_152 < 7) {
                        break;
                    }
                }
                local_156 = local_156.wrapping_add(1);
                if !(local_156 < 10) {
                    break;
                }
            }
        }
        sva = 0;
        if 0 < DAT_009a2000 {
            loop {
                local_152 = 0;
                let i = (sva as i32).wrapping_mul(0x566);
                let mut c1 = ap!(DAT_009d8c44, i).read() as i8;
                while c1 != 0 && local_152 < 0x10 {
                    let o = i.wrapping_add(local_152 as i32);
                    c1 = ap!(DAT_009d8c44, o).read() as i8;
                    if 0x60 < c1 && c1 < 0x7b {
                        ap!(DAT_009d8c44, o).write(c1.wrapping_sub(0x20) as u8);
                    }
                    local_152 = local_152.wrapping_add(1);
                    c1 = ap!(DAT_009d8c44, i.wrapping_add(local_152 as i32)).read() as i8;
                }
                sva = sva.wrapping_add(1);
                if !(sva < DAT_009a2000) {
                    break;
                }
            }
        }
        sva = 0;
        loop {
            let iv = sva as i32;
            sva = sva.wrapping_add(1);
            st!(DAT_009fd458, iv.wrapping_mul(0x1e6), 1);
            if !(sva < 10) {
                break;
            }
        }
        let mut pc26: *mut i8 = &raw mut DAT_009a808f;
        loop {
            let mut n: i32 = 3;
            let mut pc22 = pc26;
            loop {
                let c1 = pc22.offset(-0xf).read();
                let c3 = pc22.offset(-0xb).read();
                let c4 = pc22.offset(-4).read();
                let c5 = pc22.read();
                if c1 == 10 {
                } else if c3 == 10 && c4 == 10 && c5 == 10 {
                    pc22.offset(-0xb).write(0);
                    pc22.offset(-5).write(0);
                    pc22.offset(-4).write(0);
                    pc22.offset(6).write(0);
                    pc22.write(0);
                    pc22.offset(0x18).write(0);
                } else if c3 == 10 || c4 != 10 || c5 != 10 {
                    if c3 != 10 && c4 != 10 && c5 == 10 {
                        pc22.offset(6).write(0);
                        pc22.write(0);
                    }
                } else {
                    pc22.offset(-4).write(0);
                    pc22.offset(6).write(0);
                    pc22.write(0);
                    pc22.offset(0x18).write(0);
                }
                pc22 = pc22.offset(0x2a);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            pc26 = pc26.offset(0x126);
            if (pc26 as usize) > 0x9a8c0a {
                DAT_00a07bd8 = local_150;
                let r = FUN_0049e9c0(0);
                return ((r as u16) == 0) as u8;
            }
        }
    }
}
