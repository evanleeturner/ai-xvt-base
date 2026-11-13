/* SPDX-License-Identifier: MIT */
/* z_xvt 004187e0 */
extern unsigned short DAT_00523c60[];

unsigned int __cdecl FUN_004187e0(unsigned short param_1, short param_2)
{
    unsigned short uVar1;
    unsigned int uVar2;

    uVar1 = (unsigned short)(param_1 & 0x8000);
    if (uVar1 != 0) {
        param_1 = (unsigned short)(0U - (unsigned int)param_1);
    }
    uVar2 = (unsigned int)DAT_00523c60[(((unsigned int)(int)param_2 + 0x4000U) & 0x7fc0) >> 6] *
            (unsigned int)param_1 + 0x8000U;
    if ((((unsigned int)(int)param_2 + 0x4000U) & 0x8000) != (unsigned int)uVar1) {
        uVar2 = 0U - uVar2;
    }
    return uVar2 >> 0x10;
}
