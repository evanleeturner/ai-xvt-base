/* SPDX-License-Identifier: MIT */
/* z_bop 004b1d10 */
extern uint DAT_00528c80;
extern uint DAT_00528ca0;
extern uint *DAT_00664c80;

uint __cdecl FUN_004b1d10(uint *param_1, uint param_2)
{
  uint iVar1;
  uint *puVar2;
  
  if (DAT_00528c80 < param_2 + DAT_00528ca0) {
    return 0;
  }
  if (DAT_00664c80 != param_1) {
    puVar2 = DAT_00664c80;
    for (iVar1 = (param_2 & 0x7ffffff) << 3; iVar1 != 0; iVar1 = iVar1 - 1) {
      *puVar2 = *param_1;
      param_1 = param_1 + 1;
      puVar2 = puVar2 + 1;
    }
  }
  DAT_00528ca0 = DAT_00528ca0 + param_2;
  DAT_00664c80 = DAT_00664c80 + param_2 * 8;
  return 1;
}
