/* SPDX-License-Identifier: MIT */
/* z_bop 00455c80 */
extern unsigned char DAT_009a20c0[];
extern short DAT_009a20ca[];
extern char DAT_009a20cc[];
extern char DAT_009a20d4[];
extern char DAT_009a20d8[];
extern char DAT_009a20dc[];
extern char DAT_009a20e0[];
extern char DAT_009a20e8[];
extern char DAT_009a20ea[];
extern char DAT_009a20ec[];
extern char DAT_009a20ee[];
extern char DAT_009a20f6[];
extern char DAT_009a2102[];
extern char DAT_009a2103[];
extern char DAT_009a2107[];
extern char DAT_009a2109[];
extern char DAT_009a210b[];
extern char DAT_009a210d[];
extern char DAT_009a2111[];
extern char DAT_009a2118[];
extern unsigned int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned int _DAT_00a07cec;
extern unsigned short DAT_009a1834;
extern char DAT_009d695b;
extern char DAT_009d738a[];
extern char DAT_009d738b[];
extern unsigned char DAT_009d8c83[];
extern unsigned char DAT_009d8c88[];
extern char DAT_009d8c93[];
extern char DAT_009d8cb8[];
extern char DAT_009d8cbc[];
extern char DAT_009d8ccd[];
extern char DAT_009d8ccf[];

unsigned int __cdecl FUN_00431b50(void *param_1, int param_2);
void __cdecl FUN_00433d30(unsigned short param_1, unsigned short param_2, int param_3);
void __cdecl FUN_00458780(int param_1);
void __cdecl FUN_00456040(void);

