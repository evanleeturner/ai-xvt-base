/* SPDX-License-Identifier: MIT */
/* z_xvt 00406600 */
extern int DAT_00703be4;
extern int DAT_00732a20;
extern int DAT_0071f2e0;
extern int DAT_0070acec;
extern int DAT_0070a170;
extern int DAT_00525518;
extern int DAT_006db6d0;
extern int DAT_0070a144;

void __cdecl FUN_00465eb0(unsigned char *, int, int, int, int, int);

void __cdecl FUN_00406600(int *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = (unsigned int)DAT_00703be4 - param_1[1];
  param_1[1] = iVar7;
  iVar5 = (unsigned int)DAT_00703be4 - param_1[3];
  param_1[3] = iVar5;
  iVar1 = (unsigned int)DAT_00703be4 - param_1[5];
  param_1[5] = iVar1;
  iVar2 = (unsigned int)DAT_00703be4 - param_1[7];
  param_1[7] = iVar2;
  iVar6 = param_1[0];
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
  if ((-1 < iVar7) && (iVar4 < DAT_00732a20)) {
    if (DAT_00732a20 <= iVar7) {
      iVar7 = (int)DAT_0071f2e0;
    }
    if (iVar4 < 0) {
      iVar4 = 0;
    }
    if ((-1 < iVar6) && (iVar3 < DAT_0070acec)) {
      if (DAT_0070acec <= iVar6) {
        iVar6 = (int)DAT_0070a170;
      }
      if (iVar3 < 0) {
        iVar3 = 0;
      }
      FUN_00465eb0((unsigned char *)(DAT_00525518 * iVar3 + iVar4 * DAT_006db6d0 + DAT_0070a144),
                   (iVar3 - iVar6) * DAT_00525518 + DAT_006db6d0, iVar3, iVar4, iVar6, iVar7);
    }
  }
  return;
}
