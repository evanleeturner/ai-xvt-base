// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556928: i32;
    static mut DAT_00556930: i32;
    static mut DAT_00556934: u8;
    static mut DAT_00556935: u8;
    static mut DAT_00556936: u8;
    static mut DAT_0055693d: [u8; 4];
    static mut DAT_00556941: [u8; 4];
    static mut DAT_00556945: [u8; 4];
    static mut DAT_00556965: [u8; 2];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn free(p: *mut core::ffi::c_void);
    fn malloc(n: u32) -> *mut core::ffi::c_void;
    fn exit(code: i32);
    fn FUN_004438b0(a: *const i8, b: *const i8);
    fn FUN_004445e0(a: i32);
    fn FUN_004446c0(a: i32) -> i32;
    fn FUN_004444f0(a: i32);
    fn FUN_00443890(a: *const i8);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_00443ef0(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let p = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the game's image structure; all offsets and
    // globals are the game's own memory, accessed where the original does.
    unsafe {
        let old = p
            .byte_add(0x104c)
            .cast::<*mut core::ffi::c_void>()
            .read_unaligned();
        if !old.is_null() {
            free(old);
        }
        let pv = malloc((DAT_00556930 as u32).wrapping_mul(9));
        p.byte_add(0x104c)
            .cast::<*mut core::ffi::c_void>()
            .write_unaligned(pv);
        if pv.is_null() {
            FUN_004438b0(
                b"Unable to quantize image\0".as_ptr() as *const i8,
                b"Memory allocation failed\0".as_ptr() as *const i8,
            );
            exit(1);
        }
        let t = p
            .byte_add(0x1082)
            .cast::<*mut core::ffi::c_void>()
            .read_unaligned();
        if !t.is_null() {
            let t2 = p
                .byte_add(0x1082)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned();
            free(t2);
        }
        p.byte_add(0x1082).cast::<u32>().write_unaligned(0);
        let tp = p.byte_add(0x104c).cast::<u32>().read_unaligned();
        (&raw mut DAT_0055693d).cast::<u32>().write_unaligned(tp);
        DAT_00556930 = 0;
        FUN_004445e0(DAT_00556928);

        if param_2 == 2 && param_4 == 2 {
            let b = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            let a = ((b.add(9).read() as u32)
                .wrapping_mul(0x4d)
                .wrapping_add((b.add(0xb).read() as u32).wrapping_mul(0x1d))
                .wrapping_add((b.add(10).read() as u32).wrapping_mul(0x96)))
                & 0xffffff00;
            let c = ((b.add(2).read() as u32)
                .wrapping_mul(0x1d)
                .wrapping_add((b.add(1).read() as u32).wrapping_mul(0x96))
                .wrapping_add((b.read() as u32).wrapping_mul(0x4d)))
                & 0xffffff00;
            let u4 = (a < c) as u32;
            let i3 = u4.wrapping_mul(9) as usize;
            b.add(i3).write(0);
            let base = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            base.add(1 + i3).write(0);
            let i1 = ((u4 == 0) as u32).wrapping_mul(9) as usize;
            let base = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            base.add(2 + i3).write(0);
            let base = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            base.add(i1).write(0xff);
            let base = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            base.add(1 + i1).write(0xff);
            let base = p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *mut u8;
            base.add(2 + i1).write(0xff);
        }
        if param_4 != 3 {
            p.byte_add(0x1028).cast::<i32>().write_unaligned(0);
            p.byte_add(0x1024).cast::<i32>().write_unaligned(2);
        }
        p.byte_add(0x1054)
            .cast::<i32>()
            .write_unaligned(DAT_00556930);
        let mut b8 = false;
        if param_3 != 0 {
            let r = FUN_004446c0(param_1);
            b8 = r == 0;
        }
        let mut pb = p.byte_add(0x1086).cast::<u32>().read_unaligned() as usize as *mut u8;
        let mut count = p.byte_add(0x108e).cast::<u32>().read_unaligned();
        if !b8 && count != 0 {
            let mut local_4: u32 = 0;
            loop {
                let mut s6: i16 = 7;
                let mut i3: i32 = DAT_00556928;
                loop {
                    let b5 = s6 as u8;
                    let sh = (b5 & 0x1f) as u32;
                    let u4 = ((((pb.read() as u32) >> sh) & 1).wrapping_mul(2)
                        | (((pb.add(1).read() as u32) >> sh) & 1))
                        .wrapping_mul(2)
                        | (((pb.add(2).read() as u32) >> sh) & 1);
                    let flag = (i3 as usize as *const u8).add(2).read() as u32;
                    if (1u32.wrapping_shl(u4 as i8 as u32) & flag) == 0 {
                        break;
                    }
                    s6 = s6.wrapping_sub(1);
                    let addr = (i3 as u32)
                        .wrapping_add(0x32)
                        .wrapping_add(u4.wrapping_mul(4));
                    let next = (addr as usize as *const i32).read_unaligned();
                    i3 = next;
                    if s6 == 0 {
                        break;
                    }
                }
                DAT_00556934 = pb.read();
                DAT_00556935 = pb.add(1).read();
                DAT_00556936 = pb.add(2).read();
                (&raw mut DAT_00556941).cast::<u32>().write_unaligned(0);
                (&raw mut DAT_00556945)
                    .cast::<u32>()
                    .write_unaligned(0x41080000);
                let node = ((i3 as u32).wrapping_add(0x2e) as usize as *const i32).read_unaligned();
                FUN_004444f0(node);
                if p.byte_add(0x1024).cast::<i32>().read_unaligned() == 2 {
                    let us = (&raw mut DAT_00556965).cast::<u16>().read_unaligned();
                    pb.add(4).cast::<u16>().write_unaligned(us);
                } else {
                    let us = (&raw mut DAT_00556965).cast::<u16>().read_unaligned();
                    let i3 = (us as u32).wrapping_mul(9) as usize;
                    let base =
                        p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *const u8;
                    pb.write(base.add(i3).read());
                    let base =
                        p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *const u8;
                    pb.add(1).write(base.add(1).add(i3).read());
                    let base =
                        p.byte_add(0x104c).cast::<u32>().read_unaligned() as usize as *const u8;
                    pb.add(2).write(base.add(2).add(i3).read());
                }
                pb = pb.add(6);
                count = p.byte_add(0x108e).cast::<u32>().read_unaligned();
                let mut cond = count.wrapping_sub(local_4) == 1;
                if !cond {
                    let d = p.byte_add(0x1034).cast::<u32>().read_unaligned();
                    let rem: u32;
                    core::arch::asm!(
                        "div {d:e}",
                        d = in(reg) d,
                        inout("eax") local_4 => _,
                        inout("edx") 0u32 => rem,
                        options(nostack)
                    );
                    cond = rem == 0;
                }
                if cond {
                    FUN_00443890(b"  Assigning image colors...  \0".as_ptr() as *const i8);
                }
                local_4 = local_4.wrapping_add(1);
                count = p.byte_add(0x108e).cast::<u32>().read_unaligned();
                if !(local_4 < count) {
                    break;
                }
            }
        }
    }
}
