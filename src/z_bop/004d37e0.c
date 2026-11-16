/* SPDX-License-Identifier: MIT */
/* z_bop 004d37e0 */
extern void **DAT_00aa7b32;
extern void **DAT_00aa7b36;
extern void **DAT_00aa7b3a;
extern void *DAT_00b69cb0;
extern void *DAT_00b69cbc;

void __stdcall FUN_004d37e0(void)
{
  int iVar1;
  
  iVar1 = ((int (__stdcall *)(void *))((void **)*DAT_00aa7b36)[0x1b])(DAT_00aa7b36);
  if (iVar1 == 0) {
    if (DAT_00b69cb0 == 0 && DAT_00b69cbc == 0) {
      ((void (__stdcall *)(void *))((void **)*DAT_00aa7b32)[0x1b])(DAT_00aa7b32);
      return;
    }
    ((void (__stdcall *)(void *))((void **)*DAT_00aa7b3a)[0x1b])(DAT_00aa7b3a);
  }
  return;
}
