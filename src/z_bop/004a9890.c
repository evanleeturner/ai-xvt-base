/* SPDX-License-Identifier: MIT */
/* z_bop 004a9890 */
extern undefined1 DAT_009a1ed0;
extern undefined1 DAT_009a1ed1;

undefined4 __cdecl FUN_004a9890(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  ushort uVar4;
  int bVar5;
  int bVar6;
  undefined1 *local_8;
  
  local_8 = &DAT_009a1ed0;
  bVar5 = param_1 < 0;
  bVar6 = 0;
  if (bVar5) {
    param_1 = -param_1;
    DAT_009a1ed0 = 0x2d;
    local_8 = &DAT_009a1ed1;
    bVar6 = param_1 < 0;
  }
  if (param_1 == 0) {
    uVar4 = 1;
    *local_8 = 0x30;
  }
  else {
    uVar4 = 0;
    iVar1 = param_1;
    if (!bVar6) {
      do {
        uVar4 = uVar4 + 1;
        iVar1 = iVar1 / 10;
      } while (0 < iVar1);
    }
    if (uVar4 != 0) {
      iVar1 = 0;
      do {
        iVar2 = param_1 % 10;
        param_1 = param_1 / 10;
        iVar3 = iVar1 + 1;
        local_8[(uint)uVar4 + (-1 - iVar1)] = (char)iVar2 + '0';
        iVar1 = iVar3;
      } while (iVar3 < (int)(uint)uVar4);
    }
  }
  local_8[uVar4] = 0;
  if (bVar5) {
    uVar4 = uVar4 + 1;
  }
  return (undefined4)(local_8 + uVar4);
}
