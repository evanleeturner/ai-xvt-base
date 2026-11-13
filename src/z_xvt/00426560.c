/* SPDX-License-Identifier: MIT */
/* z_xvt 00426560 */
extern unsigned int DAT_0052551c;
extern unsigned int DAT_0070acc8;
extern unsigned int DAT_00525518;
extern short DAT_007091ae;
extern short DAT_006ffbf2;
extern short DAT_00732e44;
extern unsigned int _DAT_0071f2e4;
extern short DAT_006d4984;
extern short DAT_00703be4;
extern short DAT_006d4a42;
extern unsigned int _DAT_006da4fc;
extern int _DAT_006db7bc;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_00426560(uint param_1, uint param_2, undefined4 param_3, uint param_4)
{
    if (DAT_0052551c == 0xa0) {
        param_1 = param_1 >> 1;
        param_2 = param_2 >> 1;
        param_4 = param_4 + DAT_0070acc8 * 0x78u + 0xa0u;
    }
    DAT_007091ae = (short)param_1;
    DAT_006ffbf2 = (short)((unsigned short)param_1 - 1u);
    DAT_00732e44 = (short)(param_1 >> 1);
    _DAT_0071f2e4 = param_4;
    DAT_006d4984 = (short)param_2;
    DAT_00703be4 = (short)((unsigned short)param_2 - 1u);
    DAT_006d4a42 = (short)(param_2 >> 1);

    if (DAT_0070acc8 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    _DAT_006da4fc = param_4 / DAT_0070acc8;

    if (DAT_00525518 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    _DAT_006db7bc = (int)(((unsigned long long)param_4 % (unsigned long long)DAT_0070acc8)
                          / (unsigned long long)DAT_00525518);
    return;
}
