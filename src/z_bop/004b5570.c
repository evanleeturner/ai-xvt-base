/* SPDX-License-Identifier: MIT */
/* z_bop 004b5570 */
extern int DAT_00aa7201;
int __cdecl FUN_004b7a70(char *param_1);
uint __cdecl FUN_004b55b0(int *param_1, int param_2, int param_3);

uint __cdecl FUN_004b5570(char *param_1, int param_2, int param_3)
{
    int iVar1;
    uint uVar2;
    
    iVar1 = FUN_004b7a70(param_1);
    if (iVar1 == -1) {
        return 0;
    }
    uVar2 = FUN_004b55b0(*(int **)(DAT_00aa7201 + 0x40 + iVar1 * 0x44), param_2, param_3);
    return uVar2;
}
