/* SPDX-License-Identifier: MIT */
/* z_bop 00417900 */
uint __cdecl FUN_00417900(uint *param_1, byte *param_2, int param_3)
{
  int iVar1;
  uint uVar2;
  int iVar3;
  byte *pbVar4;
  int local_4;
  
  iVar1 = param_3 / 0x7c;
  if (param_3 / 0x7c == 0) {
    iVar1 = param_3;
  }
  local_4 = 0x7d;
  do {
    uVar2 = 0;
    iVar3 = iVar1;
    pbVar4 = param_2;
    if (0 < iVar1) {
      do {
        param_2 = pbVar4;
        if (param_3 != 0) {
          param_2 = pbVar4 + 1;
          param_3 = param_3 - 1;
          uVar2 = (uVar2 + *pbVar4) * 2 | (uint)((int)(uVar2 + *pbVar4) < 0);
        }
        iVar3 = iVar3 - 1;
        pbVar4 = param_2;
      } while (iVar3 != 0);
    }
    *param_1 = uVar2;
    param_1 = param_1 + 1;
    local_4 = local_4 - 1;
  } while (local_4 != 0);
  return 0x7d;
}
