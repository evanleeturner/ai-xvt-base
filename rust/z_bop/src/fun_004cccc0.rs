// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69aa5: i8;
    static mut DAT_00b69aa6: i8;
    static mut DAT_00b69aa7: i8;
    static mut DAT_00b69ba7: i8;
    static mut DAT_0052b79c: [i8; 1];
    static mut DAT_0052b7e4: [i8; 1];
    static mut DAT_0052b7a0: [i8; 1];
    static mut DAT_0052b78c: [i8; 1];
    static mut DAT_0052b748: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ccbc0() -> u32;
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn __getdrive() -> i32;
    fn __getcwd(buf: *mut i8, size: i32) -> *mut i8;
    fn __chdir(path: *const i8) -> i32;
    fn _tolower(c: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegCreateKeyA(hKey: *mut c_void, lpSubKey: *const i8, phkResult: *mut *mut c_void) -> i32;
    fn RegCloseKey(hKey: *mut c_void) -> i32;
    fn RegOpenKeyExA(
        hKey: *mut c_void,
        lpSubKey: *const i8,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut *mut c_void,
    ) -> i32;
    fn RegQueryValueExA(
        hKey: *mut c_void,
        lpValueName: *const i8,
        lpReserved: *mut u32,
        lpType: *mut u32,
        lpData: *mut u8,
        lpcbData: *mut u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cccc0(param_1: i32) {
    // SAFETY: all globals, callees and registry/CRT functions belong to the
    // game's own process; buffers are the game's fixed-size global arrays
    // and the local 256-byte buffer, used as the original does.
    unsafe {
        let mut local_108: *mut c_void = core::ptr::null_mut();
        let mut local_104: u32;
        let mut local_100: [i8; 256] = [0; 256];

        if param_1 != 0 {
            let u_var3 = FUN_004ccbc0();
            DAT_00b69aa6 = u_var3 as i8;
        }
        _sprintf(
            local_100.as_mut_ptr(),
            (&raw const DAT_0052b7e4).cast::<i8>(),
            2i32,
            0i32,
        );
        RegCreateKeyA(
            0x80000002usize as *mut c_void,
            local_100.as_ptr(),
            &raw mut local_108,
        );
        RegCloseKey(local_108);
        let mut l_var4 = RegOpenKeyExA(
            0x80000002usize as *mut c_void,
            (&raw const DAT_0052b7a0).cast::<i8>(),
            0,
            0x20019,
            &raw mut local_108,
        );
        if l_var4 != 0 {
            let i_var5 = __getdrive();
            DAT_00b69aa5 = (i_var5 as i8).wrapping_add(0x60);
            __getcwd((&raw mut DAT_00b69aa7).cast::<i8>(), 0x100);
            __chdir((&raw const DAT_0052b79c).cast::<i8>());
            __getcwd(&raw mut DAT_00b69ba7, 0x100);
            __chdir((&raw const DAT_00b69aa7).cast::<i8>());
            return;
        }
        local_104 = 0x100;
        l_var4 = RegQueryValueExA(
            local_108,
            (&raw const DAT_0052b78c).cast::<i8>(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            (&raw mut DAT_00b69aa7).cast::<u8>(),
            &raw mut local_104,
        );
        let b_var2 = DAT_00b69aa7 as u8;
        if l_var4 != 0 {
            let i_var5 = __getdrive();
            DAT_00b69aa5 = (i_var5 as i8).wrapping_add(0x60);
            __getcwd((&raw mut DAT_00b69aa7).cast::<i8>(), 0x100);
            __chdir((&raw const DAT_0052b79c).cast::<i8>());
            __getcwd(&raw mut DAT_00b69ba7, 0x100);
            __chdir((&raw const DAT_00b69aa7).cast::<i8>());
            RegCloseKey(local_108);
            return;
        }
        let mut u_var6: u32 = 0xFFFF_FFFF;
        let mut pc_var7: *const i8 = &raw const DAT_00b69aa7;
        loop {
            if u_var6 == 0 {
                break;
            }
            u_var6 = u_var6.wrapping_sub(1);
            let c_var1 = pc_var7.read();
            pc_var7 = pc_var7.wrapping_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        if (&raw mut DAT_00b69aa5)
            .wrapping_add((!u_var6) as usize)
            .read()
            == b'\\' as i8
        {
            u_var6 = 0xFFFF_FFFF;
            pc_var7 = &raw const DAT_00b69aa7;
            loop {
                if u_var6 == 0 {
                    break;
                }
                u_var6 = u_var6.wrapping_sub(1);
                let c_var1 = pc_var7.read();
                pc_var7 = pc_var7.wrapping_add(1);
                if c_var1 == 0 {
                    break;
                }
            }
            (&raw mut DAT_00b69aa5)
                .wrapping_add((!u_var6) as usize)
                .write(0);
        }
        let i_var5 = _tolower(b_var2 as u32);
        DAT_00b69aa5 = i_var5 as i8;
        RegCloseKey(local_108);
        __chdir((&raw const DAT_00b69aa7).cast::<i8>());
        l_var4 = RegOpenKeyExA(
            0x80000002usize as *mut c_void,
            (&raw const DAT_0052b748).cast::<i8>(),
            0,
            0x20019,
            &raw mut local_108,
        );
        if l_var4 != 0 {
            __chdir((&raw const DAT_0052b79c).cast::<i8>());
            __getcwd(&raw mut DAT_00b69ba7, 0x100);
            __chdir((&raw const DAT_00b69aa7).cast::<i8>());
            return;
        }
        local_104 = 0x100;
        l_var4 = RegQueryValueExA(
            local_108,
            (&raw const DAT_0052b78c).cast::<i8>(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            (&raw mut DAT_00b69ba7).cast::<u8>(),
            &raw mut local_104,
        );
        if l_var4 != 0 {
            __chdir((&raw const DAT_0052b79c).cast::<i8>());
            __getcwd(&raw mut DAT_00b69ba7, 0x100);
            __chdir((&raw const DAT_00b69aa7).cast::<i8>());
            RegCloseKey(local_108);
            return;
        }
        RegCloseKey(local_108);
    }
}
