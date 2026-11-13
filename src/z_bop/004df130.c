/* SPDX-License-Identifier: MIT */
/* z_bop 004df130 */
extern int DAT_00aa7fbf;
extern int DAT_00aa7fc3;
extern int DAT_00aa7fc7;

void __cdecl FUN_004df130(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *puVar4;
  int *puVar5;
  
  if ((-1 < param_1) && (param_1 < DAT_00aa7fc7)) {
    if (param_1 < DAT_00aa7fc7 + -1) {
      iVar3 = param_1 * 0x145;
      iVar2 = param_1;
      do {
        puVar5 = (int *)(DAT_00aa7fbf + iVar3);
        iVar2 = iVar2 + 1;
        iVar3 = iVar3 + 0x145;
        puVar4 = (int *)((int)puVar5 + 0x145);
        for (iVar1 = 0x51; iVar1 != 0; iVar1 = iVar1 + -1) {
          *puVar5 = *puVar4;
          puVar4 = puVar4 + 1;
          puVar5 = puVar5 + 1;
        }
        *(char *)puVar5 = *(char *)puVar4;
      } while (iVar2 < DAT_00aa7fc7 + -1);
    }
    iVar3 = 0;
    DAT_00aa7fc7 = DAT_00aa7fc7 + -1;
    do {
      iVar2 = *(int *)(DAT_00aa7fc3 + iVar3);
      if (param_1 < iVar2) {
        *(int *)(DAT_00aa7fc3 + iVar3) = iVar2 + -1;
      }
      iVar3 = iVar3 + 0xc;
    } while (iVar3 < 0x90);
  }
  return;
}
