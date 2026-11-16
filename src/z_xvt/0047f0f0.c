/* SPDX-License-Identifier: MIT */
/* z_xvt 0047f0f0 */
extern int DAT_006d4988;
extern short DAT_0071f714;
extern int DAT_006a89f8;
extern unsigned char DAT_00526872[];
extern unsigned short DAT_006a89f4;
extern unsigned short DAT_006d498c;
extern unsigned char DAT_005281f8[];
extern unsigned int DAT_006db7b4;
extern unsigned int DAT_006db7ac;
extern unsigned int DAT_006db7b0;
extern int DAT_0070a134;
extern int DAT_0070a00c;
extern unsigned int DAT_0070a130;
extern unsigned int DAT_0070a010;
extern unsigned int DAT_0070a12c;
extern unsigned int DAT_0070a138;
extern short DAT_006d4984;
extern short DAT_006da9e2;
extern unsigned char DAT_0071eaf0[];

void __cdecl FUN_00486750(int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00419100(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0040ca90(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0040cad0(unsigned int param_1, unsigned int param_2);

void __cdecl FUN_0047f0f0(int param_1)
{
  unsigned char bVar1;
  short sVar2;
  unsigned short uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  short sVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  unsigned short uVar11;
  unsigned short local_2;
  int iTmp;
  unsigned char *pEntry;
  unsigned int uTmp;
  short sTmp;

  iVar10 = (int)((unsigned int)param_1 * 0x23u);
  iVar8 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar10);
  DAT_0071f714 = (short)param_1;
  local_2 = (unsigned short)*(unsigned char *)(iVar8 + 3);
  __builtin_memcpy(&DAT_006a89f8, &DAT_00526872[(unsigned int)local_2 * 0x16], 4);
  if (local_2 == 0x59) {
    uVar11 = (unsigned short)(*(unsigned char *)(iVar8 + 0x19) >> 1);
  }
  else {
    if (DAT_006a89f8 == 0) {
      return;
    }
    DAT_006a89f4 = (unsigned short)*(unsigned char *)(iVar8 + 0x19);
    uVar11 = *(unsigned short *)(DAT_006a89f8 + (unsigned int)DAT_006a89f4 * 2);
  }
  if (uVar11 < 0xff00) {
    if (uVar11 < 0x8000) {
      if (local_2 == 0x59) {
        __builtin_memcpy(&iTmp, (unsigned char *)(iVar8 + 0x1f), 4);
        local_2 = (unsigned short)*(unsigned char *)(iTmp + 0x85);
      }
      DAT_006d498c = uVar11;
      FUN_00486750(iVar8, (unsigned int)uVar11);
      if (local_2 == 0x59) {
        DAT_006a89f4 = (unsigned short)*(unsigned char *)((unsigned int)DAT_006d4988 + 0x1au + (unsigned int)iVar10);
        uVar11 = *(unsigned short *)(&DAT_005281f8[(unsigned int)DAT_006a89f4 * 2]);
      }
    }
    if (((uVar11 < 0xff00) && (0x7fff < uVar11)) && (-1 < (int)DAT_006db7b4)) {
      iVar8 = DAT_0070a134;
      if (DAT_0070a134 < 0) {
        iVar8 = (int)(0u - (unsigned int)DAT_0070a134);
      }
      iVar9 = DAT_0070a00c;
      if (DAT_0070a00c < 0) {
        iVar9 = (int)(0u - (unsigned int)DAT_0070a00c);
      }
      uVar5 = DAT_0070a130;
      uVar6 = DAT_0070a010;
      if (iVar8 < iVar9) {
        uVar5 = DAT_0070a12c;
        uVar6 = DAT_0070a138;
      }
      if ((int)uVar5 < 0) {
        uVar4 = FUN_00419100(uVar6, 0u - uVar5);
        sVar2 = (short)uVar4;
      }
      else {
        uVar4 = FUN_00419100(uVar6, uVar5);
        sVar2 = (short)(0u - (unsigned int)(unsigned short)uVar4);
      }
      uVar5 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
      if (((int)(uVar5 & 0xffff0000u) < 1) && (-0x10001 < (int)(uVar5 & 0xffff0000u))) {
        uVar6 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
        if (((int)(uVar6 & 0xffff0000u) < 1) && (-0x10001 < (int)(uVar6 & 0xffff0000u))) {
          sVar7 = (short)(DAT_006d4984 - (short)uVar6);
          __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1fu + (unsigned int)iVar10), 4);
          bVar1 = *(unsigned char *)(iTmp + 1);
          if (bVar1 == 0) {
            uVar3 = 0x100;
          }
          else {
            uVar3 = (unsigned short)((unsigned short)bVar1 * 0x40);
            if (0xff < uVar3) {
              uVar3 = (unsigned short)(uVar3 + 0x100);
            }
          }
          if (DAT_006da9e2 < 0x20) {
            iVar10 = (int)DAT_006da9e2;
            pEntry = &DAT_0071eaf0[(unsigned int)iVar10 * 16u];
            __builtin_memcpy(pEntry + 0, &DAT_0071f714, 2);
            __builtin_memcpy(pEntry + 2, &uVar11, 2);
            __builtin_memcpy(pEntry + 4, &uVar3, 2);
            uVar6 = DAT_006db7b4;
            sTmp = (short)uVar5;
            __builtin_memcpy(pEntry + 6, &sTmp, 2);
            __builtin_memcpy(pEntry + 8, &sVar7, 2);
            DAT_006da9e2 = (short)(DAT_006da9e2 + 1);
            uTmp = uVar6;
            __builtin_memcpy(pEntry + 10, &uTmp, 4);
            __builtin_memcpy(pEntry + 14, &sVar2, 2);
          }
        }
      }
    }
  }
  return;
}
