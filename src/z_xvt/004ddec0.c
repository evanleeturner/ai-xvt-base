/* SPDX-License-Identifier: MIT */
/* z_xvt 004ddec0 */
extern void *DAT_00759aec;
extern void *DAT_00759af4;
extern void *DAT_00759af8;
extern unsigned int _DAT_00759af0;
extern unsigned int DAT_00759b04;

unsigned int __stdcall FUN_004ddec0(void)
{
  int iVar1;
  unsigned int __val;

  if (DAT_00759aec == NULL) {
    return 1;
  }
  (*(void (__stdcall **)(void *))((char *)DAT_00759aec + 8))(DAT_00759aec);
  iVar1 = 0;
  DAT_00759aec = NULL;
  do {
    iVar1 += 0x145;
    __val = 0;
    __builtin_memcpy((char *)DAT_00759af4 - 5 + iVar1, &__val, 4);
    *(unsigned char *)((char *)DAT_00759af4 - 0x145 + iVar1) = 0;
  } while (iVar1 < 0xa280);
  iVar1 = 0;
  do {
    iVar1 += 0xc;
    *(unsigned int *)((char *)DAT_00759af8 - 0xc + iVar1) = 0xffffffff;
    *(unsigned int *)((char *)DAT_00759af8 - 8 + iVar1) = 0;
    *(unsigned int *)((char *)DAT_00759af8 - 4 + iVar1) = 0;
  } while (iVar1 < 0x90);
  _DAT_00759af0 = 0;
  DAT_00759b04 = 0;
  return 1;
}
