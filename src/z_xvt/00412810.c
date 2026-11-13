/* SPDX-License-Identifier: MIT */
/* z_xvt 00412810 */
int __cdecl FUN_00412810(int param_1, int param_2, int param_3)
{
    if (param_1 < 0) {
        param_1 = (int)(0u - (unsigned int)param_1);
    }
    if (param_2 < 0) {
        param_2 = (int)(0u - (unsigned int)param_2);
    }
    if (param_3 < 0) {
        param_3 = (int)(0u - (unsigned int)param_3);
    }
    if ((param_2 < param_1) && (param_3 < param_1)) {
        return (int)((unsigned int)param_1 + (unsigned int)(param_3 >> 2) + (unsigned int)(param_2 >> 2));
    }
    if ((param_1 < param_2) && (param_3 < param_2)) {
        return (int)((unsigned int)(param_1 >> 2) + (unsigned int)(param_3 >> 2) + (unsigned int)param_2);
    }
    return (int)((unsigned int)(param_1 >> 2) + (unsigned int)(param_2 >> 2) + (unsigned int)param_3);
}
