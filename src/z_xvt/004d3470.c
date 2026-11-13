/* SPDX-License-Identifier: MIT */
/* z_xvt 004d3470 */
extern int DAT_00758d45;
extern char *DAT_00758d41;
extern int _strncmp(const char *, const char *, unsigned int);

void __cdecl FUN_004d3470(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  unsigned int *puVar5;
  unsigned int *puVar6;
  
  iVar3 = 0;
  if (0 < DAT_00758d45) {
    iVar4 = 0;
    do {
      iVar1 = _strncmp(param_1, (char *)(DAT_00758d41 + iVar4), 0x40);
      if (iVar1 < 0) break;
      iVar4 = iVar4 + 0x44;
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_00758d45);
  }
  if (iVar3 < DAT_00758d45) {
    iVar4 = DAT_00758d45 * 0x44;
    iVar1 = DAT_00758d45 - iVar3;
    do {
      puVar6 = (unsigned int *)(DAT_00758d41 + iVar4);
      iVar4 = iVar4 + -0x44;
      puVar5 = puVar6 + -0x11;
      for (iVar2 = 0x11; iVar2 != 0; iVar2 = iVar2 + -1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
      }
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  puVar5 = (unsigned int *)(DAT_00758d41 + iVar3 * 0x44);
  for (iVar4 = 0x11; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar5 = *(unsigned int *)param_1;
    param_1 = param_1 + 4;
    puVar5 = puVar5 + 1;
  }
  DAT_00758d45 = DAT_00758d45 + 1;
  return;
}
