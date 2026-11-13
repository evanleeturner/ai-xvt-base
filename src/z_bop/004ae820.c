/* SPDX-License-Identifier: MIT */
/* z_bop 004ae820 */
extern char DAT_00521f19[];
extern short DAT_009a7ed0[];

void __stdcall FUN_004ac740(void);
int * __cdecl FUN_004ac720(int);
void __cdecl FUN_00471ff0(int *);
int __cdecl FUN_004ae1a0(int, int);
long long __ftol(void);

int __cdecl FUN_004ae820(int param_1, int param_2)
{
    int *piVar1;
    int iVar2;
    int uVar3;
    long long lVar4;

    if (param_2 < 0) {
        return 0;
    }
    if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
        return 0;
    }
    piVar1 = (int *)FUN_004ac720((int)( ((unsigned int)(param_1 * 5) >> 0x10) | (unsigned short)DAT_009a7ed0[param_1] ));
    if (*piVar1 != (int)piVar1) {
        FUN_00471ff0(piVar1);
    }
    if (*(int *)(**(int **)((char *)piVar1 + 0xA) + 4) == 0x14) {
        param_2 = param_2 + 1;
    }
    {
        int val_6;
        __builtin_memcpy(&val_6, (char *)piVar1 + 6, 4);
        if (val_6 <= param_2) {
            param_2 = val_6 - 1;
        }
    }
    uVar3 = 0;
    iVar2 = FUN_004ae1a0(((int *)(*(int **)((char *)piVar1 + 0xA)))[param_2], (int)piVar1);
    if (iVar2 != 0) {
        lVar4 = __ftol();
        uVar3 = (int)lVar4;
    }
    FUN_004ac740();
    return uVar3;
}
