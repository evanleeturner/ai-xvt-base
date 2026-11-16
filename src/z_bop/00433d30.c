/* SPDX-License-Identifier: MIT */
/* z_bop 00433d30 */
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern short DAT_009a2000;
extern int _DAT_00a07cec;
extern unsigned short DAT_009a20ca[];
extern unsigned char DAT_009a2102[];
extern unsigned char DAT_009a20d6[];
extern unsigned char DAT_009a2108[];
extern unsigned char DAT_009a20f6[];
extern unsigned char DAT_009a2118[];
extern unsigned char DAT_009a20f4[];
extern unsigned char DAT_009a2117[];
extern unsigned char DAT_009a20fa[];
extern unsigned char DAT_009a211a[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d738a[];
extern unsigned char DAT_009a2132[];
extern unsigned char DAT_009a213c[];
extern unsigned char DAT_009a20e8[];
extern unsigned char DAT_009a2111[];
extern unsigned char DAT_009a20d8[];
extern unsigned char DAT_009a2109[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009a215a[];
extern unsigned char DAT_009a2164[];
extern unsigned char DAT_009a20d4[];
extern unsigned char DAT_009a2107[];
extern unsigned char DAT_009a20e0[];
extern unsigned char DAT_009a210d[];
extern unsigned char DAT_009a20e4[];
extern unsigned char DAT_009a210f[];
extern unsigned char DAT_009d8ccb[];
extern unsigned char DAT_009d8cca[];
extern unsigned char DAT_009d8ccd[];
extern unsigned char DAT_009d8ccc[];
extern unsigned char DAT_009d8ccf[];
extern unsigned char DAT_009d8cce[];
extern unsigned char DAT_009d8cd1[];
extern unsigned char DAT_009d8cd0[];
extern unsigned char DAT_009d8cd2[];
extern unsigned char DAT_009a20d0[];
extern unsigned char DAT_009a20ee[];
extern unsigned char DAT_009a20f0[];
extern unsigned char DAT_009a20f2[];
extern unsigned char DAT_009a20ec[];
extern unsigned char DAT_00524120[];
extern unsigned char DAT_009a7a40[];
extern unsigned char DAT_009d1320[];
extern char s_dropoffldr1pln_0051a2c4[];
extern char DAT_009e9680[];
extern char DAT_009ec468[];

void __cdecl FUN_00434440(unsigned int param_1);

void __cdecl FUN_00433d30(unsigned short param_1, unsigned short param_2, unsigned short param_3)
{
  char cVar1;
  unsigned char bVar2;
  unsigned int uVar3;
  int iVar4;
  int iVar5;
  int iVar7;
  char *pcVar8;
  unsigned char *pbVar9;
  unsigned short uVar10;
  unsigned int uVar11;
  char *pcVar12;
  int bVar13;
  int local_4;
  int iTmp;
  int iTmp2;
  unsigned short usTmp;

  __builtin_memcpy(&iTmp, (void *)(DAT_009a1fe8 + 0x1f + (unsigned int)param_1 * 0x23), 4);
  __builtin_memcpy(&iVar5, (void *)(iTmp + 0xa9), 4);
  if (*(char *)(iVar5 + 8) != '\x01') {
    uVar3 = (unsigned int)param_2;
    *(unsigned char *)(iVar5 + 8) = 1;
    DAT_009a20ca[(unsigned int)param_3 + uVar3 * 0x93] =
         DAT_009a20ca[(unsigned int)param_3 + uVar3 * 0x93] + 1;
    iVar7 = uVar3 * 0x566;
    cVar1 = DAT_009d8c80[iVar7];
    if (*(char *)(iVar5 + 0xb0) == cVar1) {
      DAT_009a2102[uVar3 * 0x126 + (unsigned int)param_3] = 1;
    }
    if (param_3 == 2) {
      if (*(char *)(iVar5 + 0x1eb) != '\0') {
        *(short *)(DAT_009a20d6 + uVar3 * 0x126) = *(short *)(DAT_009a20d6 + uVar3 * 0x126) + -1;
        if (*(char *)(iVar5 + 0xb0) == cVar1) {
          DAT_009a2108[uVar3 * 0x126] = 0;
        }
        *(unsigned char *)(iVar5 + 0x1eb) = 0;
      }
      if (*(char *)(iVar5 + 0x74) != '\0') {
        *(short *)(DAT_009a20f6 + uVar3 * 0x126) = *(short *)(DAT_009a20f6 + uVar3 * 0x126) + -1;
        if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
          DAT_009a2118[uVar3 * 0x126] = 0;
        }
      }
      if (*(char *)(iVar5 + 0x73) != '\0') {
        *(short *)(DAT_009a20f4 + uVar3 * 0x126) = *(short *)(DAT_009a20f4 + uVar3 * 0x126) + -1;
        if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
          DAT_009a2117[uVar3 * 0x126] = 0;
        }
      }
      *(short *)(DAT_009a20fa + uVar3 * 0x126) = *(short *)(DAT_009a20fa + uVar3 * 0x126) + 1;
      if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
        DAT_009a211a[uVar3 * 0x126] = 1;
      }
    }
    else {
      iVar4 = DAT_009a1fe8 + (unsigned int)param_1 * 0x23;
      __builtin_memcpy(&iTmp, (void *)(iVar4 + 0x1b), 4);
      __builtin_memcpy(&iTmp2, (void *)(iVar4 + 0x1f), 4);
      if ((((iTmp != -1) && (*(char *)(iVar5 + 0x74) == '\0')) &&
          (*(char *)(iVar5 + 0x73) == '\0')) &&
         (DAT_009d738a[(unsigned int)*(unsigned char *)(iTmp2 + 0x89) * 3] != '\x01')) {
        *(short *)(DAT_009a20f6 + uVar3 * 0x126) = *(short *)(DAT_009a20f6 + uVar3 * 0x126) + 1;
        if (*(char *)(iVar5 + 0xb0) == cVar1) {
          DAT_009a2118[uVar3 * 0x126] = 1;
        }
      }
    }
    uVar11 = 0;
    do {
      if (*(char *)(uVar11 + 0x1f6 + iVar5) == '\0') {
        DAT_009a2132[uVar3 * 0x126 + uVar11] = DAT_009a2132[uVar3 * 0x126 + uVar11] + '\x01';
        if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
          DAT_009a213c[uVar3 * 0x126 + uVar11] = 1;
        }
      }
      uVar10 = (unsigned short)uVar11 + 1;
      uVar11 = (unsigned int)uVar10;
    } while (uVar10 < 10);
    if (*(char *)(iVar5 + 0x1ea) == '\0') {
      *(short *)(DAT_009a20e8 + uVar3 * 0x126) = *(short *)(DAT_009a20e8 + uVar3 * 0x126) + 1;
      if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
        DAT_009a2111[uVar3 * 0x126] = 1;
      }
    }
    if (*(char *)(iVar5 + 0x1eb) == '\0') {
      cVar1 = DAT_009d8c80[iVar7];
      *(short *)(DAT_009a20d8 + uVar3 * 0x126) = *(short *)(DAT_009a20d8 + uVar3 * 0x126) + 1;
      iVar4 = uVar3 * 0x126;
      if (cVar1 == *(char *)(iVar5 + 0xb0)) {
        DAT_009a2109[iVar4] = 1;
      }
      uVar11 = 0;
      bVar2 = DAT_009d8c88[iVar7];
      do {
        if (bVar2 != uVar11) {
          DAT_009a215a[iVar4 + uVar11] = DAT_009a215a[iVar4 + uVar11] + '\x01';
          DAT_009a2164[iVar4 + uVar11] = 1;
        }
        uVar10 = (unsigned short)uVar11 + 1;
        uVar11 = (unsigned int)uVar10;
      } while (uVar10 < 10);
    }
    bVar13 = 0;
    uVar11 = 0;
    do {
      if (*(char *)(uVar11 + 0x1ec + iVar5) == '\x01') {
        bVar13 = 1;
      }
      uVar10 = (unsigned short)uVar11 + 1;
      uVar11 = (unsigned int)uVar10;
    } while (uVar10 < 10);
    if (!bVar13) {
      *(short *)(DAT_009a20d4 + uVar3 * 0x126) = *(short *)(DAT_009a20d4 + uVar3 * 0x126) + 1;
      if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
        DAT_009a2107[uVar3 * 0x126] = 1;
      }
    }
    if (*(char *)(iVar5 + 0x7b) == '\0') {
      *(short *)(DAT_009a20e0 + uVar3 * 0x126) = *(short *)(DAT_009a20e0 + uVar3 * 0x126) + 1;
      if ((char)DAT_009d8c80[iVar7] == *(char *)(iVar5 + 0xb0)) {
        DAT_009a210d[uVar3 * 0x126] = 1;
      }
    }
    if (*(char *)(iVar5 + 0x7d) == '\0') {
      cVar1 = DAT_009d8c80[iVar7];
      *(short *)(DAT_009a20e4 + uVar3 * 0x126) = *(short *)(DAT_009a20e4 + uVar3 * 0x126) + 1;
      if (cVar1 == *(char *)(iVar5 + 0xb0)) {
        DAT_009a210f[uVar3 * 0x126] = 1;
      }
    }
    if (param_3 == 2) {
      uVar10 = 0;
      bVar13 = DAT_009a2000 == 0;
      if (0 < DAT_009a2000) {
        do {
          if (uVar10 != param_2) {
            uVar11 = (unsigned int)uVar10;
            iVar5 = uVar11 * 0x566;
            if ((DAT_009d8ccb[uVar11 * 0x566] != '\0') && ((unsigned char)DAT_009d8cca[iVar5] == uVar3)) {
              FUN_00434440(uVar11);
            }
            if ((DAT_009d8ccd[iVar5] != '\0') && ((unsigned char)DAT_009d8ccc[iVar5] == uVar3)) {
              iVar4 = uVar11 * 0x126;
              *(short *)(DAT_009a20d0 + iVar4) =
                   *(short *)(DAT_009a20d0 + iVar4) + *(short *)(DAT_009a20ee + iVar4);
              *(short *)(DAT_009a20ee + iVar4) = 0;
            }
            if ((DAT_009d8ccf[iVar5] != '\0') && ((unsigned char)DAT_009d8cce[iVar5] == uVar3)) {
              iVar4 = uVar11 * 0x126;
              *(short *)(DAT_009a20d0 + iVar4) =
                   *(short *)(DAT_009a20d0 + iVar4) + *(short *)(DAT_009a20f0 + iVar4);
              *(short *)(DAT_009a20f0 + iVar4) = 0;
            }
            if ((DAT_009d8cd1[iVar5] != '\0') && ((unsigned char)DAT_009d8cd0[iVar5] == uVar3)) {
              iVar5 = uVar11 * 0x126;
              *(short *)(DAT_009a20d0 + iVar5) =
                   *(short *)(DAT_009a20d0 + iVar5) + *(short *)(DAT_009a20f2 + iVar5);
              *(short *)(DAT_009a20f2 + iVar5) = 0;
            }
          }
          uVar10 = uVar10 + 1;
          bVar13 = (int)(unsigned int)uVar10 == (int)DAT_009a2000;
        } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
      }
      local_4 = 4;
      pbVar9 = DAT_009d8cd2 + iVar7;
      do {
        iVar5 = 0xf;
        pcVar8 = (char *)(DAT_009d1320 +
                         (unsigned int)DAT_009a7a40[(unsigned int)DAT_00524120[*pbVar9]] * 0x55);
        pcVar12 = s_dropoffldr1pln_0051a2c4;
        do {
          if (iVar5 == 0) break;
          iVar5 = iVar5 + -1;
          bVar13 = *pcVar8 == *pcVar12;
          pcVar8 = pcVar8 + 1;
          pcVar12 = pcVar12 + 1;
        } while (bVar13);
        if (bVar13) {
          FUN_00434440((unsigned int)(unsigned short)(pbVar9[3] - 1));
        }
        pbVar9 = pbVar9 + 0x52;
        local_4 = local_4 + -1;
        bVar13 = local_4 == 0;
      } while (!bVar13);
    }
    if ((param_3 == 0x11) && (uVar10 = 0, 0 < DAT_009a2000)) {
      iVar5 = (int)DAT_009a2000;
      do {
        if (uVar10 != param_2) {
          uVar11 = (unsigned int)uVar10;
          iVar7 = uVar11 * 0x566;
          if ((DAT_009d8ccd[iVar7] != '\0') && ((unsigned char)DAT_009d8ccc[iVar7] == uVar3)) {
            iVar4 = uVar11 * 0x126;
            *(short *)(DAT_009a20ec + iVar4) =
                 *(short *)(DAT_009a20ec + iVar4) + *(short *)(DAT_009a20ee + iVar4);
            *(short *)(DAT_009a20ee + iVar4) = 0;
          }
          if ((DAT_009d8ccf[iVar7] != '\0') && ((unsigned char)DAT_009d8cce[iVar7] == uVar3)) {
            iVar4 = uVar11 * 0x126;
            *(short *)(DAT_009a20ec + iVar4) =
                 *(short *)(DAT_009a20ec + iVar4) + *(short *)(DAT_009a20f0 + iVar4);
            *(short *)(DAT_009a20f0 + iVar4) = 0;
          }
          if ((DAT_009d8cd1[iVar7] != '\0') && ((unsigned char)DAT_009d8cd0[iVar7] == uVar3)) {
            *(short *)(DAT_009a20ec + uVar11 * 0x126) =
                 *(short *)(DAT_009a20ec + uVar11 * 0x126) +
                 *(short *)(DAT_009a20f2 + uVar11 * 0x126);
            *(short *)(DAT_009a20f2 + uVar11 * 0x126) = 0;
          }
        }
        uVar10 = uVar10 + 1;
      } while ((int)(unsigned int)uVar10 < iVar5);
    }
    if (_DAT_00a07cec < DAT_009a1fe0) {
      iVar5 = _DAT_00a07cec * 0x23;
      iVar7 = _DAT_00a07cec;
      do {
        if (*(char *)(DAT_009a1fe8 + iVar5 + 3) != '\0') {
          __builtin_memcpy(&iTmp, (void *)(DAT_009a1fe8 + iVar5 + 0x1f), 4);
          __builtin_memcpy(&iVar4, (void *)(iTmp + 0xa9), 4);
          if (*(unsigned short *)(iVar4 + 0x6a) == param_1) {
            *(unsigned short *)(iVar4 + 0x6a) = 0xffff;
          }
        }
        iVar5 = iVar5 + 0x23;
        iVar7 = iVar7 + 1;
      } while (iVar7 < DAT_009a1fe0);
    }
    pcVar8 = DAT_009e9680;
    do {
      if (*pcVar8 != '\0') {
        char *puVar6 = pcVar8 + 0xe;
        iVar5 = 4;
        do {
          __builtin_memcpy(&usTmp, puVar6, 2);
          if (usTmp == param_1) {
            usTmp = 0xffff;
            __builtin_memcpy(puVar6, &usTmp, 2);
          }
          puVar6 = puVar6 + 2;
          iVar5 = iVar5 + -1;
        } while (iVar5 != 0);
      }
      pcVar8 = pcVar8 + 0x5bd;
    } while (pcVar8 < DAT_009ec468);
  }
  return;
}
