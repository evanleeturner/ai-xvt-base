/* SPDX-License-Identifier: MIT */
/* z_xvt 004063a0 */
extern short DAT_0071e522;
extern int _DAT_0074110c;

void __cdecl FUN_004063a0(int *param_1, int param_2, int param_3)
{
    do {
        int iVar1 = param_1[1];
        int iVar2 = param_1[0];
        int iVar3 = param_1[2];
        param_1 += 3;
        int *piVar4 = (int *)((DAT_0071e522 + iVar2) * 4 + param_3);
        do {
            iVar2 = *piVar4;
            piVar4++;
            *(char *)(param_2 + iVar2) = (char)iVar1;
            *(char *)(param_2 + 1 + iVar2) = 0x80;
            iVar3--;
        } while (iVar3 != 0);
        _DAT_0074110c--;
    } while (_DAT_0074110c != 0);
}
