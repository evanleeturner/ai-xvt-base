// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d50: u16;
    static mut _DAT_006d9d52: i32;
    static mut DAT_006d9d5a: u16;
    static mut DAT_006d9d5c: u32;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d62: u16;
    static mut DAT_006d9d56: *mut u8;
    static mut DAT_006d9d64: i32;
    static mut DAT_006d9d68: i32;
    static mut DAT_006d9d6c: i32;
    static mut DAT_006d9d70: u16;
    static mut DAT_006d9d74: i32;
    static mut DAT_006d9d78: u8;
    static mut DAT_006d9d72: u16;
    static mut DAT_006d9d79: i8;
    static mut DAT_006d4988: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a40: i32;
    static mut DAT_00524948: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_00481380(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004296b0(param_1: u16) {
    // SAFETY: all globals and callees are the game's own memory; the raw
    // addresses computed from them are read and written exactly where the
    // original does.
    unsafe {
        DAT_006d9d50 = param_1;
        let i_var5: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let tmp_addr1: i32 =
            ((DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5)) as *const i32).read_unaligned();
        let tmp_val1: i32 = (tmp_addr1.wrapping_add(0xa7) as *const i32).read_unaligned();
        _DAT_006d9d52 = tmp_val1;
        DAT_006d9d5a = ((_DAT_006d9d52.wrapping_add(5)) as *const u8).read() as u16;
        let pbvar4 = FUN_0042b150(_DAT_006d9d52) as *mut u8;
        let tmp_addr2: i32 = if DAT_006d9d5a == 0xff {
            ((DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5)) as *const i32).read_unaligned()
        } else {
            (((DAT_006d9d5a as u32).wrapping_mul(0x23) as i32)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988) as *const i32)
                .read_unaligned()
        };
        let tmp_val2: u32 = (tmp_addr2.wrapping_add(0xa7) as *const u32).read_unaligned();
        DAT_006d9d5c = tmp_val2;
        DAT_006d9d60 =
            ((DAT_006d4988.wrapping_add(0x16).wrapping_add(i_var5)) as *const u8).read() as u16;
        DAT_006d9d62 = pbvar4.read() as u16;
        DAT_006d9d56 = pbvar4;
        FUN_00421c70(DAT_006d9d50 as u32, DAT_006d9d60 as i32);
        let i_var5b = _DAT_006d9d52;
        DAT_006d9d64 = DAT_00732a2c;
        DAT_006d9d68 = DAT_00732a28;
        DAT_006d9d6c = DAT_00732a24;
        let psvar1: i32 = (_DAT_006d9d52.wrapping_add(0x432) as *const i32).read_unaligned();
        let u_var3: u16;
        'done: {
            if psvar1 == 0 {
                u_var3 = (i_var5b.wrapping_add(9) as *const u16).read_unaligned();
            } else {
                let a = (_DAT_006d9d52.wrapping_add(900) as *const i16).read_unaligned();
                let b = (psvar1 as *const i16).read_unaligned();
                if a == b {
                    let tmp_ptr2: i32 = (psvar1.wrapping_add(0x1f) as *const i32).read_unaligned();
                    let tmp_ptr3: i32 =
                        (tmp_ptr2.wrapping_add(0xab) as *const i32).read_unaligned();
                    if tmp_ptr2 != 0 && tmp_ptr3 != 0 {
                        u_var3 = (tmp_ptr3 as *const u16).read_unaligned();
                        break 'done;
                    }
                } else {
                    (_DAT_006d9d52.wrapping_add(0x432) as *mut u32).write_unaligned(0);
                }
                u_var3 = (i_var5b.wrapping_add(9) as *const u16).read_unaligned();
            }
        }
        if u_var3 < 0x8000 {
            DAT_006d9d70 = 0;
        } else {
            DAT_006d9d70 = 2u16.wrapping_sub((u_var3 < 0xc000) as u16);
        }
        let arr = &raw mut DAT_00732a40;
        DAT_006d9d74 = arr
            .add(pbvar4.add(10).read() as usize)
            .read()
            .wrapping_add(2);
        DAT_006d9d78 = 0;
        let base = arr.add(pbvar4.add(10).read() as usize).read();
        DAT_006d9d72 = (base.wrapping_add(1) as *const u8).read() as u16;
        let r = FUN_00481380(&raw mut DAT_00524948);
        DAT_006d9d79 = r as i8;
    }
}
