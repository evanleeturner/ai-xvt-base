/* SPDX-License-Identifier: MIT */
/* z_bop 00418760 */
extern int DAT_009a8db0[];
extern int DAT_009ed670[];

int * __cdecl FUN_00418760(int param_1, int param_2, int *param_3)
{
    int iVar1;
    int *piVar3;
    int iVar4;
    int *piVar5;
    int iVar6;

    iVar1 = DAT_009a8db0[param_1];
    piVar3 = DAT_009ed670 + param_1 * 0x7e9;
    piVar5 = (int *)(iVar1 * 0x12 + (int)piVar3);
    iVar6 = 0;
    if (iVar1 > 0) {
        do {
            iVar4 = piVar3[2];
            if (param_2 <= iVar4) goto LAB_004187b2;
            iVar6 = iVar6 + 1;
            piVar3 = (int *)((int)piVar3 + 0x12);
        } while (iVar6 < iVar1);
    }
    iVar4 = piVar3[2];
LAB_004187b2:
    if (iVar4 >= param_2 && iVar6 != iVar1) {
        if (iVar4 == param_2) {
            if (piVar3[1] == 0) {
                return (int *)0x0;
            }
            if (*piVar3 == 1) {
                return (int *)0x0;
            }
        }
    }
    else {
        if (iVar1 == 0x1c2) {
            return (int *)0x0;
        }
        DAT_009a8db0[param_1] = iVar1 + 1;
        for (; piVar3 < piVar5; piVar5 = (int *)((int)piVar5 + -0x12)) {
            *piVar5 = *(int *)((int)piVar5 + -0x12);
            piVar5[1] = *(int *)((int)piVar5 + -0xe);
            piVar5[2] = *(int *)((int)piVar5 + -10);
            piVar5[3] = *(int *)((int)piVar5 + -6);
            *(unsigned short *)(piVar5 + 4) = *(unsigned short *)((int)piVar5 + -2);
        }
    }
    piVar3[2] = param_2;
    piVar3[1] = 1;
    *piVar3 = 0;
    piVar3[3] = *param_3;
    *(short *)(piVar3 + 4) = (short)param_3[1];
    return piVar3;
}
