/* SPDX-License-Identifier: MIT */
/* z_bop 004acac0 */
extern int DAT_005280f0;
extern int DAT_005280f4;
extern int DAT_005280f8;

short __stdcall FUN_004ac830(void);
int __stdcall FUN_004acaf0(void);

int __stdcall FUN_004acac0(void)
{
    if (DAT_005280f0 == 0) {
        short sVar1 = FUN_004ac830();
        DAT_005280f4 = (int)sVar1;
        DAT_005280f8 = FUN_004acaf0();
        DAT_005280f0 = 1;
    }
    return DAT_005280f8;
}
