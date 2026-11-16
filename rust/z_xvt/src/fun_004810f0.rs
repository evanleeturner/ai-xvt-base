// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703ca0: [u32; 0x1540];
    static mut DAT_006da9e8: i32;
    static mut DAT_00732a40: [*mut i8; 256];
    static mut DAT_006db7c0: [i8; 0];
    static mut DAT_0052a328: [i8; 0];
    static mut DAT_00523570: [i8; 0];
    static mut DAT_00732948: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047ed40(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32);
    fn FUN_00480ae0(param_1: *mut i8) -> u32;
    fn _fread(
        param_1: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004810f0(param_1: *mut i8) -> u32 {
    let mut local_104: u32 = 0;
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the globals and callees are the game's own; param_1 is a
    // NUL-terminated string, copied unchecked as the original does.
    unsafe {
        let dst = local_100.as_mut_ptr();
        let mut i: usize = 0;
        loop {
            let c = param_1.add(i).read();
            dst.add(i).write(c);
            i += 1;
            if c == 0 {
                break;
            }
        }

        let mut end = dst;
        while end.read() != 0 {
            end = end.add(1);
        }
        let src = (&raw mut DAT_0052a328).cast::<i8>();
        let mut j: usize = 0;
        loop {
            let c = src.add(j).read();
            end.add(j).write(c);
            j += 1;
            if c == 0 {
                break;
            }
        }

        let p = (&raw mut DAT_00703ca0).cast::<u32>();
        for k in 0..0x1540usize {
            p.add(k).write(0);
        }
        DAT_006da9e8 = 0;
        let q = (&raw mut DAT_00732a40).cast::<u32>();
        for k in 0..0x40usize {
            q.add(k).write(0);
        }

        FUN_0047ed40(dst, (&raw mut DAT_00523570).cast::<i8>(), 0, 1);
        let file = DAT_00732948;
        if file.is_null() {
            return FUN_00480ae0(param_1);
        }

        let l4 = (&raw mut local_104).cast::<core::ffi::c_void>();
        if _fread(l4, 4, 1, file) != 1 {
            _fclose(file);
            return FUN_00480ae0(param_1);
        }
        if _fread(p.cast(), local_104, 1, file) != 1 {
            _fclose(file);
            return FUN_00480ae0(param_1);
        }
        if _fread(l4, 4, 1, file) != 1 {
            _fclose(file);
            return FUN_00480ae0(param_1);
        }
        let base = (&raw mut DAT_006db7c0).cast::<i8>();
        if _fread(base.cast(), local_104, 1, file) != 1 {
            _fclose(file);
            return FUN_00480ae0(param_1);
        }
        _fclose(file);

        let mut pc = p.cast::<i8>();
        let mut pp = (&raw mut DAT_00732a40).cast::<*mut i8>();
        for _ in 0..256 {
            if pc.read() != 0 {
                DAT_006da9e8 = DAT_006da9e8.wrapping_add(1);
                let off = pc.byte_add(0x51).cast::<i32>().read_unaligned();
                pp.write(base.offset(off as isize));
            }
            pc = pc.byte_add(0x55);
            pp = pp.add(1);
        }
        1
    }
}
