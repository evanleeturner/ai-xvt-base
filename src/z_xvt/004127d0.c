/* SPDX-License-Identifier: MIT */
/* z_xvt 004127d0 */
int __cdecl FUN_004127d0(uint param_1, uint param_2, uint param_3)
{
    if ((param_2 < param_1) && (param_3 < param_1)) {
        return (int)(param_1 + ((param_3 >> 2) + (param_2 >> 2)));
    }
    if ((param_1 < param_2) && (param_3 < param_2)) {
        return (int)(((param_3 >> 2) + (param_1 >> 2)) + param_2);
    }
    return (int)(((param_2 >> 2) + (param_1 >> 2)) + param_3);
}
