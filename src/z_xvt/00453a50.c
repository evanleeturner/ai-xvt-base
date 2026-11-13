/* SPDX-License-Identifier: MIT */
/* z_xvt 00453a50 */
long long __ftol(void);

short DAT_005f0e30;
short DAT_005f0e32;
short DAT_005f0e34;

void __stdcall FUN_00453a50(void)
{
  long long lVar1;
  
  lVar1 = __ftol();
  DAT_005f0e32 = (short)lVar1;
  lVar1 = __ftol();
  DAT_005f0e30 = (short)lVar1;
  lVar1 = __ftol();
  DAT_005f0e34 = (short)lVar1;
  return;
}
