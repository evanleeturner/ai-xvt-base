/* SPDX-License-Identifier: MIT */
/* z_bop 004d5fc0 */
extern char DAT_00aa73b0;
extern char DAT_00aa73b1;

char __stdcall FUN_004d5fc0(void)
{
  char cVar1;
  
  cVar1 = DAT_00aa73b0 != 0;
  if (DAT_00aa73b1 != 0) {
    cVar1 = cVar1 + 1;
  }
  return cVar1;
}
