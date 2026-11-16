/* SPDX-License-Identifier: MIT */
/* z_bop 00451940 */
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned char DAT_009d8ccb[];
extern unsigned char DAT_009d8c83[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_009d8c30[];
extern int DAT_009ecc34;
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e967a[];
extern unsigned char DAT_0051c56a[];
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern unsigned int DAT_009a8c00;
extern unsigned short DAT_00a07be0;
extern unsigned short DAT_00a08292;
extern short DAT_00a07be6;
extern short DAT_00a07be4;

void __cdecl FUN_00458e20(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_0046a8d0(int param_1, int param_2, int param_3);
void __cdecl FUN_00451bf0(int param_1, unsigned int param_2);
void __cdecl FUN_00450650(int param_1, int param_2);

void __cdecl FUN_00451940(unsigned int param_1, unsigned int param_2)
{
  int iVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar4;
  unsigned short uVar5;
  short sVar6;
  unsigned int uVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int uTmp;
  int iTmp;
  unsigned int uOff;
  unsigned short usTmp;

  uVar7 = param_1 & 0xffff;
  iVar1 = (int)(uVar7 * 0x566u);
  uVar5 = DAT_00a07cec;
  if (DAT_009d8ccb[iVar1] == '\0') {
    uVar9 = 0x8000;
LAB_004519e8:
    FUN_00458e20(uVar9, uVar7);
  }
  else {
    for (; (int)(unsigned int)uVar5 < DAT_009a1fe0; uVar5 = (unsigned short)(uVar5 + 1)) {
      unsigned char *pEntry;
      unsigned char *pInner;
      int pPtr;
      iVar8 = (int)((unsigned int)uVar5 * 0x23u + (unsigned int)DAT_009a1fe8);
      pEntry = (unsigned char *)iVar8;
      if ((pEntry[3] != 0) && ((unsigned int)pEntry[0x16] == uVar7)) {
        __builtin_memcpy(&pPtr, pEntry + 0x1f, 4);
        __builtin_memcpy(&pPtr, (unsigned char *)(pPtr + 0xa9), 4);
        pInner = (unsigned char *)pPtr;
        if ((char)pInner[5] == -1) {
          uVar9 = (unsigned int)uVar5;
          goto LAB_004519e8;
        }
      }
    }
  }
  uOff = (unsigned int)DAT_009ecc34 * 0x5bdu;
  iVar8 = (int)uOff;
  if (DAT_009e9683[iVar8] == '\0') {
    __builtin_memcpy(&iTmp, DAT_009e9670 + iVar8, 4);
    iVar8 = (int)((unsigned int)iTmp * 0x23u + (unsigned int)DAT_009a1fe8);
    __builtin_memcpy(&iVar3, (unsigned char *)(iVar8 + 0xc), 4);
    __builtin_memcpy(&iVar4, (unsigned char *)(iVar8 + 8), 4);
    __builtin_memcpy(&iTmp, (unsigned char *)(iVar8 + 4), 4);
    iVar8 = iTmp;
  }
  else {
    __builtin_memcpy(&iVar3, DAT_009e9a63 + iVar8, 4);
    __builtin_memcpy(&iVar4, DAT_009e9a5f + iVar8, 4);
    __builtin_memcpy(&iTmp, DAT_009e9a5b + iVar8, 4);
    iVar8 = iTmp;
  }
  FUN_0046a8d0((int)((unsigned int)DAT_00a07cdc - (unsigned int)iVar8),
               (int)((unsigned int)DAT_00a07cd8 - (unsigned int)iVar4),
               (int)((unsigned int)DAT_00a07cd4 - (unsigned int)iVar3));
  DAT_009a8c00 = DAT_009a8c00 * 0xa1u;
  sVar6 = (short)(((unsigned long long)((DAT_009a8c00 >> 0x10) + 0x32)) / 100);
  if (sVar6 == 0) {
    sVar6 = 1;
  }
  bVar2 = DAT_009d8c83[iVar1];
  DAT_00a07be0 = (unsigned short)bVar2;
  DAT_00a08292 = (unsigned short)DAT_009d8c87[iVar1];
  __builtin_memcpy(&usTmp, DAT_009e967a + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 2);
  if (usTmp == DAT_00a08292) {
    if (DAT_00a07be0 != 1) {
      __builtin_memcpy(&uTmp, DAT_0051c56a + (int)((param_2 & 0xffff) * 0x101u), 4);
      FUN_00451bf0(1, uTmp);
      FUN_00451bf0(2, (unsigned int)((int)DAT_009d8c30 + iVar1));
      DAT_00a07be6 = sVar6;
      FUN_00450650(0xec, DAT_009ecc34);
      return;
    }
    __builtin_memcpy(&uTmp, DAT_0051c56a + (int)((param_2 & 0xffff) * 0x101u), 4);
    FUN_00451bf0(0, uTmp);
    FUN_00451bf0(1, (unsigned int)((int)DAT_009d8c30 + iVar1));
    DAT_00a07be4 = sVar6;
    FUN_00450650(0xeb, DAT_009ecc34);
    return;
  }
  __builtin_memcpy(&uTmp, DAT_0051c56a + (int)((param_2 & 0xffff) * 0x101u), 4);
  FUN_00451bf0(1, uTmp);
  if (bVar2 != 1) {
    DAT_00a07be4 = sVar6;
    FUN_00450650(0x73, DAT_009ecc34);
    return;
  }
  DAT_00a07be4 = sVar6;
  FUN_00450650(0x72, DAT_009ecc34);
  return;
}
