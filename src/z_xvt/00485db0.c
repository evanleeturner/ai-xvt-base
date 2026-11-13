/* SPDX-License-Identifier: MIT */
/* z_xvt 00485db0 */
extern unsigned int DAT_0052ae18;
extern unsigned int DAT_006a885e;
extern unsigned int DAT_006a886c;
extern unsigned int DAT_006a889a;
extern unsigned int DAT_006a889e;
extern unsigned int DAT_006a8850;
extern unsigned int DAT_006a8854;
extern int DAT_006a8862;
extern int DAT_006a8870;
extern unsigned int DAT_006a8894;
extern unsigned char DAT_0052a34c;

extern void FUN_0045f100(unsigned int *param_1);
extern void FUN_004840f0(int param_1);
extern void FUN_00484970(int *param_1);
extern void FUN_004842b0(int *param_1);
extern void FUN_00484fd0(int param_1);

void FUN_00485db0(unsigned int *param_1)
{
  int iVar1;
  int *piVar2;
  unsigned int *puVar3;
  
  if (DAT_0052ae18 != 0) {
    FUN_0045f100(param_1);
    return;
  }
  DAT_006a885e = 0;
  DAT_006a886c = 0;
  if ((((DAT_006a889a != DAT_006a889e) && (param_1[0x23] + DAT_006a8850 <= DAT_006a8854)) &&
      ((int)param_1[0x1e] <= DAT_006a8862)) && ((int)param_1[0x24] <= DAT_006a8870)) {
    puVar3 = (unsigned int *)(DAT_006a889e * 0xd0 + DAT_006a8894);
    for (iVar1 = 0x34; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar3 = *param_1;
      param_1 = param_1 + 1;
      puVar3 = puVar3 + 1;
    }
    piVar2 = (int *)(DAT_006a889e * 0xd0 + DAT_006a8894);
    FUN_004840f0((int)piVar2);
    if (piVar2[0x31] != 0) {
      if (DAT_0052a34c == '\0') {
        FUN_004842b0(piVar2);
      }
      else {
        FUN_00484970(piVar2);
      }
      FUN_00484fd0((int)piVar2);
      DAT_006a889e = DAT_006a889e + 1;
    }
  }
  return;
}
