/* SPDX-License-Identifier: MIT */
/* z_xvt 00449850 */
extern void *FUN_0048f4f0(int param_1);
extern void FUN_00485fa0(void *param_1);
extern int FUN_00449140(int param_1);
extern void FUN_0048f510(void);
extern int DAT_006da850[];
extern unsigned int in_EAX;

unsigned int __cdecl FUN_00449850(int param_1, int param_2)
{
    int *piVar3;
    int iVar1;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar2;
    int val_at_6;

    piVar3 = (int *)FUN_0048f4f0(((unsigned int)in_EAX & 0xFFFF0000) | (DAT_006da850[param_1] & 0xFFFF));
    if (*piVar3 != (int)piVar3) {
        FUN_00485fa0(piVar3);
    }
    if (*(int *)(**(int **)((int)piVar3 + 10) + 4) == 0x14) {
        param_2 = param_2 + 1;
    }
    __builtin_memcpy(&val_at_6, (void *)((int)piVar3 + 6), 4);
    if (val_at_6 <= param_2) {
        param_2 = val_at_6 - 1;
    }
    iVar1 = (*(int **)((int)piVar3 + 10))[param_2];
    if (iVar1 != 0) {
        iVar4 = iVar1;
        if (*(int *)(iVar1 + 4) == 3) goto LAB_004498ce;
        iVar6 = 0;
        iVar5 = 0;
        if (0 < *(int *)(iVar1 + 8)) {
            do {
                iVar4 = *(int *)(*(int *)(iVar1 + 0xc) + iVar6);
                if ((iVar4 != 0) && (iVar4 = FUN_00449140(iVar4), iVar4 != 0)) goto LAB_004498ce;
                iVar6 = iVar6 + 4;
                iVar5 = iVar5 + 1;
            } while (iVar5 < *(int *)(iVar1 + 8));
        }
    }
    iVar4 = 0;
LAB_004498ce:
    uVar2 = *(unsigned int *)(iVar4 + 0x10);
    FUN_0048f510();
    return uVar2;
}
