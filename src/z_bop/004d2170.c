/* SPDX-License-Identifier: MIT */
/* z_bop 004d2170 */
extern uint DAT_00b53575;
extern uint DAT_00b53579;
extern uint DAT_00b53571;
extern uint DAT_00acf571[];

uint __cdecl FUN_004d2170(uint param_1)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  uint *puVar6;
  
  if (DAT_00b53575 == param_1) {
    DAT_00b53575 = DAT_00b53575 + 1;
    DAT_00b53579 = DAT_00b53579 + -1;
    if (0x3ff < (int)DAT_00b53575) {
      DAT_00b53575 = 0;
    }
    return 1;
  }
  uVar3 = param_1 + 1;
  if (0x3ff < uVar3) {
    uVar3 = 0;
  }
  uVar4 = DAT_00b53575 + DAT_00b53579;
  if (0x3ff < uVar4) {
    uVar4 = uVar4 - 0x400;
  }
  while (uVar4 != uVar3) {
    uVar2 = param_1 + 1;
    puVar5 = DAT_00acf571 + uVar3 * 0x84;
    puVar6 = DAT_00acf571 + param_1 * 0x84;
    for (iVar1 = 0x84; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar6 = *puVar5;
      puVar5 = puVar5 + 1;
      puVar6 = puVar6 + 1;
    }
    if (0x3ff < uVar2) {
      uVar2 = 0;
    }
    uVar3 = uVar3 + 1;
    param_1 = uVar2;
    if (0x3ff < uVar3) {
      uVar3 = 0;
    }
  }
  DAT_00b53579 = DAT_00b53579 + -1;
  if (DAT_00b53571 == 0) {
    DAT_00b53571 = 0x3ff;
    return 0;
  }
  DAT_00b53571 = DAT_00b53571 + -1;
  return 0;
}
