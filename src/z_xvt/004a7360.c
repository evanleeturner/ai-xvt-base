/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7360 */
extern int DAT_0052ae28;
extern int DAT_0052ae4c;
extern int DAT_0052ae18;
extern int DAT_0052ae3c;

unsigned char __stdcall FUN_004d5500(void);
unsigned char __stdcall FUN_00461da0(void);

unsigned char __stdcall FUN_004a7360(void)
{
    if (DAT_0052ae28 != 0) {
        return FUN_004d5500();
    }
    if (DAT_0052ae4c != 0 && DAT_0052ae18 != 0) {
        return FUN_00461da0();
    }
    return DAT_0052ae3c == 0x22b;
}
