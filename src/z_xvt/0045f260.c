/* SPDX-License-Identifier: MIT */
/* z_xvt 0045f260 */
extern unsigned int DAT_005f10e0;
extern unsigned int DAT_005f10d4;
extern unsigned int DAT_005279f4;
extern float _DAT_005f10e4;
extern unsigned int DAT_006572b8;
extern unsigned int DAT_00525520;
extern unsigned int _DAT_006db7bc;
extern float _DAT_005f10b4;
extern unsigned int DAT_006572d4;
extern unsigned int DAT_00525528;
extern unsigned int _DAT_006da4fc;
extern unsigned int DAT_005f10d0;
extern unsigned int DAT_006a8826;
extern unsigned int DAT_005f0fa8;
extern unsigned int DAT_0052f1c4;
extern unsigned int DAT_005f10b0;
extern unsigned int DAT_006a8820;
extern unsigned int DAT_005f10c8;

void __stdcall FUN_0045f260(void)
{
    unsigned int uVar1;
    
    DAT_005f10e0 = 0;
    DAT_005f10d4 = 0;
    DAT_005279f4 = 1;
    _DAT_005f10e4 = (float)(((unsigned int)(DAT_006572b8 - DAT_00525520) >> 1) + _DAT_006db7bc);
    _DAT_005f10b4 = (float)(((unsigned int)(DAT_006572d4 - DAT_00525528) >> 1) + _DAT_006da4fc);
    DAT_005f10d0 = (DAT_006a8826 * 3U & 0x1fffffffU) >> 4;
    DAT_005f0fa8 = (unsigned int)(DAT_006a8826 * 0x18U) / 0x14U >> 2;
    if (*(unsigned int *)(DAT_0052f1c4 + 0x18) < DAT_005f10d0) {
        DAT_005f10d0 = *(unsigned int *)(DAT_0052f1c4 + 0x18);
    }
    if (0x100 < (int)DAT_005f10d0) {
        DAT_005f10d0 = 0x100;
    }
    if (0x100 < DAT_005f0fa8) {
        DAT_005f0fa8 = 0x100;
    }
    uVar1 = (*(int *)(DAT_0052f1c4 + 0x14) + DAT_005f10d0 * -0x40) / 0x18;
    if (uVar1 < DAT_005f0fa8) {
        DAT_005f0fa8 = uVar1;
    }
    DAT_005f10b0 = DAT_006a8820;
    DAT_005f10c8 = DAT_006a8820 + (DAT_006a8826 / 2) * 0x18;
    return;
}
