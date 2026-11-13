/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9be0 */
extern int DAT_0064f788;
extern int DAT_0064f678;
extern int DAT_0064f790;

extern int __cdecl FUN_004df760(int, int, int);
extern void __cdecl FUN_004dd3e0(int *, int, int, short, int);
extern void __cdecl FUN_004dd000(int *, int *);
extern void __cdecl FUN_004dd070(int *, int, int);
extern void __cdecl FUN_004da920(int, char *, int *, uint);

void __cdecl FUN_004d9be0(int *param_1, char *param_2, int *param_3, uint param_4, char param_5)
{
  uint uVar1;
  int iVar2;
  int iVar3;
  int aiStack_10[4];

  if (DAT_0064f788 == 0) {
    DAT_0064f788 = 1;
    DAT_0064f678 = FUN_004df760(5, 99, 0x6d);
    DAT_0064f790 = FUN_004df760(99, 0xe7, 0xf7);
  }
  if (param_5 == '\0') {
    FUN_004dd3e0(param_1, 0, 0, (short)DAT_0064f678, 1);
    FUN_004dd000(aiStack_10, param_1);
    if (param_1[3] - param_1[1] < 0xf) {
      iVar3 = 1;
      iVar2 = 1;
    }
    else {
      iVar3 = 3;
      iVar2 = 3;
    }
    FUN_004dd070(aiStack_10, iVar2, iVar3);
    FUN_004dd3e0(aiStack_10, 0, 0, (short)DAT_0064f790, 1);
    uVar1 = DAT_0064f678;
  }
  else {
    FUN_004dd3e0(param_1, 0, 0, (short)DAT_0064f790, 1);
    FUN_004dd000(aiStack_10, param_1);
    if (param_1[3] - param_1[1] < 0xf) {
      iVar3 = 1;
      iVar2 = 1;
    }
    else {
      iVar3 = 3;
      iVar2 = 3;
    }
    FUN_004dd070(aiStack_10, iVar2, iVar3);
    FUN_004dd3e0(aiStack_10, 0, 0, (short)DAT_0064f678, 1);
    uVar1 = DAT_0064f790;
  }
  FUN_004da920((int)param_3, param_2, param_1, uVar1);
  return;
}
