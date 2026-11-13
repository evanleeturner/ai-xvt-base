/* SPDX-License-Identifier: MIT */
/* z_xvt 0040ca90 */
extern unsigned short DAT_00732e44;

int __cdecl FUN_00413f20(unsigned int param_1, unsigned int param_2);

int __cdecl FUN_0040ca90(unsigned int param_1, unsigned int param_2)
{
    int iVar1;

    if ((int)param_1 < 0) {
        iVar1 = FUN_00413f20(0u - param_1, param_2);
        iVar1 = (int)(0u - (unsigned int)iVar1);
    }
    else {
        iVar1 = FUN_00413f20(param_1, param_2);
    }
    return (int)((unsigned int)iVar1 + (unsigned int)DAT_00732e44);
}
