// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da4f8: i32;
    static mut DAT_006db59c: i32;
    static mut DAT_0071e53c: u32;
    static mut DAT_006d4988: *mut core::ffi::c_void;
    static mut DAT_0052686c: *mut core::ffi::c_void;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_0071e6b8: *mut core::ffi::c_void;
    static mut DAT_00732a38: u32;
    static mut DAT_00703c88: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004340e0(param_1: i32, param_2: u32, param_3: i32) -> i32;
    fn FUN_0045b880(param_1: i32, param_2: u32) -> i32;
}

unsafe fn record(u_var5: u32) {
    // SAFETY: the globals and the record buffer are the game's own memory;
    // writes go where the original writes them.
    unsafe {
        if DAT_006da4f8 < 0x128 {
            let base = DAT_0071e6b8 as *mut u8;
            let n = DAT_006da4f8;
            let off = n.wrapping_mul(0xc) as isize;
            base.byte_offset(off)
                .cast::<u32>()
                .write_unaligned(DAT_006db7b4 as u32);
            base.byte_offset(off)
                .byte_add(4)
                .cast::<u32>()
                .write_unaligned(u_var5);
            base.byte_offset(off)
                .byte_add(8)
                .cast::<i32>()
                .write_unaligned(DAT_006db59c);
            let t = n
                .wrapping_add(-2)
                .wrapping_add(n.wrapping_add(1).wrapping_mul(2))
                .wrapping_mul(4);
            DAT_006db59c = (base as u32 as i32).wrapping_add(t);
            DAT_006da4f8 = DAT_006da4f8.wrapping_add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00433e80() {
    // SAFETY: all globals and callees are the game's own; the tables read
    // are the ones the original reads, at the original's offsets.
    unsafe {
        let mut u_var5: u32 = 0;
        DAT_006da4f8 = 0;
        DAT_006db59c = 0;

        if DAT_0071e53c != 0 {
            let mut i_var6: i32 = 0;
            loop {
                let tbl = DAT_006d4988 as *const u8;
                let u_var3 = tbl.byte_offset(i_var6.wrapping_add(3) as isize).read() as u32;
                if u_var3 != 0 {
                    let switch_val = tbl.byte_offset(i_var6.wrapping_add(2) as isize).read();
                    let mut call_result: i32 = 0;
                    match switch_val {
                        0..=5 => {
                            let w = (DAT_0052686c as *const u16)
                                .add(u_var3 as usize)
                                .read_unaligned();
                            call_result = FUN_004340e0(u_var5 as i32, w as u32, DAT_0071ecf4);
                        }
                        6 | 7 | 0xb | 0xd => {
                            let w = (DAT_0052686c as *const u16)
                                .add(u_var3 as usize)
                                .read_unaligned();
                            call_result = FUN_0045b880(u_var5 as i32, w as u32);
                        }
                        _ => {}
                    }
                    if call_result != 0 {
                        record(u_var5);
                    }
                }
                i_var6 = i_var6.wrapping_add(0x23);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_0071e53c) {
                    break;
                }
            }
        }

        if DAT_00732a38 < DAT_00703c88.wrapping_add(DAT_00732a38) {
            let mut i_var6: i32 = DAT_00732a38.wrapping_mul(0x23) as i32;
            u_var5 = DAT_00732a38;
            loop {
                let tbl = DAT_006d4988 as *const u8;
                let u_var3 = tbl.byte_offset(i_var6.wrapping_add(3) as isize).read() as u32;
                let b_var1 = tbl.byte_offset(i_var6.wrapping_add(2) as isize).read();
                if u_var3 != 0 && b_var1 > 7 && b_var1 < 10 {
                    let w = (DAT_0052686c as *const u16)
                        .add(u_var3 as usize)
                        .read_unaligned();
                    let call_result = FUN_004340e0(u_var5 as i32, w as u32, DAT_0071ecf4);
                    if call_result != 0 {
                        record(u_var5);
                    }
                }
                i_var6 = i_var6.wrapping_add(0x23);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                    break;
                }
            }
        }
    }
}
