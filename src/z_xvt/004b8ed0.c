/* SPDX-License-Identifier: MIT */
/* z_xvt 004b8ed0 */
extern int DAT_0064ef64;

void __cdecl FUN_004dcfd0(int *param_1, uint param_2, uint param_3, uint param_4, uint param_5);
char * __cdecl FUN_004dcfa0(uint param_1);
void __cdecl FUN_004da920(uint param_1, char *param_2, int *param_3, uint param_4);
void __cdecl FUN_004b9010(int param_1);
void __cdecl FUN_004b90b0(int param_1);
void __cdecl FUN_004b9140(int param_1);
void __cdecl FUN_004b91c0(int param_1);
void __cdecl FUN_004b9240(int param_1);
void __cdecl FUN_004b92c0(int param_1);
void __cdecl FUN_004b9340(int param_1);
void __cdecl FUN_004b93c0(int param_1);
void __cdecl FUN_004b9440(int param_1);
void __cdecl FUN_004b94c0(int param_1);
void __cdecl FUN_004b9540(int param_1);
void __cdecl FUN_004b9600(int param_1);
void __cdecl FUN_004b9680(int param_1);
void __cdecl FUN_004b9760(int param_1);
void __cdecl FUN_004b97e0(int param_1);
void __cdecl FUN_004b9870(int param_1);

void __stdcall FUN_004b8ed0(void)
{
  char *pcVar1;
  uint uVar2;
  int *piVar3;
  uint uVar4;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x54, 0x5a, 0x25c, 0x6a);
  piVar3 = local_10;
  uVar4 = 0xffff;
  if (DAT_0064ef64 == 1) {
    uVar2 = 0x273;
  }
  else {
    uVar2 = 0xce;
  }
  pcVar1 = FUN_004dcfa0(uVar2);
  FUN_004da920(0xf, pcVar1, piVar3, uVar4);
  FUN_004b9010(DAT_0064ef64 + -1);
  FUN_004b90b0(DAT_0064ef64 + -1);
  FUN_004b9140(DAT_0064ef64 + -1);
  FUN_004b91c0(DAT_0064ef64 + -1);
  FUN_004b9240(DAT_0064ef64 + -1);
  FUN_004b92c0(DAT_0064ef64 + -1);
  FUN_004b9340(DAT_0064ef64 + -1);
  FUN_004b93c0(DAT_0064ef64 + -1);
  FUN_004b9440(DAT_0064ef64 + -1);
  FUN_004b94c0(DAT_0064ef64 + -1);
  FUN_004b9540(DAT_0064ef64 + -1);
  FUN_004b9600(DAT_0064ef64 + -1);
  FUN_004b9680(DAT_0064ef64 + -1);
  FUN_004b9760(DAT_0064ef64 + -1);
  FUN_004b97e0(DAT_0064ef64 + -1);
  FUN_004b9870(DAT_0064ef64 + -1);
  return;
}
