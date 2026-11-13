// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_0052a560: u32;
    static mut DAT_0052a55c: i8;
    static mut DAT_00625738: [u32; 0x8000];
    static mut DAT_00605738: [u32; 0x8000];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(param_1: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f3e0(param_1: u32, param_2: u32, param_3: i32) -> i32 {
    let _ = param_2;
    // SAFETY: the globals and _malloc are the game's own; the table indices
    // stay below 0x8000, and the fill writes stay within the param_1 bytes
    // returned by _malloc.
    unsafe {
        let a = (&raw mut DAT_00625738).cast::<u32>();
        let b = (&raw mut DAT_00605738).cast::<u32>();
        let mut u_var4: u16 = 0;
        _DAT_0052a560 = _DAT_0052a560.wrapping_add(1);
        if DAT_0052a55c == 0 {
            loop {
                let u_var3 = u_var4.wrapping_add(1);
                a.add(u_var4 as usize).write(0);
                b.add(u_var4 as usize).write(0);
                u_var4 = u_var3;
                if u_var3 >= 0x8000 {
                    break;
                }
            }
            DAT_0052a55c = 1;
        }
        let mut u_var5: u32 = 0;
        loop {
            if a.add(u_var5 as usize).read() == 0 {
                break;
            }
            u_var4 = (u_var5 as u16).wrapping_add(1);
            u_var5 = u_var4 as u32;
            if u_var4 >= 0x8000 {
                break;
            }
        }
        if (u_var5 as u16 as i16) == -0x8000 {
            return 0;
        }
        let mut p = _malloc(param_1).cast::<u32>();
        a.add(u_var5 as usize).write(p as usize as u32);
        if !p.is_null() {
            if param_3 != 0 {
                let mut n = param_1 >> 2;
                while n != 0 {
                    p.write_unaligned(0);
                    p = p.add(1);
                    n -= 1;
                }
                let mut n = param_1 & 3;
                while n != 0 {
                    p.cast::<u8>().write(0);
                    p = p.byte_add(1);
                    n -= 1;
                }
            }
            b.add(u_var5 as usize).write(param_1);
            return u_var5.wrapping_add(1) as i32;
        }
        0
    }
}
