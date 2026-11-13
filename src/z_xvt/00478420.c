/* SPDX-License-Identifier: MIT */
/* z_xvt 00478420 */
unsigned int __cdecl FUN_00478420(unsigned int param_1, unsigned int param_2)
{
    if (param_2 == param_1) {
        return 0xffff;
    }
    if (param_2 == 0) {
        return 0xffff;
    }
    if (param_2 <= param_1) {
        return 0xffff;
    }
    for (; (0xffff < param_1 || (0xffff < param_2)); param_2 = param_2 >> 1) {
        param_1 = param_1 >> 1;
    }
    /* param_2 is never 0 here: it started > param_1 >= 0 (so >= 1), and is only
       shifted while it is > 0xffff or param_1 is > 0xffff, which keeps it >= 0x8000
       after any shift. The divide is a plain 32-bit unsigned DIV. */
    return (param_1 << 0x10) / param_2;
}
