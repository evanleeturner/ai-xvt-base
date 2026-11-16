// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_00521f2e: [u8; 0];

    static mut DAT_00a083ba: u16;
    static mut DAT_00a08a02: u16;
    static mut DAT_00a08344: u16;
    static mut DAT_00a08c7e: u16;
    static mut DAT_00a08c88: u16;
    static mut DAT_00a08c86: u16;
    static mut DAT_00a0a1d2: u16;
    static mut DAT_00a0a1d4: u16;
    static mut DAT_00a08364: u8;
    static mut DAT_00a0837a: u8;

    static mut DAT_00a083c0: [u8; 0];
    static mut DAT_00a084e0: [u8; 0];
    static mut DAT_00a0a540: [u8; 0];
    static mut DAT_00a0a660: [u8; 0];
    static mut DAT_00a08360: *mut core::ffi::c_void;
    static mut DAT_00a08a04: *mut core::ffi::c_void;
    static mut DAT_00a08bb0: *mut core::ffi::c_void;
    static mut DAT_00a08340: *mut core::ffi::c_void;

    static mut DAT_009a1fec: *const core::ffi::c_void;
    static mut DAT_009d12e4: *const core::ffi::c_void;
    static mut DAT_009a20a8: *const core::ffi::c_void;
    static mut DAT_009ec46c: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(a: u16) -> u32;
    fn FUN_0043a0c0(a: u16);
}

