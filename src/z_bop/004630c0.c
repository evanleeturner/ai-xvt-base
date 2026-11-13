/* SPDX-License-Identifier: MIT */
/* z_bop 004630c0 */
uint __cdecl FUN_004630c0(uint *param_1, uint param_2)
{
  uint uVar1;
  uint local_4;
  
  local_4 = 0;
  if (3 < param_2) {
    uVar1 = param_2 >> 2;
    param_2 = param_2 + uVar1 * -4;
    do {
      local_4 = (local_4 ^ *param_1) << 1 | (uint)((int)(local_4 ^ *param_1) < 0);
      param_1 = param_1 + 1;
      uVar1 = uVar1 - 1;
    } while (uVar1 != 0);
  }
  if (param_2 < 2) {
    if (param_2 == 1) {
      local_4 = local_4 ^ (byte)*param_1;
      local_4 = local_4 << 1 | (uint)((int)local_4 < 0);
    }
  }
  else {
    local_4 = local_4 ^ (ushort)*param_1;
    local_4 = local_4 << 1 | (uint)((int)local_4 < 0);
    if (param_2 == 3) {
      local_4 = local_4 ^ *(byte *)((int)param_1 + 2);
      return local_4 << 1 | (uint)((int)local_4 < 0);
    }
  }
  return local_4;
}
