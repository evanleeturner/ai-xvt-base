// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071bb44: i32;
    static mut DAT_0071bb4a: i32;
    static mut DAT_0071bb4c: i32;
    static mut DAT_0071bb2f: i32;
    static mut DAT_0071bbc4: i32;
    static mut DAT_0071bb2b: i32;
    static mut DAT_0071bc3c: i32;
    static mut DAT_0071bb2d: i32;
    static mut DAT_0071bb40: i32;
    static mut DAT_0071bb27: i32;
    static mut DAT_0071b730: i32;
    static mut DAT_0071bb3c: i32;
    static mut DAT_0071bb35: i32;
    static mut DAT_0071bb3e: i32;
    static mut DAT_0071bb3a: i32;
    static mut DAT_0071bb33: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040cbf0(param_1: u32, param_2: i32);
}

unsafe fn at(base: *mut i32, off: i32) -> *mut u8 {
    (base as *mut u8).wrapping_offset(off as isize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00441830(param_1: i32) {
    let i_var2: i32 = param_1.wrapping_mul(0x5bd);
    // SAFETY: all globals are the game's own memory, accessed at the byte
    // offsets the original uses; the callee is the game's function.
    unsafe {
        let v44 = at(&raw mut DAT_0071bb44, i_var2)
            .cast::<u16>()
            .read_unaligned();
        if v44 != 0 {
            let v4a = at(&raw mut DAT_0071bb4a, i_var2)
                .cast::<u16>()
                .read_unaligned();
            let u_var5: u32 = if v4a == 0 { 0x12 } else { 0x14 };
            FUN_0040cbf0(u_var5, param_1);

            let mut u_var3: u16 = 0;
            loop {
                let u_var4: u16 = u_var3.wrapping_add(1);
                let off2 = i_var2.wrapping_add((u_var3 as i32).wrapping_mul(2));

                let a = at(&raw mut DAT_0071bb2f, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(&raw mut DAT_0071bb4c, off2)
                    .cast::<u16>()
                    .write_unaligned(a);

                let b = at(&raw mut DAT_0071bb2b, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(&raw mut DAT_0071bbc4, off2)
                    .cast::<u16>()
                    .write_unaligned(b);

                let c = at(&raw mut DAT_0071bb2d, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(&raw mut DAT_0071bc3c, off2)
                    .cast::<u16>()
                    .write_unaligned(c);

                u_var3 = u_var4;
                if u_var4 >= 0x3c {
                    break;
                }
            }
            return;
        }

        at(&raw mut DAT_0071bb40, i_var2)
            .cast::<u16>()
            .write_unaligned(0);

        let v27 = at(&raw mut DAT_0071bb27, i_var2)
            .cast::<u16>()
            .read_unaligned();
        let v730 = at(&raw mut DAT_0071b730, i_var2)
            .cast::<u32>()
            .read_unaligned();

        if (v27 as u32) == v730 {
            let v3c = at(&raw mut DAT_0071bb3c, i_var2)
                .cast::<u16>()
                .read_unaligned();
            at(&raw mut DAT_0071bb33, i_var2)
                .cast::<u16>()
                .write_unaligned(v3c);

            let v3e = at(&raw mut DAT_0071bb3e, i_var2)
                .cast::<u16>()
                .read_unaligned();
            at(&raw mut DAT_0071bb35, i_var2)
                .cast::<u16>()
                .write_unaligned(v3e);

            let v3a = at(&raw mut DAT_0071bb3a, i_var2).read();
            FUN_0040cbf0(v3a as u32, param_1);
            return;
        }

        at(&raw mut DAT_0071bb33, i_var2)
            .cast::<u16>()
            .write_unaligned(0);
        at(&raw mut DAT_0071bb35, i_var2)
            .cast::<u16>()
            .write_unaligned(0);

        FUN_0040cbf0(0x12, param_1);

        at(&raw mut DAT_0071bb44, i_var2)
            .cast::<u16>()
            .write_unaligned(1);
    }
}
