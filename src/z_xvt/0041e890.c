/* SPDX-License-Identifier: MIT */
/* z_xvt 0041e890 */
extern unsigned char DAT_006d4a61[];
extern short DAT_006d4a6a[];
extern short DAT_006d4a8c[];
extern short DAT_006d4a8e[];
extern unsigned char DAT_006d4a6c[];
extern unsigned char DAT_006d4a74[];
extern unsigned char DAT_006d4a78[];
extern unsigned char DAT_006d4a7c[];
extern unsigned char DAT_006d4a80[];
extern unsigned char DAT_006d4a88[];
extern unsigned char DAT_006d4a8a[];
extern unsigned char DAT_006d4a96[];
extern char DAT_006d4aa2[];
extern char DAT_006d4aa3[];
extern char DAT_006d4aa7[];
extern char DAT_006d4aa9[];
extern char DAT_006d4aab[];
extern char DAT_006d4aad[];
extern char DAT_006d4ab1[];
extern char DAT_006d4ab8[];
extern unsigned int DAT_006d4980;
extern unsigned char *DAT_006d4988;
extern unsigned int DAT_00732a3c;
extern unsigned short DAT_0073b374;
extern char DAT_007092db;
extern char DAT_00709d0a[];
extern char DAT_00709d0b[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad48[];
extern char DAT_0070ad53[];
extern char DAT_0070ad54[];
extern char DAT_0070ad78[];
extern char DAT_0070ad7c[];
extern char DAT_0070ad8d[];
extern char DAT_0070ad8f[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052687a[];

unsigned int FUN_0049d810(void *param_1, int param_2);
void FUN_0049f9c0(unsigned short param_1, unsigned short param_2, int param_3);
void FUN_0041eea0(unsigned short param_1);
void FUN_004215c0(unsigned short param_1);

void __cdecl FUN_0041e890(unsigned short param_1)
{
  char cVar1;
  int iVar2;
  int bVar3;
  unsigned int uVar4;
  int iVar5;
  char cVar6;
  unsigned char *piVar7;
  short sVar8;
  unsigned int uVar9;
  unsigned char *puVar10;
  char *pcVar11;
  int iVar12;
  int iStack_4;
  unsigned char *pObj;
  unsigned char *pTmp;

  uVar9 = (unsigned int)param_1;
  iVar5 = (int)(uVar9 * 0x126);
  if ((DAT_006d4a61[uVar9 * 0x126] != '\0') && (DAT_006d4a6a[uVar9 * 0x93] != 0)) {
    bVar3 = 1;
    if (DAT_00732a3c < DAT_006d4980) {
      iVar12 = (int)(DAT_006d4980 - DAT_00732a3c);
      puVar10 = DAT_006d4988 + (DAT_00732a3c * 0x23 + 0x1f);
      do {
        __builtin_memcpy(&pObj, puVar10, 4);
        if ((puVar10[-0x1c] != 0) && (*pObj == 0) &&
            (puVar10[-9] == uVar9)) {
          __builtin_memcpy(&pTmp, pObj + 0xa7, 4);
          cVar1 = (char)pTmp[7];
          if (cVar1 != 3 && cVar1 != 4) {
            bVar3 = 0;
          }
        }
        puVar10 = puVar10 + 0x23;
        iVar12 = iVar12 - 1;
      } while (iVar12 != 0);
    }
    if (bVar3) {
      iVar12 = (int)(uVar9 * 0x566);
      iVar2 = (int)((unsigned int)DAT_0070ad48[iVar12] * 3);
      if ((((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar12] * 3] != '\x01') ||
          ((DAT_00709d0b[iVar2] != '\x01' && (DAT_00709d0b[iVar2] != '\x02')))) &&
         ((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar12] * 3] != '\x02' ||
          (DAT_00709d0b[iVar2] != '\x01'))))) {
        bVar3 = 0;
        if (DAT_007092db != '\x02') {
          if (((DAT_0070ad78[iVar12] != '\0') || (DAT_0070ad7c[iVar12] != '\0')) &&
             (uVar4 = FUN_0049d810(DAT_0070ad78 + iVar12, 0), (uVar4 & 1) != 0)) {
            bVar3 = 1;
          }
          cVar1 = DAT_0070ad53[iVar12];
          if ((cVar1 == '\x01') && (*(short *)(DAT_006d4a8a + iVar5) != 0)) {
            bVar3 = 1;
          }
          if ((cVar1 == '\x02') &&
             (DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar12] * 3] == '\x01')) {
            bVar3 = 1;
          }
          if ((cVar1 == '\x03') &&
             ((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar12] * 3] == '\x02' ||
              (DAT_00709d0b[(unsigned int)DAT_0070ad48[iVar12] * 3] == '\x01')))) {
            bVar3 = 1;
          }
        }
        if (bVar3) {
          sVar8 = (short)(DAT_006d4a6a[uVar9 * 0x93] - *(short *)(DAT_006d4a6c + iVar5));
          *(short *)(DAT_006d4a6c + iVar5) = (short)(*(short *)(DAT_006d4a6c + iVar5) + sVar8);
          cVar6 = (char)(DAT_006d4aa2[iVar5] - DAT_006d4aa3[iVar5]);
          *(short *)(DAT_006d4a96 + iVar5) = (short)(*(short *)(DAT_006d4a96 + iVar5) + sVar8);
          DAT_006d4ab8[iVar5] = (char)(DAT_006d4ab8[iVar5] + cVar6);
          *(short *)(DAT_006d4a7c + iVar5) = (short)(*(short *)(DAT_006d4a7c + iVar5) + sVar8);
          DAT_006d4aab[iVar5] = (char)(DAT_006d4aab[iVar5] + cVar6);
          *(short *)(DAT_006d4a88 + iVar5) = (short)(*(short *)(DAT_006d4a88 + iVar5) + sVar8);
          DAT_006d4ab1[iVar5] = (char)(DAT_006d4ab1[iVar5] + cVar6);
          *(short *)(DAT_006d4a78 + iVar5) = (short)(*(short *)(DAT_006d4a78 + iVar5) + sVar8);
          DAT_006d4aa9[iVar5] = (char)(DAT_006d4aa9[iVar5] + cVar6);
          *(short *)(DAT_006d4a74 + iVar5) = (short)(*(short *)(DAT_006d4a74 + iVar5) + sVar8);
          DAT_006d4aa7[iVar5] = (char)(DAT_006d4aa7[iVar5] + cVar6);
          *(short *)(DAT_006d4a80 + iVar5) = (short)(*(short *)(DAT_006d4a80 + iVar5) + sVar8);
          cVar1 = DAT_0070ad8d[iVar12];
          DAT_006d4aad[iVar5] = (char)(DAT_006d4aad[iVar5] + cVar6);
          DAT_006d4a61[iVar5] = 0;
          if ((cVar1 == '\0') && (DAT_0070ad8f[iVar12] == '\0')) {
            DAT_006d4a8c[uVar9 * 0x93] = (short)(DAT_006d4a8c[uVar9 * 0x93] + sVar8);
            return;
          }
          DAT_006d4a8e[uVar9 * 0x93] = (short)(DAT_006d4a8e[uVar9 * 0x93] + sVar8);
          return;
        }
        uVar9 = 0;
        DAT_0073b374 = param_1;
        if (DAT_00732a3c < DAT_006d4980) {
          iVar5 = (int)(DAT_006d4980 - DAT_00732a3c);
          pcVar11 = (char *)(DAT_006d4988 + (DAT_00732a3c * 0x23 + 3));
          do {
            if (*pcVar11 == '\0') {
              uVar9 = uVar9 + 1;
            }
            pcVar11 = pcVar11 + 0x23;
            iVar5 = iVar5 - 1;
          } while (iVar5 != 0);
        }
        if (uVar9 < DAT_0070ad43[iVar12]) {
          iStack_4 = (int)(DAT_0070ad43[iVar12] - uVar9);
          if (DAT_00732a3c < DAT_006d4980) {
            iVar5 = (int)(DAT_00732a3c * 0x23);
            uVar9 = DAT_00732a3c;
            do {
              int iEnt;
              int iLink;
              iEnt = (int)DAT_006d4988 + iVar5;
              __builtin_memcpy(&iLink, (unsigned char *)iEnt + 0x1b, 4);
              if (((*(char *)(iEnt + 3) != '\0') && (*(char *)(iEnt + 2) == '\r')) &&
                 (iLink == -1)) {
                *(unsigned char *)(iEnt + 3) = 0;
                FUN_0049f9c0((unsigned short)uVar9,
                             (unsigned short)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar5), 2);
                iStack_4 = iStack_4 - 1;
                if (iStack_4 == 0) break;
              }
              iVar5 = iVar5 + 0x23;
              uVar9 = uVar9 + 1;
            } while (uVar9 < DAT_006d4980);
          }
          if ((iStack_4 != 0) && (DAT_00732a3c < DAT_006d4980)) {
            iVar5 = (int)(DAT_00732a3c * 0x23);
            uVar9 = DAT_00732a3c;
            do {
              unsigned char *pEnt;
              int iLink;
              pEnt = DAT_006d4988 + iVar5;
              __builtin_memcpy(&iLink, pEnt + 0x1b, 4);
              __builtin_memcpy(&pObj, pEnt + 0x1f, 4);
              if (((pEnt[3] != 0) && (*pObj == 0)) && (iLink == -1)) {
                unsigned char *pSub;
                unsigned char *pPtr;
                __builtin_memcpy(&pSub, pObj + 0xa7, 4);
                if ((pSub[7] == 3) || (pSub[7] == 4)) {
                  pEnt[3] = 0;
                  __builtin_memcpy(&pPtr, pSub + 0x432, 4);
                  if (pPtr != 0) {
                    pPtr[3] = 0;
                    pPtr = 0;
                    __builtin_memcpy(pSub + 0x432, &pPtr, 4);
                  }
                  piVar7 = pSub + 0x3f2;
                  iVar12 = 0x10;
                  do {
                    __builtin_memcpy(&pPtr, piVar7, 4);
                    if (pPtr != 0) {
                      pPtr[3] = 0;
                      pPtr = 0;
                      __builtin_memcpy(piVar7, &pPtr, 4);
                    }
                    piVar7 = piVar7 + 4;
                    iVar12 = iVar12 - 1;
                  } while (iVar12 != 0);
                  FUN_0049f9c0((unsigned short)uVar9,
                               (unsigned short)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar5), 2);
                  iStack_4 = iStack_4 - 1;
                  if (iStack_4 == 0) break;
                }
              }
              iVar5 = iVar5 + 0x23;
              uVar9 = uVar9 + 1;
            } while (uVar9 < DAT_006d4980);
          }
        }
        if ((DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)DAT_0073b374 * 0x566]] * 0x16]
            & 0x80) == 0) {
          FUN_0041eea0(0xffff);
        }
        else {
          FUN_004215c0(0xffff);
        }
        uVar9 = (unsigned int)DAT_0073b374;
        cVar1 = DAT_006d4a61[uVar9 * 0x126];
        if (cVar1 != '\0') {
          if (DAT_0070ad54[uVar9 * 0x566] == '\0') {
            DAT_006d4a61[uVar9 * 0x126] = (unsigned char)(cVar1 - 1);
          }
          else if (DAT_007092db != '\x02') {
            DAT_006d4a61[uVar9 * 0x126] = (unsigned char)(cVar1 - 1);
            return;
          }
        }
      }
    }
  }
  return;
}
