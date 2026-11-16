// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: u8;
    static mut DAT_00aa6cfc: u32;
    static mut DAT_00acebc5: *mut i32;
    static mut DAT_00acebf1: [u8; 0];
    static mut DAT_00acf145: [u8; 0];
    static mut DAT_00acf14d: [u8; 0];
    static mut DAT_00acf359: [u8; 0];
    static mut DAT_00b53571: u32;
    static mut DAT_00b53579: u32;
    static mut DAT_00b63d7d: u32;
    static mut DAT_00b69a89: u32;
    static mut DAT_00b69a8d: u32;
    static mut DAT_00b69a91: u32;

    static mut DAT_00acf571: [u8; 0];
    static mut DAT_00b5357d: [u8; 0];
    static mut DAT_00b53581: [u8; 0];
    static mut DAT_00b5358a: [u8; 0];
    static mut DAT_00b5358b: [u8; 0];
    static mut DAT_00b5358d: [u8; 0];
    static mut DAT_00b63f99: [u8; 0];
    static mut DAT_00b63fa1: [u8; 0];
    static mut DAT_00b63fa9: [u8; 0];
    static mut DAT_00b63fad: [u8; 0];
    static mut DAT_00b641b9: [u8; 0];
    static mut DAT_00b641bd: [u8; 0];
    static mut DAT_00b641c1: [u8; 0];
    static mut DAT_00b641c5: [u8; 0];

    static mut DAT_00665050: [i32; 0];
    static mut DAT_00665054: [u32; 0];
    static mut DAT_00665058: [u32; 0];
    static mut DAT_0066505c: [u32; 0];
    static mut DAT_00665060: [u32; 0];
    static mut DAT_00665064: [i32; 0];
    static mut DAT_00665410: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4970();
    fn FUN_004d1ea0();
    fn FUN_004d25d0();
    fn FUN_0046fad0() -> u32;
    #[link_name = "FUN_0046fad0"]
    fn FUN_0046fad0_arg(a: i32) -> u32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004d23b0(a: i32) -> u32;
    fn FUN_004cf980(a: i32, b: u32, c: u32, d: *mut u32, e: u32);
    fn FUN_004cf830(a: i32, b: *mut u32, c: i32, ...) -> i32;
    fn FUN_004d1fa0(a: i32, b: u32, c: u32, d: u32) -> i32;
    fn FUN_004d48e0() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

macro_rules! gp {
    ($x:ident) => {
        (&raw mut $x).cast::<u8>()
    };
}

