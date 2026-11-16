/* SPDX-License-Identifier: MIT */
/* z_xvt 00433450 */
extern undefined4 DAT_005251b0;
extern undefined4 DAT_005251b4;
extern undefined4 DAT_005251ac;
extern undefined4 DAT_00539a08[];

undefined4 __cdecl FUN_00433450(uint param_1)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  
  if (param_1 == DAT_005251b0) {
    DAT_005251b0 = DAT_005251b0 + 1;
    DAT_005251b4 = DAT_005251b4 + -1;
    if (0x3ff < (int)DAT_005251b0) {
      DAT_005251b0 = 0;
    }
    return 1;
  }
  uVar2 = param_1 + 1;
  if (0x3ff < uVar2) {
    uVar2 = 0;
  }
  uVar3 = DAT_005251b4 + DAT_005251b0;
  if (0x3ff < uVar3) {
    uVar3 = uVar3 - 0x400;
  }
  while (uVar3 != uVar2) {
    uVar4 = param_1 + 1;
    puVar5 = (undefined4 *)&DAT_00539a08 + uVar2 * 0x84;
    puVar6 = (undefined4 *)&DAT_00539a08 + param_1 * 0x84;
    for (iVar1 = 0x84; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar6 = *puVar5;
      puVar5 = puVar5 + 1;
      puVar6 = puVar6 + 1;
    }
    if (0x3ff < uVar4) {
      uVar4 = 0;
    }
    uVar2 = uVar2 + 1;
    param_1 = uVar4;
    if (0x3ff < uVar2) {
      uVar2 = 0;
    }
  }
  DAT_005251b4 = DAT_005251b4 + -1;
  if (DAT_005251ac == 0) {
    DAT_005251ac = 0x3ff;
    return 0;
  }
  DAT_005251ac = DAT_005251ac + -1;
  return 0;
}
