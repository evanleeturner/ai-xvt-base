/* SPDX-License-Identifier: MIT */
/* z_xvt 00464640 */
extern int DAT_0052550c;
extern int DAT_0052552c;
extern int DAT_00525534;
extern int DAT_005f1d20;
extern short DAT_0071f2e8;

void __cdecl FUN_00464640(undefined1 *param_1, int param_2, int param_3, short param_4, int param_5)
{
  undefined1 uVar1;
  undefined1 *puVar2;
  uint uVar3;
  int iVar4;
  
  if (param_5 != 0) {
    iVar4 = param_3 << 2;
    do {
      uVar3 = *(int *)(DAT_005f1d20 + iVar4) + param_2;
      if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
        uVar3 = uVar3 % DAT_0052550c;
      }
      puVar2 = (undefined1 *)(uVar3 + DAT_00525534);
      for (DAT_0071f2e8 = param_4; DAT_0071f2e8 != 0; DAT_0071f2e8 = DAT_0071f2e8 + -1) {
        uVar1 = *param_1;
        param_1 = param_1 + 1;
        *puVar2 = uVar1;
        puVar2 = puVar2 + 1;
      }
      param_5 = param_5 + -1;
      iVar4 = iVar4 + 4;
      DAT_0071f2e8 = 0;
    } while (param_5 != 0);
  }
  return;
}