unsafe fn ld(p: *const u8) -> u32 {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn st(p: *mut u8, v: u32) {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

fn at(base: *mut u8, idx: u32, size: u32) -> *mut u8 {
    base.wrapping_add(idx.wrapping_mul(size) as usize)
}

unsafe fn udiv(a: u32, b: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own unsigned divide, faulting on a zero divisor.
    unsafe {
        core::arch::asm!("div {d}", d = in(reg) b, inout("eax") a => q, inout("edx") 0u32 => _, options(nomem, nostack));
    }
    q
}

unsafe fn copy(mut pd: *mut u8, mut ps: *const u8, n: u32) {
    let mut c = n >> 2;
    while c != 0 {
        // SAFETY: dword copy between the buffers the original copies.
        unsafe {
            pd.cast::<u32>()
                .write_unaligned(ps.cast::<u32>().read_unaligned());
        }
        ps = ps.wrapping_add(4);
        pd = pd.wrapping_add(4);
        c -= 1;
    }
    let mut t = n & 3;
    while t != 0 {
        // SAFETY: byte tail of the same copy.
        unsafe {
            pd.write(ps.read());
        }
        ps = ps.wrapping_add(1);
        pd = pd.wrapping_add(1);
        t -= 1;
    }
}

unsafe fn outp() -> *mut u8 {
    // SAFETY: the game's own globals.
    unsafe { at(gp!(DAT_00acf571), ld(gp!(DAT_00b53571)), 0x210) }
}

unsafe fn advance_out() {
    // SAFETY: the game's own globals.
    unsafe {
        let mut t = ld(gp!(DAT_00b53571)).wrapping_add(1);
        st(gp!(DAT_00b53579), ld(gp!(DAT_00b53579)).wrapping_add(1));
        if 0x3ff < t {
            t = 0;
        }
        st(gp!(DAT_00b53571), t);
    }
}

unsafe fn bump_bd(i: u32) {
    // SAFETY: the game's own per-peer table.
    unsafe {
        let t = ld(at(gp!(DAT_00b641b9), i, 0x238));
        if 0x14 < t {
            let p = at(gp!(DAT_00b641bd), i, 0x238);
            st(p, ld(p).wrapping_add(1));
        }
    }
}

unsafe fn exit_path(l: u32) -> u32 {
    if l != 0 {
        // SAFETY: the game's own function and global.
        unsafe {
            DAT_00aa6cfc = FUN_004d48e0();
        }
    }
    0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "stdcall" fn FUN_004ce130() -> u32 {
    // SAFETY: every global, callee and pointer here is the game's own memory
    // or code, used where the original uses it; local buffers are in bounds.
    unsafe {
        let local_420: u32 = DAT_00aa7b0d as u32;
        FUN_004d4970();
        let mut p_obj: *mut i32 = ld(gp!(DAT_00acebc5)) as usize as *mut i32;
        if p_obj.is_null() {
            return exit_path(local_420);
        }
        FUN_004d1ea0();
        FUN_004d25d0();

        let mut local_43c: i32 = 0;
        let mut local_404: i32 = 0;
        let mut local_400: [u8; 0x400] = [0; 0x400];
        let b400: *mut u8 = local_400.as_mut_ptr();
        let mut local_428: u32 = 0;
        let mut st41c: [u32; 5] = [0; 5];
        let p41c: *mut u32 = st41c.as_mut_ptr();
        let mut au_pkt: [u32; 4] = [0; 4];
        let mut u_stack_438: u32 = 0;
        let mut u_stack_434: u32;
        let mut u_stack_430: u32;
        let mut u_stack_42c: u32;
        let mut i_stack_424: i32;
        let mut p11: *mut u8;
        let mut u7: u32;
        let mut u9: u32;
        let mut u6: u32;
        let mut u8_: u32;
        let mut pb_out: *mut u8;

        'top: loop {
            loop {
                loop {
                    loop {
                        if 0x3fe < ld(gp!(DAT_00b53579)) {
                            return exit_path(local_420);
                        }
                        local_428 = 0x400;
                        p_obj = ld(gp!(DAT_00acebc5)) as usize as *mut i32;
                        let vt = p_obj.read_unaligned() as u32;
                        let fp = ld(vt.wrapping_add(100) as usize as *const u8);
                        let f: unsafe extern "stdcall" fn(
                            *mut i32,
                            *mut i32,
                            *mut i32,
                            i32,
                            *mut u8,
                            *mut u32,
                        ) -> i32 = core::mem::transmute(fp as usize as *const c_void);
                        let r = f(
                            p_obj,
                            &raw mut local_43c,
                            &raw mut local_404,
                            1,
                            b400,
                            &raw mut local_428,
                        );
                        if r != 0 {
                            return exit_path(local_420);
                        }
                        if local_43c != 0 {
                            break;
                        }
                        if 0x200 < local_428 {
                            local_428 = 0x200;
                        }
                        pb_out = outp();
                        st(pb_out, 0);
                        st(pb_out.wrapping_add(4), local_428);
                        pb_out.wrapping_add(0xf).write(0);
                        copy(pb_out.wrapping_add(0x10), b400, local_428);
                        advance_out();
                    }
                    if ld(gp!(DAT_00acf145)) as i32 != local_404 {
                        continue;
                    }
                    break;
                }
                let hdr: u16 = b400.cast::<u16>().read_unaligned();
                p11 = b400.wrapping_add(2);
                u9 = (hdr & 0x7f) as u32;
                u_stack_42c = ((hdr & 0x80) != 0) as u32;
                u7 = ((hdr & 0x8000) == 0) as u32;
                u_stack_430 = ((hdr as u32) & 0xffff7f00) >> 8;
                if u9 < 0x3c || 0x3f < u9 {
                    i_stack_424 = 1;
                } else {
                    i_stack_424 = 0;
                }
                if i_stack_424 != 0 {
                    u_stack_438 = FUN_0046fad0();
                    if u_stack_438 == 0 {
                        p11 = b400.wrapping_add(4);
                        u_stack_438 = b400.wrapping_add(2).cast::<u16>().read_unaligned() as u32;
                    }
                }
                if u9 != 0x35 {
                    break;
                }
                st41c[0] = 0x34;
                FUN_004cef70(local_43c, p41c, 4);
            }

            if u9 == 0x36 {
                au_pkt = p11.cast::<[u32; 4]>().read_unaligned();
                u7 = FUN_004d23b0(local_43c);
                let mut dv = GetTickCount();
                st(at(gp!(DAT_00b641c5), u7, 0x238), dv);
                u7 = au_pkt[0];
                dv = GetTickCount();
                if u7 < dv.wrapping_sub(0x28) {
                    u7 = dv.wrapping_sub(0x28).wrapping_sub(u7);
                } else {
                    u7 = 1;
                }
                u9 = 0;
                let t50 = (&raw mut DAT_00665050).cast::<i32>();
                let t54 = (&raw mut DAT_00665054).cast::<u32>();
                let t58 = (&raw mut DAT_00665058).cast::<u32>();
                let t5c = (&raw mut DAT_0066505c).cast::<u32>();
                let t60 = (&raw mut DAT_00665060).cast::<u32>();
                let t64 = (&raw mut DAT_00665064).cast::<i32>();
                let lim = (&raw mut DAT_00665410) as *mut i32;
                let mut pv5: *mut i32 = t50;
                loop {
                    if pv5.read() == local_43c {
                        let ix = u9.wrapping_mul(6) as usize;
                        if u7 < 0x2ee {
                            if t64.wrapping_add(ix).read() != 0 {
                                u6 = t54.wrapping_add(ix).read();
                                u_stack_434 = u6 / (t64.wrapping_add(ix).read() as u32);
                                if u_stack_434 < u7 {
                                    if u_stack_434 == 0 {
                                        RaiseException(0xC0000094, 0, 0, core::ptr::null());
                                    }
                                    if udiv(
                                        u7.wrapping_sub(u_stack_434).wrapping_mul(100),
                                        u_stack_434,
                                    ) < 0x33
                                    {
                                        t54.wrapping_add(ix).write(u6.wrapping_add(u7));
                                        t58.wrapping_add(ix).write(au_pkt[1]);
                                        t5c.wrapping_add(ix).write(au_pkt[2]);
                                        t60.wrapping_add(ix).write(au_pkt[3]);
                                        t64.wrapping_add(ix)
                                            .write(t64.wrapping_add(ix).read().wrapping_add(1));
                                    } else {
                                        t58.wrapping_add(ix).write(au_pkt[1]);
                                        t5c.wrapping_add(ix).write(au_pkt[2]);
                                        t60.wrapping_add(ix).write(au_pkt[3]);
                                    }
                                } else {
                                    t54.wrapping_add(ix).write(u6.wrapping_add(u7));
                                    t58.wrapping_add(ix).write(au_pkt[1]);
                                    t5c.wrapping_add(ix).write(au_pkt[2]);
                                    t60.wrapping_add(ix).write(au_pkt[3]);
                                    t64.wrapping_add(ix)
                                        .write(t64.wrapping_add(ix).read().wrapping_add(1));
                                }
                            }
                        } else {
                            t58.wrapping_add(ix).write(au_pkt[1]);
                            t5c.wrapping_add(ix).write(au_pkt[2]);
                            t60.wrapping_add(ix).write(au_pkt[3]);
                        }
                        break;
                    }
                    pv5 = pv5.wrapping_add(6);
                    u9 = u9.wrapping_add(1);
                    if !(pv5 < lim) {
                        break;
                    }
                }
                if 0x27 < u9 {
                    let mut i3: u32 = 0;
                    pv5 = t50;
                    loop {
                        if pv5.read() == 0 {
                            if 0x2ee < u7 {
                                u7 = 0x2ee;
                            }
                            let ix = i3.wrapping_mul(6) as usize;
                            t50.wrapping_add(ix).write(local_43c);
                            t54.wrapping_add(ix).write(u7);
                            t58.wrapping_add(ix).write(au_pkt[1]);
                            t5c.wrapping_add(ix).write(au_pkt[2]);
                            t60.wrapping_add(ix).write(au_pkt[3]);
                            t64.wrapping_add(ix).write(1);
                            break;
                        }
                        pv5 = pv5.wrapping_add(6);
                        i3 = i3.wrapping_add(1);
                        if !(pv5 < lim) {
                            break;
                        }
                    }
                }
                continue 'top;
            }

            if u9 == 0x33 {
                if ld(gp!(DAT_00b69a8d)) != 0 {
                    au_pkt = p11.cast::<[u32; 4]>().read_unaligned();
                    u7 = au_pkt[0];
                    u9 = au_pkt[1];
                    u6 = FUN_004d23b0(local_43c);
                    bump_bd(u6);
                    u8_ = 0;
                    u6 = ld(gp!(DAT_00b69a91));
                    loop {
                        let rec = (u6.wrapping_mul(0x210).wrapping_add(ld(gp!(DAT_00b69a8d))))
                            as usize as *mut u8;
                        if ld(rec.wrapping_add(4)) != 0
                            && (ld(rec.wrapping_add(0x14)) & 0x7fffffff) == u7
                        {
                            FUN_004cf980(
                                local_43c,
                                0,
                                rec.wrapping_add(0xe).read() as u32,
                                rec.wrapping_add(0x10).cast::<u32>(),
                                ld(rec.wrapping_add(4)),
                            );
                            break;
                        }
                        u8_ += 1;
                        u6 = u6.wrapping_add(1);
                        if 0xff < u6 {
                            u6 = 0;
                        }
                        if !(u8_ < 0x100) {
                            break;
                        }
                    }
                    if 0xff < u8_ {
                        st41c[0] = 0x39;
                        FUN_004cf980(local_43c, 0, u9, p41c, 4);
                    }
                }
            } else if u9 == 0x38 {
                au_pkt = p11.cast::<[u32; 4]>().read_unaligned();
                u7 = au_pkt[0];
                u9 = au_pkt[1];
                u6 = FUN_004d23b0(local_43c);
                let dv = GetTickCount();
                st(at(gp!(DAT_00b641c5), u6, 0x238), dv);
                bump_bd(u6);
                u8_ = 0;
                u6 = ld(gp!(DAT_00b63d7d));
                loop {
                    let u_dat = ld(at(gp!(DAT_00b53581), u6, 0x210));
                    if u_dat != 0 {
                        let b8b = at(gp!(DAT_00b5358b), u6, 0x210).read() as u32;
                        let b8a = at(gp!(DAT_00b5358a), u6, 0x210).read() as u32;
                        let mut chk = false;
                        if u9 == 0 || u9 == 2 {
                            if b8b == u7 {
                                chk = true;
                            }
                        } else if ld(at(gp!(DAT_00b5357d), u6, 0x210)) == local_43c as u32
                            && b8b == u7
                        {
                            chk = true;
                        }
                        if chk && b8a == u9 {
                            FUN_004cf980(
                                local_43c,
                                u9,
                                u7,
                                at(gp!(DAT_00b5358d), u6, 0x210).cast::<u32>(),
                                u_dat,
                            );
                            break;
                        }
                    }
                    u8_ += 1;
                    u6 = u6.wrapping_add(1);
                    if 0x7f < u6 {
                        u6 = 0;
                    }
                    if !(u8_ < 0x80) {
                        break;
                    }
                }
                if 0x7f < u8_ {
                    st41c[0] = 0x39;
                    FUN_004cf980(local_43c, u9, u7, p41c, 4);
                }
            } else if u9 == 0x37 {
                au_pkt = p11.cast::<[u32; 4]>().read_unaligned();
                u7 = au_pkt[0];
                u9 = au_pkt[1];
                u6 = au_pkt[2];
                let mut i_peer = ld(gp!(DAT_00acebf1)) as i32;
                if i_peer == local_43c {
                    u_stack_434 = FUN_004d23b0(local_43c);
                    u_stack_434 = u_stack_434.wrapping_mul(0x238);
                    let dv = GetTickCount();
                    st41c[0] = 0x36;
                    st(gp!(DAT_00b641c5).wrapping_add(u_stack_434 as usize), dv);
                    st41c[2] = ld(gp!(DAT_00b641b9).wrapping_add(u_stack_434 as usize));
                    st41c[1] = au_pkt[3];
                    st41c[3] = ld(gp!(DAT_00b641bd).wrapping_add(u_stack_434 as usize));
                    st41c[4] = ld(gp!(DAT_00b641c1).wrapping_add(u_stack_434 as usize));
                    i_peer = ld(gp!(DAT_00acebf1)) as i32;
                    FUN_004cf830(i_peer, p41c, 0x14, 0i32);
                }
                if ld(gp!(DAT_00acf14d)) == u7 {
                    u7 = 0x80;
                }
                if ld(gp!(DAT_00acf359)) == u9 {
                    u9 = 0x80;
                }
                u8_ = FUN_004d23b0(local_43c);
                if ld(gp!(DAT_00b69a89)) == u8_ || ld(at(gp!(DAT_00b63fad), u8_, 0x238)) == u6 {
                    u6 = 0x80;
                }
                u_stack_434 = 0x80;
                u8_ = ld(gp!(DAT_00b63d7d));
                loop {
                    if u7 == 0x80 && u9 == 0x80 && u6 == 0x80 {
                        break;
                    }
                    let u2 = ld(at(gp!(DAT_00b53581), u8_, 0x210));
                    let mut u14 = u6;
                    if u2 != 0 {
                        let b8a = at(gp!(DAT_00b5358a), u8_, 0x210).read();
                        let b8b = at(gp!(DAT_00b5358b), u8_, 0x210).read() as u32;
                        let pd = at(gp!(DAT_00b5358d), u8_, 0x210).cast::<u32>();
                        if b8a == 0 {
                            if b8b == u7 {
                                FUN_004cf980(local_43c, 0, u7, pd, u2);
                                u7 = 0x80;
                            }
                        } else if b8a == 2 {
                            if b8b == u9 {
                                FUN_004cf980(local_43c, 2, u9, pd, u2);
                                u9 = 0x80;
                            }
                        } else if ld(at(gp!(DAT_00b5357d), u8_, 0x210)) == local_43c as u32
                            && b8b == u6
                        {
                            u14 = 0x80;
                            FUN_004cf980(local_43c, 1, u6, pd, u2);
                        }
                    }
                    u8_ = u8_.wrapping_add(1);
                    if 0x7f < u8_ {
                        u8_ = 0;
                    }
                    u_stack_434 = u_stack_434.wrapping_sub(1);
                    u6 = u14;
                    if u_stack_434 == 0 {
                        break;
                    }
                }
            } else {
                u6 = FUN_004d23b0(local_43c);
                let dv = GetTickCount();
                st(at(gp!(DAT_00b641c5), u6, 0x238), dv);
                if u7 != 0 && u_stack_42c != 0 {
                    let b17 = b400.wrapping_add(2).read() != 0;
                    if b17 {
                        u_stack_42c = (b400.wrapping_add(2).read() == 2) as u32;
                    } else {
                        u_stack_42c = 0;
                    }
                    let mut p13 = b400.wrapping_add(3);
                    if i_stack_424 == 0 {
                        u_stack_438 = local_428.wrapping_sub(2);
                    } else {
                        u_stack_438 = FUN_0046fad0();
                        if u_stack_438 == 0 {
                            p13 = b400.wrapping_add(5);
                            u_stack_438 =
                                b400.wrapping_add(3).cast::<u16>().read_unaligned() as u32;
                        }
                    }
                    if 0x1fc < u_stack_438 {
                        u_stack_438 = 0x1fc;
                    }
                    pb_out = outp();
                    st(pb_out.wrapping_add(0x10), u9);
                    copy(pb_out.wrapping_add(0x14), p13, u_stack_438);
                    st(pb_out, local_43c as u32);
                    st(pb_out.wrapping_add(4), u_stack_438.wrapping_add(4));
                    st(pb_out.wrapping_add(8), 0);
                    pb_out.wrapping_add(0xc).write(0);
                    pb_out.wrapping_add(0xf).write(1);
                    u7 = FUN_004d23b0(local_43c);
                    let mut sel: *mut u8 = core::ptr::null_mut();
                    if b17 {
                        if u_stack_42c == 0 {
                            pb_out.wrapping_add(0xd).write(1);
                            if u7 < ld(gp!(DAT_00b69a89)) && u7 < 0x28 {
                                sel = at(gp!(DAT_00b63f99), u7, 0x238);
                            }
                        } else {
                            pb_out.wrapping_add(0xd).write(2);
                            if u7 < ld(gp!(DAT_00b69a89)) && u7 < 0x28 {
                                sel = at(gp!(DAT_00b63fa9), u7, 0x238);
                            }
                        }
                    } else {
                        pb_out.wrapping_add(0xd).write(0);
                        if u7 < ld(gp!(DAT_00b69a89)) && u7 < 0x28 {
                            sel = at(gp!(DAT_00b63fa1), u7, 0x238);
                        }
                    }
                    if !sel.is_null() {
                        u7 = ld(sel).wrapping_add(1);
                        if 0x7f < u7 {
                            u7 = 0;
                        }
                        if u7 == u_stack_430 {
                            st(sel, u7);
                        }
                    }
                    pb_out.wrapping_add(0xe).write(u_stack_430 as u8);
                    advance_out();
                } else {
                    if i_stack_424 != 0 {
                        let mut i3: i32 = 0x7f;
                        if u_stack_430 != 0 {
                            i3 = u_stack_430.wrapping_sub(1) as i32;
                        }
                        i3 = FUN_004d1fa0(local_43c, i3 as u32, u7, u_stack_42c);
                        if i3 == 0 {
                            bump_bd(u6);
                            u6 = p11.wrapping_add(u_stack_438 as usize).read() as u32;
                            let pb12 = p11.wrapping_add(u_stack_438 as usize).wrapping_add(1);
                            if u6 != 0x39 {
                                u8_ = local_428.wrapping_sub(
                                    ((pb12 as usize).wrapping_sub(b400 as usize) as u32) & 0xffff,
                                );
                                if 0x1fc < u8_ {
                                    u8_ = 0x1fc;
                                }
                                pb_out = outp();
                                st(pb_out.wrapping_add(0x10), u6);
                                copy(pb_out.wrapping_add(0x14), pb12, u8_);
                                st(pb_out, local_43c as u32);
                                st(pb_out.wrapping_add(4), u8_.wrapping_add(4));
                                st(pb_out.wrapping_add(8), 0);
                                pb_out.wrapping_add(0xc).write(0);
                                pb_out.wrapping_add(0xf).write(0);
                                if u7 == 0 {
                                    if u_stack_42c == 0 {
                                        pb_out.wrapping_add(0xd).write(1);
                                    } else {
                                        pb_out.wrapping_add(0xd).write(2);
                                    }
                                } else {
                                    pb_out.wrapping_add(0xd).write(0);
                                }
                                if u_stack_430 == 0 {
                                    pb_out.wrapping_add(0xe).write(0x7f);
                                } else {
                                    pb_out
                                        .wrapping_add(0xe)
                                        .write((u_stack_430 as u8).wrapping_sub(1));
                                }
                                advance_out();
                            }
                        }
                    }
                    u_stack_434 = FUN_004d1fa0(local_43c, u_stack_430, u7, u_stack_42c) as u32;
                    if u_stack_434 == 0 {
                        let mut p13 = b400.wrapping_add(2);
                        if i_stack_424 == 0 {
                            u_stack_438 = local_428.wrapping_sub(2);
                        } else {
                            u_stack_438 = FUN_0046fad0_arg(0);
                            if u_stack_438 == 0 {
                                p13 = b400.wrapping_add(4);
                                u_stack_438 =
                                    b400.wrapping_add(2).cast::<u16>().read_unaligned() as u32;
                            }
                        }
                        if 0x1fc < u_stack_438 {
                            u_stack_438 = 0x1fc;
                        }
                        pb_out = outp();
                        st(pb_out.wrapping_add(0x10), u9);
                        copy(pb_out.wrapping_add(0x14), p13, u_stack_438);
                        st(pb_out, local_43c as u32);
                        st(pb_out.wrapping_add(4), u_stack_438.wrapping_add(4));
                        st(pb_out.wrapping_add(8), 0);
                        pb_out.wrapping_add(0xc).write(0);
                        pb_out.wrapping_add(0xf).write(0);
                        if u_stack_434 == 0 {
                            pb_out.wrapping_add(0xf).write(0);
                        } else {
                            pb_out.wrapping_add(0xf).write(1);
                        }
                        if u7 == 0 {
                            if u_stack_42c == 0 {
                                pb_out.wrapping_add(0xd).write(1);
                            } else {
                                pb_out.wrapping_add(0xd).write(2);
                            }
                        } else {
                            pb_out.wrapping_add(0xd).write(0);
                        }
                        pb_out.wrapping_add(0xe).write(u_stack_430 as u8);
                        advance_out();
                    }
                }
            }
        }
    }
}
