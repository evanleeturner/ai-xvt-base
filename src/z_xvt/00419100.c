/* SPDX-License-Identifier: MIT */
/* z_xvt 00419100 */
extern unsigned short DAT_0071ed04;
extern unsigned short DAT_0071ed02;

unsigned int __cdecl FUN_00419000(unsigned int param_1, unsigned int param_2, unsigned short *param_3, unsigned short *param_4);

unsigned int __cdecl FUN_00419100(unsigned int param_1, unsigned int param_2)
{
    unsigned short local_4;
    unsigned short local_2;
    unsigned int extraout;
    _Bool bVar1;

    bVar1 = (int)param_1 < 0;
    if (bVar1) {
        param_1 = 0u - param_1;
    }
    DAT_0071ed04 = (unsigned short)bVar1;
    bVar1 = (int)param_2 < 0;
    if (bVar1) {
        param_2 = 0u - param_2;
    }
    DAT_0071ed02 = (unsigned short)bVar1;
    extraout = FUN_00419000(param_2, param_1, &local_2, &local_4);
    if (DAT_0071ed04 != 0) {
        local_2 = (unsigned short)(0u - (unsigned int)local_2);
    }
    if (DAT_0071ed02 != 0) {
        local_2 = (unsigned short)(0x8000u - (unsigned int)local_2);
    }
    return (extraout & 0xFFFF0000u) | (unsigned int)local_2;
}
