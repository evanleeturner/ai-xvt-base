/* SPDX-License-Identifier: MIT */
/* z_bop 004da8e0 */
extern int DAT_00665570;
extern int DAT_00665460;
extern int DAT_00665578;

extern int __cdecl FUN_004d6e30(int, int, int);
extern void __cdecl FUN_004d67e0(int *, int, int, short, int);
extern void __cdecl FUN_004d6480(int *, int *);
extern void __cdecl FUN_004d64c0(int *, int, int);
extern void __cdecl FUN_004db640(int, char *, int *, unsigned int);

void __cdecl FUN_004da8e0(int *param_1, char *param_2, int param_3, int param_4, char param_5)
{
  unsigned int uVar1;
  int iVar2;
  int iVar3;
  int local_10[4];

  if (DAT_00665570 == 0) {
    DAT_00665570 = 1;
    DAT_00665460 = FUN_004d6e30(5, 99, 0x6d);
    DAT_00665578 = FUN_004d6e30(99, 0xe7, 0xf7);
  }
  if (param_5 == '\0') {
    FUN_004d67e0(param_1, 0, 0, (short)DAT_00665460, 1);
    FUN_004d6480(local_10, param_1);
    if (param_1[3] - param_1[1] < 0xf) {
      iVar3 = 1;
      iVar2 = 1;
    }
    else {
      iVar3 = 3;
      iVar2 = 3;
    }
    FUN_004d64c0(local_10, iVar2, iVar3);
    FUN_004d67e0(local_10, 0, 0, (short)DAT_00665578, 1);
    uVar1 = DAT_00665460;
  }
  else {
    FUN_004d67e0(param_1, 0, 0, (short)DAT_00665578, 1);
    FUN_004d6480(local_10, param_1);
    if (param_1[3] - param_1[1] < 0xf) {
      iVar3 = 1;
      iVar2 = 1;
    }
    else {
      iVar3 = 3;
      iVar2 = 3;
    }
    FUN_004d64c0(local_10, iVar2, iVar3);
    FUN_004d67e0(local_10, 0, 0, (short)DAT_00665460, 1);
    uVar1 = DAT_00665578;
  }
  FUN_004db640(param_3, param_2, param_1, uVar1);
  return;
}
