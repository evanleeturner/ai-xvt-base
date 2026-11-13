/* SPDX-License-Identifier: MIT */
/* z_xvt 004264b0 */
extern int DAT_006d4a50;
extern int DAT_006ffbec;
extern int DAT_0072f4e8;

void __cdecl FUN_004264b0(int param_1)
{
    int iVar1;

    iVar1 = (int)((unsigned int)*(int *)(param_1 + 4) + (unsigned int)DAT_006d4a50);
    *(int *)(param_1 + 4) = iVar1;
    if (iVar1 < -0x1000000) {
        *(unsigned int *)(param_1 + 4) = 0xff000000u;
    }
    if (0x1000000 < *(int *)(param_1 + 4)) {
        *(unsigned int *)(param_1 + 4) = 0x1000000u;
    }

    iVar1 = (int)((unsigned int)*(int *)(param_1 + 8) + (unsigned int)DAT_006ffbec);
    *(int *)(param_1 + 8) = iVar1;
    if (iVar1 < -0x1000000) {
        *(unsigned int *)(param_1 + 8) = 0xff000000u;
    }
    if (0x1000000 < *(int *)(param_1 + 8)) {
        *(unsigned int *)(param_1 + 8) = 0x1000000u;
    }

    iVar1 = (int)((unsigned int)*(int *)(param_1 + 0xc) + (unsigned int)DAT_0072f4e8);
    *(int *)(param_1 + 0xc) = iVar1;
    if (iVar1 < -0x1000000) {
        *(unsigned int *)(param_1 + 0xc) = 0xff000000u;
    }
    if (0x1000000 < *(int *)(param_1 + 0xc)) {
        *(unsigned int *)(param_1 + 0xc) = 0x1000000u;
    }
    return;
}
