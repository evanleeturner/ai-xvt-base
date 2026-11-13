/* SPDX-License-Identifier: MIT */
/* z_bop 0045deb0 */
extern ushort DAT_009a73c0;
extern byte DAT_009d8cde[];
extern byte DAT_009d8cdf[];
extern byte DAT_009d8ce2[];
extern byte DAT_009d8ce0[];
extern byte DAT_009d8ce1[];
extern byte DAT_009d8cd8[];
extern byte DAT_009d8cda[];
extern byte DAT_009d8cdc[];
extern byte DAT_009d8cd9[];
extern byte DAT_009d8cdb[];

int __cdecl FUN_0045dfa0(ushort, ushort, ushort, ushort, ushort);

void __cdecl FUN_0045deb0(uint param_1)
{
  uint iVar1;
  uint iVar2;
  
  iVar1 = (param_1 & 0xffff) * 0x52;
  iVar2 = iVar1 + (uint)DAT_009a73c0 * 0x566;
  iVar2 = FUN_0045dfa0((ushort)(byte)DAT_009d8cde[iVar2],
                       (ushort)(byte)DAT_009d8cdf[iVar2],
                       (ushort)(byte)DAT_009d8ce2[iVar2],
                       (ushort)(byte)DAT_009d8ce0[iVar2],
                       (ushort)(byte)DAT_009d8ce1[(uint)DAT_009a73c0 * 0x566 + iVar1]);
  if ((short)iVar2 == -1) {
    iVar2 = iVar1 + (uint)DAT_009a73c0 * 0x566;
    FUN_0045dfa0((ushort)(byte)DAT_009d8cd8[iVar2],
                 (ushort)(byte)DAT_009d8cda[iVar2],
                 (ushort)(byte)DAT_009d8cdc[iVar2],
                 (ushort)(byte)DAT_009d8cd9[iVar2],
                 (ushort)(byte)DAT_009d8cdb[(uint)DAT_009a73c0 * 0x566 + iVar1]);
  }
  return;
}
