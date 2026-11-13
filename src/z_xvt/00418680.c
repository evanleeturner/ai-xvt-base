/* SPDX-License-Identifier: MIT */
/* z_xvt 00418680 */
extern unsigned char DAT_00523c60[];

unsigned int __cdecl FUN_00418680(unsigned short param_1, unsigned short param_2)
{
    unsigned short uVar1;
    unsigned int uVar2;

    uVar1 = (unsigned short)(param_1 & 0x8000);
    if (uVar1 != 0) {
        param_1 = (unsigned short)(0u - (unsigned int)param_1);
    }
    uVar2 = (unsigned int)*(unsigned short *)(&DAT_00523c60[0] + (unsigned int)((unsigned short)(param_2 & 0x7fc0) >> 6) * 2) *
            (unsigned int)param_1 + 0x8000u;
    if ((unsigned short)(param_2 & 0x8000) != uVar1) {
        uVar2 = 0u - uVar2;
    }
    return uVar2 >> 0x10;
}
