/* SPDX-License-Identifier: MIT */
/* z_bop 00463d00 */
extern int DAT_009a8c2c;
extern int DAT_00556efc;
extern char DAT_009e9680[];
extern char DAT_009ec468[];
extern int DAT_009ecc34;
extern int DAT_00a080f8;
extern int DAT_00557358;
extern int DAT_0055734c;
extern char *DAT_00a60a08;
extern char *DAT_00a60a10;
extern char *DAT_00a60a20;
extern int DAT_00a07cc8;
extern int DAT_009e9658;
extern unsigned char DAT_009d6940;   /* only low byte accessed; real size unresolved */
extern int DAT_00556ed8[];
extern unsigned int DAT_00557560[];
extern unsigned int DAT_00557360;
extern int DAT_00557364;
extern int DAT_00a07bb0[];
extern int DAT_009d8a30[];
extern int DAT_005242cc;
extern int DAT_009a8c24;
extern int DAT_0052342c;
extern int DAT_00556f00;
extern int DAT_00556f08[];
extern int DAT_005242dc;
extern int DAT_005242e0;
extern int DAT_005242e4;
extern int DAT_009ed23c;
extern int DAT_00523428;
extern int DAT_009fd394;
extern int DAT_005242c4;
extern int DAT_00556ed0;
extern int DAT_0052340c;
extern int DAT_00523418;
extern int DAT_00523420;
extern int DAT_0055978c;
extern int DAT_00557588[];

int FUN_004ac760(void);
int FUN_004aa7f0(void);
unsigned int FUN_004aa870(void);
void FUN_00448f90(int, unsigned char *, int);
void FUN_00448da0(void);
char *FUN_00462a10(void);
int FUN_0046e040(void);
void FUN_00470060(void);
void FUN_00463c10(int);
int *FUN_0046e050(int *, int *);
int FUN_00464c80(int);
void __cdecl FUN_00464d70(int);   /* one stack arg, caller cleans (stack-check finding); value unverified */
int FUN_0046f660(int);
void FUN_00418650(int);
int *FUN_00418760(int, unsigned int, int *);
void FUN_0046d350(int, unsigned int *, int);
void FUN_00418f80(int *);
void FUN_00463c50(int);
void FUN_004193c0(int, int);
void FUN_00463cc0(int);
void FUN_00448ed0(void);
void FUN_00419510(int);
void FUN_00463b60(void);
int FUN_0046f700(void);
void FUN_00465a20(int *);
void FUN_00463bd0(void);
int __cdecl _sprintf(char *, const char *, ...);

