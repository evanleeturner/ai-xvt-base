/* SPDX-License-Identifier: MIT */
/* z_xvt 004f23b0 */
extern short DAT_00650406;

void __stdcall FUN_004f3500(void);
void __stdcall FUN_004f3680(void);

void __stdcall FUN_004f23b0(void)
{
  if (DAT_00650406 != 0) {
    FUN_004f3500();
    FUN_004f3680();
    return;
  }
  return;
}
