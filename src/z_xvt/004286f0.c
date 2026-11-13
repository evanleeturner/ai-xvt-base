/* SPDX-License-Identifier: MIT */
/* z_xvt 004286f0 */
extern int DAT_00536ef8;
extern int DAT_00536f05;
long long __cdecl __ftol(void);

void __cdecl FUN_004286f0(int param_1)
{
  unsigned int uVar1;
  long long lVar2;
  
  if (*(char *)(param_1 + 2) != '\0') {
    uVar1 = 0;
    do {
      if ((1 << ((unsigned int)uVar1 & 0x1f) & (unsigned int)*(unsigned char *)(param_1 + 2)) != 0) {
        FUN_004286f0(*(int *)(param_1 + 0x32 + uVar1 * 4));
      }
      uVar1 = uVar1 + 1;
    } while (uVar1 < 8);
  }
  if (*(int *)(param_1 + 10) != 0) {
    lVar2 = __ftol();
    *(char *)(DAT_00536ef8 * 9 + DAT_00536f05) = (char)lVar2;
    lVar2 = __ftol();
    *(char *)(DAT_00536ef8 * 9 + 1 + DAT_00536f05) = (char)lVar2;
    lVar2 = __ftol();
    *(char *)(DAT_00536ef8 * 9 + 2 + DAT_00536f05) = (char)lVar2;
    *(int *)(param_1 + 6) = DAT_00536ef8;
    DAT_00536ef8 = DAT_00536ef8 + 1;
  }
  return;
}
