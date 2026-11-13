/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b340 */
extern int DAT_005250d4;
extern int DAT_0070aaf0[];
extern char DAT_0071b740[];

int __cdecl FUN_00432ca0(void);
int __cdecl FUN_00432c10(int);
int * __cdecl FUN_00432cc0(int);

void __stdcall FUN_0042b340(void)
{
    int iVar1;

    if (DAT_005250d4 == 0) {
        DAT_005250d4 = FUN_00432ca0();
    }
    iVar1 = FUN_00432c10(DAT_005250d4);
    if (DAT_0070aaf0[iVar1] != 0) {
        DAT_005250d4 = FUN_00432ca0();
        iVar1 = FUN_00432ca0();
        iVar1 = FUN_00432c10(iVar1);
    }
    if (DAT_0071b740[iVar1 * 0x5bd] == '\0') {
        DAT_005250d4 = FUN_00432ca0();
        iVar1 = FUN_00432ca0();
        iVar1 = FUN_00432c10(iVar1);
    }
    FUN_00432cc0(iVar1);
    return;
}
