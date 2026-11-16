// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c818: *mut u32;
    static mut DAT_0052c81c: i32;
    static mut DAT_0052c820: [i32; 2];
    static mut DAT_0052c82c: i32;
    static mut DAT_0052c83c: u32;
    static mut DAT_0052c840: u32;
    static mut DAT_0052c844: u16;
    static mut DAT_0052c846: u8;
    static mut DAT_0052c848: i8;
    static mut DAT_0052c850: [u8; 8];
    static mut DAT_0052c808: u32;
    static mut DAT_0052c80c: u32;
    static mut _DAT_0052c810: u32;
    static mut DAT_0052c814: u32;
    static mut DAT_0051c4bc: i8;
    static mut DAT_00aa6078: *mut core::ffi::c_void;
    static mut DAT_00aa6080: i32;
    static mut DAT_00aa6084: i32;
    static mut DAT_00665d98: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ClientToScreen(h: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> i32;
    fn PeekMessageA(
        msg: *mut core::ffi::c_void,
        h: *mut core::ffi::c_void,
        a: u32,
        b: u32,
        c: u32,
    ) -> i32;
    fn TranslateMessage(msg: *const core::ffi::c_void) -> i32;
    fn DispatchMessageA(msg: *const core::ffi::c_void) -> i32;
    #[link_name = "\u{1}_SmackSoundUseDirectSound_4"]
    fn _SmackSoundUseDirectSound_4(a: u32);
    #[link_name = "\u{1}_SmackOpen_12"]
    fn _SmackOpen_12(a: *mut i8, b: u32, c: u32) -> i32;
    #[link_name = "\u{1}_SmackWait_4"]
    fn _SmackWait_4(a: i32) -> i32;
    #[link_name = "\u{1}_SmackClose_4"]
    fn _SmackClose_4(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ef590(h: *mut core::ffi::c_void);
    fn FUN_004ef500() -> i32;
    fn FUN_004d4c10();
    fn FUN_004d4f80();
    fn FUN_004d49a0();
    fn FUN_004ccb90() -> u8;
    fn FUN_004d5b80() -> i32;
    fn FUN_004d5b70(a: u8);
    fn FUN_004ce130();
    fn FUN_004ef600();
    #[link_name = "\u{1}_FID_conflict___wfopen"]
    fn FID_conflict___wfopen(a: *mut i8, b: *const i8) -> *mut core::ffi::c_void;
    #[link_name = "\u{1}_fclose"]
    fn _fclose(a: *mut core::ffi::c_void) -> i32;
}

unsafe fn strcat_(dst: *mut u8, src: *const u8) {
    // SAFETY: both point at NUL-terminated byte strings; dst has room.
    unsafe {
        let mut n = 0usize;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut d = dst;
        while *d != 0 {
            d = d.add(1);
        }
        let mut i = 0usize;
        while i < n {
            *d.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ef100(param_1: *mut u32) -> u8 {
    let mut buf: [u32; 75] = [0; 75];
    let p = buf.as_mut_ptr() as *mut u8;
    // SAFETY: the globals and callees are the game's own; the local buffer
    // mirrors the original stack frame (MSG at 0, string at 0x1c, 0x28).
    unsafe {
        let msg = p as *mut core::ffi::c_void;
        let s110 = p.add(0x1c);
        let s104 = p.add(0x28);

        DAT_0052c818 = param_1;
        FUN_004ef590(param_1.add(8).read() as usize as *mut core::ffi::c_void);
        DAT_0052c82c = FUN_004ef500();
        ClientToScreen(
            DAT_0052c818.add(8).read() as usize as *mut core::ffi::c_void,
            (&raw mut DAT_0052c820) as *mut core::ffi::c_void,
        );
        FUN_004d4c10();
        FUN_004d4f80();
        FUN_004d49a0();
        FUN_004d4c10();
        _SmackSoundUseDirectSound_4(DAT_0052c818.add(6).read());
        let mv = &raw const DAT_0052c850 as *const u8;
        let mut i = 0usize;
        while i < 8 {
            *s104.add(i) = mv.add(i).read();
            i += 1;
        }
        strcat_(s104, DAT_0052c818.read() as usize as *const u8);
        strcat_(s104, &raw const DAT_0052c848 as *const u8);
        DAT_0052c81c = _SmackOpen_12(s104 as *mut i8, 0xfe000, 0xffffffff);
        if DAT_0052c81c == 0 {
            let mut u110 = DAT_0052c83c;
            s110.add(4).cast::<u32>().write_unaligned(DAT_0052c840);
            s110.add(8).cast::<u16>().write_unaligned(DAT_0052c844);
            s110.add(10).write(DAT_0052c846);
            let u2 = FUN_004ccb90();
            u110 = (u110 & 0xffffff00) | (u2 as u32);
            s110.cast::<u32>().write_unaligned(u110);
            strcat_(s110, DAT_0052c818.read() as usize as *const u8);
            strcat_(s110, &raw const DAT_0052c848 as *const u8);
            DAT_0052c81c = _SmackOpen_12(s110 as *mut i8, 0xfe000, 0xffffffff);
            if DAT_0052c81c == 0 {
                DAT_0052c818 = core::ptr::null_mut();
                return 2;
            }
        }
        let mut n = 0usize;
        while *s110.add(n) != 0 {
            n += 1;
        }
        let u4 = (n as u32).wrapping_add(1) as usize;
        let a114 = p.add(0x18);
        *a114.add(u4 + 2) = b't';
        *a114.add(u4) = b't';
        *a114.add(u4 + 1) = b'x';
        DAT_00aa6078 = FID_conflict___wfopen(s110 as *mut i8, &raw const DAT_0051c4bc);
        let smk = DAT_0052c81c;
        let u4 = ((smk + 4) as *const u32).read_unaligned();
        let u6 = DAT_0052c818.add(1).read();
        if u6 < u4 && DAT_0052c818.add(2).read() < ((smk + 8) as *const u32).read_unaligned() {
            DAT_0052c818 = core::ptr::null_mut();
            _SmackClose_4(DAT_0052c81c);
            return 3;
        }
        DAT_0052c808 = u6.wrapping_sub(u4) >> 1;
        DAT_0052c80c = DAT_0052c818
            .add(2)
            .read()
            .wrapping_sub(((DAT_0052c81c + 8) as *const u32).read_unaligned())
            >> 1;
        _DAT_0052c810 = DAT_0052c818
            .add(1)
            .read()
            .wrapping_sub(((DAT_0052c81c + 4) as *const u32).read_unaligned())
            .wrapping_sub(DAT_0052c808);
        DAT_0052c814 = DAT_0052c818
            .add(2)
            .read()
            .wrapping_sub(((DAT_0052c81c + 8) as *const u32).read_unaligned())
            .wrapping_sub(DAT_0052c80c);
        DAT_00aa6080 = FUN_004d5b80();
        FUN_004d5b70(2);
        DAT_00aa6084 = 0;
        DAT_00665d98 = 0;
        let mut r = FUN_004d5b80();
        while r == 2 {
            FUN_004ce130();
            let b = PeekMessageA(msg, core::ptr::null_mut(), 0, 0, 1);
            if b == 0 {
                if DAT_00aa6084 == 0 {
                    let w = _SmackWait_4(DAT_0052c81c);
                    if w == 0 {
                        FUN_004ef600();
                    }
                } else if DAT_0052c818.add(0xb).read() == 0 {
                    FUN_004d5b70(DAT_00aa6080 as u8);
                } else {
                    let cb: unsafe extern "C" fn(u32, i32, u32) -> i32 =
                        core::mem::transmute(DAT_0052c818.add(0xb).read() as usize);
                    let a0 = ((DAT_0052c81c + 0x374) as *const u32).read_unaligned();
                    let a1 = ((DAT_0052c81c + 0xc) as *const i32)
                        .read_unaligned()
                        .wrapping_sub(1);
                    let a2 = DAT_0052c818.add(0xc).read();
                    let rr = cb(a0, a1, a2);
                    if rr == 1 {
                        FUN_004d5b70(DAT_00aa6080 as u8);
                    }
                    if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
                        let obj = DAT_0052c818.add(3).read() as usize;
                        let vt = (obj as *const u32).read_unaligned() as usize;
                        let f = ((vt + 0x2c) as *const usize).read_unaligned();
                        let fp: unsafe extern "stdcall" fn(usize, i32, i32) -> i32 =
                            core::mem::transmute(f);
                        fp(obj, 0, 1);
                    }
                }
            } else {
                TranslateMessage(msg);
                DispatchMessageA(msg);
            }
            r = FUN_004d5b80();
        }
        if !DAT_00aa6078.is_null() {
            _fclose(DAT_00aa6078);
            DAT_00aa6078 = core::ptr::null_mut();
        }
        _SmackClose_4(DAT_0052c81c);
        DAT_0052c818 = core::ptr::null_mut();
        ((((DAT_00665d98 == 0) as u32).wrapping_sub(1)) & 5) as u8
    }
}
