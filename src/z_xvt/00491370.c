/* SPDX-License-Identifier: MIT */
/* z_xvt 00491370 */
extern int DAT_00658654;
extern char DAT_0065865c;
extern int DAT_006a8814;
extern int DAT_006a8610;
extern int DAT_006a8614;
extern int DAT_006a8670;

int _strncmp(char *, char *, int);

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wuninitialized"
int __cdecl FUN_00491370(char *param_1)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  char *_Str1;
  int unaff_ESI;
  int iVar4;
  int iVar5;
  char auStack_8[4];
  char *pcStack_4;
  
  if (DAT_00658654 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar3 = 0;
  pcStack_4 = (char *)&DAT_0065865c;
  iVar4 = DAT_006a8814 + -1;
  while (-1 < iVar4) {
    iVar5 = iVar4 >> 1;
    _Str1 = pcStack_4 + iVar5 * 0x145;
    iVar1 = _strncmp(_Str1, param_1, 0x40);
    if (iVar1 == 0) {
      iVar5 = iVar5 + iVar3;
      goto LAB_00491406;
    }
    if (iVar4 < 1) break;
    if (iVar1 < 0) {
      iVar3 = iVar5 + 1 + iVar3;
      iVar4 = iVar4 - (iVar5 + 1);
      pcStack_4 = _Str1 + 0x145;
    }
    else {
      iVar4 = iVar5 + -1;
    }
  }
  iVar5 = -1;
LAB_00491406:
  if (iVar5 == -1) {
    return 0;
  }
  iVar4 = -1;
  iVar1 = 0;
  iVar3 = -1;
  piVar2 = &DAT_006a8610;
  do {
    if ((piVar2[-1] == iVar5) && (iVar4 < *piVar2)) {
      iVar4 = *piVar2;
      iVar3 = iVar1;
    }
    piVar2 = piVar2 + 3;
    iVar1 = iVar1 + 1;
  } while (piVar2 < &DAT_006a8670);
  if (iVar3 == -1) {
    return 0;
  }
  iVar4 = ((int (__stdcall *)(int *, char *))(*(int *)(&DAT_006a8614)[iVar3 * 3] + 0x18))
                    ((int *)(&DAT_006a8614)[iVar3 * 3], auStack_8);
  if (iVar4 != 0) {
    return 0;
  }
  return (unaff_ESI * 0x7f) / 2000 + 0x7f;
}
#pragma GCC diagnostic pop
