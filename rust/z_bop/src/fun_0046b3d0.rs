// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527008: [i8; 1];
    static mut DAT_0051a854: [i8; 1];
    static mut DAT_009d1320: [u32; 0x1540];
    static mut DAT_009a8068: i32;
    static mut DAT_00a07cf0: [u32; 256];
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_009a8e40: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32) -> i32;
    fn FUN_0046aed0(param_1: *mut i8) -> u32;
    fn _fread(
        dst: *mut core::ffi::c_void,
        element_size: u32,
        count: u32,
        file: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(file: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046b3d0(param_1: *mut i8) -> u32 {
    let mut local_104: u32 = 0;
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: globals and callees are the game's own; param_1 is a
    // NUL-terminated string, copied without bounds checks as the original does.
    unsafe {
        let dst = local_100.as_mut_ptr();
        let mut n: usize = 0;
        loop {
            let c = param_1.add(n).read();
            dst.add(n).write(c);
            n += 1;
            if c == 0 {
                break;
            }
        }
        let mut end = n - 1;
        let src2 = (&raw mut DAT_00527008).cast::<i8>();
        let mut m: usize = 0;
        loop {
            let c = src2.add(m).read();
            dst.add(end).write(c);
            end += 1;
            m += 1;
            if c == 0 {
                break;
            }
        }

        let p = (&raw mut DAT_009d1320).cast::<u32>();
        for i in 0..0x1540usize {
            p.add(i).write(0);
        }
        DAT_009a8068 = 0;
        let q = (&raw mut DAT_00a07cf0).cast::<u32>();
        for i in 0..0x40usize {
            q.add(i).write(0);
        }
        FUN_0049e720(dst, (&raw mut DAT_0051a854).cast::<i8>(), 0, 1);
        let file = DAT_00a07bd8;
        if file.is_null() {
            return FUN_0046aed0(param_1);
        }
        let hdr = (&raw mut local_104).cast::<core::ffi::c_void>();
        if _fread(hdr, 4, 1, file) != 1 {
            _fclose(file);
            return FUN_0046aed0(param_1);
        }
        if _fread(p.cast(), local_104, 1, file) != 1 {
            _fclose(file);
            return FUN_0046aed0(param_1);
        }
        if _fread(hdr, 4, 1, file) != 1 {
            _fclose(file);
            return FUN_0046aed0(param_1);
        }
        let base = (&raw mut DAT_009a8e40).cast::<u8>();
        if _fread(base.cast(), local_104, 1, file) != 1 {
            _fclose(file);
            return FUN_0046aed0(param_1);
        }
        _fclose(file);

        let mut pc = p.cast::<i8>();
        let mut idx: usize = 0;
        loop {
            if pc.read() != 0 {
                DAT_009a8068 = DAT_009a8068.wrapping_add(1);
                let off = pc.byte_add(0x51).cast::<i32>().read_unaligned();
                q.add(idx).write((base as u32).wrapping_add(off as u32));
            }
            pc = pc.byte_add(0x55);
            idx += 1;
            if idx >= 256 {
                break;
            }
        }
        1
    }
}
