/* SPDX-License-Identifier: MIT */
/* z_xvt 00418840 */
extern unsigned short DAT_00523c60[];

int __cdecl FUN_00418840(unsigned int param_1, short param_2)
{
    unsigned int iVar1;
    unsigned char bVar2;
    unsigned short uVar3;
    unsigned int uVar4;

    bVar2 = 0;
    if ((int)param_1 < 0) {
        param_1 = 0u - param_1;
        bVar2 = 0x80;
    }
    uVar3 = (unsigned short)((unsigned short)param_2 + 0x4000u);
    uVar4 = (unsigned int)DAT_00523c60[(unsigned int)(((unsigned int)(uVar3 >> 5) & 0x3fe) >> 1)];
    iVar1 = (((param_1 & 0xffff) * uVar4 + 0x8000) >> 0x10) +
            (param_1 >> 0x10) * uVar4;
    if (((bVar2 ^ (unsigned char)(uVar3 >> 8)) & 0x80) != 0) {
        iVar1 = 0u - iVar1;
    }
    return (int)iVar1;
}
