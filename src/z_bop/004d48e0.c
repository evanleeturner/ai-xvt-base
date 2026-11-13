/* SPDX-License-Identifier: MIT */
/* z_bop 004d48e0 */
extern int DAT_00aa7b2e;
extern int *DAT_00aa7b3a;
extern int DAT_00aa7b13;
extern int DAT_00aa7b17;
extern char DAT_00aa7b0d;
extern int _DAT_00aa7aa1;
extern int DAT_00aa7ac5;

int __stdcall FUN_004d48e0(void)
{
  int iVar1;
  
  if (DAT_00aa7b2e == 0) {
    return 0;
  }
  if (DAT_00aa7b3a == (int *)0) {
    return 0;
  }
  DAT_00aa7b13 = DAT_00aa7b17;
  if (DAT_00aa7b0d != '\0') {
    return DAT_00aa7ac5;
  }
  _DAT_00aa7aa1 = 0x6c;
  do {
    while (1) {
      iVar1 = ((int (__stdcall *)(void *, int, int *, int, int))(*DAT_00aa7b3a + 100))(DAT_00aa7b3a, 0, &_DAT_00aa7aa1, 0, 0);
      if (iVar1 != -0x7789fe3e) break;
      ((void (__stdcall *)(void *))(*DAT_00aa7b3a + 0x6c))(DAT_00aa7b3a);
    }
  } while (iVar1 == -0x7789fde4 || iVar1 == -0x7789fe52 || iVar1 == -0x7789fe48);
  DAT_00aa7b0d = 1;
  return DAT_00aa7ac5;
}
