/* SPDX-License-Identifier: MIT */
/* z_xvt 00405c50 */
extern unsigned short DAT_00732f92;
extern unsigned short DAT_0071b6b0;
extern unsigned short DAT_0070acc6;
extern unsigned short DAT_00732f94;
extern unsigned short _DAT_00732e80;
extern unsigned int DAT_00536920;
extern unsigned int DAT_00536924;

extern unsigned short __stdcall FUN_0048f940(void);

void __stdcall FUN_00405c50(void)
{
    DAT_00732f92 = 0;
    DAT_0071b6b0 = 0;
    DAT_0070acc6 = 0;
    DAT_00732f94 = 0;
    _DAT_00732e80 = FUN_0048f940();
    DAT_00732f92 = (_DAT_00732e80 != 0);
    DAT_0071b6b0 = 0;
    DAT_00536920 = 0;
    DAT_00536924 = 0xFFFFFFFF;
    return;
}
