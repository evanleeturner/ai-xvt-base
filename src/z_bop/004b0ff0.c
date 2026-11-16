/* SPDX-License-Identifier: MIT */
/* z_bop 004b0ff0 */
extern void **DAT_00664970;
extern int _DAT_00528cb8;

void __stdcall FUN_004079e0(void);

void __stdcall FUN_004b0ff0(void)
{
  if (DAT_00664970 != 0) {
    ((void (__stdcall *)(void *))((void **)*DAT_00664970)[2])(DAT_00664970);
  }
  FUN_004079e0();
  _DAT_00528cb8 = 0;
  return;
}
