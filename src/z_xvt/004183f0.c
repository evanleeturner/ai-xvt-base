/* SPDX-License-Identifier: MIT */
/* z_xvt 004183f0 */
extern short DAT_00523c60[];

short __cdecl FUN_004183f0(unsigned short param_1)
{
    unsigned int iVar1;
    unsigned short uVar2;
    unsigned short uVar3;
    short sVar4;

    iVar1 = (unsigned int)((param_1 & 0x7fc0) >> 6);
    uVar2 = (unsigned short)((unsigned short)DAT_00523c60[iVar1 + 1] - (unsigned short)DAT_00523c60[iVar1]);
    uVar3 = uVar2;
    if ((short)uVar2 < 0) {
        uVar3 = (unsigned short)(0u - (unsigned int)uVar2);
    }
    sVar4 = (short)(((unsigned int)(unsigned short)(param_1 << 10) * (unsigned int)uVar3) >> 0x10);
    if ((short)uVar2 < 0) {
        sVar4 = (short)(0u - (unsigned int)(unsigned short)sVar4);
    }
    return (short)((unsigned int)(unsigned short)sVar4 + (unsigned int)(unsigned short)DAT_00523c60[iVar1]);
}
