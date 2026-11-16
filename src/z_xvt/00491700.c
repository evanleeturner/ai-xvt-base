/* SPDX-License-Identifier: MIT */
/* z_xvt 00491700 */
extern int DAT_00658654;
extern int DAT_0065865c;
extern int DAT_006a8814;
extern int DAT_006a8610;
extern int DAT_006a8614;
extern int DAT_006a8670;
extern int _strncmp(const char *s1, const char *s2, int n);

bool __cdecl FUN_00491700(char *param_1, undefined4 param_2)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  char *pcStack_4;
  
  if (DAT_00658654 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar2 = 0;
  pcStack_4 = (char *)&DAT_0065865c;
  iVar4 = DAT_006a8814 + -1;
  while (-1 < iVar4) {
    iVar5 = iVar4 >> 1;
    iVar1 = _strncmp(pcStack_4 + iVar5 * 0x145, param_1, 0x40);
    if (iVar1 == 0) {
      iVar5 = iVar5 + iVar2;
      goto LAB_00491796;
    }
    if (iVar4 < 1) break;
    if (iVar1 < 0) {
      iVar2 = iVar5 + 1 + iVar2;
      iVar4 = iVar4 - (iVar5 + 1);
      pcStack_4 = pcStack_4 + iVar5 * 0x145 + 0x145;
    }
    else {
      iVar4 = iVar5 + -1;
    }
  }
  iVar5 = -1;
LAB_00491796:
  if (iVar5 == -1) {
    return 0;
  }
  iVar4 = -1;
  iVar1 = 0;
  iVar2 = -1;
  piVar3 = &DAT_006a8610;
  do {
    if ((piVar3[-1] == iVar5) && (iVar4 < *piVar3)) {
      iVar4 = *piVar3;
      iVar2 = iVar1;
    }
    piVar3 = piVar3 + 3;
    iVar1 = iVar1 + 1;
  } while (piVar3 < &DAT_006a8670);
  if (iVar2 == -1) {
    return 0;
  }
  iVar4 = (*(int (__stdcall **)(int, undefined4))(*(void **)(*(int *)(&DAT_006a8614)[iVar2 * 3] + 0x44)))
                    (*(int *)(&DAT_006a8614)[iVar2 * 3], param_2);
  return iVar4 == 0;
}
