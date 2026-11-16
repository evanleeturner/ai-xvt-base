/* SPDX-License-Identifier: MIT */
/* z_xvt 00490490 */
extern void **DAT_0052a59c;

void __stdcall FUN_00490490(void)
{
  if (DAT_0052a59c != 0) {
    ((void (__stdcall *)(void *, int))*(void **)((char *)*DAT_0052a59c + 0x34))(DAT_0052a59c, 0);
    ((void (__stdcall *)(void *, int))*(void **)((char *)*DAT_0052a59c + 0x3c))(DAT_0052a59c, 0);
    ((void (__stdcall *)(void *, int))*(void **)((char *)*DAT_0052a59c + 0x40))(DAT_0052a59c, 0);
    ((void (__stdcall *)(void *, int, int, int))*(void **)((char *)*DAT_0052a59c + 0x30))(DAT_0052a59c, 0, 0, DAT_0052a59c != 0);
  }
  return;
}
