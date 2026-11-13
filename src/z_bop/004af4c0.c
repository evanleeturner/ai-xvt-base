/* SPDX-License-Identifier: MIT */
/* z_bop 004af4c0 */
void * __cdecl FUN_004ac720(int param_1);
void __cdecl FUN_00471ff0(int * param_1);
int __cdecl FUN_004af150(unsigned int * param_1, int param_2, int param_3);
long long __cdecl __ftol(void);
void __stdcall FUN_004ac740(void);

extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];

void __cdecl FUN_004af4c0(int param_1, int param_2, int param_3, unsigned int * param_4, unsigned int * param_5, int * param_6, unsigned int * param_7)
{
    int * piVar1;
    int iVar2;
    long long lVar3;

    if ((DAT_00521f19[param_1 * 0x1a] & 1) != 0) {
        int concat_val = ((int)(short)((unsigned int)(param_1 * 5) >> 0x10) << 16) | (unsigned short)DAT_009a7ed0[param_1];
        piVar1 = (int *)FUN_004ac720(concat_val);
        if ((int *)*piVar1 != piVar1) {
            FUN_00471ff0(piVar1);
        }
        if (*(int *)(**(int **)((int)piVar1 + 10) + 4) == 0x14) {
            param_2 = param_2 + 1;
        }
        if (*(int *)((int)piVar1 + 6) <= param_2) {
            param_2 = *(int *)((int)piVar1 + 6) - 1;
        }
        iVar2 = FUN_004af150((unsigned int *)(*(int **)((int)piVar1 + 10))[param_2], (int)piVar1, param_3);
        if (iVar2 == 0) {
            *param_4 = 0;
            *param_5 = 0;
            *param_6 = 0;
            *param_7 = 0;
        }
        else {
            *param_4 = **(unsigned int **)(iVar2 + 0x14);
            lVar3 = __ftol();
            *param_5 = (unsigned int)lVar3;
            lVar3 = __ftol();
            *param_6 = -(int)lVar3;
            lVar3 = __ftol();
            *param_7 = (unsigned int)lVar3;
        }
        FUN_004ac740();
    }
    return;
}
