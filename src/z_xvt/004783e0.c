/* SPDX-License-Identifier: MIT */
/* z_xvt 004783e0 */
unsigned int __cdecl FUN_004783e0(unsigned short param_1, unsigned short param_2)
{
    if (param_2 == param_1) {
        return 0xFFFF;
    }
    if (param_2 == 0) {
        return 0;
    }
    if (param_1 < param_2) {
        return 0xFFFF;
    }
    return ((unsigned int)param_1 << 16) / param_2;
}
