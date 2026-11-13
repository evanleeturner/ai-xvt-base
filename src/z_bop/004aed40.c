/* SPDX-License-Identifier: MIT */
/* z_bop 004aed40 */
extern unsigned int __cdecl FUN_004ac720(unsigned int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern int __cdecl FUN_004ae1a0(int param_1, int *param_2);
extern long long __cdecl __ftol(void);
extern void __stdcall FUN_004ac740(void);
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

unsigned int __cdecl FUN_004aed40(int param_1, int param_2)
{
    int *piVar1;
    int iVar2;
    unsigned int uVar3;
    long long lVar4;

    if (param_2 < 0) {
        return 0;
    }
    if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
        return 0;
    }
    piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) >> 0x10) << 16 | (unsigned short)DAT_009a7ed0[param_1]);
    if (*piVar1 != (int)piVar1) {
        FUN_00471ff0(piVar1);
    }
    if (*(int *)(**(int **)((int)piVar1 + 0xA) + 4) == 0x14) {
        param_2 = param_2 + 1;
    }
    if (*(int *)((int)piVar1 + 0x6) <= param_2) {
        param_2 = *(int *)((int)piVar1 + 0x6) - 1;
    }
    iVar2 = FUN_004ae1a0((*(int **)((int)piVar1 + 0xA))[param_2], piVar1);
    if (iVar2 == 0) {
        uVar3 = 0;
    }
    else if (*(int *)(iVar2 + 0x38) == 0) {
        lVar4 = __ftol();
        uVar3 = (unsigned int)lVar4;
    }
    else {
        lVar4 = __ftol();
        uVar3 = (unsigned int)lVar4;
    }
    FUN_004ac740();
    return uVar3;
}
