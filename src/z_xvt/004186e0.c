/* SPDX-License-Identifier: MIT */
/* z_xvt 004186e0 */
extern unsigned short DAT_00523c60[];

int __cdecl FUN_004186e0(unsigned int param_1, unsigned short param_2)
{
    unsigned int iVar1;
    unsigned char bVar2;
    unsigned int uVar3;

    bVar2 = 0;
    if ((int)param_1 < 0) {
        param_1 = 0u - param_1;
        bVar2 = 0x80;
    }
    uVar3 = (unsigned int)DAT_00523c60[(unsigned short)((unsigned short)((unsigned short)(param_2 >> 5) & 0x3fe) >> 1)];
    iVar1 = (((param_1 & 0xffff) * uVar3 + 0x8000) >> 0x10) + (param_1 >> 0x10) * uVar3;
    if (((bVar2 ^ (unsigned char)(param_2 >> 8)) & 0x80) != 0) {
        iVar1 = 0u - iVar1;
    }
    return (int)iVar1;
}
