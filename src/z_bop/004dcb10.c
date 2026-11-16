/* SPDX-License-Identifier: MIT */
/* z_bop 004dcb10 */
extern uint DAT_00aa7a98;
extern uint DAT_00aa7a9c;
extern unsigned char DAT_00aa7698[];

uint __stdcall FUN_004dcb10(void)
{
    if (DAT_00aa7a98 == DAT_00aa7a9c) {
        return DAT_00aa7a9c & 0xffffff00;
    }
    return (DAT_00aa7a9c & 0xffffff00) | DAT_00aa7698[DAT_00aa7a9c];
}
