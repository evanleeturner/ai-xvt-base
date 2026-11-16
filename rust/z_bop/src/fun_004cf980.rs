// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebc5: *mut i32;
    static mut DAT_00acf145: i32;
    static mut DAT_00b53571: u32;
    static mut DAT_00b53579: u32;
    static mut DAT_00acf571: [u8; 0];
    static mut DAT_00acf575: [u8; 0];
    static mut DAT_00acf579: [u8; 0];
    static mut DAT_00acf57d: [u8; 0];
    static mut DAT_00acf57e: [u8; 0];
    static mut DAT_00acf57f: [u8; 0];
    static mut DAT_00acf580: [u8; 0];
    static mut DAT_00acf581: [u8; 0];
    static mut s__RSB__u__0052b984: [i8; 0];
    static mut s__RSG__u__0052b978: [i8; 0];
    static mut s__RSS__u__0052b96c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_0046fad0(param_1: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf980(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: *mut u32,
    param_5: u32,
) -> u8 {
    let mut local_504: i32 = 0;
    let mut local_500 = [0i8; 256];
    let mut local_400 = [0u8; 0x400];
    // SAFETY: globals and callees are the game's own memory; the buffers and
    // pointers are used exactly as the original does.
    unsafe {
        let p_glob_this: *mut i32 = (&raw const DAT_00acebc5).read_unaligned();
        if p_glob_this.is_null() {
            return 1;
        }
        let u_var3 = param_4.read_unaligned();
        let b_var1: u8 = if u_var3 < 0x3c || 0x3f < u_var3 { 1 } else { 0 };
        let format: *const i8 = if param_2 == 0 {
            (&raw const s__RSB__u__0052b984).cast::<i8>()
        } else if param_2 == 2 {
            (&raw const s__RSG__u__0052b978).cast::<i8>()
        } else {
            (&raw const s__RSS__u__0052b96c).cast::<i8>()
        };
        sprintf(local_500.as_mut_ptr(), format, param_3);
        let base = local_400.as_mut_ptr();
        let mut puvar4 = base.add(3);
        let mut ivar5: i32 = 3;
        let us: u16 = (((u_var3 as u8) & 0x7f) as u16) | (((param_3 as u16) & 0x7f) << 8) | 0x80;
        base.cast::<u16>().write_unaligned(us);
        *base.add(2) = param_2 as u8;
        if b_var1 != 0 && FUN_0046fad0(u_var3) == 0 {
            let us2 = (param_5 as u16).wrapping_sub(4);
            base.add(3).cast::<u16>().write_unaligned(us2);
            puvar4 = base.add(5);
            ivar5 = 5;
        }
        let mut puvar7 = puvar4;
        let mut puvar6 = param_4.add(1).cast::<u8>();
        let mut n = param_5.wrapping_sub(4) >> 2;
        while n != 0 {
            puvar7
                .cast::<u32>()
                .write_unaligned(puvar6.cast::<u32>().read_unaligned());
            puvar6 = puvar6.add(4);
            puvar7 = puvar7.add(4);
            n -= 1;
        }
        ivar5 = param_5.wrapping_sub(4).wrapping_add(ivar5 as u32) as i32;
        let mut n = param_5.wrapping_sub(4) & 3;
        while n != 0 {
            *puvar7 = *puvar6;
            puvar6 = puvar6.add(1);
            puvar7 = puvar7.add(1);
            n -= 1;
        }
        if b_var1 != 0 {
            ivar5 = ivar5.wrapping_add(1);
            *puvar4.add(param_5.wrapping_sub(4) as usize) = 0x39;
        }
        let cur = (&raw const DAT_00acf145).read_unaligned();
        if cur == param_1 {
            let a = (&raw const DAT_00b53579).read_unaligned();
            if a < 0x400 {
                let mut src = param_4.cast::<u8>();
                let idx = (&raw const DAT_00b53571).read_unaligned();
                let off = idx.wrapping_mul(0x210) as usize;
                let mut dst = (&raw mut DAT_00acf581).cast::<u8>().add(off);
                let mut n = param_5 >> 2;
                while n != 0 {
                    dst.cast::<u32>()
                        .write_unaligned(src.cast::<u32>().read_unaligned());
                    src = src.add(4);
                    dst = dst.add(4);
                    n -= 1;
                }
                let mut n = param_5 & 3;
                while n != 0 {
                    *dst = *src;
                    src = src.add(1);
                    dst = dst.add(1);
                    n -= 1;
                }
                let idx = (&raw const DAT_00b53571).read_unaligned();
                let off = idx.wrapping_mul(0x210) as usize;
                let cur = (&raw const DAT_00acf145).read_unaligned();
                (&raw mut DAT_00acf571)
                    .cast::<u8>()
                    .add(off)
                    .cast::<i32>()
                    .write_unaligned(cur);
                (&raw mut DAT_00acf575)
                    .cast::<u8>()
                    .add(off)
                    .cast::<u32>()
                    .write_unaligned(param_5);
                *(&raw mut DAT_00acf57e).cast::<u8>().add(off) = param_2 as u8;
                *(&raw mut DAT_00acf57f).cast::<u8>().add(off) = param_3 as u8;
                *(&raw mut DAT_00acf580).cast::<u8>().add(off) = 1;
                (&raw mut DAT_00acf579)
                    .cast::<u8>()
                    .add(off)
                    .cast::<u32>()
                    .write_unaligned(0);
                *(&raw mut DAT_00acf57d).cast::<u8>().add(off) = 0;
                let b = (&raw const DAT_00b53579).read_unaligned();
                (&raw mut DAT_00b53579).write_unaligned(b.wrapping_add(1));
                let a2 = (&raw const DAT_00b53571).read_unaligned().wrapping_add(1);
                (&raw mut DAT_00b53571).write_unaligned(a2);
                if 0x3ff < a2 {
                    (&raw mut DAT_00b53571).write_unaligned(0);
                }
            }
        } else {
            let p_this: *mut i32 = (&raw const DAT_00acebc5).read_unaligned();
            let cur = (&raw const DAT_00acf145).read_unaligned();
            let vtbl = p_this.read_unaligned() as u32;
            let f = ((vtbl.wrapping_add(0x68)) as *const *const core::ffi::c_void).read_unaligned();
            let func: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, *mut u8, i32) -> i32 =
                core::mem::transmute(f);
            local_504 = func(p_this, cur, param_1, 0, base, ivar5);
        }
        (local_504 == 0) as u8
    }
}
