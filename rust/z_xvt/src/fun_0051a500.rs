// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals, non_snake_case)]
unsafe extern "C" {
    #[link_name = "fopen"]
    fn _fopen(filename: *const i8, mode: *const i8) -> *mut c_void;
    #[link_name = "_beginthreadex"]
    fn __beginthreadex(
        security: *mut c_void,
        stack_size: u32,
        start_address: *const c_void,
        arglist: *mut c_void,
        create_flags: u32,
        thread_id: *mut u32,
    ) -> u32;
    fn FUN_0051a1e0(param_1: u32, param_2: *mut i8, param_3: u32) -> i32;
    fn FUN_0051c100(param_1: i8, param_2: i8);
    fn FUN_0051cdc0(param_1: i8, param_2: *const i8, param_3: i32);
    fn FUN_0051c160(param_1: u32, param_2: u32) -> u32;
    fn FUN_0048f3c0(param_1: u32, param_2: u32) -> u32;
    fn FUN_0051aff0(param_1: *mut u32) -> i32;
    fn FUN_0051c600(param_1: *mut u32);
    fn FUN_0051c6f0(param_1: u32);

    static LAB_0051a910: u8;

    static mut DAT_0074278c: i32;
    static mut DAT_00742780: *mut c_void;
    static mut DAT_0052a330: i8;
    static mut DAT_00742784: *mut c_void;
    static mut DAT_007427a0: u8;
    static mut DAT_00742728: i8;
    static mut s__amovie_a_wrk_00531a40: i8;
    static mut DAT_006550d4: u32;
    static mut DAT_007426c4: u32;
    static mut DAT_00742790: u32;
    static mut DAT_00742788: u32;
    static mut DAT_00742724: u32;
    static mut _DAT_00742720: u32;
    static mut DAT_00742814: u32;
    static mut DAT_007426d0: u32;
    static mut DAT_00742660: i32;
    static mut _DAT_005366d8: u32;
    static mut DAT_00742730: i32;
    static mut DAT_00742668: i32;
    static mut DAT_00757c30: *mut u32;
    static mut DAT_005366f0: i16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetThreadPriority(hThread: *mut c_void) -> i32;
    fn InitializeCriticalSection(lpCriticalSection: *mut c_void);
    fn timeGetTime() -> u32;
    fn ResumeThread(hThread: *mut c_void) -> u32;
    fn FUN_0051a700() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051a500(param_1: *mut u32) -> u32 {
    let mut local_8: [u8; 4] = [0; 4];
    let mut local_4: u32 = 0;
    // SAFETY: all globals and callees are the game's own; param_1 points at
    // the caller's parameter block, read and written where the original does.
    unsafe {
        let mut i_var4: i32 = 0;
        let mut i_var1: i32;
        DAT_0074278c = 0;
        DAT_00742780 = _fopen(
            param_1.read() as usize as *const i8,
            &raw const DAT_0052a330,
        );
        i_var1 = 0;
        'lab: {
            if !DAT_00742780.is_null() {
                DAT_00742784 = __beginthreadex(
                    core::ptr::null_mut(),
                    0,
                    &raw const LAB_0051a910 as *const c_void,
                    local_8.as_mut_ptr().cast::<c_void>(),
                    4,
                    &raw mut local_4,
                ) as usize as *mut c_void;
                GetThreadPriority(DAT_00742784);
                InitializeCriticalSection((&raw mut DAT_007427a0).cast::<c_void>());
                i_var1 = FUN_0051a1e0(0x531a40, core::ptr::null_mut(), param_1.add(3).read());
                if i_var1 != 0 {
                    FUN_0051c100(param_1.add(3).read() as i8, DAT_00742728);
                    FUN_0051cdc0(
                        DAT_00742728,
                        &raw const s__amovie_a_wrk_00531a40,
                        param_1 as usize as i32,
                    );
                    let u_var2 = FUN_0051c160(param_1.add(1).read(), param_1.add(2).read());
                    if (u_var2 as u16) == 0 {
                        return (u_var2 & 0xffff0000) | 0xffff;
                    }
                    DAT_006550d4 = FUN_0048f3c0(0x20000, 2);
                    if DAT_006550d4 != 0 {
                        DAT_007426c4 = FUN_0048f3c0(0x300, 2);
                        if DAT_007426c4 != 0 {
                            DAT_00742790 = param_1.add(7).read();
                            DAT_00742788 = param_1.add(8).read();
                            DAT_00742724 = param_1.add(9).read();
                            _DAT_00742720 = param_1.add(10).read();
                            DAT_00742814 = param_1.add(0xb).read();
                            i_var1 = FUN_0051aff0(param_1);
                            DAT_0074278c = i_var1;
                            if 0 < i_var1 {
                                loop {
                                    let p = (&raw mut DAT_007426d0).add(i_var4 as usize).read();
                                    FUN_0051c600(p as usize as *mut u32);
                                    if i_var4 == 0 {
                                        FUN_0051c6f0(DAT_007426d0);
                                    }
                                    i_var4 = i_var4.wrapping_add(1);
                                    if i_var4 >= DAT_0074278c {
                                        break;
                                    }
                                }
                                DAT_00742660 = 0;
                                _DAT_005366d8 = timeGetTime();
                                ResumeThread(DAT_00742784);
                                i_var1 =
                                    (&raw mut DAT_00742730).offset(DAT_00742660 as isize).read();
                                DAT_00742668 = i_var1;
                            }
                            break 'lab;
                        }
                    }
                    FUN_0051a700();
                    return 0xffffffff;
                }
                i_var1 = FUN_0051a700();
            }
        }
        let u_var3 = ((i_var1 as u32) >> 16) as u16;
        if DAT_0074278c < 0 {
            return ((u_var3 as u32) << 16) | (DAT_0074278c as u16 as u32);
        }
        if DAT_0074278c == 0 {
            param_1.add(0xd).write(1);
            return ((u_var3 as u32) << 16) | (DAT_0074278c as u16 as u32);
        }
        DAT_00757c30 = param_1;
        ((u_var3 as u32) << 16) | (DAT_005366f0 as u16 as u32)
    }
}
