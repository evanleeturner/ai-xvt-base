/* SPDX-License-Identifier: MIT */
/* z_xvt 0045b760 */
extern uint DAT_00525730;
extern uint DAT_005254f8;
extern uint DAT_00703c88;
extern uint DAT_00732a38;
extern uint DAT_006d4988;

void __cdecl FUN_0045b420(uint param_1);

void __cdecl FUN_0045b760(uint param_1)
{
  DAT_00525730 = 0;
  if ((DAT_005254f8 != 0) && (param_1 >= (uint)(DAT_00703c88 + DAT_00732a38))) {
    FUN_0045b420(param_1 * 0x23 + DAT_006d4988);
  }
  return;
}
