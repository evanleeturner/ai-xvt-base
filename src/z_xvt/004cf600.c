/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf600 */
extern int DAT_0064f070[];
extern int DAT_0064f430;

int __cdecl FUN_004cf600(int param_1, int param_2)
{
    int *piVar1;
    int iVar2;
    
    piVar1 = &DAT_0064f070[0];
    iVar2 = 0;
    do {
        if (*piVar1 == param_1) {
            DAT_0064f070[iVar2 * 6] = param_1;
            DAT_0064f070[iVar2 * 6 + 3] = param_2;
            break;
        }
        piVar1 += 6;
        iVar2++;
    } while (piVar1 < &DAT_0064f430);
    
    if (iVar2 == 0x28) {
        iVar2 = 0;
        piVar1 = &DAT_0064f070[0];
        while (*piVar1 != 0) {
            piVar1 += 6;
            iVar2++;
            if ((int *)0x64f42f < piVar1) {
                return 1;
            }
        }
        DAT_0064f070[iVar2 * 6] = param_1;
        DAT_0064f070[iVar2 * 6 + 1] = 1;
        DAT_0064f070[iVar2 * 6 + 2] = 0;
        DAT_0064f070[iVar2 * 6 + 3] = param_2;
        DAT_0064f070[iVar2 * 6 + 4] = 0;
    }
    return 1;
}
