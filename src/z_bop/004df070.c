/* SPDX-License-Identifier: MIT */
/* z_bop 004df070 */
extern int _strncmp(const char *, const char *, unsigned int);
extern int DAT_00aa7fbf;
extern int DAT_00aa7fc3;
extern int DAT_00aa7fc7;

void __cdecl FUN_004df070(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *puVar5;
  int *puVar6;
  char *pcVar7;
  
  iVar3 = 0;
  if (0 < DAT_00aa7fc7) {
    iVar4 = 0;
    do {
      iVar1 = _strncmp(param_1, (char *)(DAT_00aa7fbf + iVar4), 0x40);
      if (iVar1 < 0) break;
      iVar4 = iVar4 + 0x145;
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_00aa7fc7);
  }
  if (iVar3 < DAT_00aa7fc7) {
    iVar4 = DAT_00aa7fc7 * 0x145;
    iVar1 = DAT_00aa7fc7 - iVar3;
    do {
      puVar6 = (int *)(DAT_00aa7fbf + iVar4);
      iVar4 = iVar4 - 0x145;
      puVar5 = (int *)((int)puVar6 - 0x145);
      for (iVar2 = 0x51; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
      }
      *(char *)puVar6 = *(char *)puVar5;
      iVar1 = iVar1 - 1;
    } while (iVar1 != 0);
  }
  iVar1 = 0;
  pcVar7 = (char *)(iVar3 * 0x145 + DAT_00aa7fbf);
  for (iVar4 = 0x51; iVar4 != 0; iVar4 = iVar4 - 1) {
    *(int *)pcVar7 = *(int *)param_1;
    param_1 = param_1 + 4;
    pcVar7 = pcVar7 + 4;
  }
  *pcVar7 = *param_1;
  DAT_00aa7fc7 = DAT_00aa7fc7 + 1;
  do {
    iVar4 = *(int *)(DAT_00aa7fc3 + iVar1);
    if (iVar3 <= iVar4) {
      *(int *)(DAT_00aa7fc3 + iVar1) = iVar4 + 1;
    }
    iVar1 = iVar1 + 0xc;
  } while (iVar1 < 0x90);
  return;
}
