/* SPDX-License-Identifier: MIT */
/* z_bop 004aef60 */
extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];

void *FUN_004ac720(int param_1);
void FUN_00471ff0(void *param_1);
int FUN_004ae1a0(int param_1, void *param_2);
void FUN_004ac740(void);

unsigned int __cdecl FUN_004aef60(int param_1, int param_2)
{
    int *piVar1;
    int iVar2;
    unsigned int uVar3;

    if (param_2 < 0) {
        return 0;
    }
    if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
        return 0;
    }
    piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) >> 16) << 16 | DAT_009a7ed0[param_1]);
    if ((int *)*piVar1 != piVar1) {
        FUN_00471ff0(piVar1);
    }
    if (*(int *)((char *)**(int **)((char *)piVar1 + 10) + 4) == 0x14) {
        param_2 = param_2 + 1;
    }
    if (*(int *)((char *)piVar1 + 6) <= param_2) {
        param_2 = *(int *)((char *)piVar1 + 6) - 1;
    }
    uVar3 = 0;
    iVar2 = FUN_004ae1a0((*(int **)((char *)piVar1 + 10))[param_2], piVar1);
    if (iVar2 != 0) {
        uVar3 = *(unsigned int *)(iVar2 + 4) & 1;
    }
    FUN_004ac740();
    return uVar3;
}
