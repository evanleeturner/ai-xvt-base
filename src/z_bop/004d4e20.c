/* SPDX-License-Identifier: MIT */
/* z_bop 004d4e20 */
extern unsigned int DAT_00aa7b2e;
extern void *DAT_00aa7b32;
extern unsigned int DAT_00aa7b13;
extern unsigned int DAT_00aa7b1b;
extern unsigned int DAT_00aa6cfc;

unsigned int __stdcall FUN_004d4e20(void)
{
  unsigned int *puVar1;
  int iVar2;
  unsigned int local_6c[27];
  unsigned int uStack_48 = 0;

  if (DAT_00aa7b2e == 0) {
    return 0;
  }
  if (DAT_00aa7b32 == (void *)0) {
    return 0;
  }
  puVar1 = local_6c;
  for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar1 = 0;
    puVar1 = puVar1 + 1;
  }
  local_6c[0] = 0x6c;
  do {
    while (1) {
      iVar2 = (*(int (__stdcall **)(void *, unsigned int, unsigned int *, unsigned int, unsigned int))(*(void ***)DAT_00aa7b32 + 25))(DAT_00aa7b32, 0, local_6c, 0, 0);
      if (iVar2 == 0) {
        DAT_00aa7b13 = DAT_00aa7b1b;
        DAT_00aa6cfc = uStack_48;
        return 1;
      }
      if (iVar2 != -0x7789fe3e) break;
      (*(void (__stdcall **)(void *))(*(void ***)DAT_00aa7b32 + 27))(DAT_00aa7b32);
    }
  } while (iVar2 == -0x7789fde4);
  return 0;
}
