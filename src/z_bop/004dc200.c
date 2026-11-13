/* SPDX-License-Identifier: MIT */
/* z_bop 004dc200 */
extern int DAT_00aa73a2;
extern unsigned char DAT_00aa739f;

unsigned char __stdcall FUN_004dc200(void)
{
    if (DAT_00aa73a2 != 0) {
        return 0;
    }
    return DAT_00aa739f;
}