void __stdcall FUN_00463d00(void)
{
  char cVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  char *pcVar6;
  int *piVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned char bVar10;
  unsigned char *pbVar11;
  unsigned char *pbVar12;
  char *pcVar13;
  int iVar14;
  int *piVar15;
  struct { unsigned int uStack_80; unsigned short uStack_7c; } stk80; /* adjacent stack slots -0x80/-0x7c */
  int local_78;
  unsigned int uStack_74;
  int local_70;
  int local_6c;
  int local_68;
  int local_64;
  unsigned int local_60;
  int local_5c;
  int local_58;
  int local_54;
  unsigned char abStack_50[80];

  iVar3 = DAT_009a8c2c;
  iVar4 = DAT_00556efc;
  if (DAT_009e9680[DAT_009ecc34 * 0x5bd] != '\0') {
    local_58 = 0;
    local_70 = 0;
    local_68 = 0;
    local_64 = 0;
    local_6c = 0;
    if (DAT_00a080f8 != 1) {
      if (DAT_00557358 == 0) {
        iVar4 = FUN_004ac760();
        iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4);
        local_5c = iVar3;
      }
      else {
        local_5c = (int)((unsigned int)DAT_0055734c - 0x33aU);
        iVar3 = FUN_004ac760();
        iVar3 = (int)((unsigned int)iVar4 + 1U + (unsigned int)iVar3);
      }
switchD_00463f55_caseD_5:
      if (0x33a < (int)((unsigned int)iVar3 - (unsigned int)local_5c)) {
        if (DAT_00557358 == 0) {
          local_60 = 1;
          FUN_00448da0();
          uVar5 = 0xffffffff;
          pcVar6 = DAT_00a60a08;
          do {
            pcVar13 = pcVar6;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar13 = pcVar6 + 1;
            cVar1 = *pcVar6;
            pcVar6 = pcVar13;
          } while (cVar1 != '\0');
          uVar5 = ~uVar5;
          pbVar11 = (unsigned char *)(pcVar13 - uVar5);
          pbVar12 = abStack_50;
          for (uVar9 = uVar5 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            *(unsigned int *)pbVar12 = *(unsigned int *)pbVar11;
            pbVar11 = pbVar11 + 4;
            pbVar12 = pbVar12 + 4;
          }
          for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pbVar12 = *pbVar11;
            pbVar11 = pbVar11 + 1;
            pbVar12 = pbVar12 + 1;
          }
          pcVar6 = FUN_00462a10();
          if (pcVar6 != (char *)0) {
            uVar5 = 0xffffffff;
            do {
              pcVar13 = pcVar6;
              if (uVar5 == 0) break;
              uVar5 = uVar5 - 1;
              pcVar13 = pcVar6 + 1;
              cVar1 = *pcVar6;
              pcVar6 = pcVar13;
            } while (cVar1 != '\0');
            uVar5 = ~uVar5;
            iVar4 = -1;
            pbVar11 = abStack_50;
            do {
              pbVar12 = pbVar11;
              if (iVar4 == 0) break;
              iVar4 = iVar4 + -1;
              pbVar12 = pbVar11 + 1;
              bVar2 = *pbVar11;
              pbVar11 = pbVar12;
            } while (bVar2 != 0);
            pbVar11 = (unsigned char *)(pcVar13 - uVar5);
            pbVar12 = pbVar12 - 1;
            for (uVar9 = uVar5 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
              *(unsigned int *)pbVar12 = *(unsigned int *)pbVar11;
              pbVar11 = pbVar11 + 4;
              pbVar12 = pbVar12 + 4;
            }
            for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
              *pbVar12 = *pbVar11;
              pbVar11 = pbVar11 + 1;
              pbVar12 = pbVar12 + 1;
            }
          }
          FUN_00448f90(1, abStack_50, 0x34);
          DAT_00557358 = 1;
          DAT_00556efc = iVar3;
          DAT_0055734c = iVar3;
          iVar4 = FUN_0046e040();
          if (iVar4 == 0) {
            FUN_00470060();
            FUN_00463c10(DAT_009ecc34);
          }
        }
        else {
          iVar4 = FUN_004aa7f0();
          if ((iVar4 != 0) && (uVar5 = FUN_004aa870(), uVar5 == 0x1b)) {
            DAT_00557358 = 0;
            FUN_00448ed0();
            DAT_009a8c2c = DAT_0055734c;
            DAT_00a07cc8 = DAT_009e9658;
            DAT_009d6940 = 1;
            DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
            FUN_00463c50(DAT_009ecc34);
            iVar3 = FUN_0046e040();
            if (iVar3 != 0) {
              FUN_00463bd0();
              return;
            }
            FUN_00463cc0(DAT_009ecc34);
            return;
          }
          if (0x76 < (int)((unsigned int)iVar3 - (unsigned int)DAT_00556efc)) {
            local_60 = (unsigned int)(local_60 == 0);
            if (local_60 == 0) {
              DAT_00556efc = iVar3;
              FUN_00448f90(3, (unsigned char *)DAT_00a60a10, 0x34);
            }
            else {
              DAT_00556efc = iVar3;
              FUN_00448f90(3, (unsigned char *)DAT_00a60a20, 0x34);
            }
          }
        }
      }
      iVar4 = FUN_004ac760();
      piVar7 = FUN_0046e050(&local_78, &local_54);
      iVar8 = FUN_004ac760();
      iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4 + (unsigned int)iVar8);
      local_64 = (int)((unsigned int)local_64 + (unsigned int)iVar8);
      if (piVar7 != (int *)0) {
        switch (*piVar7) {
        case 1:
          iVar4 = FUN_004ac760();
          iVar8 = FUN_0046f660(local_78);
          if (DAT_009e9680[iVar8 * 0x5bd] == '\0') {
            DAT_00557360 = 0x15;
            DAT_00557364 = iVar8;
            FUN_0046d350(local_78, &DAT_00557360, 8);
          }
          else {
            if (0 < DAT_00556ed8[iVar8]) {
              DAT_00556ed8[iVar8] = 0;
            }
            FUN_00418650(iVar8);
            stk80.uStack_80 = stk80.uStack_80 & 0xff000000U;
            bVar2 = *(unsigned char *)(piVar7 + 1);
            uStack_74 = (uStack_74 & 0xffffff00U) | (unsigned int)bVar2;
            bVar10 = bVar2 & 0x7f;
            if (bVar10 == 0x7f) {
              __builtin_memcpy(&uVar5, (unsigned char *)piVar7 + 5, 4);
              if ((bVar2 & 0x80) == 0) {
                pbVar11 = (unsigned char *)piVar7 + 9;
              }
              else {
                pbVar11 = (unsigned char *)piVar7 + 10;
                stk80.uStack_80 = (stk80.uStack_80 & 0xff000000U) | ((unsigned int)*((unsigned char *)piVar7 + 9) << 0x10);
              }
            }
            else {
              uVar5 = DAT_00557560[iVar8];
              if ((unsigned int)bVar10 < (uVar5 & 0x7f)) {
                uVar5 = uVar5 + 0x80;
              }
              uVar5 = (uVar5 & 0xffffff80) | (unsigned int)bVar10;
              if ((bVar2 & 0x80) == 0) {
                pbVar11 = (unsigned char *)piVar7 + 5;
              }
              else {
                pbVar11 = (unsigned char *)piVar7 + 6;
                stk80.uStack_80 = (stk80.uStack_80 & 0xff000000U) | ((unsigned int)*((unsigned char *)piVar7 + 5) << 0x10);
              }
            }
            DAT_00557560[iVar8] = uVar5;
            stk80.uStack_80 = (((unsigned int)*pbVar11 << 24) | (stk80.uStack_80 & 0xffffff)) & 0xfeffffff;
            stk80.uStack_7c = (unsigned short)(((((unsigned int)(pbVar11[1] & 1) * 2) | (unsigned int)(*pbVar11 & 1)) << 8 | (unsigned int)pbVar11[1]) & 0xfffe);
            piVar7 = FUN_00418760(iVar8, uVar5, (int *)&stk80.uStack_80);
            if (piVar7 != (int *)0) {
              iVar8 = FUN_0046e040();
              *piVar7 = 1;
              if (iVar8 == 0) {
                *piVar7 = 0;
              }
              piVar7[1] = 1;
            }
          }
          iVar8 = FUN_004ac760();
          iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4 + (unsigned int)iVar8);
          local_68 = (int)((unsigned int)local_68 + (unsigned int)iVar8);
          break;
        case 2:
          iVar14 = 0;
          iVar4 = FUN_004ac760();
          DAT_009a8c24 = 0;
          DAT_005242cc = (int)((unsigned int)DAT_005242cc + 1U);
          FUN_00418f80(piVar7);
          iVar8 = FUN_0046e040();
          if (iVar8 != 0) {
            piVar7 = DAT_00556ed8;
            pcVar6 = DAT_009e9680;
            do {
              if ((((iVar14 != DAT_009ecc34) && (*pcVar6 != '\0')) && (*piVar7 != -1)) &&
                 (iVar8 = (int)((unsigned int)*piVar7 + (unsigned int)DAT_00523418), *piVar7 = iVar8, 0x1ba8 < iVar8)) {
                FUN_00463c50(iVar14);
                *piVar7 = 0;
              }
              piVar7 = piVar7 + 1;
              pcVar6 = pcVar6 + 0x5bd;
              iVar14 = iVar14 + 1;
            } while (pcVar6 < DAT_009ec468);
          }
          if (DAT_009d6940 == 1) {
            if (DAT_00557358 == 0) {
              return;
            }
            DAT_00557358 = 0;
            FUN_00448ed0();
            DAT_009a8c2c = DAT_0055734c;
            return;
          }
          iVar8 = FUN_004ac760();
          iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4 + (unsigned int)iVar8);
          local_6c = (int)((unsigned int)local_6c + (unsigned int)iVar8);
          local_58 = (int)((unsigned int)local_58 + 1U);
          break;
        case 3:
          iVar4 = piVar7[1];
          if ((-1 < iVar4) && (iVar4 < 8)) {
            DAT_00a07bb0[iVar4] = 0;
          }
          break;
        case 4:
          iVar4 = FUN_0046f660(local_78);
          if (DAT_009e9680[iVar4 * 0x5bd] != '\0') {
            FUN_004193c0(local_78, (int)piVar7);
          }
          break;
        case 8:
          DAT_0052342c = 1;
          DAT_009d6940 = 1;
          DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
          if (DAT_00557358 == 0) {
            return;
          }
          DAT_00557358 = 0;
          FUN_00448ed0();
          DAT_009a8c2c = DAT_0055734c;
          return;
        case 0x10:
          DAT_00556f00 = 1;
          if ((unsigned int)piVar7[1] < 0x10) {
            DAT_00556f08[piVar7[1]] = 1;
          }
          break;
        case 0x15:
          iVar4 = piVar7[1];
          if ((-1 < iVar4) && (iVar4 < 8)) {
            DAT_009d8a30[iVar4] = 1;
          }
          if (iVar4 == DAT_009ecc34) {
            DAT_009d6940 = 1;
            DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
            iVar3 = DAT_009ecc34;
            DAT_009d8a30[DAT_009ecc34] = 1;
            FUN_00463cc0(iVar3);
            if (DAT_00557358 == 0) {
              return;
            }
            DAT_00557358 = 0;
            FUN_00448ed0();
            DAT_009a8c2c = DAT_0055734c;
            return;
          }
          break;
        case 0x16:
          iVar4 = FUN_0046f660(local_78);
          if (DAT_009e9680[iVar4 * 0x5bd] == '\0') {
            DAT_00557360 = 0x15;
            DAT_00557364 = iVar4;
            FUN_0046d350(local_78, &DAT_00557360, 8);
          }
          else {
            if (0 < DAT_00556ed8[iVar4]) {
              DAT_00556ed8[iVar4] = 0;
            }
            pbVar11 = (unsigned char *)piVar7 + 5;
            FUN_00418650(iVar4);
            uVar5 = (unsigned int)*(unsigned char *)(piVar7 + 1);
            if (*(unsigned char *)(piVar7 + 1) != 0) {
              do {
                uStack_74 = uVar5;
                stk80.uStack_80 = stk80.uStack_80 & 0xff000000U;
                bVar2 = *pbVar11;
                bVar10 = bVar2 & 0x7f;
                if (bVar10 == 0x7f) {
                  __builtin_memcpy(&uVar5, pbVar11 + 1, 4);
                  if ((bVar2 & 0x80) == 0) {
                    pbVar12 = pbVar11 + 5;
                  }
                  else {
                    pbVar12 = pbVar11 + 6;
                    stk80.uStack_80 = (stk80.uStack_80 & 0xff000000U) | ((unsigned int)pbVar11[5] << 0x10);
                  }
                }
                else {
                  uVar5 = DAT_00557560[iVar4];
                  if ((unsigned int)bVar10 < (uVar5 & 0x7f)) {
                    uVar5 = uVar5 + 0x80;
                  }
                  uVar5 = (uVar5 & 0xffffff80) | (unsigned int)bVar10;
                  if ((bVar2 & 0x80) == 0) {
                    pbVar12 = pbVar11 + 1;
                  }
                  else {
                    pbVar12 = pbVar11 + 2;
                    stk80.uStack_80 = (stk80.uStack_80 & 0xff000000U) | ((unsigned int)pbVar11[1] << 0x10);
                  }
                }
                pbVar11 = pbVar12 + 2;
                DAT_00557560[iVar4] = uVar5;
                stk80.uStack_80 = (((unsigned int)*pbVar12 << 24) | (stk80.uStack_80 & 0xffffff)) & 0xfeffffff;
                stk80.uStack_7c = (unsigned short)(((((unsigned int)(pbVar12[1] & 1) * 2) | (unsigned int)(*pbVar12 & 1)) << 8 | (unsigned int)pbVar12[1]) & 0xfffe);
                piVar7 = FUN_00418760(iVar4, uVar5, (int *)&stk80.uStack_80);
                if (piVar7 != (int *)0) {
                  iVar8 = FUN_0046e040();
                  *piVar7 = 1;
                  if (iVar8 == 0) {
                    *piVar7 = 0;
                  }
                  piVar7[1] = 1;
                }
                uVar5 = uStack_74 - 1;
              } while (uStack_74 - 1 != 0);
              uStack_74 = 0;
            }
          }
          break;
        case 0x17:
          DAT_005242dc = piVar7[1];
          break;
        case 0x18:
          FUN_00419510((int)piVar7);
          FUN_00463b60();
          break;
        case 0x1b:
          if ((DAT_005242e0 != 0) && (DAT_005242e0 = DAT_005242e0 + -1, DAT_005242e0 == 0)) {
            DAT_005242e4 = 0;
            if (DAT_00557358 == 0) {
              DAT_005242e4 = 0;
              return;
            }
            DAT_00557358 = 0;
            FUN_00448ed0();
            DAT_009a8c2c = DAT_0055734c;
            return;
          }
          break;
        case 0x1c:
          DAT_009ed23c = piVar7[1];
          break;
        case 0x1d:
          iVar4 = FUN_0046f700();
          if (iVar4 == local_78) {
            DAT_009a8c24 = 0;
          }
          else {
            iVar4 = FUN_0046f660(local_78);
            if ((DAT_009e9680[iVar4 * 0x5bd] != '\0') && (0 < DAT_00556ed8[iVar4])) {
              DAT_00556ed8[iVar4] = 0;
            }
          }
          break;
        case 0x1e:
          DAT_00557360 = 0x1f;
          DAT_00557364 = piVar7[1];
          FUN_0046d350(local_78, &DAT_00557360, 8);
          iVar4 = piVar7[2];
          if ((DAT_00523428 == 0) || (DAT_009fd394 < DAT_005242c4)) {
            iVar4 = iVar4 >> 1;
          }
          if (DAT_009ed23c < iVar4) {
            iVar4 = (int)((unsigned int)iVar4 - (unsigned int)DAT_009ed23c) >> 1;
            if (iVar4 == 0) {
              iVar4 = 1;
            }
            DAT_009ed23c = (int)((unsigned int)DAT_009ed23c + (unsigned int)iVar4);
          }
          else if (iVar4 < DAT_009ed23c) {
            iVar4 = (int)((unsigned int)DAT_009ed23c - (unsigned int)iVar4) >> 1;
            if (iVar4 == 0) {
              iVar4 = 1;
            }
            DAT_009ed23c = (int)((unsigned int)DAT_009ed23c - (unsigned int)iVar4);
          }
          break;
        case 0x1f:
          iVar4 = FUN_0046e040();
          if (((iVar4 == 0) && (piVar7[1] == DAT_00556ed0)) &&
             (iVar4 = (int)((((unsigned int)DAT_0052340c + (unsigned int)DAT_009a8c2c) - (unsigned int)piVar7[1]) + 0x14U), iVar4 < 0x1d8)) {
            if (DAT_009ed23c < iVar4) {
              iVar4 = (int)((unsigned int)iVar4 - (unsigned int)DAT_009ed23c) >> 1;
              if (iVar4 == 0) {
                iVar4 = 1;
              }
              DAT_009ed23c = (int)((unsigned int)DAT_009ed23c + (unsigned int)iVar4);
            }
            else if (iVar4 < DAT_009ed23c) {
              iVar4 = (int)((unsigned int)DAT_009ed23c - (unsigned int)iVar4) >> 1;
              if (iVar4 == 0) {
                iVar4 = 1;
              }
              DAT_009ed23c = (int)((unsigned int)DAT_009ed23c - (unsigned int)iVar4);
            }
          }
          break;
        case 0x3c:
          DAT_0055978c = 1;
          piVar15 = DAT_00557588;
          for (iVar4 = 0x7e; piVar7 = piVar7 + 1, iVar4 != 0; iVar4 = iVar4 + -1) {
            *piVar15 = *piVar7;
            piVar15 = piVar15 + 1;
          }
          break;
        case 0x3d:
        case 0x3e:
        case 0x3f:
          if (piVar7[1] == DAT_00523420) {
            FUN_00465a20(piVar7);
            if (0x1ba8 < DAT_009a8c24) {
              if (DAT_00557358 == 0) {
                return;
              }
              DAT_00557358 = 0;
              FUN_00448ed0();
              DAT_009a8c2c = DAT_0055734c;
              return;
            }
            if (DAT_009e9680[DAT_009ecc34 * 0x5bd] == '\0') {
              if (DAT_00557358 == 0) {
                return;
              }
              DAT_00557358 = 0;
              FUN_00448ed0();
              DAT_009a8c2c = DAT_0055734c;
              return;
            }
          }
        }
        goto switchD_00463f55_caseD_5;
      }
      iVar4 = FUN_0046e040();
      if (iVar4 != 0) {
        iVar4 = FUN_004ac760();
        iVar8 = FUN_00464c80(DAT_009a8c2c);
        if (iVar8 != 0) {
          FUN_00464d70(DAT_009a8c2c);   /* pushed value unverified; callee does not read it */
          iVar8 = FUN_004ac760();
          iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4 + (unsigned int)iVar8);
          local_70 = (int)((unsigned int)local_70 + (unsigned int)iVar8);
          goto switchD_00463f55_caseD_5;
        }
        iVar8 = FUN_004ac760();
        iVar3 = (int)((unsigned int)iVar3 + (unsigned int)iVar4 + (unsigned int)iVar8);
        local_70 = (int)((unsigned int)local_70 + (unsigned int)iVar8);
      }
      if (DAT_00557358 != 0) {
        DAT_00557358 = 0;
        FUN_00448ed0();
        DAT_009a8c2c = DAT_0055734c;
        iVar3 = DAT_0055734c;
      }
      iVar4 = FUN_004ac760();
      DAT_009a8c2c = (int)((unsigned int)iVar3 + (unsigned int)iVar4);
      _sprintf((char *)abStack_50, "RcvMsg:%-2d In2Svr:%-2d SvrSnd:%-2d SvrFrm:%-2d NumFrm:%-2d AllPIN:%-2d Misc:%-2d\n",
               local_64, local_68, local_70, local_6c, local_58,
               (int)((unsigned int)DAT_009a8c2c - (unsigned int)local_5c),
               (int)((((((unsigned int)DAT_009a8c2c - (unsigned int)local_5c) - (unsigned int)local_70) - (unsigned int)local_68) - (unsigned int)local_64) - (unsigned int)local_6c));
      return;
    }
    while (FUN_0046e050(&local_78, &local_54) != (int *)0) {
    }
  }
  return;
}
