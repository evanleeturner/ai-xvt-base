/* SPDX-License-Identifier: MIT */
/* z_bop 00464c80 */
extern int DAT_005242e0;
extern int DAT_00557358;
extern int DAT_005242e4;
extern int _DAT_009ed23c;
extern int DAT_0052340c;
extern int DAT_00523418;
extern int DAT_009e9680;
extern int DAT_009ec468;
extern int DAT_005242e8;

extern int *FUN_00418890(int);

int __cdecl FUN_00464c80(int param_1)
{
  int *piVar1;
  int iVar2;
  char *pcVar3;
  int iVar4;
  
  if (DAT_005242e0 != 0) {
    return 0;
  }
  if (DAT_00557358 != 0) {
    return 0;
  }
  if (DAT_005242e4 == 0) {
    DAT_005242e4 = (_DAT_009ed23c >> 3) + param_1 + DAT_0052340c;
  }
  iVar2 = (param_1 + DAT_0052340c) - DAT_005242e4;
  if (iVar2 < DAT_00523418) {
    return 0;
  }
  if ((int)((unsigned)DAT_00523418 * 5) < iVar2) {
    DAT_005242e4 = DAT_005242e4 + DAT_00523418;
    return 1;
  }
  iVar2 = 0x7fffffff;
  iVar4 = 0;
  pcVar3 = (char *)&DAT_009e9680;
  do {
    if (*pcVar3 != '\0') {
      piVar1 = FUN_00418890(iVar4);
      if (piVar1 == (int *)0x0) {
        iVar2 = 0;
        break;
      }
      if (piVar1[2] < iVar2) {
        iVar2 = piVar1[2];
      }
    }
    pcVar3 = pcVar3 + 0x5bd;
    iVar4 = iVar4 + 1;
  } while (pcVar3 < (char *)&DAT_009ec468);
  if (iVar2 <= DAT_005242e8 + DAT_00523418) {
    return 0;
  }
  DAT_005242e4 = DAT_005242e4 + DAT_00523418;
  return 1;
}
