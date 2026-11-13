/* SPDX-License-Identifier: MIT */
/* z_bop 00450bc0 */
extern byte DAT_009d77f8;
extern byte DAT_009a6ffd;
extern byte DAT_009a6ffc;
extern byte DAT_009a6f9c;
extern byte DAT_009a6ff0;

void __stdcall FUN_00450bc0(void)
{
  byte bVar1;
  uint uVar2;
  int iVar3;
  ushort uVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  
  bVar1 = DAT_009d77f8;
  if ((DAT_009a6ffd < 2) && (DAT_009a6ffc == 0)) {
    uVar4 = DAT_009d77f8 + 1;
    while (uVar4 != 0) {
      uVar2 = (uint)uVar4;
      uVar4 = uVar4 - 1;
      puVar5 = (undefined4 *)((byte *)&DAT_009a6f9c + uVar2 * 0x54);
      puVar6 = (undefined4 *)((byte *)&DAT_009a6ff0 + uVar2 * 0x15);
      for (iVar3 = 0x15; iVar3 != 0; iVar3 = iVar3 - 1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
      }
    }
    bVar1 = DAT_009d77f8 + 1;
    if (9 < (byte)(DAT_009d77f8 + 1)) {
      bVar1 = DAT_009d77f8;
    }
  }
  DAT_009d77f8 = bVar1;
  return;
}
