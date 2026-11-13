// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_007092d9: i8;
    static mut DAT_0071b73c: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070ad48: u8;
    static mut DAT_0072f508: i8;
    static mut DAT_006d4a02: i8;
    static mut DAT_006d4870: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
    fn FUN_004ac210(param_1: u16, param_2: u8);
    fn FUN_004a3490(param_1: u16, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00443db0(param_1: u16, param_2: *mut u32, param_3: u32) {
    // SAFETY: all globals and callees are the game's own; the pointers read
    // are the game's records and string buffers, accessed where the original does.
    unsafe {
        let i_var2 = (param_3.wrapping_mul(0x23)).wrapping_add(DAT_006d4988 as u32);
        let i_var1 = ((i_var2.wrapping_add(0x1b)) as usize as *const i32).read_unaligned();

        let mut src: *const u8;
        let mut use_default: Option<u8> = None;
        let mut fetch = false;

        let mut cond = false;
        let mut u_var3: u32 = 0;
        if DAT_007092d9 == 0 {
            let base = (&raw mut DAT_0071b73c) as usize as u32;
            let addr = base.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd));
            u_var3 = (addr as usize as *const u16).read_unaligned() as u32;
            let p = ((i_var2.wrapping_add(0x1f)) as usize as *const u32).read_unaligned();
            let q = (p.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
            let a = u_var3.wrapping_add(0x1f0).wrapping_add(q);
            if (a as usize as *const i8).read() == 0 {
                cond = true;
            }
        }

        if cond {
            let idx = (param_3 & 0xffff)
                .wrapping_mul(0x23)
                .wrapping_add(0x16)
                .wrapping_add(DAT_006d4988 as u32);
            let b = (idx as usize as *const u8).read() as usize;
            let owner = (&raw mut DAT_0070ad48).byte_add(b * 0x566).read() as u32;
            let b9: bool = if u_var3 == owner {
                false
            } else {
                let off = owner.wrapping_add(u_var3.wrapping_mul(0x1e5)) as usize;
                (&raw mut DAT_0072f508).byte_add(off).read() == 0
            };
            if !b9 {
                fetch = true;
            } else if DAT_006d4a02 != 2 {
                use_default = Some(3);
            } else {
                use_default = Some(1);
            }
        } else {
            fetch = true;
        }

        if fetch {
            if i_var1 != -1 {
                src = FUN_00432cc0(i_var1) as *const u8;
            } else {
                use_default = Some(3);
                src = core::ptr::null();
            }
        } else {
            src = core::ptr::null();
        }

        if let Some(b10) = use_default {
            src = (&raw mut DAT_006d4870) as *const u8;
            FUN_004ac210(param_3 as u16, b10);
        }

        let mut len: u32 = 0;
        while src.byte_add(len as usize).read() != 0 {
            len = len.wrapping_add(1);
        }
        len = len.wrapping_add(1);

        let mut s = src;
        let mut d = param_2 as *mut u8;
        let mut n = len >> 2;
        while n != 0 {
            (d as *mut u32).write_unaligned((s as *const u32).read_unaligned());
            s = s.byte_add(4);
            d = d.byte_add(4);
            n -= 1;
        }
        let mut r = len & 3;
        while r != 0 {
            d.write(s.read());
            s = s.byte_add(1);
            d = d.byte_add(1);
            r -= 1;
        }

        FUN_004a3490(param_1, param_2);
    }
}
