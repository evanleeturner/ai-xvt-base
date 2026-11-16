/* SPDX-License-Identifier: MIT */
/* z_xvt 004498f0 */
extern short DAT_006da850[];
extern int __cdecl FUN_0048f4f0(int param_1);
extern void __cdecl FUN_00485fa0(int *param_1);
extern int __cdecl FUN_00449140(int param_1);
extern long long __cdecl __ftol(void);
extern void __cdecl FUN_0048f510(int param_1);

int __cdecl FUN_004498f0(int param_1, int param_2)
{
    int *piVar2;
    int iVar1;
    int iVar3;
    int iVar4;
    int iVar5;
    long long lVar6;
    int field_6;
    int ptr_at_10;
    int val_at_ptr10_plus_4;
    int arr_val;

    piVar2 = (int *)FUN_0048f4f0((int)(unsigned short)DAT_006da850[param_1]);
    if ((int *)*piVar2 != piVar2) {
        FUN_00485fa0(piVar2);
    }
    __builtin_memcpy(&ptr_at_10, (char *)piVar2 + 10, 4);
    __builtin_memcpy(&val_at_ptr10_plus_4, (char *)ptr_at_10 + 4, 4);
    if (val_at_ptr10_plus_4 == 0x14) {
        param_2 = param_2 + 1;
    }
    __builtin_memcpy(&field_6, (char *)piVar2 + 6, 4);
    if (field_6 <= param_2) {
        param_2 = field_6 - 1;
    }
    __builtin_memcpy(&arr_val, (char *)piVar2 + 10 + param_2 * 4, 4);
    iVar1 = arr_val;
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
    FUN_0048f510((int)lVar6);
    return (int)lVar6;
}
