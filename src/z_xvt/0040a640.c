/* SPDX-License-Identifier: MIT */
/* z_xvt 0040a640 */
extern int DAT_006d4988;
extern int DAT_0071ecf4;
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;
extern int DAT_0073255c;
extern int DAT_00732584;
extern int DAT_0073253c;
extern int DAT_00732568;
extern int DAT_00732530;
extern int DAT_00732548;
extern int DAT_00732558;
extern int DAT_00732578;
extern int DAT_00732538;

extern void FUN_00421c70(unsigned int param_1, int param_2);
extern int FUN_00413d00(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6);
extern unsigned int FUN_00419100(int param_1, int param_2);

int __cdecl FUN_0040a640(unsigned int param_1, int *param_2)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  short sVar4;
  short sVar5;
  int iVar6;
  int iVar7;
  short sVar8;
  int iVar9;

  if (param_1 == 0xffffffff) {
    return 0x40;
  }
  iVar1 = *(int *)(param_1 * 0x23 + 0x1f + DAT_006d4988);
  if (iVar1 == 0) {
    FUN_00421c70(param_1, 0);
    sVar5 = (short)DAT_00732a28 - (short)*(int *)(&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd);
    sVar8 = (short)DAT_00732a2c - (short)*(int *)(&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd);
    sVar4 = (short)DAT_00732a24 - (short)*(int *)(&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd);
  }
  else {
    sVar8 = (short)*(int *)(iVar1 + 6) - (short)*(int *)(&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd);
    sVar5 = (short)*(int *)(iVar1 + 10) - (short)*(int *)(&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd);
    sVar4 = (short)*(int *)(iVar1 + 0xe) - (short)*(int *)(&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd);
  }
  iVar7 = (int)sVar4;
  iVar6 = (int)sVar5;
  iVar9 = (int)sVar8;
  iVar1 = FUN_00413d00(iVar9, iVar6, iVar7, DAT_0073255c, DAT_00732584, DAT_0073253c);
  iVar2 = FUN_00413d00(iVar9, iVar6, iVar7, DAT_00732568, DAT_00732530, DAT_00732548);
  uVar3 = FUN_00419100((int)(short)iVar1, (int)(short)iVar2);
  sVar4 = (short)uVar3;
  if ((0x3fff < sVar4) || (sVar4 < -0x3fff)) {
    iVar1 = FUN_00413d00(iVar9, iVar6, iVar7, DAT_00732558, DAT_00732578, DAT_00732538);
    uVar3 = FUN_00419100((int)(short)iVar1, (int)(short)iVar2);
    sVar5 = -0x8000 - (short)uVar3;
    sVar4 = -0x8000 - sVar4;
    if (sVar5 < 0) {
      sVar5 = -sVar5;
    }
    sVar8 = sVar4;
    if (sVar4 < 0) {
      sVar8 = -sVar4;
    }
    iVar1 = (int)(short)(((short)(0x4000 - sVar5) >> 8) * ((short)(0x4000 - sVar8) >> 8));
    iVar1 = (int)(short)((short)*param_2 * (short)((int)(iVar1 + ((iVar1 >> 0x1f) & 0x3fU)) >> 6));
    *param_2 = *param_2 - (int)(short)((int)(iVar1 + ((iVar1 >> 0x1f) & 0x7fU)) >> 7);
  }
  sVar4 = sVar4 >> 7;
  if (sVar4 < -0x40) {
    sVar4 = -0x40;
  }
  if (0x3f < sVar4) {
    sVar4 = 0x3f;
  }
  return (int)(short)(sVar4 + 0x40);
}
