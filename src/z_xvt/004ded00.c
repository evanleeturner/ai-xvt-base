/* SPDX-License-Identifier: MIT */
/* z_xvt 004ded00 */
extern int _strncmp(const char *, const char *, int);
extern int DAT_00759af4;
extern int DAT_00759af8;
extern int DAT_00759afc;

void __cdecl FUN_004ded00(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  unsigned int *puVar5;
  unsigned int *puVar6;
  char *pcVar7;
  
  iVar3 = 0;
  if (0 < DAT_00759afc) {
    iVar4 = 0;
    do {
      iVar1 = _strncmp(param_1, (char *)(DAT_00759af4 + iVar4), 0x40);
      if (iVar1 < 0) break;
      iVar4 = iVar4 + 0x145;
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_00759afc);
  }
  if (iVar3 < DAT_00759afc) {
    iVar4 = DAT_00759afc * 0x145;
    iVar1 = DAT_00759afc - iVar3;
    do {
      puVar6 = (unsigned int *)(DAT_00759af4 + iVar4);
      iVar4 = iVar4 - 0x145;
      puVar5 = (unsigned int *)((int)puVar6 - 0x145);
      for (iVar2 = 0x51; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
      }
      *(unsigned char *)puVar6 = *(unsigned char *)puVar5;
      iVar1 = iVar1 - 1;
    } while (iVar1 != 0);
  }
  pcVar7 = (char *)(iVar3 * 0x145 + DAT_00759af4);
  for (iVar4 = 0x51; iVar4 != 0; iVar4 = iVar4 - 1) {
    *(unsigned int *)pcVar7 = *(unsigned int *)param_1;
    param_1 = param_1 + 4;
    pcVar7 = pcVar7 + 4;
  }
  *pcVar7 = *param_1;
  DAT_00759afc = DAT_00759afc + 1;
  iVar4 = 0;
  do {
    iVar1 = *(int *)(DAT_00759af8 + iVar4);
    if (iVar3 <= iVar1) {
      *(int *)(DAT_00759af8 + iVar4) = iVar1 + 1;
    }
    iVar4 = iVar4 + 0xc;
  } while (iVar4 < 0x90);
  return;
}
