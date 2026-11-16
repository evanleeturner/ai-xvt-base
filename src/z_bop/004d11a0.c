/* SPDX-License-Identifier: MIT */
/* z_bop 004d11a0 */
extern int DAT_00acebfd;
extern unsigned int DAT_00acec45[];

int __cdecl FUN_004d11a0(unsigned int param_1)
{
  int iVar1 = 0;
  if (DAT_00acebfd > 0) {
    unsigned int *puVar3 = DAT_00acec45;
    int iVar2 = DAT_00acebfd;
    do {
      if (*puVar3 < param_1) {
        iVar1 = iVar1 + 1;
      }
      puVar3 = puVar3 + 10;
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
  }
  return iVar1;
}
