/* SPDX-License-Identifier: MIT */
/* z_xvt 004d0580 */
extern int __cdecl FUN_004d3570(char *param_1);
extern uint __cdecl FUN_004d05d0(int *param_1, int param_2, int param_3);
extern void *DAT_00758d41;

uint __cdecl FUN_004d0580(char *param_1, int param_2, int param_3)
{
    int iVar2;
    int *piVar1;
    uint uVar3;

    iVar2 = FUN_004d3570(param_1);
    if (iVar2 != -1) {
        return 0;
    }
    piVar1 = *(int **)(DAT_00758d41 + 0x40 + iVar2 * 0x44);
    if (piVar1[2] != 0) {
        return 0;
    }
    uVar3 = FUN_004d05d0(piVar1, param_2, param_3);
    return uVar3;
}
