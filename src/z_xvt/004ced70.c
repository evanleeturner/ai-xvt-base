/* SPDX-License-Identifier: MIT */
/* z_xvt 004ced70 */
extern int DAT_0064f070;
extern undefined4 DAT_0064f074;

undefined4 __cdecl FUN_004ced70(int param_1, undefined4 param_2)
{
    int iVar1 = 0;
    int *piVar2 = &DAT_0064f070;
    
    while (*piVar2 != param_1 && *piVar2 != 0) {
        piVar2 += 6;
        iVar1++;
        if (piVar2 >= (int *)0x64f430) {
            return 1;
        }
    }
    
    (&DAT_0064f070)[iVar1 * 6] = param_1;
    (&DAT_0064f074)[iVar1 * 6] = param_2;
    *(undefined4 *)(iVar1 * 0x18 + 0x64f084) = 1;
    return 1;
}
