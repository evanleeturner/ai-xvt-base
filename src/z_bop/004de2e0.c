/* SPDX-License-Identifier: MIT */
/* z_bop 004de2e0 */
extern void *DAT_00aa7fb7;
extern void *DAT_00aa7fbf;
extern void *DAT_00aa7fc3;
extern void *_DAT_00aa7fbb;
extern void *DAT_00aa7fcf;

int __stdcall FUN_004de2e0(void)
{
  int iVar1;
  int local_fb7;
  int local_vtable;
  int zero_val = 0;
  int ff_val = 0xffffffff;

  __builtin_memcpy(&local_fb7, &DAT_00aa7fb7, 4);
  if (local_fb7 == 0) {
    return 1;
  }
  __builtin_memcpy(&local_vtable, (char *)local_fb7 + 8, 4);
  ((void (__stdcall *)(int))local_vtable)(local_fb7);

  __builtin_memcpy(&DAT_00aa7fb7, &zero_val, 4);
  iVar1 = 0;
  do {
    iVar1 = iVar1 + 0x145;
    __builtin_memcpy((char *)DAT_00aa7fbf + -5 + iVar1, &zero_val, 4);
    *(char *)((char *)DAT_00aa7fbf + -0x145 + iVar1) = 0;
  } while (iVar1 < 0xa280);
  iVar1 = 0;
  do {
    iVar1 = iVar1 + 0xc;
    __builtin_memcpy((char *)DAT_00aa7fc3 + -0xc + iVar1, &ff_val, 4);
    __builtin_memcpy((char *)DAT_00aa7fc3 + -8 + iVar1, &zero_val, 4);
    __builtin_memcpy((char *)DAT_00aa7fc3 + -4 + iVar1, &zero_val, 4);
  } while (iVar1 < 0x90);
  __builtin_memcpy(&_DAT_00aa7fbb, &zero_val, 4);
  __builtin_memcpy(&DAT_00aa7fcf, &zero_val, 4);
  return 1;
}
