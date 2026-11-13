/* SPDX-License-Identifier: MIT */
/* z_bop 00421700 */
extern unsigned short DAT_009d1260;
extern short DAT_00a07cd0;
extern short DAT_009ed230;
extern short DAT_009d8c2c;
extern short DAT_009d77f0;
extern int DAT_009a8d50;
extern int DAT_005233d8;
extern int DAT_009d77c4;

void __cdecl FUN_00486470(unsigned char *param_1, int param_2, int param_3, int param_4, int param_5, int param_6);

void __cdecl FUN_00421700(int *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = (unsigned int)DAT_009d1260 - param_1[1];
  param_1[1] = iVar7;
  iVar5 = (unsigned int)DAT_009d1260 - param_1[3];
  param_1[3] = iVar5;
  iVar1 = (unsigned int)DAT_009d1260 - param_1[5];
  param_1[5] = iVar1;
  iVar2 = (unsigned int)DAT_009d1260 - param_1[7];
  param_1[7] = iVar2;
  iVar6 = *param_1;
  iVar3 = iVar6;
  if (param_1[2] < iVar6) {
    iVar3 = param_1[2];
  }
  if (param_1[4] < iVar3) {
    iVar3 = param_1[4];
  }
  if (param_1[6] < iVar3) {
    iVar3 = param_1[6];
  }
  if (iVar6 < param_1[2]) {
    iVar6 = param_1[2];
  }
  if (iVar6 < param_1[4]) {
    iVar6 = param_1[4];
  }
  if (iVar6 < param_1[6]) {
    iVar6 = param_1[6];
  }
  iVar4 = iVar7;
  if (iVar5 < iVar7) {
    iVar4 = iVar5;
  }
  if (iVar1 < iVar4) {
    iVar4 = iVar1;
  }
  if (iVar2 < iVar4) {
    iVar4 = iVar2;
  }
  if (iVar7 < iVar5) {
    iVar7 = iVar5;
  }
  if (iVar7 < iVar1) {
    iVar7 = iVar1;
  }
  if (iVar7 < iVar2) {
    iVar7 = iVar2;
  }
  iVar3 = iVar3 + -2;
  iVar4 = iVar4 + -2;
  iVar6 = iVar6 + 2;
  iVar7 = iVar7 + 2;
  if ((-1 < iVar7) && (iVar4 < DAT_00a07cd0)) {
    if (DAT_00a07cd0 <= iVar7) {
      iVar7 = (int)DAT_009ed230;
    }
    if (iVar4 < 0) {
      iVar4 = 0;
    }
    if ((-1 < iVar6) && (iVar3 < DAT_009d8c2c)) {
      if (DAT_009d8c2c <= iVar6) {
        iVar6 = (int)DAT_009d77f0;
      }
      if (iVar3 < 0) {
        iVar3 = 0;
      }
      FUN_00486470((unsigned char *)(iVar4 * DAT_009a8d50 + iVar3 * DAT_005233d8 + DAT_009d77c4),
                   (iVar3 - iVar6) * DAT_005233d8 + DAT_009a8d50, iVar3, iVar4, iVar6, iVar7);
    }
  }
  return;
}
