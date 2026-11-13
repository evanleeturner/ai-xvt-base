/* SPDX-License-Identifier: MIT */
/* z_xvt 00413d00 */
int __cdecl FUN_00413d00(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    int iVar1;

    iVar1 = (int)((unsigned int)param_1 * (unsigned int)param_4
                + (unsigned int)param_2 * (unsigned int)param_5
                + (unsigned int)param_3 * (unsigned int)param_6);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    return iVar1 >> 0xf;
}
