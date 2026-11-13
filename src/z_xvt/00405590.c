/* SPDX-License-Identifier: MIT */
/* z_xvt 00405590 */
extern unsigned short DAT_0071ecf0;
extern unsigned short DAT_0071eae4;

void __stdcall FUN_00405590(void)
{
    unsigned short uVar1;

    uVar1 = DAT_0071ecf0;
    if (0x7fff < DAT_0071ecf0) {
        uVar1 = (unsigned short)(0u - (unsigned int)DAT_0071ecf0);
    }
    if ((short)uVar1 < 0x41) {
        DAT_0071ecf0 = 0;
    }

    uVar1 = DAT_0071eae4;
    if (0x7fff < DAT_0071eae4) {
        uVar1 = (unsigned short)(0u - (unsigned int)DAT_0071eae4);
    }
    if ((short)uVar1 < 0x19) {
        DAT_0071eae4 = 0;
    }
    return;
}
