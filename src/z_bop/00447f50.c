/* SPDX-License-Identifier: MIT */
/* z_bop 00447f50 */
/* Prototypes */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
int  __stdcall FUN_004ac760(void);
void __stdcall FUN_00463d00(void);
int  __stdcall FUN_0046e040(void);
unsigned int __stdcall FUN_00448d00(void);
void __stdcall FUN_00464900(void);
void __cdecl   FUN_00417d70(int param_1);
void __stdcall FUN_0042d0d0(void);
int  __cdecl   _sprintf(char *, const char *, ...);
void __cdecl   FUN_00463c10(int param_1);
void __stdcall FUN_00448da0(void);
char * __stdcall FUN_00462a10(void);
void __cdecl   FUN_00448f90(int param_1, unsigned char *param_2, int param_3);
int  __stdcall FUN_004aa7f0(void);
unsigned int __stdcall FUN_004aa870(void);
void __stdcall FUN_00463bf0(void);
void __stdcall FUN_00448ed0(void);
void __cdecl   FUN_00463cc0(int param_1);
void __stdcall FUN_00450550(void);
void __cdecl   FUN_00463c50(int param_1);
void __cdecl   FUN_00418500(int param_1);
int  __stdcall FUN_0046f700(void);
int  __cdecl   FUN_00470020(int param_1);
void __stdcall FUN_00418b70(void);
void __fastcall FUN_004501c0(void *param_1);
void __stdcall FUN_00418970(void);

/* Globals */
extern unsigned int DAT_0055697c;
extern int DAT_00a0085c;
extern int DAT_00523414;
extern int DAT_00556980;
extern int DAT_00556974;
extern int DAT_00523650[20];
extern unsigned int DAT_009a8c2c;
extern unsigned int DAT_009e9658;
extern int DAT_009a8c24;
extern int DAT_009e9680[];
extern int DAT_009ecc34;
extern unsigned int DAT_00a07cc8;
extern int DAT_009ecc3c;
extern int DAT_00a080f8;
extern int DAT_00556978;
extern int DAT_00523410;
extern int DAT_00523418;
extern int DAT_0052340c;
extern int _DAT_009ed23c;
extern char s_Fell_Behind__tickcounter___7d_se_005238cc[];
extern char s_Too_far_Ahead__tickcounter___7d_s_00523890[];
extern char *DAT_00a60a08;
extern int DAT_009d8a30[];
extern unsigned char DAT_009d6940;
extern unsigned char *DAT_00a60a10;
extern unsigned char *DAT_00a60a20;
extern char *DAT_00a60a24;
extern int DAT_009ec45c;
extern int DAT_009a8bfc;
extern int DAT_009a7ba0;
extern int DAT_00a0829c;
extern int DAT_009a8d90;
extern char DAT_005235e0;
extern char s_R___2d_U___2d_N___2d_O___2d_T____00523840[];
extern char DAT_00a00750[];
extern char s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_005237f4[];
extern char s_Raw_10__2d_11__2d_12__2d_13__2d_1_005237a8[];
extern char s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052375c[];
extern char s_Pct_10__2d_11__2d_12__2d_13__2d_1_00523710[];
extern char s________Long_Update___d_______War_005236c4[];
extern int DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_009a1fe8;

