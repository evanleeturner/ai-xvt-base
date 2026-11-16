// SPDX-License-Identifier: MIT

unsafe extern "C" {
    static mut DAT_00aa6cf4: u32;
    static mut DAT_0052bf50: u32;
    static mut DAT_00aa62d0: i32;
    static mut DAT_00aa62d4: u32;
    static mut DAT_00aa62d8: i32;
    static mut DAT_00aa62dc: i32;
    static mut DAT_00aa62b0: i32;
    static mut DAT_00aa6ae8: u32;
    static mut DAT_00665440: u32;
    static mut DAT_00aa6118: *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d9280(a: i32, b: u32, c: i32, d: i32) -> i32;
    #[link_name = "qsort"]
    fn _qsort(
        base: *mut core::ffi::c_void,
        n: u32,
        size: u32,
        cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
    );
    fn FUN_004d95d0(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cd9d0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d9680() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed from their addresses and used as the original does.
    unsafe {
        if DAT_00aa6cf4 == 0 {
            return 0xffffffff;
        }
        let mut local_4: i32 = 0;
        if DAT_0052bf50 != 0xffffffff {
            let off = DAT_0052bf50.wrapping_mul(0x3e) as usize;
            let a = (&raw mut DAT_00aa62d0)
                .cast::<u8>()
                .wrapping_add(off)
                .cast::<i32>()
                .read_unaligned();
            let b = (&raw mut DAT_00aa62d4)
                .cast::<u8>()
                .wrapping_add(off)
                .cast::<u32>()
                .read_unaligned();
            let c = (&raw mut DAT_00aa62d8)
                .cast::<u8>()
                .wrapping_add(off)
                .cast::<i32>()
                .read_unaligned();
            let d = (&raw mut DAT_00aa62dc)
                .cast::<u8>()
                .wrapping_add(off)
                .cast::<i32>()
                .read_unaligned();
            local_4 = FUN_004d9280(a, b, c, d);
        }
        _qsort(
            (&raw mut DAT_00aa62b0).cast(),
            DAT_00aa6cf4,
            0x3e,
            FUN_004d95d0,
        );
        if DAT_0052bf50 != 0xffffffff {
            let mut u_var4: u32 = 0;
            if DAT_00aa6cf4 != 0 {
                let mut p = (&raw mut DAT_00aa62d0).cast::<u8>();
                loop {
                    let a = p.cast::<i32>().read_unaligned();
                    let b = p.byte_add(4).cast::<u32>().read_unaligned();
                    let c = p.byte_add(8).cast::<i32>().read_unaligned();
                    let d = p.byte_add(12).cast::<i32>().read_unaligned();
                    let i_var2 = FUN_004d9280(a, b, c, d);
                    let mut u_var1 = u_var4;
                    if i_var2 == local_4 {
                        DAT_0052bf50 = u_var1;
                        break;
                    }
                    p = p.wrapping_add(0x3e);
                    u_var4 = u_var4.wrapping_add(1);
                    u_var1 = DAT_0052bf50;
                    if !(u_var4 < DAT_00aa6cf4) {
                        DAT_0052bf50 = u_var1;
                        break;
                    }
                }
                if u_var4 != DAT_00aa6cf4 {
                    return DAT_0052bf50;
                }
            }
            DAT_0052bf50 = 0xffffffff;
            DAT_00aa6ae8 = 0xffffffff;
            let mut p5 = &raw mut DAT_00665440;
            let mut n: i32 = 8;
            while n != 0 {
                p5.write_unaligned(0);
                p5 = p5.wrapping_add(1);
                n -= 1;
            }
            let mut p5 = DAT_00aa6118;
            let mut n: i32 = 0x400;
            while n != 0 {
                p5.write_unaligned(0);
                p5 = p5.wrapping_add(1);
                n -= 1;
            }
            FUN_004cd9d0();
        }
        DAT_0052bf50
    }
}
