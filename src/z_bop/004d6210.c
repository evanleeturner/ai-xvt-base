/* SPDX-License-Identifier: MIT */
/* z_bop 004d6210 */
extern unsigned char DAT_00aa73b0[];
extern unsigned char DAT_00aa73f7[];

unsigned char __cdecl FUN_004d6210(int param_1)
{
    if (param_1 >= 2) {
        return 0;
    }
    if (DAT_00aa73b0[param_1] != 0) {
        return 0;
    }
    return DAT_00aa73f7[param_1 * 0x20];
}
