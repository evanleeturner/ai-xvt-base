/* SPDX-License-Identifier: MIT */
/* z_xvt 004539c0 */
long long __ftol(void);

extern int DAT_00703c6c;
extern int DAT_00703c70;
extern int DAT_00703c84;

void __stdcall FUN_004539c0(void)
{
  long long lVar1;
  
  lVar1 = __ftol();
  DAT_00703c6c = (int)(short)lVar1;
  lVar1 = __ftol();
  DAT_00703c70 = (int)(short)lVar1;
  lVar1 = __ftol();
  DAT_00703c84 = (int)(short)lVar1;
  return;
}
