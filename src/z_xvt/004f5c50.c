/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5c50 */
extern int DAT_0081b850;
extern int DAT_00743860[8];

int __stdcall FUN_004cde00(void);

unsigned char __stdcall FUN_004f5c50(void)
{
    int iVar1;
    int *piVar2;
    int iVar3;

    iVar3 = 0;
    if (DAT_0081b850 == 2) {
        return 0;
    }
    iVar1 = FUN_004cde00();
    piVar2 = DAT_00743860;
    do {
        if (*piVar2 != 0) {
            iVar3 = iVar3 + 1;
        }
        piVar2 = piVar2 + 1;
    } while (piVar2 < DAT_00743860 + 8);
    return (unsigned char)(iVar3 == iVar1);
}
