// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c64: i32;
    static mut DAT_00528c68: i32;
    static mut DAT_00664550: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b0b90(param_1: *mut u16, param_2: u32) {
    // SAFETY: the globals are the game's own; the two format tables they point
    // at and the source/destination buffers are read and written exactly where
    // the original does.
    unsafe {
        let i_var2 = DAT_00528c68;
        let i_var1 = DAT_00528c64;
        let mut src: *mut u16 = param_1;
        if i_var2 == i_var1 {
            let mut dst: *mut u8 = (&raw mut DAT_00664550).cast::<u8>();
            let mut n = (param_2 & 0x7fff_ffff) >> 1;
            while n != 0 {
                dst.cast::<u32>()
                    .write_unaligned(src.cast::<u32>().read_unaligned());
                src = src.add(2);
                dst = dst.add(4);
                n -= 1;
            }
            let mut m = param_2.wrapping_mul(2) & 3;
            while m != 0 {
                dst.write(src.read_unaligned() as u8);
                src = src.cast::<u8>().add(1).cast::<u16>();
                dst = dst.add(1);
                m -= 1;
            }
            return;
        }
        if 0 < (param_2 as i32) {
            let b1 = i_var1 as usize as *const u8;
            let b2 = i_var2 as usize as *const u8;
            let base: *mut u16 = &raw mut DAT_00664550;
            let mut p3: *mut u16 = base;
            let mut local_4 = param_2;
            loop {
                let x = src.read_unaligned() as u32;
                let v = (((x >> (b1.add(0x14).read() as u32 & 0x1f))
                    << (b1.add(0x20).read() as u32 & 0x1f)) as u8) as u32;
                let mut u5: u16 = ((((v >> (b2.add(0x20).read() as u32 & 0x1f)) as u16) as u32)
                    << (b2.add(0x14).read() as u32 & 0x1f))
                    as u16;
                p3.write_unaligned(u5);
                let x = src.read_unaligned() as u32;
                let v = (((x >> (b1.add(0x18).read() as u32 & 0x1f))
                    << (b1.add(0x24).read() as u32 & 0x1f)) as u8) as u32;
                u5 = (((((v >> (b2.add(0x24).read() as u32 & 0x1f)) as u16) as u32)
                    << (b2.add(0x18).read() as u32 & 0x1f)) as u16)
                    | u5;
                p3.write_unaligned(u5);
                let x = src.read_unaligned() as u32;
                let v = (((x >> (b1.add(0x1c).read() as u32 & 0x1f))
                    << (b1.add(0x28).read() as u32 & 0x1f)) as u8) as u32;
                u5 = (((((v >> (b2.add(0x28).read() as u32 & 0x1f)) as u16) as u32)
                    << (b2.add(0x1c).read() as u32 & 0x1f)) as u16)
                    | u5;
                p3.write_unaligned(u5);
                if p3 != base {
                    let a = ((0xffu32 >> (b2.add(0x34).read() as u32 & 0x1f)) as u16) as u32;
                    let w = ((a << (b2.add(0x30).read() as u32 & 0x1f)) as u16) | u5;
                    p3.write_unaligned(w);
                }
                src = src.add(1);
                p3 = p3.add(1);
                local_4 = local_4.wrapping_sub(1);
                if local_4 == 0 {
                    break;
                }
            }
        }
    }
}
