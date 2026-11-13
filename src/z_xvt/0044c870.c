/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c870 */
extern ushort DAT_006d9d60;
extern byte DAT_0070ad9e[];
extern byte DAT_0070ad9f[];
extern byte DAT_0070ada0[];
extern byte DAT_0070ada1[];
extern byte DAT_0070ada2[];
extern byte DAT_0070ad98[];
extern byte DAT_0070ad99[];
extern byte DAT_0070ad9a[];
extern byte DAT_0070ad9b[];
extern byte DAT_0070ad9c[];

extern int FUN_0044c960(ushort, ushort, ushort, ushort, ushort);

void __cdecl FUN_0044c870(uint param_1)
{
  int iVar1;
  int iVar2;
  
  iVar1 = (param_1 & 0xffff) * 0x52;
  iVar2 = iVar1 + (uint)DAT_006d9d60 * 0x566;
  iVar2 = FUN_0044c960((ushort)DAT_0070ad9e[iVar2],
                       (ushort)DAT_0070ad9f[iVar2],
                       (ushort)DAT_0070ada2[iVar2],
                       (ushort)DAT_0070ada0[iVar2],
                       (ushort)DAT_0070ada1[iVar2]);
  if ((short)iVar2 == -1) {
    iVar2 = iVar1 + (uint)DAT_006d9d60 * 0x566;
    FUN_0044c960((ushort)DAT_0070ad98[iVar2],
                 (ushort)DAT_0070ad9a[iVar2],
                 (ushort)DAT_0070ad9c[iVar2],
                 (ushort)DAT_0070ad99[iVar2],
                 (ushort)DAT_0070ad9b[iVar2]);
  }
  return;
}
