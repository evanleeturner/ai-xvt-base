/* SPDX-License-Identifier: MIT */
/* z_bop 00450c30 */
extern uchar DAT_009d77f8;
extern undefined4 DAT_009a7044[];
extern undefined4 DAT_009a6ff0[];

void __stdcall FUN_00450c30(void)
{
  int iVar1;
  ushort uVar2;
  ushort uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  
  if (DAT_009d77f8 != 0) {
    uVar2 = 0;
    do {
      uVar3 = uVar2 + 1;
      puVar4 = DAT_009a7044 + (uint)uVar2 * 0x15;
      puVar5 = DAT_009a6ff0 + (uint)uVar2 * 0x15;
      for (iVar1 = 0x15; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
      }
      uVar2 = uVar3;
    } while (uVar3 < (ushort)DAT_009d77f8);
  }
  DAT_009d77f8 = DAT_009d77f8 - 1;
  return;
}
