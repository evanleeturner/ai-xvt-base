/* SPDX-License-Identifier: MIT */
/* z_bop 00497d80 */
extern int DAT_005233d8;
extern int DAT_00527ecc;
extern int DAT_00612294;

void __cdecl FUN_00497d80(int param_1, int param_2, int param_3)
{
  unsigned char uVar1;
  unsigned char *puVar2;
  unsigned char *puVar3;
  
  if (0 < param_3) {
    if (DAT_005233d8 == 2) {
      puVar2 = (unsigned char *)(param_1 + param_2 * 2);
      puVar3 = (unsigned char *)(DAT_00527ecc + DAT_00612294 + param_2 * 2);
      do {
        uVar1 = puVar2[1];
        *puVar3 = *puVar2;
        puVar3[1] = uVar1;
        puVar2 = puVar2 + 2;
        puVar3 = puVar3 + 2;
        param_3 = param_3 + -1;
      } while (param_3 != 0);
      return;
    }
    puVar2 = (unsigned char *)(param_1 + param_2);
    puVar3 = (unsigned char *)(DAT_00527ecc + param_2 + DAT_00612294);
    do {
      uVar1 = *puVar2;
      puVar2 = puVar2 + 1;
      *puVar3 = uVar1;
      puVar3 = puVar3 + 1;
      param_3 = param_3 + -1;
    } while (param_3 != 0);
  }
  return;
}