void __stdcall FUN_00447f50(void)
{
  char cVar1;
  unsigned char bVar2;
  unsigned int uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  int iVar8;
  unsigned int uVar9;
  int iVar10;
  void *extraout_ECX = NULL;
  unsigned char *pbVar11;
  int iVar12;
  int *puVar13;
  unsigned char *pbVar15;
  int iVar16;
  unsigned int uVar17;
  char *pcVar14;
  char *pcVar18;
  int local_37c;
  unsigned int uStack_378;
  unsigned char abStack_36c[80];
  unsigned char local_31c[256];
  char local_21c[180];
  char local_168[180];
  char local_b4[180];
  long long lVarQ;
  int iVarQ;

  DAT_0055697c = 0;
  DAT_00a0085c = 0;
  DAT_00523414 = 0;
  DAT_00556980 = 0;
  DAT_00556974 = 0;
  puVar13 = DAT_00523650;
  for (iVar8 = 0x14; iVar8 != 0; iVar8 = iVar8 + -1) {
    *puVar13 = 0;
    puVar13 = puVar13 + 1;
  }
  do {
    while (1) {
      local_37c = 0;
      iVar8 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8;
      iVar16 = (int)DAT_009a8c2c;
      iVar8 = FUN_004ac760();
      iVar8 = (int)(DAT_009a8c2c + (unsigned int)iVar8);
      iVar4 = (int)((unsigned int)iVar8 - DAT_009e9658);
      while (DAT_009a8c2c = (unsigned int)iVar8, iVar4 < 8) {
        iVar8 = FUN_004ac760();
        iVar8 = (int)(DAT_009a8c2c + (unsigned int)iVar8);
        iVar4 = (int)((unsigned int)iVar8 - DAT_009e9658);
      }
      FUN_00463d00();
      iVar4 = FUN_0046e040();
      if ((iVar4 == 0) && (0x1ba8 < DAT_009a8c24)) {
        DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
        FUN_00463cc0(DAT_009ecc34);
        return;
      }
      iVar4 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar4;
      iVar4 = (int)DAT_009a8c2c;
      uVar3 = FUN_00448d00();
      iVar12 = (int)DAT_009a8c2c;
      if (uVar3 == 0) break;
      if (DAT_00a07cc8 == DAT_009e9658) {
        if (DAT_009ecc3c == 0) {
          return;
        }
        FUN_00450550();
        DAT_009ecc3c = 0;
        return;
      }
      if (0x49c < (int)(DAT_009a8c2c - DAT_009e9658)) {
        return;
      }
      iVar8 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8;
      iVar8 = (int)(DAT_009a8c2c - DAT_009e9658);
      while (iVar8 < 8) {
        iVar8 = FUN_004ac760();
        DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8;
        iVar8 = (int)(DAT_009a8c2c - DAT_009e9658);
      }
    }
    if (DAT_00a080f8 == 1) {
      iVar5 = FUN_004ac760();
      uVar9 = DAT_009a8c2c + (unsigned int)iVar5;
      uVar17 = uVar9;
      if (DAT_0055697c != 0) {
        uVar17 = (unsigned int)DAT_00556978 + DAT_0055697c;
        iVar5 = DAT_00556978 >> 3;
        if (uVar17 < uVar9) {
          iVar6 = (int)(uVar9 - uVar17);
          if (iVar5 < (int)(uVar9 - uVar17)) {
            iVar6 = iVar5;
          }
          if (iVar6 == 0) {
            iVar6 = 1;
          }
          if (4 < iVar6) {
            iVar6 = (int)(uVar9 - uVar17);
          }
          uVar17 = uVar17 + (unsigned int)iVar6;
        }
        else if (uVar9 < uVar17) {
          iVar6 = (int)(uVar17 - uVar9);
          if (iVar5 < (int)(uVar17 - uVar9)) {
            iVar6 = iVar5;
          }
          if (iVar6 == 0) {
            iVar6 = 1;
          }
          if (4 < iVar6) {
            iVar6 = (int)(uVar17 - uVar9);
          }
          uVar17 = uVar17 - (unsigned int)iVar6;
        }
      }
      if ((int)(uVar17 - DAT_009e9658) < 4) {
        uVar17 = DAT_009e9658 + 4;
      }
      DAT_00556978 = (int)(uVar17 - DAT_0055697c);
      DAT_009a8c2c = uVar17;
      FUN_00464900();
      DAT_00523410 = 0;
      DAT_00523418 = (int)(DAT_009a8c2c - DAT_009e9658);
      FUN_00417d70((int)DAT_009a8c2c);
      DAT_009e9658 = DAT_009a8c2c;
      DAT_00a07cc8 = DAT_009a8c2c;
      DAT_0055697c = DAT_009a8c2c;
      DAT_009a8c2c = (DAT_009a8c2c - uVar17) + uVar9;
      FUN_0042d0d0();
    }
    else {
      iVar5 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar5;
      iVar5 = DAT_00556978 >> 3;
      if ((unsigned int)_DAT_009ed23c + DAT_00a07cc8 < DAT_009a8c2c) {
        iVar10 = (int)((DAT_009a8c2c - (unsigned int)_DAT_009ed23c) - DAT_00a07cc8) >> 4;
        if (iVar10 == 0) {
          iVar10 = 1;
        }
        if (iVar5 < iVar10) {
          iVar10 = iVar5;
        }
        iVar6 = (int)(0u - (unsigned int)iVar10);
LAB_004482ae:
        DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar6;
        DAT_0052340c = (int)((unsigned int)DAT_0052340c + (unsigned int)iVar10);
      }
      else if (DAT_009a8c2c < (unsigned int)_DAT_009ed23c + DAT_00a07cc8) {
        iVar6 = (int)(((unsigned int)_DAT_009ed23c - DAT_009a8c2c) + DAT_00a07cc8) >> 4;
        if (iVar6 == 0) {
          iVar6 = 1;
        }
        if (iVar5 < iVar6) {
          iVar6 = iVar5;
        }
        iVar10 = (int)(0u - (unsigned int)iVar6);
        goto LAB_004482ae;
      }
      if ((int)DAT_009a8c2c < (int)DAT_00a07cc8) {
        _sprintf(local_b4, s_Fell_Behind__tickcounter___7d_se_005238cc, DAT_009a8c2c, DAT_00a07cc8,
                 ((unsigned int)_DAT_009ed23c - DAT_009a8c2c) + DAT_00a07cc8);
        iVar5 = (int)(((unsigned int)_DAT_009ed23c - DAT_009a8c2c) + DAT_00a07cc8);
        DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar5;
        DAT_0052340c = (int)((unsigned int)DAT_0052340c - (unsigned int)iVar5);
      }
      if (((int)((unsigned int)_DAT_009ed23c + 0x674u) < (int)(DAT_009a8c2c - DAT_00a07cc8)) &&
          (iVar5 = FUN_0046e040(), iVar5 == 0)) {
        FUN_00463c10(DAT_009ecc34);
        _sprintf(local_168, s_Too_far_Ahead__tickcounter___7d_s_00523890, DAT_009a8c2c, DAT_00a07cc8,
                 DAT_009a8c2c - DAT_00a07cc8);
        iVar6 = 0;
        iVar5 = FUN_004ac760();
        DAT_009a8c24 = 0;
        DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar5;
        FUN_00448da0();
        uVar17 = 0xffffffff;
        pcVar18 = DAT_00a60a08;
        do {
          pcVar14 = pcVar18;
          if (uVar17 == 0) break;
          uVar17 = uVar17 - 1;
          pcVar14 = pcVar18 + 1;
          cVar1 = *pcVar18;
          pcVar18 = pcVar14;
        } while (cVar1 != '\0');
        uVar17 = ~uVar17;
        pbVar11 = (unsigned char *)(pcVar14 - uVar17);
        pbVar15 = local_31c;
        for (uVar9 = uVar17 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          __builtin_memcpy(pbVar15, pbVar11, 4);
          pbVar11 = pbVar11 + 4;
          pbVar15 = pbVar15 + 4;
        }
        for (uVar17 = uVar17 & 3; uVar17 != 0; uVar17 = uVar17 - 1) {
          *pbVar15 = *pbVar11;
          pbVar11 = pbVar11 + 1;
          pbVar15 = pbVar15 + 1;
        }
        pcVar18 = (char *)FUN_00462a10();
        if (pcVar18 != (char *)0) {
          uVar17 = 0xffffffff;
          do {
            pcVar14 = pcVar18;
            if (uVar17 == 0) break;
            uVar17 = uVar17 - 1;
            pcVar14 = pcVar18 + 1;
            cVar1 = *pcVar18;
            pcVar18 = pcVar14;
          } while (cVar1 != '\0');
          uVar17 = ~uVar17;
          iVar5 = -1;
          pbVar11 = local_31c;
          do {
            pbVar15 = pbVar11;
            if (iVar5 == 0) break;
            iVar5 = (int)((unsigned int)iVar5 + 0xffffffffu);
            pbVar15 = pbVar11 + 1;
            bVar2 = *pbVar11;
            pbVar11 = pbVar15;
          } while (bVar2 != 0);
          pbVar11 = (unsigned char *)(pcVar14 - uVar17);
          pbVar15 = pbVar15 - 1;
          for (uVar9 = uVar17 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pbVar15, pbVar11, 4);
            pbVar11 = pbVar11 + 4;
            pbVar15 = pbVar15 + 4;
          }
          for (uVar17 = uVar17 & 3; uVar17 != 0; uVar17 = uVar17 - 1) {
            *pbVar15 = *pbVar11;
            pbVar11 = pbVar11 + 1;
            pbVar15 = pbVar15 + 1;
          }
        }
        FUN_00448f90(1, local_31c, 0x34);
        do {
          if ((int)(DAT_009a8c2c - DAT_00a07cc8) <= _DAT_009ed23c) break;
          iVar5 = FUN_004aa7f0();
          if ((iVar5 != 0) && (uVar17 = FUN_004aa870(), uVar17 == 0x1b)) {
            DAT_009d6940 = 1;
            FUN_00463c50(DAT_009ecc34);
            iVar8 = DAT_009ecc34;
            DAT_009d8a30[DAT_009ecc34] = 1;
            DAT_009e9680[iVar8 * 0x5bd] = 0;
            FUN_00463cc0(DAT_009ecc34);
            return;
          }
          iVar5 = (int)DAT_009a8c2c;
          FUN_00463d00();
          iVar10 = FUN_004ac760();
          DAT_009a8c24 = (int)((unsigned int)DAT_009a8c24 +
                               ((DAT_009a8c2c + (unsigned int)iVar10) - (unsigned int)iVar5));
          iVar6 = (int)((((unsigned int)iVar6 - (unsigned int)iVar5) + DAT_009a8c2c) + (unsigned int)iVar10);
          DAT_009a8c2c = (unsigned int)iVar5;
          if (0x1ba8 < DAT_009a8c24) {
            DAT_009d6940 = 1;
            FUN_00463c50(DAT_009ecc34);
            iVar8 = DAT_009ecc34;
            DAT_009d8a30[DAT_009ecc34] = 1;
            DAT_009e9680[iVar8 * 0x5bd] = 0;
            FUN_00463cc0(DAT_009ecc34);
            return;
          }
          if (0xec < iVar6) {
            iVar6 = 0;
            FUN_00463bf0();
          }
          uVar17 = (0x1ba8u - (unsigned int)DAT_009a8c24) / 0x76u;
          if (uStack_378 != uVar17) {
            if ((int)uVar17 < 0x32) {
              _sprintf((char *)abStack_36c, DAT_00a60a24, (int)uVar17 / 2, (uVar17 & 1) * 5);
              pbVar11 = abStack_36c;
            }
            else {
              pbVar11 = DAT_00a60a10;
              if ((uVar17 & 1) != 0) {
                pbVar11 = DAT_00a60a20;
              }
            }
            FUN_00448f90(3, pbVar11, 0x34);
            uStack_378 = uVar17;
          }
          uVar3 = FUN_00448d00();
        } while ((uVar3 == 0) || (DAT_00a07cc8 != DAT_009e9658));
        FUN_00448ed0();
        FUN_004ac760();
        uVar3 = FUN_00448d00();
        if ((uVar3 != 0) && (DAT_00a07cc8 == DAT_009e9658)) {
          return;
        }
        DAT_009a8c2c = (unsigned int)_DAT_009ed23c + DAT_00a07cc8;
      }
      uVar17 = DAT_009a8c2c;
      if (DAT_009e9658 < DAT_009a8c2c) {
        uVar9 = DAT_009a8c2c;
        if (DAT_0055697c != 0) {
          uVar9 = (unsigned int)DAT_00556978 + DAT_0055697c;
          iVar5 = DAT_00556978 >> 3;
          if (uVar9 < DAT_009a8c2c) {
            iVar6 = (int)(DAT_009a8c2c - uVar9);
            if (iVar5 < (int)(DAT_009a8c2c - uVar9)) {
              iVar6 = iVar5;
            }
            if (iVar6 == 0) {
              iVar6 = 1;
            }
            if (4 < iVar6) {
              iVar6 = (int)(DAT_009a8c2c - uVar9);
            }
            uVar9 = uVar9 + (unsigned int)iVar6;
          }
          else if (DAT_009a8c2c < uVar9) {
            iVar6 = (int)(uVar9 - DAT_009a8c2c);
            if (iVar5 < (int)(uVar9 - DAT_009a8c2c)) {
              iVar6 = iVar5;
            }
            if (iVar6 == 0) {
              iVar6 = 1;
            }
            if (4 < iVar6) {
              iVar6 = (int)(uVar9 - DAT_009a8c2c);
            }
            uVar9 = uVar9 - (unsigned int)iVar6;
          }
        }
        if ((int)(uVar9 - DAT_009e9658) < 4) {
          uVar9 = DAT_009e9658 + 4;
        }
        DAT_00556978 = (int)(uVar9 - DAT_0055697c);
        DAT_009a8c2c = uVar9;
        FUN_00464900();
        FUN_00418500(DAT_00556978);
        DAT_00523410 = 1;
        local_37c = (int)(DAT_009a8c2c - DAT_009e9658);
        FUN_00417d70((int)DAT_009a8c2c);
        DAT_009e9658 = DAT_009a8c2c;
        DAT_0055697c = DAT_009a8c2c;
        DAT_009a8c2c = (DAT_009a8c2c - uVar9) + uVar17;
      }
    }
    iVar5 = FUN_004ac760();
    DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar5;
    iVar12 = (int)(DAT_009a8c2c - (unsigned int)iVar12);
    iVar5 = FUN_004ac760();
    iVar5 = (int)(DAT_009a8c2c + (unsigned int)iVar5);
    iVar6 = (int)(((unsigned int)iVar5 - (unsigned int)_DAT_009ed23c) - DAT_00a07cc8);
    if (iVar6 < 0x1d8) {
      DAT_009ec45c = 0;
    }
    else if (iVar6 < 0x3b0) {
      DAT_009ec45c = 1;
    }
    else {
      DAT_009ec45c = 2;
      if (0x587 < iVar6) {
        DAT_009ec45c = 3;
      }
    }
    DAT_009a8c2c = (unsigned int)iVar5;
    if (DAT_00556980 == 0) {
      DAT_009a8bfc = 0;
    }
    else {
      iVar6 = FUN_0046f700();
      iVar6 = FUN_00470020(iVar6);
      DAT_00556974 = (int)((unsigned int)DAT_00556974 +
                           (((unsigned int)iVar6 - (unsigned int)DAT_00556980) * 10u));
      if (DAT_00556974 == 0) {
        DAT_009a8bfc = 0;
      }
      else if (DAT_00556974 < 10) {
        DAT_009a8bfc = 1;
      }
      else {
        DAT_009a8bfc = 2;
        if (0x13 < DAT_00556974) {
          DAT_009a8bfc = 3;
        }
      }
      DAT_00556980 = iVar6;
      if (DAT_00556974 != 0) {
        DAT_00556974 = (int)((unsigned int)DAT_00556974 + 0xffffffffu);
      }
    }
    FUN_00418b70();
    if (1 < DAT_00a080f8) {
      DAT_00523414 = 1;
    }
    FUN_004501c0(extraout_ECX);
    DAT_00523414 = 0;
    FUN_0042d0d0();
    FUN_00418970();
    iVar6 = FUN_004ac760();
    DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar6;
    iVar16 = (int)(DAT_009a8c2c - (unsigned int)iVar16);
    if (iVar16 == 0) {
      iVar16 = 1;
    }
    if (DAT_005235e0 == '\0') {
      DAT_00a0829c = 0;
      DAT_009a7ba0 = 0;
    }
    else {
      if (0x3b0 < DAT_009a7ba0) {
        DAT_00a0829c = 0;
        DAT_009a7ba0 = 0;
      }
      DAT_009a7ba0 = (int)((unsigned int)DAT_009a7ba0 + (unsigned int)iVar16);
      DAT_00a0829c = (int)((unsigned int)DAT_00a0829c + 1u);
      DAT_009a8d90 = iVar16;
      iVarQ = 0;
      if (iVar16 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      else {
        lVarQ = 0xec / (long long)iVar16;
        if ((lVarQ > 2147483647LL) || (lVarQ < -2147483647LL - 1)) {
          RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        else {
          iVarQ = (int)lVarQ;
        }
      }
      _sprintf(local_21c, s_R___2d_U___2d_N___2d_O___2d_T____00523840,
               DAT_009a8c2c - (unsigned int)iVar5, iVar12,
               (int)((unsigned int)iVar4 - (unsigned int)iVar8),
               (int)((((unsigned int)iVar16 - ((unsigned int)iVar4 - (unsigned int)iVar8)) -
                      (unsigned int)iVar12) - (DAT_009a8c2c - (unsigned int)iVar5)),
               iVar16,
               iVarQ, DAT_009a8c2c, DAT_009e9658, DAT_00a07cc8,
               DAT_009a8c2c - DAT_00a07cc8, _DAT_009ed23c, DAT_0052340c);
      if (iVar12 < 0) {
        iVar12 = 0;
      }
      if (iVar12 < 0x14) {
        DAT_00523650[iVar12] = (int)((unsigned int)DAT_00523650[iVar12] + 1u);
      }
      else {
        DAT_00523650[19] = (int)((unsigned int)DAT_00523650[19] + 1u);
      }
      _sprintf(DAT_00a00750, s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_005237f4, DAT_00523650[0], DAT_00523650[1],
               DAT_00523650[2], DAT_00523650[3], DAT_00523650[4], DAT_00523650[5], DAT_00523650[6],
               DAT_00523650[7], DAT_00523650[8], DAT_00523650[9]);
      uVar17 = 0;
      _sprintf(DAT_00a00750, s_Raw_10__2d_11__2d_12__2d_13__2d_1_005237a8, DAT_00523650[10], DAT_00523650[11],
               DAT_00523650[12], DAT_00523650[13], DAT_00523650[14], DAT_00523650[15], DAT_00523650[16],
               DAT_00523650[17], DAT_00523650[18], DAT_00523650[19]);
      piVar7 = DAT_00523650;
      do {
        iVar8 = *piVar7;
        piVar7 = piVar7 + 1;
        uVar17 = uVar17 + (unsigned int)iVar8;
      } while (piVar7 < DAT_00523650 + 20);
      if (uVar17 != 0) {
        _sprintf(DAT_00a00750, s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052375c,
                 ((unsigned int)DAT_00523650[0] * 100u) / uVar17, ((unsigned int)DAT_00523650[1] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[2] * 100u) / uVar17, ((unsigned int)DAT_00523650[3] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[4] * 100u) / uVar17, ((unsigned int)DAT_00523650[5] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[6] * 100u) / uVar17, ((unsigned int)DAT_00523650[7] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[8] * 100u) / uVar17, ((unsigned int)DAT_00523650[9] * 100u) / uVar17);
        _sprintf(DAT_00a00750, s_Pct_10__2d_11__2d_12__2d_13__2d_1_00523710,
                 ((unsigned int)DAT_00523650[10] * 100u) / uVar17, ((unsigned int)DAT_00523650[11] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[12] * 100u) / uVar17, ((unsigned int)DAT_00523650[13] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[14] * 100u) / uVar17, ((unsigned int)DAT_00523650[15] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[16] * 100u) / uVar17, ((unsigned int)DAT_00523650[17] * 100u) / uVar17,
                 ((unsigned int)DAT_00523650[18] * 100u) / uVar17, ((unsigned int)DAT_00523650[19] * 100u) / uVar17);
      }
      iVar16 = 0;
      iVar8 = 0;
      if (DAT_009e9640 < DAT_00a00854) {
        iVar4 = (int)((unsigned int)DAT_00a00854 - (unsigned int)DAT_009e9640);
        pcVar18 = (char *)((unsigned int)DAT_009e9640 * 0x23u + 3u + (unsigned int)DAT_009a1fe8);
        do {
          if (*pcVar18 != '\0') {
            if (pcVar18[-1] == '\x06') {
              iVar8 = (int)((unsigned int)iVar8 + 1u);
            }
            else {
              iVar16 = (int)((unsigned int)iVar16 + 1u);
            }
          }
          pcVar18 = pcVar18 + 0x23;
          iVar4 = (int)((unsigned int)iVar4 + 0xffffffffu);
        } while (iVar4 != 0);
      }
      if (7 < iVar12) {
        _sprintf(DAT_00a00750, s________Long_Update___d_______War_005236c4, iVar12, local_37c, iVar8,
                 iVar16);
      }
    }
  } while (1);
}
