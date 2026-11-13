/* SPDX-License-Identifier: MIT */
/* z_xvt 00461f20 */
extern int DAT_00525518;
extern int DAT_00525534;
extern int DAT_005f10f0;
extern int DAT_005f1120[];
extern int *DAT_005f1d20;
extern int *DAT_005f1d24;
extern int DAT_006fede0[];
extern int DAT_0070acc8;

int __cdecl FUN_0048f570(void);

void __cdecl FUN_00461f20(int param_1, int param_2, uint param_3, int param_4)
{
    int iVar1;
    int iVar2;
    short sVar3;

    iVar1 = DAT_00525518;
    if (param_1 == 0) {
        DAT_00525534 = FUN_0048f570();
        DAT_005f1d24 = &DAT_0070acc8;
        DAT_005f1d20 = DAT_006fede0;
        return;
    }
    DAT_00525534 = param_1;
    if (param_4 != -1) {
        sVar3 = 0;
        if (param_3 != 0) {
            do {
                iVar2 = (int)sVar3;
                sVar3 = (short)(sVar3 + 1);
                DAT_005f1120[iVar2] = (int)((unsigned int)((unsigned int)iVar1 * (unsigned int)iVar2) * (unsigned int)param_2);
            } while ((uint)(int)sVar3 < param_3);
        }
        DAT_005f10f0 = param_4;
        DAT_005f1d24 = &DAT_005f10f0;
        DAT_005f1d20 = DAT_005f1120;
        return;
    }
    sVar3 = 0;
    if (param_3 != 0) {
        do {
            iVar1 = (int)sVar3;
            sVar3 = (short)(sVar3 + 1);
            DAT_006fede0[iVar1] = (int)((unsigned int)DAT_0070acc8 * (unsigned int)iVar1);
        } while ((uint)(int)sVar3 < param_3);
    }
    DAT_005f1d24 = &DAT_0070acc8;
    DAT_005f1d20 = DAT_006fede0;
    return;
}
