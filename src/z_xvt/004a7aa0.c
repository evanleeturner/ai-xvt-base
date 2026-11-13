/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7aa0 */
extern unsigned char DAT_0052687b[];

unsigned int __cdecl FUN_004a7aa0(unsigned int param_1)
{
    unsigned int uVar1;
    unsigned int uVar2;

    uVar1 = param_1 & 0xffff;
    uVar2 = ((uVar1 * 0xb) >> 8) & 0xffffff;
    return ((uVar2 << 8) | (unsigned int)DAT_0052687b[uVar1 * 0x16]) & 0xffff00ff;
}
