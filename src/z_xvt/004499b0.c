/* SPDX-License-Identifier: MIT */
/* z_xvt 004499b0 */
extern short DAT_006da850[];
extern int __cdecl FUN_0048f4f0(int param_1);
extern void __cdecl FUN_00485fa0(int *param_1);
extern int __cdecl FUN_00449140(int param_1);
extern long long __cdecl __ftol(void);
extern void __cdecl FUN_0048f510(int param_1);

int __cdecl FUN_004499b0(int param_1, int param_2)
{
    int *piVar2;
    int iVar1;
    int iVar3;
    int iVar4;
    int iVar5;
    long long lVar6;
    int in_EAX = 0;
    int pTable;
    int val_6;

    piVar2 = (int *)FUN_0048f4f0((in_EAX & 0xFFFF0000) | (unsigned short)DAT_006da850[param_1]);
    if ((int *)*piVar2 != piVar2) {
        FUN_00485fa0(piVar2);
    }
    __builtin_memcpy(&pTable, (char *)piVar2 + 10, 4);
    if (*(int *)(pTable + 4) == 0x14) {
        param_2 = param_2 + 1;
    }
    __builtin_memcpy(&val_6, (char *)piVar2 + 6, 4);
    if (val_6 <= param_2) {
        param_2 = val_6 - 1;
    }
    __builtin_memcpy(&pTable, (char *)piVar2 + 10, 4);
    iVar1 = *(int *)(pTable + param_2 * 4);
    if ((iVar1 != 0) && (*(int *)(iVar1 + 4) != 3)) {
        iVar5 = 0;
        iVar4 = 0;
        if (0 < *(int *)(iVar1 + 8)) {
            do {
                iVar3 = *(int *)(*(int *)(iVar1 + 0xc) + iVar5);
                if ((iVar3 != 0) && (iVar3 = FUN_00449140(iVar3), iVar3 != 0)) break;
                iVar5 = iVar5 + 4;
                iVar4 = iVar4 + 1;
            } while (iVar4 < *(int *)(iVar1 + 8));
        }
    }
    lVar6 = __ftol();
    FUN_0048f510(0);
    return (int)lVar6;
}
