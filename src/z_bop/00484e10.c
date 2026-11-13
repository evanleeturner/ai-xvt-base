/* SPDX-License-Identifier: MIT */
/* z_bop 00484e10 */
extern int DAT_00a00870[];
extern int DAT_00a00874[];

int __cdecl FUN_004ae2a0(int param_1, int param_2);
int __cdecl FUN_004aeb40(int param_1, int param_2);
int __cdecl FUN_004adc40(int param_1);

bool __cdecl FUN_00484e10(int param_1, int param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;

  if (param_1 < 0x49) {
    if (param_2 < 0) {
      iVar5 = 0;
    }
    else {
      iVar5 = param_2;
      if (DAT_00a00870[param_1 * 0x65] <= param_2) {
        iVar5 = DAT_00a00870[param_1 * 0x65] - 1;
      }
      iVar5 = DAT_00a00874[param_1 * 0x65 + iVar5];
    }
  }
  else {
    iVar5 = FUN_004ae2a0(param_1, param_2);
  }

  if (iVar5 == 0x12 || iVar5 == 0x13) {
    return 0;
  }

  iVar1 = FUN_004aeb40(param_1, param_2);
  if (iVar1 == 0) {
    return 1;
  }
  if (iVar1 == 1 && iVar5 != 1 && iVar5 != 3) {
    return 1;
  }

  if (param_1 < 0x49) {
    iVar2 = DAT_00a00870[param_1 * 0x65];
  }
  else {
    iVar2 = FUN_004adc40(param_1);
  }

  iVar4 = 0;
  if (0 < iVar2) {
    do {
      iVar3 = FUN_004aeb40(param_1, iVar4);
      if (iVar3 == iVar1) {
        if (param_1 < 0x49) {
          if (iVar4 < 0) {
            iVar3 = 0;
          }
          else {
            iVar3 = iVar4;
            if (DAT_00a00870[param_1 * 0x65] <= iVar4) {
              iVar3 = DAT_00a00870[param_1 * 0x65] - 1;
            }
            iVar3 = DAT_00a00874[param_1 * 0x65 + iVar3];
          }
        }
        else {
          iVar3 = FUN_004ae2a0(param_1, iVar4);
        }
        if (iVar5 == iVar3) {
          return iVar4 == param_2;
        }
      }
      iVar4 = iVar4 + 1;
    } while (iVar4 < iVar2);
  }
  return 0;
}
