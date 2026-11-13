/* SPDX-License-Identifier: MIT */
/* z_xvt 004dedc0 */
extern int DAT_00759af4;
extern int DAT_00759af8;
extern int DAT_00759afc;

void __cdecl FUN_004dedc0(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *puVar4;
  int *puVar5;
  
  if ((-1 < param_1) && (param_1 < DAT_00759afc)) {
    if (param_1 < DAT_00759afc - 1) {
      iVar3 = param_1 * 0x145;
      iVar2 = param_1;
      do {
        puVar5 = (int *)((int)DAT_00759af4 + iVar3);
        iVar2 = iVar2 + 1;
        iVar3 = iVar3 + 0x145;
        puVar4 = (int *)((int)puVar5 + 0x145);
        for (iVar1 = 0x51; iVar1 != 0; iVar1 = iVar1 - 1) {
          *puVar5 = *puVar4;
          puVar4 = puVar4 + 1;
          puVar5 = puVar5 + 1;
        }
        *(char *)puVar5 = *(char *)puVar4;
      } while (iVar2 < DAT_00759afc - 1);
    }
    iVar3 = 0;
    DAT_00759afc = DAT_00759afc - 1;
    do {
      iVar2 = *(int *)((int)DAT_00759af8 + iVar3);
      if (param_1 < iVar2) {
        *(int *)((int)DAT_00759af8 + iVar3) = iVar2 - 1;
      }
      iVar3 = iVar3 + 0xc;
    } while (iVar3 < 0x90);
  }
  return;
}
