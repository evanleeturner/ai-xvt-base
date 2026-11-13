/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c970 */
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_006db7b4;

int __cdecl FUN_00478290(unsigned int, int, int);

void __cdecl FUN_0040c970(int param_1, int param_2, int param_3)
{
    int iVar1;
    unsigned int uVar2;

    uVar2 = 0u - (unsigned int)DAT_006db7b4;
    if (DAT_006db7ac < param_1) {
        iVar1 = FUN_00478290(uVar2, (int)((unsigned int)param_1 - (unsigned int)DAT_006db7ac), (int)((unsigned int)param_3 + uVar2));
        DAT_006db7ac = (int)((unsigned int)DAT_006db7ac + (unsigned int)iVar1);
    }
    else {
        iVar1 = FUN_00478290(uVar2, (int)((unsigned int)DAT_006db7ac - (unsigned int)param_1), (int)((unsigned int)param_3 + uVar2));
        DAT_006db7ac = (int)((unsigned int)DAT_006db7ac - (unsigned int)iVar1);
    }
    if (DAT_006db7b0 < param_2) {
        iVar1 = FUN_00478290(uVar2, (int)((unsigned int)param_2 - (unsigned int)DAT_006db7b0), (int)((unsigned int)param_3 + uVar2));
        DAT_006db7b0 = (int)((unsigned int)DAT_006db7b0 + (unsigned int)iVar1);
    }
    else {
        iVar1 = FUN_00478290(uVar2, (int)((unsigned int)DAT_006db7b0 - (unsigned int)param_2), (int)((unsigned int)param_3 + uVar2));
        DAT_006db7b0 = (int)((unsigned int)DAT_006db7b0 - (unsigned int)iVar1);
    }
    DAT_006db7b4 = 1;
    return;
}
