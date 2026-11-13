/* SPDX-License-Identifier: MIT */
/* z_bop 0040bbc0 */
extern void *DAT_00803f80;
extern void *DAT_00a68748;

void __stdcall FUN_0040bbc0(void)
{
  if (DAT_00a68748 != (void *)0x0) {
    ((void (__stdcall *)(void *, int, void *))(*(void **)DAT_00803f80 + 0x20))(DAT_00803f80, 0, DAT_00a68748);
    ((void (__stdcall *)(void *))(*(void **)DAT_00a68748 + 8))(DAT_00a68748);
    DAT_00a68748 = (void *)0x0;
  }
  return;
}
