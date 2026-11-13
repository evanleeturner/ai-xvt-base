/* SPDX-License-Identifier: MIT */
/* z_xvt 004dbec0 */
extern int DAT_007595d0;
extern int DAT_007595d4;

unsigned int __stdcall FUN_004dbec0(void)
{
    if (DAT_007595d0 == DAT_007595d4) {
        return 0;
    }
    DAT_007595d4 = (int)((unsigned int)DAT_007595d4 + 1u);
    if (DAT_007595d4 == 0x400) {
        DAT_007595d4 = 0;
    }
    return 1;
}
