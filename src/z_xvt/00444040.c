/* SPDX-License-Identifier: MIT */
/* z_xvt 00444040 */
extern uint DAT_006d4988;
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0072f508[];

bool __cdecl FUN_00444040(uint param_1, uint param_2)
{
    uint idx1 = ((param_1 & 0xffff) * 0x23) + 0x16 + DAT_006d4988;
    uint val1 = DAT_0070ad48[idx1];
    uint val2 = DAT_0070ad48[val1 * 0x566];
    if (val2 == param_2) {
        return 0;
    }
    return DAT_0072f508[(param_2 * 0x1e5) + val2] == 0;
}
