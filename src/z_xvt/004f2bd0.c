/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2bd0 */
extern unsigned short DAT_00650478;
extern unsigned short DAT_0065047a;
extern unsigned short DAT_0065047c;
extern unsigned short DAT_0065047e;
extern unsigned short DAT_00650480;
extern unsigned short DAT_00650482;
extern unsigned short DAT_00650484;

void __stdcall FUN_004f2c10(void);

void __stdcall FUN_004f2bd0(void)
{
    DAT_00650478 = 0xc8;
    DAT_0065047e = 0x2;
    DAT_00650482 = 0x270f;
    DAT_00650484 = 0x22;
    DAT_0065047a = 0x0;
    DAT_0065047c = 0x0;
    DAT_00650480 = 0x0;
    FUN_004f2c10();
    return;
}
