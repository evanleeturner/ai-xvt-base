/* SPDX-License-Identifier: MIT */
/* z_xvt 004479a0 */
extern int DAT_006d4988;
extern int DAT_00732e54;
extern int DAT_006db798;
extern int DAT_0071b704;
extern int DAT_0071ecf4;
extern char s_w1pln_00525b86[];
extern unsigned int DAT_00709310[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070b215[];
extern unsigned char DAT_0071b7bd[];
extern unsigned char DAT_0071b79d[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b759[];
extern unsigned char DAT_0071b776[];
extern unsigned char DAT_0071b777[];
extern unsigned char DAT_0071bcdd[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0052b0b4[];

int __cdecl FUN_0042b150(int param_1);
unsigned int __cdecl FUN_00447cf0(int param_1, int param_2, int param_3);
void __cdecl FUN_0040a3c0(unsigned int param_1, int param_2);
void __cdecl FUN_004a1d70(int param_1, int param_2);
void __cdecl FUN_0040b730(int param_1, int param_2, int param_3, int param_4, unsigned int param_5, int param_6);

unsigned int __cdecl FUN_004479a0(int param_1, int param_2, int param_3, unsigned int param_4)
{
  int iVar1;
  unsigned char *pcVar2;
  unsigned char *pbVar3;
  unsigned short uVar4;
  int iVar5;
  int iVar6;
  unsigned short uVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned short sVar;
  int iVal;
  unsigned int uIdx;

  iVar1 = (int)((unsigned int)param_1 * 0x23u);
  __builtin_memcpy(&iVar5, (unsigned char *)(DAT_006d4988 + 0x1b + iVar1), 4);
  iVar8 = FUN_0042b150(DAT_00732e54);
  uVar9 = 0xffffffff;
  if ((*(char *)(DAT_00732e54 + 0x344 + param_2 * 4) != '\0') &&
      (*(char *)(DAT_00732e54 + param_2 * 4 + 0x346) != '\0')) {
    uVar9 = FUN_00447cf0(param_1, param_2, param_3);
    if (uVar9 != 0xffffffff) {
      *(char *)(DAT_00732e54 + 0x24a) = (char)(*(char *)(DAT_00732e54 + 0x24a) + 1);
      if (iVar5 != -1) {
        uVar4 = *(unsigned short *)(s_w1pln_00525b86 + param_3 * 2);
        __builtin_memcpy(&sVar, DAT_0071b7bd + (unsigned int)iVar5 * 0x5bdu, 2);
        sVar = (unsigned short)(sVar + 1);
        __builtin_memcpy(DAT_0071b7bd + (unsigned int)iVar5 * 0x5bdu, &sVar, 2);
        __builtin_memcpy(&uVar10, DAT_0071b79d + (unsigned int)iVar5 * 0x5bdu, 4);
        uVar10 = uVar10 - (unsigned int)uVar4;
        __builtin_memcpy(DAT_0071b79d + (unsigned int)iVar5 * 0x5bdu, &uVar10, 4);
      }
      uIdx = DAT_0070ad48[(unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar1) * 0x566u];
      DAT_00709310[uIdx] =
          DAT_00709310[uIdx] - (unsigned int)*(unsigned short *)(s_w1pln_00525b86 + param_3 * 2);
      FUN_0040a3c0(uVar9, iVar5);
      iVar1 = (int)((unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar1) * 0x566u);
      if ((DAT_0070ad44[iVar1] != 0x15) && (DAT_0070b215[iVar1] != 0x15)) {
        pcVar2 = (unsigned char *)(DAT_00732e54 + 0x346 + param_2 * 4);
        *pcVar2 = (unsigned char)(*pcVar2 - 1);
      }
      uVar9 = uVar9 - (unsigned int)DAT_0071b704;
      if (param_4 < 2) {
        short s231;
        __builtin_memcpy(&s231, (unsigned char *)(DAT_00732e54 + 0x231), 2);
        *(char *)(DAT_006db798 + 1 + (int)(uVar9 * 8u)) = (char)(s231 / 0xec);
        if (6 < *(unsigned char *)(DAT_006db798 + 1 + (int)(uVar9 * 8u))) {
          *(unsigned char *)(DAT_006db798 + 1 + (int)(uVar9 * 8u)) = 6;
        }
      }
      iVar1 = (int)(uVar9 * 8u);
      if (iVar5 == -1) {
        *(unsigned short *)(iVar1 + 4 + DAT_006db798) = *(unsigned short *)(iVar8 + 0x18);
        uVar7 = *(unsigned short *)(iVar8 + 0x1c);
      }
      else {
        __builtin_memcpy(&uVar7, DAT_0071b74a + (unsigned int)iVar5 * 0x5bdu, 2);
        *(unsigned short *)(iVar1 + 4 + DAT_006db798) = uVar7;
        __builtin_memcpy(&uVar7, DAT_0071b759 + (unsigned int)iVar5 * 0x5bdu, 2);
      }
      *(unsigned short *)(iVar1 + 2 + DAT_006db798) = uVar7;
      *(char *)(iVar1 + DAT_006db798) = (char)iVar5;
      iVar8 = DAT_0071ecf4;
      iVar5 = DAT_0071b704;
      uVar4 = *(unsigned short *)(iVar1 + 4 + DAT_006db798);
      if (uVar4 != 0xffff) {
        __builtin_memcpy(&iVal, (unsigned char *)((unsigned int)uVar4 * 0x23u + 0x1bu + (unsigned int)DAT_006d4988), 4);
        if (iVal != -1) {
          unsigned short uVar12 = *(unsigned short *)(DAT_006db798 + 4 + (int)((uVar9 & 0xffff) * 8u));
          __builtin_memcpy(&iVar1, (unsigned char *)((unsigned int)uVar12 * 0x23u + 0x1bu + (unsigned int)DAT_006d4988), 4);
          if (iVar1 != -1) {
            iVar6 = (int)((unsigned int)iVar1 * 0x5bdu);
            if (DAT_0071b776[iVar6] == '\0') {
              DAT_0071b776[iVar6] = 1;
              sVar = (unsigned short)((unsigned short)iVar5 + (unsigned short)uVar9);
              __builtin_memcpy(DAT_0071b777 + iVar6, &sVar, 2);
              uVar11 = 0x588;
              __builtin_memcpy(DAT_0071bcdd + iVar6, &uVar11, 4);
              if (iVar8 == iVar1) {
                FUN_004a1d70(0x73, iVar1);
                __builtin_memcpy(&uVar10, DAT_0071b730 + iVar6, 4);
                FUN_0040b730(DAT_0071ecf4, -1, 0xc, -1, uVar10, 0xffff);
              }
            }
          }
        }
      }
      if (param_4 < 2) {
        uIdx = (unsigned int)DAT_0052b0b4[param_4 + (unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xefu];
        if (*(unsigned char *)(DAT_00732e54 + 0x34a + (int)(uIdx * 4u)) <=
            *(unsigned char *)(DAT_00732e54 + 0x346 + (int)(uIdx * 4u))) {
          *(unsigned char *)(DAT_00732e54 + 0x22b + (int)param_4) =
              (unsigned char)(*(unsigned char *)(DAT_00732e54 + 0x22b + (int)param_4) & 0x7f);
          return uVar9;
        }
        pbVar3 = (unsigned char *)(DAT_00732e54 + 0x22b + (int)param_4);
        *pbVar3 = (unsigned char)(*pbVar3 | 0x80);
      }
    }
  }
  return uVar9;
}
