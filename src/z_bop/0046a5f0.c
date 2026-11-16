/* SPDX-License-Identifier: MIT */
/* z_bop 0046a5f0 */
extern unsigned short DAT_00524520[];

unsigned int __cdecl FUN_0046a5f0(unsigned short param_1, unsigned short param_2)
{
    unsigned short uVar1;
    unsigned int uVar2;

    uVar1 = param_1 & 0x8000;
    if (uVar1 != 0) {
        param_1 = -param_1;
    }
    uVar2 = (unsigned int)DAT_00524520[(param_2 & 0x7fc0) >> 6] * (unsigned int)param_1 + 0x8000;
    if ((param_2 & 0x8000) != uVar1) {
        uVar2 = -uVar2;
    }
    return uVar2 >> 0x10;
}
