/* SPDX-License-Identifier: MIT */
/* z_xvt 004010f0 */
extern ushort DAT_006da9f8;
extern ushort DAT_0071f714;
extern ushort DAT_0070a1d0;
extern int DAT_006d4988;
extern ushort DAT_006d498c;
extern ushort DAT_006fedd0;
extern ushort DAT_006d4a46;
extern int unaff_EBP;

extern void __cdecl FUN_0049c2b0(int param_1);
extern void __cdecl FUN_00486310(int param_1);
extern int __cdecl FUN_004490f0(int param_1);
extern int __cdecl FUN_00449790(int param_1, int param_2);
extern void __cdecl FUN_00486750(int param_1, uint param_2);

void __cdecl FUN_004010f0(ushort param_1)
{
  ushort uVar7;
  int iVar2;
  int iVar4;
  int iVar5;
  uint uVar6;
  unsigned char uVar1;
  short sVar3;

  if (DAT_006da9f8 != param_1) {
    if ((uint)DAT_006da9f8 - (uint)param_1 != (uint)-1) {
      DAT_0071f714 = param_1 + 0x7000;
      goto LAB_00401165;
    }
    if (param_1 < DAT_006da9f8) {
      DAT_0070a1d0 = param_1;
    }
  }
  FUN_0049c2b0(((int)(short)((uint)unaff_EBP >> 0x10) << 16) | param_1);
  FUN_00486310(DAT_006d4988 + (uint)param_1 * 0x23);
LAB_00401165:
  uVar7 = 0;
  DAT_006d498c = 0;
  iVar2 = (uint)param_1 * 0x23;
  iVar4 = FUN_004490f0((uint)*(unsigned char *)(DAT_006d4988 + 3 + iVar2));
  if (0 < iVar4) {
    do {
      DAT_006d498c = DAT_006d498c + 1;
      iVar5 = FUN_00449790((uint)*(unsigned char *)(DAT_006d4988 + 3 + iVar2), (uint)uVar7);
      if (iVar5 == 1) break;
      uVar7 = uVar7 + 1;
    } while ((int)(uint)uVar7 < iVar4);
  }
  sVar3 = DAT_006fedd0;
  DAT_006d498c = DAT_006d498c - 1;
  if (param_1 < DAT_006da9f8) {
    DAT_006d4a46 = 1;
    DAT_006fedd0 = DAT_0071f714;
  }
  uVar6 = (uint)DAT_006d498c;
  iVar4 = *(int *)(DAT_006d4988 + 0x1f + iVar2);
  uVar1 = *(unsigned char *)(iVar4 + 0x88);
  *(unsigned char *)(iVar4 + 0x88) = 1;
  FUN_00486750(DAT_006d4988 + iVar2, uVar6);
  *(unsigned char *)(*(int *)(DAT_006d4988 + 0x1f + iVar2) + 0x88) = uVar1;
  DAT_006fedd0 = sVar3;
  return;
}