void __cdecl FUN_00455c80(unsigned short param_1)
{
  char cVar1;
  int iVar2;
  int bVar3;
  unsigned int uVar4;
  int iVar5;
  int iVar6;
  char cVar7;
  unsigned int uVar8;
  short sVar9;
  char *puVar10;
  int iVar11;
  char *pcVar12;
  char *pcTmp;
  int iTmp;

  uVar8 = (unsigned int)param_1;
  iVar5 = (int)(uVar8 * 0x126);
  if ((*(char *)((int)&DAT_009a20c0 + uVar8 * 0x126 + 1) != '\0') &&
      (DAT_009a20ca[uVar8 * 0x93] != 0)) {
    bVar3 = 1;
    if (_DAT_00a07cec < DAT_009a1fe0) {
      iVar11 = (int)(DAT_009a1fe0 - _DAT_00a07cec);
      puVar10 = (char *)(_DAT_00a07cec * 0x23 + 0x1f + DAT_009a1fe8);
      do {
        __builtin_memcpy(&pcTmp, puVar10, 4);
        if ((((*(char *)(puVar10 - 0x1c) != '\0') && (*pcTmp == '\0')) &&
             (*(unsigned char *)(puVar10 - 9) == uVar8))) {
          __builtin_memcpy(&iTmp, pcTmp + 0xa9, 4);
          cVar1 = *(char *)(iTmp + 7);
          if ((cVar1 != '\x03') && (cVar1 != '\x04')) {
            bVar3 = 0;
          }
        }
        puVar10 = puVar10 + 0x23;
        iVar11 = (int)((unsigned int)iVar11 - 1u);
      } while (iVar11 != 0);
    }
    if (bVar3) {
      iVar11 = (int)(uVar8 * 0x566);
      iVar6 = (unsigned int)DAT_009d8c88[iVar11] * 3;
      if ((((DAT_009d738a[(unsigned int)DAT_009d8c88[iVar11] * 3] != '\x01') ||
            ((DAT_009d738b[iVar6] != '\x01') && (DAT_009d738b[iVar6] != '\x02')))) &&
          ((DAT_009d738a[(unsigned int)DAT_009d8c88[iVar11] * 3] != '\x02') ||
           (DAT_009d738b[iVar6] != '\x01'))) {
        bVar3 = 0;
        if (DAT_009d695b != '\x02') {
          if (((DAT_009d8cb8[iVar11] != '\0') || (DAT_009d8cbc[iVar11] != '\0')) &&
              (uVar4 = FUN_00431b50(DAT_009d8cb8 + iVar11, 0), (uVar4 & 1) != 0)) {
            bVar3 = 1;
          }
          cVar1 = DAT_009d8c93[iVar11];
          if ((cVar1 == '\x01') && (*(short *)(DAT_009a20ea + iVar5) != 0)) {
            bVar3 = 1;
          }
          if ((cVar1 == '\x02') &&
              (DAT_009d738a[(unsigned int)DAT_009d8c88[iVar11] * 3] == '\x01')) {
            bVar3 = 1;
          }
          if ((cVar1 == '\x03') &&
              ((DAT_009d738a[(unsigned int)DAT_009d8c88[iVar11] * 3] == '\x02' ||
                (DAT_009d738b[(unsigned int)DAT_009d8c88[iVar11] * 3] == '\x01')))) {
            bVar3 = 1;
          }
        }
        if (bVar3) {
          sVar9 = (short)(DAT_009a20ca[uVar8 * 0x93] - *(short *)(DAT_009a20cc + iVar5));
          *(short *)(DAT_009a20cc + iVar5) = (short)(*(short *)(DAT_009a20cc + iVar5) + sVar9);
          cVar7 = (char)(DAT_009a2102[iVar5] - DAT_009a2103[iVar5]);
          *(short *)(DAT_009a20f6 + iVar5) = (short)(*(short *)(DAT_009a20f6 + iVar5) + sVar9);
          DAT_009a2118[iVar5] = (char)(DAT_009a2118[iVar5] + cVar7);
          *(short *)(DAT_009a20dc + iVar5) = (short)(*(short *)(DAT_009a20dc + iVar5) + sVar9);
          DAT_009a210b[iVar5] = (char)(DAT_009a210b[iVar5] + cVar7);
          *(short *)(DAT_009a20e8 + iVar5) = (short)(*(short *)(DAT_009a20e8 + iVar5) + sVar9);
          DAT_009a2111[iVar5] = (char)(DAT_009a2111[iVar5] + cVar7);
          *(short *)(DAT_009a20d8 + iVar5) = (short)(*(short *)(DAT_009a20d8 + iVar5) + sVar9);
          DAT_009a2109[iVar5] = (char)(DAT_009a2109[iVar5] + cVar7);
          *(short *)(DAT_009a20d4 + iVar5) = (short)(*(short *)(DAT_009a20d4 + iVar5) + sVar9);
          DAT_009a2107[iVar5] = (char)(DAT_009a2107[iVar5] + cVar7);
          *(short *)(DAT_009a20e0 + iVar5) = (short)(*(short *)(DAT_009a20e0 + iVar5) + sVar9);
          cVar1 = DAT_009d8ccd[iVar11];
          DAT_009a210d[iVar5] = (char)(DAT_009a210d[iVar5] + cVar7);
          *(unsigned char *)((int)&DAT_009a20c0 + iVar5 + 1) = 0;
          if ((cVar1 == '\0') && (DAT_009d8ccf[iVar11] == '\0')) {
            *(short *)(DAT_009a20ec + iVar5) = (short)(*(short *)(DAT_009a20ec + iVar5) + sVar9);
            return;
          }
          *(short *)(DAT_009a20ee + iVar5) = (short)(*(short *)(DAT_009a20ee + iVar5) + sVar9);
          return;
        }
        uVar8 = 0;
        DAT_009a1834 = param_1;
        if (_DAT_00a07cec < DAT_009a1fe0) {
          iVar5 = (int)(DAT_009a1fe0 - _DAT_00a07cec);
          pcVar12 = (char *)(_DAT_00a07cec * 0x23 + 3 + DAT_009a1fe8);
          do {
            if (*pcVar12 == '\0') {
              uVar8 = uVar8 + 1;
            }
            pcVar12 = pcVar12 + 0x23;
            iVar5 = (int)((unsigned int)iVar5 - 1u);
          } while (iVar5 != 0);
        }
        if (uVar8 < DAT_009d8c83[iVar11]) {
          iVar5 = (int)(DAT_009d8c83[iVar11] - uVar8);
          if (_DAT_00a07cec < DAT_009a1fe0) {
            iVar11 = (int)(_DAT_00a07cec * 0x23);
            uVar8 = _DAT_00a07cec;
            do {
              iVar6 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar11);
              __builtin_memcpy(&iTmp, (char *)iVar6 + 0x1b, 4);
              if (((*(char *)(iVar6 + 3) != '\0') && (*(char *)(iVar6 + 2) == '\r')) &&
                  (iTmp == -1)) {
                *(unsigned char *)(iVar6 + 3) = 0;
                FUN_00433d30((unsigned short)uVar8,
                             (unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar11), 2);
                iVar5 = (int)((unsigned int)iVar5 - 1u);
                if (iVar5 == 0) break;
              }
              iVar11 = (int)((unsigned int)iVar11 + 0x23u);
              uVar8 = uVar8 + 1;
            } while (uVar8 < DAT_009a1fe0);
          }
          if ((iVar5 != 0) && (_DAT_00a07cec < DAT_009a1fe0)) {
            iVar11 = (int)(_DAT_00a07cec * 0x23);
            uVar8 = _DAT_00a07cec;
            do {
              iVar6 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar11);
              __builtin_memcpy(&pcTmp, (char *)iVar6 + 0x1f, 4);
              __builtin_memcpy(&iTmp, (char *)iVar6 + 0x1b, 4);
              if (((*(char *)(iVar6 + 3) != '\0') && (*pcTmp == '\0')) && (iTmp == -1)) {
                __builtin_memcpy(&iVar2, pcTmp + 0xa9, 4);
                cVar1 = *(char *)(iVar2 + 7);
                if ((cVar1 == '\x03') || (cVar1 == '\x04')) {
                  *(unsigned char *)(iVar6 + 3) = 0;
                  FUN_00458780(iVar2);
                  FUN_00433d30((unsigned short)uVar8,
                               (unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar11), 2);
                  iVar5 = (int)((unsigned int)iVar5 - 1u);
                  if (iVar5 == 0) break;
                }
              }
              iVar11 = (int)((unsigned int)iVar11 + 0x23u);
              uVar8 = uVar8 + 1;
            } while (uVar8 < DAT_009a1fe0);
          }
        }
        FUN_00456040();
      }
    }
  }
  return;
}
