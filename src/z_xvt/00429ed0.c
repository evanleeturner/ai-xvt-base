/* SPDX-License-Identifier: MIT */
/* z_xvt 00429ed0 */
extern unsigned short DAT_006d9d60;
extern unsigned char DAT_0070ad98[];
extern unsigned char DAT_0070ad99[];
extern unsigned char DAT_0070ad9a[];
extern unsigned char DAT_0070ad9b[];
extern unsigned char DAT_0070ad9c[];
extern unsigned char DAT_0070ad9e[];
extern unsigned char DAT_0070ad9f[];
extern unsigned char DAT_0070ada0[];
extern unsigned char DAT_0070ada1[];
extern unsigned char DAT_0070ada2[];

unsigned int __cdecl FUN_0042a3f0(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short);

void __cdecl FUN_00429ed0(unsigned int param_1)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  
  iVar2 = (param_1 & 0xffff) * 0x52;
  iVar1 = iVar2 + (unsigned int)DAT_006d9d60 * 0x566;
  uVar3 = FUN_0042a3f0((unsigned short)DAT_0070ad9e[iVar1], (unsigned short)DAT_0070ad9f[iVar1],
                       (unsigned short)DAT_0070ada2[iVar1], (unsigned short)DAT_0070ada0[iVar1],
                       (unsigned short)DAT_0070ada1[iVar1]);
  if ((short)uVar3 == -1) {
    iVar1 = iVar2 + (unsigned int)DAT_006d9d60 * 0x566;
    FUN_0042a3f0((unsigned short)DAT_0070ad98[iVar1], (unsigned short)DAT_0070ad9a[iVar1],
                 (unsigned short)DAT_0070ad9c[iVar1], (unsigned short)DAT_0070ad99[iVar1],
                 (unsigned short)DAT_0070ad9b[iVar1]);
  }
  return;
}
