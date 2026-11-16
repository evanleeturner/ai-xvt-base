// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut _DAT_00732a3c: i32;
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_0052687a: [u8; 1];

    static mut DAT_006551ca: u16;
    static mut DAT_00655812: u16;
    static mut DAT_00655154: u16;
    static mut DAT_00655a8e: u16;
    static mut DAT_00655a98: u16;
    static mut DAT_00655a96: u16;
    static mut DAT_00656c82: u16;
    static mut DAT_00656c84: u16;
    static mut DAT_00655174: u8;
    static mut DAT_0065518a: u8;

    static mut DAT_00655170: *mut c_void;
    static mut DAT_00655814: *mut c_void;
    static mut DAT_006559c0: *mut c_void;
    static mut DAT_00655150: *mut c_void;
    static mut DAT_006551d0: [u8; 1];
    static mut DAT_006552f0: [u8; 1];
    static mut DAT_00656f60: [u8; 1];
    static mut DAT_00657080: [u8; 1];

    static mut DAT_006d4990: *const c_void;
    static mut DAT_00703c64: *const c_void;
    static mut DAT_006d4a48: *const c_void;
    static mut DAT_0071e52c: *const c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004440e0(param_1: u16) -> u32;
    fn FUN_004a9bd0(param_1: u16);
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a98a0() {
    // SAFETY: all globals, callees and pointers belong to the game; reads and
    // writes happen in the original's order.
    unsafe {
        let mut p_slot: *mut u8 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
        let u_tmp = p_slot.cast::<u32>().read_unaligned();
        let i_tmp = u_tmp
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32) as i32;
        let i_tmp2 = rd32(i_tmp);
        let i_tmp = rd32(i_tmp2.wrapping_add(0xa7));
        let u_var2 = ((i_tmp.wrapping_add(0x1db)) as usize as *const u16).read_unaligned();

        if (u_var2 & 0x80) != 0 && (u_var2 & 0x100) != 0 {
            DAT_006551ca = DAT_00655812;
            DAT_00655154 = DAT_00655a8e;
            DAT_00655a8e = 0;
            DAT_00655812 = 0;
            DAT_00655a98 = DAT_00656c82;
            DAT_00655a96 = DAT_00656c84;
            if DAT_00655174 == 0 {
                DAT_00655170 = (&raw mut DAT_006551d0).cast::<c_void>();
                DAT_00655814 = (&raw mut DAT_006552f0).cast::<c_void>();
                DAT_006559c0 = (&raw mut DAT_00656f60).cast::<c_void>();
                DAT_00655150 = (&raw mut DAT_00657080).cast::<c_void>();
            } else {
                DAT_00655170 = (&raw mut DAT_006552f0).cast::<c_void>();
                DAT_00655814 = (&raw mut DAT_006551d0).cast::<c_void>();
                DAT_006559c0 = (&raw mut DAT_00657080).cast::<c_void>();
                DAT_00655150 = (&raw mut DAT_00656f60).cast::<c_void>();
            }
            let flags = (&raw mut DAT_0052687a).cast::<u8>();
            if _DAT_00732a3c < DAT_006d4980 {
                let mut i_var8 = (_DAT_00732a3c as u32).wrapping_mul(0x23) as i32;
                let mut u_var5 = _DAT_00732a3c as u32;
                loop {
                    p_slot = (&raw mut DAT_0071b730)
                        .cast::<u8>()
                        .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
                    let u_tmp = p_slot.cast::<u32>().read_unaligned();
                    let b = *((DAT_006d4988.wrapping_add(i_var8).wrapping_add(3)) as usize
                        as *const u8);
                    if u_tmp != u_var5
                        && (*flags.byte_add((b as u32).wrapping_mul(0x16) as usize) & 1) != 0
                    {
                        let i_tmp = rd32(DAT_006d4988.wrapping_add(i_var8).wrapping_add(0x1f));
                        let i_var6 = rd32(i_tmp.wrapping_add(0xa7));
                        let s_tmp = p_slot.byte_add(0x1a).cast::<u16>().read_unaligned();
                        let mut call = u32::from(s_tmp) == u_var5;
                        if !call {
                            let u_var3 = FUN_004440e0(u_var5 as u16);
                            if (u_var3 as u8) == 0 {
                                let c_var1 = *((i_var6.wrapping_add(7)) as usize as *const i8);
                                if c_var1 != 3 && c_var1 != 4 {
                                    call = true;
                                }
                            }
                        }
                        if call {
                            FUN_004a9bd0(u_var5 as u16);
                        }
                    }
                    i_var8 = i_var8.wrapping_add(0x23);
                    u_var5 = u_var5.wrapping_add(1);
                    if !((u_var5 as i32) < DAT_006d4980) {
                        break;
                    }
                }
            }
            if DAT_0071b704 < DAT_00732904 {
                let mut i_var8 = (DAT_0071b704 as u32).wrapping_mul(0x23) as i32;
                let mut i_var6 = DAT_0071b704;
                loop {
                    let b = *((DAT_006d4988.wrapping_add(3).wrapping_add(i_var8)) as usize
                        as *const u8);
                    if (*flags.byte_add((b as u32).wrapping_mul(0x16) as usize) & 1) != 0 {
                        FUN_004a9bd0(i_var6 as u16);
                    }
                    i_var8 = i_var8.wrapping_add(0x23);
                    i_var6 = i_var6.wrapping_add(1);
                    if !(i_var6 < DAT_00732904) {
                        break;
                    }
                }
            }
            let mut i_var8 = DAT_00703c88.wrapping_add(DAT_00732a38);
            if DAT_00732a38 < i_var8 {
                let mut i_var6 = (DAT_00732a38 as u32).wrapping_mul(0x23) as i32;
                let mut i_var7 = DAT_00732a38;
                loop {
                    let b = *((DAT_006d4988.wrapping_add(3).wrapping_add(i_var6)) as usize
                        as *const u8);
                    if (*flags.byte_add((b as u32).wrapping_mul(0x16) as usize) & 1) != 0 {
                        FUN_004a9bd0(i_var7 as u16);
                    }
                    i_var6 = i_var6.wrapping_add(0x23);
                    i_var7 = i_var7.wrapping_add(1);
                    i_var8 = DAT_00703c88.wrapping_add(DAT_00732a38);
                    if !(i_var7 < i_var8) {
                        break;
                    }
                }
            }
            let _ = i_var8;
            if DAT_0065518a != 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_006d4990);
                f();
            }
            if DAT_006551ca != 0 {
                let f: unsafe extern "C" fn(*mut c_void, u32) = core::mem::transmute(DAT_00703c64);
                f(DAT_00655170, u32::from(DAT_006551ca));
            }
            if DAT_00655812 != 0 {
                let f: unsafe extern "C" fn(*mut c_void, u32) = core::mem::transmute(DAT_006d4a48);
                f(DAT_00655814, u32::from(DAT_00655812));
            }
            if DAT_00655154 != 0 {
                let f: unsafe extern "C" fn(*mut c_void, u32) = core::mem::transmute(DAT_00703c64);
                f(DAT_006559c0, u32::from(DAT_00655154));
            }
            if DAT_00655a8e != 0 {
                let f: unsafe extern "C" fn(*mut c_void, u32) = core::mem::transmute(DAT_006d4a48);
                f(DAT_00655150, u32::from(DAT_00655a8e));
            }
            p_slot = (&raw mut DAT_0071b730)
                .cast::<u8>()
                .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
            let s_tmp = p_slot.byte_add(0x1a).cast::<u16>().read_unaligned();
            let b_var9: u8 = ((s_tmp as i16) != -1) as u8;
            if b_var9 != 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_0071e52c);
                f();
            }
            DAT_00655174 ^= 1;
            DAT_0065518a = b_var9;
        }
    }
}
