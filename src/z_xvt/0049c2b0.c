/* SPDX-License-Identifier: MIT */
/* z_xvt 0049c2b0 */
extern uint DAT_006d4988;
void __cdecl FUN_0049c2e0(uint param_1, uint param_2);

void __cdecl FUN_0049c2b0(uint param_1)
{
  FUN_0049c2e0(param_1 & 0xffff, (uint)*(byte *)((param_1 & 0xffff) * 0x23 + 3 + DAT_006d4988));
  return;
}
