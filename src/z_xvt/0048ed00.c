/* SPDX-License-Identifier: MIT */
/* z_xvt 0048ed00 */
extern unsigned int DAT_006da538;
extern unsigned int DAT_006db594;
extern short DAT_00732e8c;
extern short DAT_00732a34;
extern short DAT_006d9980;
extern short DAT_0070acc0;

void __cdecl FUN_0048ed00(short param_1, short param_2, short param_3, short param_4)
{
    if (param_2 < 0) {
        param_2 = 0;
    }
    if (param_1 < 0) {
        param_1 = 0;
    }
    if (DAT_006da538 < (unsigned int)(int)param_3) {
        param_3 = (short)DAT_006da538;
    }
    if (DAT_006db594 < (unsigned int)(int)param_4) {
        param_4 = (short)DAT_006db594;
    }
    DAT_00732e8c = param_2;
    DAT_00732a34 = param_4;
    DAT_006d9980 = param_1;
    DAT_0070acc0 = param_3;
    return;
}
