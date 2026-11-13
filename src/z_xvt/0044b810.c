/* SPDX-License-Identifier: MIT */
/* z_xvt 0044b810 */
extern ushort DAT_006d9d60;
extern byte DAT_0070ad9e[];
extern byte DAT_0070ad9f[];
extern byte DAT_0070ada2[];
extern byte DAT_0070ada0[];
extern byte DAT_0070ada1[];
extern byte DAT_0070ad98[];
extern byte DAT_0070ad9a[];
extern byte DAT_0070ad9c[];
extern byte DAT_0070ad99[];
extern byte DAT_0070ad9b[];

extern uint __cdecl FUN_0044b900(ushort, ushort, ushort, ushort, ushort);

void __cdecl FUN_0044b810(uint param_1)
{
  uint iVar2;
  uint iVar1;
  uint uVar3;
  
  iVar2 = (param_1 & 0xffff) * 0x52;
  iVar1 = iVar2 + (uint)DAT_006d9d60 * 0x566;
  uVar3 = FUN_0044b900((ushort)DAT_0070ad9e[iVar1], (ushort)DAT_0070ad9f[iVar1],
                       (ushort)DAT_0070ada2[iVar1], (ushort)DAT_0070ada0[iVar1],
                       (ushort)DAT_0070ada1[iVar1]);
  if ((short)uVar3 == -1) {
    iVar1 = iVar2 + (uint)DAT_006d9d60 * 0x566;
    FUN_0044b900((ushort)DAT_0070ad98[iVar1], (ushort)DAT_0070ad9a[iVar1],
                 (ushort)DAT_0070ad9c[iVar1], (ushort)DAT_0070ad99[iVar1],
                 (ushort)DAT_0070ad9b[iVar1]);
  }
  return;
}
