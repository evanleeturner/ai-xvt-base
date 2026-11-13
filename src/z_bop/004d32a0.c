/* SPDX-License-Identifier: MIT */
/* z_bop 004d32a0 */
extern unsigned DAT_00aa7fd3;
extern unsigned DAT_00aa7fd7;
extern unsigned DAT_00aa7fdf;

unsigned __stdcall FUN_004d32a0(void)
{
    if (DAT_00aa7fd3 == 0) {
        return 0;
    }
    if (DAT_00aa7fd7 == 0) {
        return 0;
    }
    return DAT_00aa7fdf;
}
