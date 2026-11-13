/* SPDX-License-Identifier: MIT */
/* z_xvt 00491960 */
extern int DAT_00658654;
extern char DAT_0065865c[];
extern int DAT_006a8814;
extern int DAT_006a8614;
extern int DAT_006a8674;

int _strncmp(const char *s1, const char *s2, int n);

int __cdecl FUN_00491960(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *puVar5;
  char *pcStack_8;
  unsigned int uStack_4;
  
  if (DAT_00658654 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar2 = 0;
  pcStack_8 = (char *)&DAT_0065865c;
  iVar3 = DAT_006a8814 - 1;
  while (iVar3 >= 0) {
    iVar4 = iVar3 >> 1;
    iVar1 = _strncmp(pcStack_8 + iVar4 * 0x145, param_1, 0x40);
    if (iVar1 == 0) {
      iVar2 = iVar2 + iVar4;
      goto LAB_004919f5;
    }
    if (iVar3 < 1) break;
    if (iVar1 < 0) {
      iVar2 = iVar4 + 1 + iVar2;
      iVar3 = iVar3 - (iVar4 + 1);
      pcStack_8 = pcStack_8 + iVar4 * 0x145 + 0x145;
    }
    else {
      iVar3 = iVar4 - 1;
    }
  }
  iVar2 = -1;
LAB_004919f5:
  if (iVar2 == -1) {
    return 0;
  }
  iVar3 = 0;
  puVar5 = (int *)&DAT_006a8614;
  do {
    if (puVar5[-2] == iVar2) {
      iVar1 = ((int (__stdcall *)(int *, unsigned int *))(*(int *)*puVar5 + 0x24))((int *)*puVar5, &uStack_4);
      if ((iVar1 == 0) && ((uStack_4 & 1) != 0 || (uStack_4 & 4) != 0)) {
        iVar3 = iVar3 + 1;
      }
    }
    puVar5 = puVar5 + 3;
  } while (puVar5 < (int *)&DAT_006a8674);
  return iVar3;
}