unsafe fn rd32(a: u32) -> i32 {
    // SAFETY: the caller supplies an address inside the game's own memory.
    unsafe { (a as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00439d90() {
    // SAFETY: all globals, tables and callees belong to the game; addresses
    // are computed and dereferenced exactly as the original does.
    unsafe {
        let base = (&raw mut DAT_009e9670) as u32;
        let flags = (&raw mut DAT_00521f2e) as u32;
        let mut pb13: u32 = base.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd));

        let mut i10 = rd32(pb13);
        let mut u11 = (i10 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32);
        i10 = rd32(u11);
        u11 = (i10 as u32).wrapping_add(0xa9);
        i10 = rd32(u11);
        u11 = (i10 as u32).wrapping_add(0x1e1);
        let u2: u16 = (u11 as *const u16).read_unaligned();

        if (u2 & 0x80) != 0 && (u2 & 0x100) != 0 {
            DAT_00a083ba = DAT_00a08a02;
            DAT_00a08344 = DAT_00a08c7e;
            DAT_00a08c7e = 0;
            DAT_00a08a02 = 0;
            DAT_00a08c88 = DAT_00a0a1d2;
            DAT_00a08c86 = DAT_00a0a1d4;
            if DAT_00a08364 == 0 {
                DAT_00a08360 = (&raw mut DAT_00a083c0).cast();
                DAT_00a08a04 = (&raw mut DAT_00a084e0).cast();
                DAT_00a08bb0 = (&raw mut DAT_00a0a540).cast();
                DAT_00a08340 = (&raw mut DAT_00a0a660).cast();
            } else {
                DAT_00a08360 = (&raw mut DAT_00a084e0).cast();
                DAT_00a08a04 = (&raw mut DAT_00a083c0).cast();
                DAT_00a08bb0 = (&raw mut DAT_00a0a660).cast();
                DAT_00a08340 = (&raw mut DAT_00a0a540).cast();
            }

            if _DAT_00a07cec < DAT_009a1fe0 {
                let mut i8v: u32 = (_DAT_00a07cec as u32).wrapping_mul(0x23);
                let mut u5: u32 = _DAT_00a07cec as u32;
                loop {
                    pb13 = base.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd));
                    let v11 = (pb13 as *const u32).read_unaligned();
                    if v11 != u5 {
                        let b =
                            *((DAT_009a1fe8 as u32).wrapping_add(i8v).wrapping_add(3) as *const u8);
                        if (*((flags.wrapping_add((b as u32).wrapping_mul(0x1a))) as *const u8) & 1)
                            != 0
                        {
                            let a = (DAT_009a1fe8 as u32).wrapping_add(i8v).wrapping_add(0x1f);
                            let i10 = rd32(a);
                            let a2 = (i10 as u32).wrapping_add(0xa9);
                            let i6 = rd32(a2);
                            let u12 = (pb13.wrapping_add(0x1a) as *const u16).read_unaligned();
                            let mut call = u12 as u32 == u5;
                            if !call {
                                let u3 = FUN_00484f80(u5 as u16);
                                if (u3 as u8) == 0 {
                                    let c1 = *((i6 as u32).wrapping_add(7) as *const i8);
                                    if c1 != 3 && c1 != 4 {
                                        call = true;
                                    }
                                }
                            }
                            if call {
                                FUN_0043a0c0(u5 as u16);
                            }
                        }
                    }
                    i8v = i8v.wrapping_add(0x23);
                    u5 = u5.wrapping_add(1);
                    if !((u5 as i32) < DAT_009a1fe0) {
                        break;
                    }
                }
            }

            if DAT_009e9640 < DAT_00a00854 {
                let mut i8v: u32 = (DAT_009e9640 as u32).wrapping_mul(0x23);
                let mut i6: i32 = DAT_009e9640;
                loop {
                    let b = *((DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(i8v) as *const u8);
                    if (*((flags.wrapping_add((b as u32).wrapping_mul(0x1a))) as *const u8) & 1)
                        != 0
                    {
                        FUN_0043a0c0(i6 as u16);
                    }
                    i8v = i8v.wrapping_add(0x23);
                    i6 = i6.wrapping_add(1);
                    if !(i6 < DAT_00a00854) {
                        break;
                    }
                }
            }

            let mut i8v: i32 = (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32;
            if DAT_00a07ce8 < i8v {
                let mut i6: u32 = (DAT_00a07ce8 as u32).wrapping_mul(0x23);
                let mut i7: i32 = DAT_00a07ce8;
                loop {
                    let b = *((DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(i6) as *const u8);
                    if (*((flags.wrapping_add((b as u32).wrapping_mul(0x1a))) as *const u8) & 1)
                        != 0
                    {
                        FUN_0043a0c0(i7 as u16);
                    }
                    i6 = i6.wrapping_add(0x23);
                    i7 = i7.wrapping_add(1);
                    i8v = (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32;
                    if !(i7 < i8v) {
                        break;
                    }
                }
            }

            if DAT_00a0837a != 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009a1fec);
                f();
            }
            if DAT_00a083ba != 0 {
                let f: unsafe extern "C" fn(*mut core::ffi::c_void, u32) =
                    core::mem::transmute(DAT_009d12e4);
                f(DAT_00a08360, DAT_00a083ba as u32);
            }
            if DAT_00a08a02 != 0 {
                let f: unsafe extern "C" fn(*mut core::ffi::c_void, u32) =
                    core::mem::transmute(DAT_009a20a8);
                f(DAT_00a08a04, DAT_00a08a02 as u32);
            }
            if DAT_00a08344 != 0 {
                let f: unsafe extern "C" fn(*mut core::ffi::c_void, u32) =
                    core::mem::transmute(DAT_009d12e4);
                f(DAT_00a08bb0, DAT_00a08344 as u32);
            }
            if DAT_00a08c7e != 0 {
                let f: unsafe extern "C" fn(*mut core::ffi::c_void, u32) =
                    core::mem::transmute(DAT_009a20a8);
                f(DAT_00a08340, DAT_00a08c7e as u32);
            }

            pb13 = base.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd));
            let u12 = (pb13.wrapping_add(0x1a) as *const u16).read_unaligned();
            let b9: u8 = ((u12 as i16) != -1) as u8;
            if b9 != 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009ec46c);
                f();
            }
            DAT_00a08364 ^= 1;
            DAT_00a0837a = b9;
        }
    }
}
