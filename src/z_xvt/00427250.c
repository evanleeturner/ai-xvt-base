/* SPDX-License-Identifier: MIT */
/* z_xvt 00427250 */
extern int DAT_00536ed0;
extern int DAT_00536ecc;
extern int DAT_00536ec8;

void __stdcall FUN_00427250(void)
{
  if (DAT_00536ed0 != 0) {
    (*(void (__stdcall **)(int))(*(int *)DAT_00536ecc + 0x20))((int)DAT_00536ecc);
    DAT_00536ed0 = 0;
  }
  if (DAT_00536ecc != 0) {
    (*(void (__stdcall **)(int))(*(int *)DAT_00536ecc + 8))((int)DAT_00536ecc);
  }
  if (DAT_00536ec8 != 0) {
    (*(void (__stdcall **)(int))(*(int *)DAT_00536ec8 + 8))((int)DAT_00536ec8);
  }
}
