/* SPDX-License-Identifier: MIT */
/* z_xvt 004054e0 */
extern unsigned short DAT_0071eae4;
extern unsigned short DAT_0071ecf0;
extern unsigned short DAT_007092b0;
extern unsigned short DAT_006db592;
extern unsigned short DAT_0071e534;
extern unsigned short DAT_0071b6b0;
extern unsigned short DAT_006d9d88;
extern unsigned short DAT_006d9d92;
extern unsigned short DAT_0070acc6;
extern unsigned short DAT_00732f92;
extern unsigned short DAT_0073254c;
extern unsigned short DAT_00732554;
extern unsigned short DAT_00732f94;

void __stdcall FUN_004054e0(void)
{
    DAT_0071eae4 = 0;
    DAT_0071ecf0 = 0;
    DAT_007092b0 = DAT_006db592;
    DAT_0071e534 = 0;
    if (DAT_0071b6b0 != 0) {
        DAT_0071ecf0 = (unsigned short)((unsigned int)DAT_006d9d88 << 7);
        DAT_0071eae4 = (unsigned short)((unsigned int)DAT_006d9d92 << 6);
        DAT_0071e534 = DAT_0070acc6;
    }
    if ((unsigned char)((unsigned char)DAT_00732f92 | 1) != 0) {
        if (DAT_0071ecf0 == 0) {
            DAT_0071ecf0 = (unsigned short)((unsigned int)DAT_0073254c * 0x78);
        }
        if (DAT_0071eae4 == 0) {
            DAT_0071eae4 = (unsigned short)((unsigned int)DAT_00732554 * 0x32);
        }
        DAT_0071e534 = DAT_0071e534 | DAT_00732f94;
    }
    return;
}
