/* SPDX-License-Identifier: MIT */
/* z_xvt 00413df0 */
int __cdecl FUN_00413df0(int param_1, int param_2, int param_3, int param_4)
{
    int iVar1;
    unsigned int uVar2;

    uVar2 = (unsigned int)param_1 * (unsigned int)param_2;
    iVar1 = (int)((unsigned int)((int)uVar2 >> 0xf) * (unsigned int)param_3
                  + uVar2
                  + (unsigned int)param_4 * 0x8000u);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    return iVar1 >> 0xf;
}
