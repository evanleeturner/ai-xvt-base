/* SPDX-License-Identifier: MIT */
/* z_xvt 00452d00 */
extern int DAT_00525e14;
extern int DAT_00525e10;

void __stdcall FUN_00465d20(void);

void __stdcall FUN_00452d00(void)
{
  if (DAT_00525e14 != 0) {
    FUN_00465d20();
    DAT_00525e14 = 0;
  }
  DAT_00525e10 = 0;
  return;
}
