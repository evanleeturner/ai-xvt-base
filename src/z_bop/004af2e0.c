/* SPDX-License-Identifier: MIT */
/* z_bop 004af2e0 */
extern int __cdecl FUN_004ac720(int);
extern void __cdecl FUN_00471ff0(int *);
extern int __cdecl FUN_004af150(int *, int, int);
extern long long __ftol(void);
extern void __stdcall FUN_004ac740(void);

extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

int __cdecl FUN_004af2e0(int param_1, int param_2, int param_3)
{
    int *piVar1;
    int iVar2;
    int uVar3;
    long long lVar4;
    int val_at_6;
    int *ptr_at_10;

    if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
        return 0;
    }
    piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) & 0xFFFF0000) | (unsigned short)DAT_009a7ed0[param_1]);
    if ((int *)*piVar1 != piVar1) {
        FUN_00471ff0(piVar1);
    }
    
    __builtin_memcpy(&val_at_6, (char *)piVar1 + 6, 4);
    __builtin_memcpy(&ptr_at_10, (char *)piVar1 + 10, 4);
    
    if (ptr_at_10[1] == 0x14) {
        param_2 = param_2 + 1;
    }
    if (val_at_6 <= param_2) {
        param_2 = val_at_6 - 1;
    }
    uVar3 = 0;
    iVar2 = FUN_004af150((int *)ptr_at_10[param_2], (int)piVar1, param_3);
    if (iVar2 != 0) {
        lVar4 = __ftol();
        uVar3 = (int)lVar4;
    }
    FUN_004ac740();
    return uVar3;
}
