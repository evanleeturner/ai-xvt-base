/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a1f0 */
extern int __cdecl FUN_0048f4f0(int);
extern void __cdecl FUN_00485fa0(int *);
extern int __cdecl FUN_00449670(int, int);
extern void __cdecl FUN_0048f510(void);
extern short DAT_006da850[];

int __cdecl FUN_0044a1f0(int param_1, int param_2)
{
    int iVar1;
    int iVar4;
    int iVar5;
    int iVar6;
    int *piVar3;
    int field_6;
    int uVar2;

    if (param_2 < 0) {
        return 0;
    }

    piVar3 = (int *)FUN_0048f4f0(((unsigned int)(short)((unsigned int)param_1 >> 16) << 16) | (unsigned int)(short)DAT_006da850[param_1]);

    if (*piVar3 != (int)piVar3) {
        FUN_00485fa0(piVar3);
    }

    if (*(int *)(*(int **)((char *)piVar3 + 10) + 4) == 0x14) {
        param_2 = param_2 + 1;
    }

    __builtin_memcpy(&field_6, (char *)piVar3 + 6, 4);
    if (field_6 <= param_2) {
        param_2 = field_6 - 1;
    }

    iVar1 = (*(int **)((char *)piVar3 + 10))[param_2];

    if (iVar1 != 0) {
        if (*(int *)(iVar1 + 4) == 0x19) {
            iVar4 = *(int *)(iVar1 + 0x14);
            goto LAB_0044a27f;
        }

        iVar6 = 0;
        iVar5 = 0;
        if (0 < *(int *)(iVar1 + 8)) {
            do {
                iVar4 = *(int *)(*(int *)(iVar1 + 0xc) + iVar6);
                if (iVar4 != 0) {
                    iVar4 = FUN_00449670(iVar4, (int)piVar3);
                    if (iVar4 != 0) goto LAB_0044a27f;
                }
                iVar6 = iVar6 + 4;
                iVar5 = iVar5 + 1;
            } while (iVar5 < *(int *)(iVar1 + 8));
        }
    }
    iVar4 = 0;

LAB_0044a27f:
    uVar2 = *(int *)(iVar4 + 0x38);
    FUN_0048f510();
    return uVar2;
}
