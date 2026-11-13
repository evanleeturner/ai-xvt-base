/* SPDX-License-Identifier: MIT */
/* z_xvt 00491010 */
extern int DAT_00658654;
extern int DAT_0065865c;
extern int DAT_006a8814;
extern int DAT_006a8818;
extern int DAT_006a881c;
extern int DAT_006a8610;
extern int DAT_006a8614;
extern int DAT_006a860c;
extern int DAT_006a8670;

int __cdecl _strncmp(const char *s1, const char *s2, unsigned int n);

bool __cdecl FUN_00491010(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  char *pcStack_4;
  
  if (DAT_00658654 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar2 = 0;
  pcStack_4 = (char *)&DAT_0065865c;
  iVar3 = DAT_006a8814 + -1;
  while (-1 < iVar3) {
    iVar4 = iVar3 >> 1;
    iVar1 = _strncmp(pcStack_4 + iVar4 * 0x145, param_1, 0x40);
    if (iVar1 == 0) {
      iVar2 = iVar2 + iVar4;
      goto LAB_004910a5;
    }
    if (iVar3 < 1) break;
    if (iVar1 < 0) {
      iVar2 = iVar4 + 1 + iVar2;
      iVar3 = iVar3 - (iVar4 + 1);
      pcStack_4 = pcStack_4 + iVar4 * 0x145 + 0x145;
    }
    else {
      iVar3 = iVar4 + -1;
    }
  }
  iVar2 = -1;
LAB_004910a5:
  if (iVar2 == -1) {
    return 0;
  }
  iVar1 = -1;
  iVar3 = DAT_006a881c + 1;
  iVar4 = 0;
  piVar5 = &DAT_006a8610;
  do {
    if ((piVar5[-1] == iVar2) && (*piVar5 < iVar3)) {
      iVar3 = *piVar5;
      iVar1 = iVar4;
    }
    piVar5 = piVar5 + 3;
    iVar4 = iVar4 + 1;
  } while (piVar5 < &DAT_006a8670);
  if (iVar1 == -1) {
    return 0;
  }
  piVar5 = (int *)(&DAT_006a8614)[iVar1 * 3];
  if (piVar5 == (int *)0x0) {
    return 0;
  }
  iVar3 = ((int (*)(int *))(*(char **)*piVar5 + 0x48))(piVar5);
  ((void (*)(int *))(*(char **)*piVar5 + 8))(piVar5);
  (&DAT_006a8614)[iVar1 * 3] = 0;
  (&DAT_006a860c)[iVar1 * 3] = 0xffffffff;
  DAT_006a8818 = DAT_006a8818 + -1;
  return -1 < iVar3;
}
