/* SPDX-License-Identifier: MIT */
/* z_bop 004dcb50 */
extern unsigned int DAT_00aa7a98;
extern unsigned int DAT_00aa7a9c;

unsigned int __stdcall FUN_004dcb50(void)
{
    if (DAT_00aa7a98 == DAT_00aa7a9c) {
        return 0;
    }
    DAT_00aa7a9c = DAT_00aa7a9c + 1;
    if (DAT_00aa7a9c == 0x400) {
        DAT_00aa7a9c = 0;
    }
    return 1;
}
