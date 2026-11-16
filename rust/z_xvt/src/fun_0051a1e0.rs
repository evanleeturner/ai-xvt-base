// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00742728: [i8; 3];
    static mut DAT_0052a330: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __getcwd(buf: *mut i8, size: i32) -> *mut i8;
    fn _fopen(name: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(file: *mut core::ffi::c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetLogicalDriveStringsA(n: u32, buf: *mut i8) -> u32;
    fn GetDriveTypeA(root: *const i8) -> u32;
    fn GetVolumeInformationA(
        root: *const i8,
        name_buf: *mut i8,
        name_len: u32,
        serial: *mut u32,
        max_comp: *mut u32,
        flags: *mut u32,
        fs_buf: *mut i8,
        fs_len: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051a1e0(param_1: i32, param_2: *mut i8, param_3: i32) -> u32 {
    let mut local_1b0: u32 = 0;
    let mut local_1ac: u32 = 0;
    let mut local_1a8: u32 = 0;
    let mut local_1a4: [i8; 80] = [0; 80];
    let mut local_154: [i8; 260] = [0; 260];
    let mut local_50: [i8; 80] = [0; 80];
    let mut param_2 = param_2;

    // SAFETY: the callees and globals are the game's own; pointers are
    // walked exactly as the original does, including its unchecked copies.
    unsafe {
        let dat = (&raw mut DAT_00742728).cast::<i8>();
        if param_3 == 1 {
            let p = __getcwd(local_154.as_mut_ptr(), 0x104);
            if !p.is_null() {
                *dat.add(1) = 0x3a;
                *dat.add(2) = 0;
                *dat = local_154[0];
                return 1;
            }
            return 0;
        }
        let n = GetLogicalDriveStringsA(0, core::ptr::null_mut());
        let mut iv9: i32 = 0;
        GetLogicalDriveStringsA(n, local_154.as_mut_ptr());
        let mut p3: *mut i8 = local_154.as_mut_ptr();
        let mut found = false;
        if 0 < (n as i32) {
            loop {
                'next: {
                    if GetDriveTypeA(p3) != 5 {
                        break 'next;
                    }
                    // copy drive string into local_1a4
                    let mut s = p3 as *const i8;
                    let mut d = local_1a4.as_mut_ptr();
                    loop {
                        let c = *s;
                        *d = c;
                        s = s.add(1);
                        d = d.add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    // append param_1 + 1 string at the terminator
                    let mut s = (param_1.wrapping_add(1) as u32 as usize) as *const i8;
                    let mut d = local_1a4.as_mut_ptr();
                    while *d != 0 {
                        d = d.add(1);
                    }
                    loop {
                        let c = *s;
                        *d = c;
                        s = s.add(1);
                        d = d.add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let f = _fopen(local_1a4.as_ptr(), (&raw const DAT_0052a330).cast::<i8>());
                    if f.is_null() {
                        break 'next;
                    }
                    _fclose(f);
                    if !param_2.is_null() {
                        let mut i = 0;
                        while i < 80 {
                            local_1a4[i] = 0;
                            i += 1;
                        }
                        GetVolumeInformationA(
                            p3,
                            local_1a4.as_mut_ptr(),
                            0x50,
                            &raw mut local_1b0,
                            &raw mut local_1ac,
                            &raw mut local_1a8,
                            local_50.as_mut_ptr(),
                            0x50,
                        );
                        let mut p5: *const i8 = local_1a4.as_ptr();
                        loop {
                            if *p5 != *param_2 || *p5 == 0 {
                                break;
                            }
                            let p12 = p5.add(1);
                            if *p12 != *param_2.add(1) {
                                break;
                            }
                            p5 = p5.add(2);
                            param_2 = param_2.add(2);
                            if *p12 == 0 {
                                break;
                            }
                        }
                    }
                    found = true;
                    break 'next;
                }
                if found {
                    break;
                }
                p3 = p3.add(4);
                iv9 = iv9.wrapping_add(4);
                if (n as i32) <= iv9 {
                    break;
                }
            }
        }
        if !found {
            return 0;
        }
        let mut s = p3 as *const i8;
        let mut d = dat;
        loop {
            let c = *s;
            *d = c;
            s = s.add(1);
            d = d.add(1);
            if c == 0 {
                break;
            }
        }
        *dat.add(2) = 0;
        1
    }
}
