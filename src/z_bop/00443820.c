/* SPDX-License-Identifier: MIT */
/* z_bop 00443820 */
extern void *DAT_00556900;
extern void *DAT_00556904;
extern void *DAT_00556908;

void __stdcall FUN_00443820(void)
{
  if (DAT_00556908 != 0) {
    (*(void (__stdcall **)(void *))(*(void **)DAT_00556904 + 0x20))(DAT_00556904);
    DAT_00556908 = 0;
  }
  if (DAT_00556904 != 0) {
    (*(void (__stdcall **)(void *))(*(void **)DAT_00556904 + 8))(DAT_00556904);
  }
  if (DAT_00556900 != 0) {
    (*(void (__stdcall **)(void *))(*(void **)DAT_00556900 + 8))(DAT_00556900);
  }
  return;
}
