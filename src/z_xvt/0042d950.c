/* SPDX-License-Identifier: MIT */
/* z_xvt 0042d950 */
extern int DAT_005250d8;
extern int DAT_005373c8;
extern int DAT_005250dc;
extern int DAT_0052554c;
extern int DAT_00525558;
extern int DAT_0071f2ec;
extern char DAT_0071b740;
extern char DAT_0071e528;
extern int DAT_005250e0;

int * __cdecl FUN_0049b200(int param_1);

int __cdecl FUN_0042d950(int param_1)
{
  int *piVar1;
  int iVar2;
  char *pcVar3;
  int iVar4;
  
  if (DAT_005250d8 != 0) {
    return 0;
  }
  if (DAT_005373c8 != 0) {
    return 0;
  }
  if (DAT_005250dc == 0) {
    DAT_005250dc = (DAT_0071f2ec >> 3) + param_1 + DAT_0052554c;
  }
  iVar2 = (param_1 + DAT_0052554c) - DAT_005250dc;
  if (iVar2 < DAT_00525558) {
    return 0;
  }
  if (DAT_00525558 * 5 < iVar2) {
    DAT_005250dc = DAT_005250dc + DAT_00525558;
    return 1;
  }
  iVar2 = 0x7fffffff;
  iVar4 = 0;
  pcVar3 = &DAT_0071b740;
  do {
    if (*pcVar3 != '\0') {
      piVar1 = FUN_0049b200(iVar4);
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
  } while (pcVar3 < &DAT_0071e528);
  if (iVar2 <= DAT_00525558 + DAT_005250e0) {
    return 0;
  }
  DAT_005250dc = DAT_005250dc + DAT_00525558;
  return 1;
}
