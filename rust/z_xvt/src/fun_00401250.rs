// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00732940: *mut u8;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_007092c4: i32;
    static mut DAT_007092d0: i16;
    static mut DAT_0070b2aa: u8;
    static mut DAT_006db700: i32;
    static mut DAT_006db6e0: i32;
    static mut DAT_006db70c: i32;
    static mut DAT_00703c68: i32;
    static mut DAT_00703be0: i32;
    static mut DAT_00703adc: i32;
    static mut DAT_006db7a0: i32;
    static mut DAT_006db79c: i32;
    static mut DAT_006db7a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483410(a: i16, b: i16, c: *mut u8);
    fn FUN_00483540(a: i16, b: i32, c: *mut u8);
    fn FUN_00449550(a: u32) -> i64;
    fn FUN_004495f0(a: u32) -> i64;
    fn FUN_004494c0(a: u32) -> i64;
    fn FUN_004494f0(a: u32) -> i64;
    fn FUN_00449630(a: u32) -> i64;
    fn FUN_00413ca0(a: i32, b: i32) -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00401250() {
    let mut au70: [u16; 14] = [0; 14];
    let mut au54: [u16; 14] = [0; 14];
    let mut au38: [u16; 14] = [0; 14];
    let mut au1c: [u16; 14] = [0; 14];
    let mut local_88: i16 = 0;
    let mut local_86: i16 = 0;
    let mut local_84: i16 = 0;
    let mut local_7c: i32 = 0;
    let mut local_78: i32 = 0;
    let mut local_74: i32 = 0;

    // SAFETY: all globals and callees are the game's own memory; the
    // pointers are the game's tables, written where the original writes.
    unsafe {
        DAT_007092c4 = 0;
        DAT_007092d0 = 0;
        au70[1] = 0x62;
        au38[1] = 0;
        au54[1] = 0;
        au38[2] = 0;
        au54[2] = 0;
        au1c[3] = 0;
        au38[3] = 0;
        au54[3] = 0;
        au1c[1] = 0x4000;
        au1c[4] = 0;
        au70[2] = 99;
        au54[4] = 0;
        au1c[2] = 0x4000;
        au70[3] = 0x62;
        au70[4] = 99;
        au54[5] = 0;
        au38[4] = 0x4000;
        au70[5] = 0x62;
        au1c[5] = 0x4000;
        au38[5] = 0xc000;
        au70[6] = 99;
        au38[7] = 0;
        au54[7] = 0;
        au1c[6] = 0x4000;
        au38[6] = 0xc000;
        au54[6] = 0x8000;
        au70[7] = 0x62;
        au54[8] = 0;
        au1c[7] = 0x8000;
        au70[8] = 99;
        au1c[8] = 0x8000;
        au54[9] = 0;
        au38[8] = 0x8000;
        au70[9] = 0x62;
        au1c[9] = 0x4000;
        au38[9] = 0x8000;
        au70[10] = 99;
        au1c[10] = 0x4000;
        au38[10] = 0x8000;
        au54[0xb] = 0;
        au54[10] = 0x4000;
        au70[0xb] = 0x62;
        au1c[0xb] = 0x4000;
        au38[0xb] = 0x4000;
        au70[0xc] = 99;
        au1c[0xc] = 0x4000;
        let mut local_82: u16 = 1;
        au38[0xc] = 0x4000;
        au54[0xc] = 0x4000;
        loop {
            let u5 = local_82 as u32;
            let u1 = au70[u5 as usize];
            let i4 = u5.wrapping_mul(0x23);
            let ent = DAT_006d4988.wrapping_add(i4 as usize);
            ent.add(3).write(u1 as u8);
            ent.cast::<u16>().write_unaligned(1);
            let p = ent.add(0x1f).cast::<*mut u8>().read_unaligned();
            p.write(6);
            ent.add(2).write(0xe);
            let s7: [u16; 7] = [0, 0, 0, 0x7fff, 0, 0, 0];
            let mut k = 0usize;
            while k < 7 {
                p.add(0x77 + k * 2).cast::<u16>().write_unaligned(s7[k]);
                k += 1;
            }
            p.add(0x85).write(0);
            p.add(0x86).write(0);
            p.add(0x90).write(1);
            p.add(0x89).write(p.add(0x90).read());
            ent.add(0x16).write(1);
            DAT_0070b2aa = 5;
            DAT_00732e54 = DAT_00732940.wrapping_add(u5.wrapping_mul(0x436) as usize);
            let q = DAT_00732e54;
            p.add(0xa7).cast::<*mut u8>().write_unaligned(q);
            let mut u6: u16 = 0;
            loop {
                let u7 = u6.wrapping_add(1);
                ent.add(u6 as usize + 0x19).write(0);
                u6 = u7;
                if !(u7 < 2) {
                    break;
                }
            }
            u6 = 0;
            loop {
                let u7 = u6.wrapping_add(1);
                q.add(0x2ca + u6 as usize).write(0);
                q.add(0x2f2 + u6 as usize).write(0);
                q.add(0x31a + u6 as usize).write(0xff);
                u6 = u7;
                if !(u7 < 0x28) {
                    break;
                }
            }
            q.add(0xc3).cast::<u16>().write_unaligned(0);
            q.add(0xc5).cast::<u16>().write_unaligned(0x7fff);
            q.add(0xc7).cast::<u16>().write_unaligned(0x7fff);
            q.add(0xcb).cast::<u16>().write_unaligned(0);
            q.add(0x1e3).write(0);
            q.add(0x1e6).write(0);
            q.add(0x1e4).write(0);
            q.add(0x1e5).write(0);
            q.add(0x29).write(0);
            u6 = 0;
            loop {
                let u7 = u6.wrapping_add(1);
                q.add(0x15 + (u6 as usize) * 4)
                    .cast::<u32>()
                    .write_unaligned(0);
                u6 = u7;
                if !(u7 < 5) {
                    break;
                }
            }
            q.add(0x1dd).cast::<u16>().write_unaligned(1);
            q.add(0x1df).cast::<u16>().write_unaligned(1);
            q.add(0x210).write(0);
            q.add(0x20b).cast::<u16>().write_unaligned(0x7fff);
            q.add(0x20d).cast::<u16>().write_unaligned(0);
            ent.add(0x10)
                .cast::<u16>()
                .write_unaligned(au38[u5 as usize]);
            ent.add(0x12)
                .cast::<u16>()
                .write_unaligned(au1c[u5 as usize]);
            ent.add(0x14)
                .cast::<u16>()
                .write_unaligned(au54[u5 as usize]);
            let rd0 = ent.add(0x10).cast::<u16>().read_unaligned();
            let rd1 = ent.add(0x12).cast::<u16>().read_unaligned();
            FUN_00483410(rd1 as i16, rd0 as i16, ent);
            let rd14 = DAT_006d4988
                .wrapping_add(i4 as usize)
                .add(0x14)
                .cast::<u16>()
                .read_unaligned();
            FUN_00483540(rd14 as i16, 0, DAT_006d4988.wrapping_add(i4 as usize));
            let l13 = FUN_00449550(u1 as u32);
            if u1 == 0x62 {
                local_88 = 0;
                let l14 = FUN_004495f0(0x62);
                local_86 = l14 as i16;
                local_84 = 0;
            } else if u1 == 99 {
                local_88 = 0;
                let l14 = FUN_004495f0(99);
                let l15 = FUN_004494c0(99);
                local_86 = (l15 as i16).wrapping_add(l14 as i16);
                let l14 = FUN_004494f0(99);
                let l15 = FUN_00449630(99);
                local_84 = (l15 as i16).wrapping_add(l14 as i16);
            }
            let v16 = FUN_00413ca0(0, DAT_006db700) as u32;
            let v17 = FUN_00413ca0(0, DAT_006db6e0) as u32;
            let i8a = (0i16.wrapping_sub(l13 as i16)) as i32;
            let v18 = FUN_00413ca0(0, DAT_006db70c) as u32;
            let v19 = FUN_00413ca0(i8a, DAT_00703c68) as u32;
            let v20 = FUN_00413ca0(i8a, DAT_00703be0) as u32;
            let v21 = FUN_00413ca0(i8a, DAT_00703adc) as u32;
            let mut i8 = v18.wrapping_add(v21);
            let v18 = FUN_00413ca0(0, DAT_006db7a0) as u32;
            let v21 = FUN_00413ca0(0, DAT_006db79c) as u32;
            let v22 = FUN_00413ca0(0, DAT_006db7a8) as u32;
            let m = 0xfffffffeu32;
            let i3_0 = (local_7c as u32)
                .wrapping_add(v16.wrapping_add(v19).wrapping_add(v18).wrapping_mul(m));
            let i3_1 = (local_78 as u32)
                .wrapping_add(v17.wrapping_add(v20).wrapping_add(v21).wrapping_mul(m));
            let i12 = local_88 as i32;
            i8 = (local_74 as u32).wrapping_add(i8.wrapping_add(v22).wrapping_mul(m));
            let i3_2 = i8;
            let b = DAT_006d4988.wrapping_add(i4 as usize).add(4);
            b.cast::<u32>().write_unaligned(i3_0);
            b.add(4).cast::<u32>().write_unaligned(i3_1);
            b.add(8).cast::<u32>().write_unaligned(i3_2);
            let v16 = FUN_00413ca0(i12, DAT_006db700) as u32;
            let v17 = FUN_00413ca0(i12, DAT_006db6e0) as u32;
            let v18 = FUN_00413ca0(i12, DAT_006db70c) as u32;
            let i8b = local_86 as i32;
            let v19 = FUN_00413ca0(i8b, DAT_00703c68) as u32;
            let v20 = FUN_00413ca0(i8b, DAT_00703be0) as u32;
            let v21 = FUN_00413ca0(i8b, DAT_00703adc) as u32;
            let i12b = local_84 as i32;
            let i8c = v18.wrapping_add(v21);
            let v18 = FUN_00413ca0(i12b, DAT_006db7a0) as u32;
            let v21 = FUN_00413ca0(i12b, DAT_006db79c) as u32;
            let v22 = FUN_00413ca0(i12b, DAT_006db7a8) as u32;
            local_7c = (local_7c as u32)
                .wrapping_add(v16.wrapping_add(v19).wrapping_add(v18).wrapping_mul(2))
                as i32;
            local_78 = (local_78 as u32)
                .wrapping_add(v17.wrapping_add(v20).wrapping_add(v21).wrapping_mul(2))
                as i32;
            local_74 = (local_74 as u32).wrapping_add(i8c.wrapping_add(v22).wrapping_mul(2)) as i32;
            let p2 = DAT_006d4988
                .wrapping_add(i4 as usize)
                .add(0x1f)
                .cast::<*mut u8>()
                .read_unaligned();
            p2.add(6).cast::<u32>().write_unaligned(i3_0);
            p2.add(10).cast::<u32>().write_unaligned(i3_1);
            p2.add(14).cast::<u32>().write_unaligned(i3_2);
            local_82 = local_82.wrapping_add(1);
            if !(local_82 < 0xd) {
                break;
            }
        }
    }
}
