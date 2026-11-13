// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: i32;
    static mut DAT_006d4988: *mut core::ffi::c_void;
    static mut DAT_0052b0b4: [u8; 0];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b757: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004479a0(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u32;
    fn FUN_00443ee0(param_1: u32) -> i32;
    fn FUN_004a1d70(param_1: u32, param_2: i32);
}

unsafe fn rd_u8(addr: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const u8).read() }
}

unsafe fn rd_i32(addr: u32) -> i32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00447740(param_1: i32, param_2: u32) {
    // SAFETY: all globals and addresses are the game's own memory, read and
    // written where the original does.
    unsafe {
        let base = DAT_006d4988 as usize as u32;
        let rec = param_1.wrapping_mul(0x23) as u32;
        let inner = rd_i32(base.wrapping_add(0x1f).wrapping_add(rec)) as u32;
        DAT_00732e54 = rd_i32(inner.wrapping_add(0xa7));
        let d = DAT_00732e54 as u32;
        let b_var3 = rd_u8(param_2.wrapping_add(0x22b).wrapping_add(d));
        let i_var1 = param_2.wrapping_add(d);
        let mut s_var9: i16 = 0;
        let mut b_var6: i32 = 0;
        let idx = param_2.wrapping_add((rd_u8(d.wrapping_add(4)) as u32).wrapping_mul(0xef));
        let b_var4 = rd_u8((&raw mut DAT_0052b0b4 as usize as u32).wrapping_add(idx));
        let mut u_var8: u32 = b_var4 as u32;
        let c_var5: u8;

        'lab: {
            if (b_var3 & 0x7f) == 3 {
                let mut u_var7 = FUN_004479a0(
                    param_1,
                    u_var8,
                    rd_u8(i_var1.wrapping_add(0x229)) as u32,
                    param_2,
                );
                if u_var7 == 0xffff_ffff {
                    let dd = DAT_00732e54 as u32;
                    if rd_u8(dd.wrapping_add(0x346).wrapping_add(u_var8.wrapping_mul(4))) != 0 {
                        b_var6 = 1;
                    }
                } else {
                    s_var9 = 1;
                }
                u_var7 = b_var4.wrapping_add(1) as u32;
                // b_var4 + 1 promoted to int then truncated to unsigned short
                u_var7 = (b_var4 as u32).wrapping_add(1) & 0xffff;
                let dd = DAT_00732e54 as u32;
                u_var8 = FUN_004479a0(
                    param_1,
                    u_var7,
                    rd_u8(dd.wrapping_add(0x229).wrapping_add(param_2)) as u32,
                    param_2,
                );
                if u_var8 != 0xffff_ffff {
                    s_var9 = s_var9.wrapping_add(1);
                    break 'lab;
                }
                let dd = DAT_00732e54 as u32;
                c_var5 = rd_u8(dd.wrapping_add(0x346).wrapping_add(u_var7.wrapping_mul(4)));
            } else if (b_var3 & 0x80) == 0 {
                let u_var7 = FUN_004479a0(
                    param_1,
                    u_var8,
                    rd_u8(i_var1.wrapping_add(0x229)) as u32,
                    param_2,
                );
                if u_var7 != 0xffff_ffff {
                    s_var9 = 1;
                    break 'lab;
                }
                let dd = DAT_00732e54 as u32;
                c_var5 = rd_u8(dd.wrapping_add(0x346).wrapping_add(u_var8.wrapping_mul(4)));
            } else {
                let u_var7 = (b_var4 as u32).wrapping_add(1) & 0xffff;
                u_var8 = FUN_004479a0(
                    param_1,
                    u_var7,
                    rd_u8(i_var1.wrapping_add(0x229)) as u32,
                    param_2,
                );
                if u_var8 != 0xffff_ffff {
                    s_var9 = 1;
                    break 'lab;
                }
                let dd = DAT_00732e54 as u32;
                c_var5 = rd_u8(dd.wrapping_add(0x346).wrapping_add(u_var7.wrapping_mul(4)));
            }
            if c_var5 != 0 {
                b_var6 = 1;
            }
        }

        let paddr = (DAT_00732e54 as u32)
            .wrapping_add(0x22d)
            .wrapping_add(param_2.wrapping_mul(2));
        let psv = paddr as usize as *mut i16;
        psv.write_unaligned(psv.read_unaligned().wrapping_add(0x1d8));

        let base = DAT_006d4988 as usize as u32;
        if rd_i32(base.wrapping_add(0x1b).wrapping_add(rec)) == DAT_0071ecf4 && b_var6 == 0 {
            let off = DAT_0071ecf4.wrapping_mul(0x5bd) as u32;
            let sel = rd_u8((&raw mut DAT_0071b757 as usize as u32).wrapping_add(off));
            let dd = DAT_00732e54 as u32;
            let arg = (dd & 0xffff_0000)
                | (rd_u8(dd.wrapping_add(0x229).wrapping_add(sel as u32)) as u32);
            let u8v = FUN_00443ee0(arg) as u32;
            if s_var9 == 0 {
                FUN_004a1d70((u8v & 0xffff).wrapping_add(0x26), DAT_0071ecf4);
                return;
            }
            if s_var9 == 1 {
                FUN_004a1d70((u8v & 0xffff).wrapping_add(0x30), DAT_0071ecf4);
                return;
            }
            if s_var9 == 2 {
                FUN_004a1d70((u8v & 0xffff).wrapping_add(0x3a), DAT_0071ecf4);
            }
        }
    }
}
