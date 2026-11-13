/* SPDX-License-Identifier: MIT */
/* z_xvt 0042c600 */
extern uint _DAT_005373d4;
extern uint _DAT_005373d0;
extern void __cdecl FUN_004310c0(uint *, uint);

void __cdecl FUN_0042c600(uint param_1)
{
  _DAT_005373d4 = param_1;
  _DAT_005373d0 = 0x15;
  FUN_004310c0(&_DAT_005373d0, 8);
  return;
}
