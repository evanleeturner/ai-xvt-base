/* SPDX-License-Identifier: MIT */
/* z_bop 004fa690 */
extern int DAT_00b6a5a6;
extern int DAT_00aa5ac0[];
extern int DAT_00b69e30;
extern int DAT_00aa6af4;
extern int DAT_00aa6af0;

void __cdecl FUN_004cef70(int param1, int *param2, int param3);

int __stdcall FUN_004fa690(void)
{
    int iVar1;
    int iVar2;
    int iVar3;
    
    iVar2 = DAT_00b6a5a6 * 8;
    iVar3 = 0;
    do {
        iVar1 = iVar2 + iVar3;
        iVar3 = iVar3 + 1;
        DAT_00aa5ac0[iVar1] = 0xffffffff;
    } while (iVar3 < 8);
    if (DAT_00b69e30 != 2) {
        DAT_00aa6af4 = DAT_00b6a5a6;
        DAT_00aa6af0 = 0x56;
        FUN_004cef70(0, &DAT_00aa6af0, 8);
    }
    return 1;
}
