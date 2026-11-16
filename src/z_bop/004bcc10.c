/* SPDX-License-Identifier: MIT */
/* z_bop 004bcc10 */
extern int DAT_00664e88;
extern int DAT_00664e8c;
extern int DAT_00664e90;
extern int DAT_00664e94;
extern unsigned int DAT_00b6a2a0;
extern unsigned int DAT_00b6a2c0;
extern int DAT_00bb2810;
extern int DAT_00bb72a0;
extern char DAT_00bb73a7;
extern char DAT_00b69d20[];
extern char DAT_00bb2820[];
extern char DAT_00bb2821;
extern char DAT_00bb2835;
extern char DAT_00bb7380[];
extern char DAT_00bb7390[];
extern char DAT_0052ada8[];
extern char s_configsound_0052a708[];
extern char s_settingsound_0052ad80[];
extern char s__s__s___c_s_0052ad90[];
extern char s__s__d___c_s_0052ad9c[];
extern char s__c_s___c_s_0052adb0[];
extern char s__c_s__d__c_s_0052adbc[];
extern char s__c_s__s__c_s_0052adcc[];

void FUN_004de090(int *, int *);
void FUN_004d6460(int *, int, int, int, int);
char *FUN_004ddc10(unsigned int);
void FUN_004db640(int, char *, int *, unsigned int);
int _sprintf(char *, const char *, ...);
void FUN_004db7d0(int, char *, int *, int, int, unsigned int);
void FUN_004d64a0(int *, int, int);
unsigned char FUN_004d62c0(int);
int FUN_004d6240(int);
void FUN_004de5a0(char *, int, int, int);
unsigned char FUN_004d6300(int);
unsigned char FUN_004d62e0(int);
int FUN_004d9d70(int *, int, int, int, int, int, int);
void FUN_004d67e0(int *, int, int, int, int);
void FUN_004d64c0(int *, int, int);
int FUN_004d6b90(int *, int, int);
void FUN_004d6950(int *, int, int, short);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
unsigned int FUN_004bd770(void);

void __stdcall FUN_004bcc10(void)
{
  int iVar1;
  unsigned char uVar2;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  unsigned short uVar9;
  unsigned char *puVar10;
  char *_Format;
  int *piVar11;
  int iVar12;
  int uVar13;
  int iVar14;
  unsigned int uVar15;
  /* local_1c..local_10 form one contiguous 16-byte stack rect:
     local_1c = rect[0], local_18 = rect[1], local_14 = rect[2], local_10 = rect[3] */
  int local_1c[4];
  int local_c;
  int local_8;
  int local_4;

  FUN_004de090(&local_4, &local_8);
  FUN_004d6460(local_1c, 0x54, 0x5a, 0x25c, 0x6a);
  piVar11 = local_1c;
  uVar15 = 0xffff;
  pcVar3 = FUN_004ddc10(0x19f);
  FUN_004db640(0xf, pcVar3, piVar11, uVar15);
  FUN_004d6460(local_1c, 0x58, 0x6f, 0x142, 0x7d);
  if (DAT_00664e90 < 0x10) {
    pcVar4 = (char *)((unsigned int)DAT_00664e90 + 1u);
    pcVar3 = FUN_004ddc10(0x1a1);
    uVar13 = 1;
    pcVar5 = FUN_004ddc10(0x1a0);
    _Format = s__c_s__d__c_s_0052adbc;
  }
  else {
    uVar15 = (unsigned int)DAT_00664e90 + 0x277u;
    pcVar3 = FUN_004ddc10(0x1a1);
    uVar13 = 1;
    pcVar4 = FUN_004ddc10(uVar15);
    pcVar5 = FUN_004ddc10(0x286);
    _Format = s__c_s__s__c_s_0052adcc;
  }
  _sprintf(DAT_00b69d20, _Format, 4, pcVar5, pcVar4, uVar13, pcVar3);
  FUN_004db7d0(0xc, DAT_00b69d20, local_1c, 0, 1, 0xffff);
  FUN_004d64a0(local_1c, 0, 0xf);
  if (DAT_00bb72a0 != 0) {
    if (DAT_00664e94 != 0) {
      _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 2,
               (int)((unsigned int)&DAT_00bb2821 + (unsigned int)DAT_00664e94 * 0x95u), 1,
               (int)((unsigned int)&DAT_00bb2835 + (unsigned int)DAT_00664e94 * 0x95u));
    }
    else {
      _sprintf(DAT_00b69d20, DAT_0052ada8, 2, &DAT_00bb2821);
    }
    FUN_004db7d0(0xc, DAT_00b69d20, local_1c, 0, 1, 0xffff);
  }
  FUN_004d64a0(local_1c, 0, 0x1e);
  piVar11 = local_1c; iVar14 = 1; iVar12 = 0; uVar15 = DAT_00b6a2a0;
  pcVar3 = FUN_004ddc10(0x28d);
  FUN_004db7d0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
  FUN_004d64a0(local_1c, 0, 0xf);
  piVar11 = local_1c; iVar14 = 1; iVar12 = 0; uVar15 = DAT_00b6a2a0;
  pcVar3 = FUN_004ddc10(0x28e);
  FUN_004db7d0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
  FUN_004d64a0(local_1c, 0, 0xf);
  piVar11 = local_1c; iVar14 = 1; iVar12 = 0; uVar15 = DAT_00b6a2a0;
  pcVar3 = FUN_004ddc10(0x28f);
  FUN_004db7d0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
  FUN_004d64a0(local_1c, 0, 0xf);
  piVar11 = local_1c; iVar14 = 1; iVar12 = 0; uVar15 = DAT_00b6a2a0;
  pcVar3 = FUN_004ddc10(0x290);
  FUN_004db7d0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
  FUN_004d64a0(local_1c, 0, 0xf);
  piVar11 = local_1c; iVar14 = 1; iVar12 = 0; uVar15 = DAT_00b6a2a0;
  pcVar3 = FUN_004ddc10(0x291);
  FUN_004db7d0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
  uVar2 = FUN_004d62c0(0);
  iVar12 = (int)uVar2;
  iVar14 = FUN_004d6240(0);
  if (((iVar14 != -1) && (iVar14 < 0x10)) || (iVar12 != 0)) {
    if (iVar14 == -1) {
      if (iVar12 != 0) {
        if (((int)((unsigned int)iVar12 - (unsigned int)DAT_00664e90) != -0xf) &&
            (DAT_00bb73a7 != '\0')) {
          FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
        }
        DAT_00664e90 = (int)((unsigned int)iVar12 + 0xfu);
        uVar2 = FUN_004d6300(0);
        iVar12 = (int)((unsigned int)iVar12 + 0xffffffffu + (unsigned int)(int)uVar2);
        if ((iVar12 < DAT_00664e8c) ||
            (9 < (int)((unsigned int)iVar12 - (unsigned int)DAT_00664e8c))) {
          DAT_00664e8c = iVar12;
        }
      }
    }
    else {
      if ((DAT_00664e90 != iVar14) && (DAT_00bb73a7 != '\0')) {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      DAT_00664e90 = iVar14;
      if ((iVar14 < DAT_00664e8c) ||
          (9 < (int)((unsigned int)iVar14 - (unsigned int)DAT_00664e8c))) {
        DAT_00664e8c = iVar14;
      }
    }
    iVar12 = 0;
    if (0 < DAT_00bb72a0) {
      pcVar3 = DAT_00bb2820;
      do {
        if (*pcVar3 == DAT_00bb7380[DAT_00664e90]) {
          DAT_00664e94 = iVar12;
        }
        pcVar3 = pcVar3 + 0x95;
        iVar12 = (int)((unsigned int)iVar12 + 1u);
      } while (iVar12 < DAT_00bb72a0);
    }
  }
  uVar2 = FUN_004d6300(0);
  iVar12 = (int)uVar2;
  local_c = iVar12;
  uVar2 = FUN_004d62e0(0);
  if ((int)uVar2 != 0) {
    local_c = (int)((unsigned int)iVar12 + 4u);
  }
  FUN_004d6460(local_1c, 0x58, 0xf6, 0x142, 0x104);
  piVar11 = local_1c;
  uVar15 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x28c);
  FUN_004db640(0xc, pcVar3, piVar11, uVar15);
  uVar13 = 0x142;
  if (10 < local_c) {
    FUN_004d6460(local_1c, 0x139, 0x105, 0x142, 0x19f);
    DAT_00664e8c = FUN_004d9d70(local_1c, DAT_00664e8c, local_c, 0, 5, DAT_00bb2810, 1);
    uVar13 = 0x138;
  }
  FUN_004d6460(local_1c, 0x58, 0x105, uVar13, 0x19f);
  FUN_004d67e0(local_1c, 0, 0, 0xffff, 0);
  FUN_004d64c0(local_1c, 2, 2);
  FUN_004d67e0(local_1c, 0, 0, 0, 0);
  local_1c[3] = (int)((unsigned int)local_1c[1] + 0xeu);
  FUN_004d64c0(local_1c, 2, 0);
  iVar14 = DAT_00664e8c;
  if (DAT_00664e8c < (int)((unsigned int)DAT_00664e8c + 10u)) {
    do {
      if (local_c <= iVar14) break;
      iVar8 = 0;
      if (0 < DAT_00bb72a0) {
        pcVar3 = DAT_00bb2820;
        do {
          if (iVar14 < iVar12) {
            if (DAT_00bb7380[iVar14] == *pcVar3) break;
          }
          else if (DAT_00bb7390[(int)((unsigned int)iVar14 - (unsigned int)iVar12)] == *pcVar3) break;
          pcVar3 = pcVar3 + 0x95;
          iVar8 = (int)((unsigned int)iVar8 + 1u);
        } while (iVar8 < DAT_00bb72a0);
      }
      if (iVar8 < DAT_00bb72a0) {
        iVar6 = FUN_004d6b90(local_1c, local_4, local_8);
        iVar7 = DAT_00664e90;
        iVar1 = DAT_00664e94;
        if (iVar6 != 0) {
          FUN_004d6950(local_1c, 0, 0, (short)DAT_00b6a2a0);
          uVar2 = FUN_004dc220();
          if (((int)uVar2 != 0) ||
              (uVar2 = FUN_004dc240(), iVar7 = DAT_00664e90, iVar1 = DAT_00664e94,
               (int)uVar2 != 0)) {
            iVar1 = iVar8;
            if (iVar14 < iVar12) {
              iVar7 = iVar14;
              if ((DAT_00664e90 != iVar14) && (DAT_00bb73a7 != '\0')) {
                FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
              }
            }
            else {
              if (((int)(((unsigned int)iVar12 - (unsigned int)DAT_00664e90) + (unsigned int)iVar14) != 0x10) &&
                  (DAT_00bb73a7 != '\0')) {
                FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
              }
              iVar7 = (int)(((unsigned int)iVar14 - (unsigned int)iVar12) + 0x10u);
            }
          }
        }
        DAT_00664e94 = iVar1;
        DAT_00664e90 = iVar7;
        if (iVar14 < iVar12) {
          uVar9 = 0xffff;
          if (iVar7 == iVar14) {
            uVar9 = (unsigned short)DAT_00b6a2c0;
          }
          pcVar3 = (char *)((unsigned int)iVar14 + 1u);
          iVar8 = (int)((unsigned int)&DAT_00bb2821 + (unsigned int)iVar8 * 0x95u);
          uVar13 = 2;
          pcVar4 = FUN_004ddc10(0x1a0);
          pcVar5 = s__s__d___c_s_0052ad9c;
        }
        else {
          uVar9 = 0xffff;
          if ((int)(((unsigned int)iVar14 - (unsigned int)iVar12) - (unsigned int)iVar7) == -0x10) {
            uVar9 = (unsigned short)DAT_00b6a2c0;
          }
          iVar8 = (int)((unsigned int)&DAT_00bb2821 + (unsigned int)iVar8 * 0x95u);
          uVar13 = 2;
          pcVar3 = FUN_004ddc10(((unsigned int)iVar14 - (unsigned int)iVar12) + 0x287u);
          pcVar4 = FUN_004ddc10(0x286);
          pcVar5 = s__s__s___c_s_0052ad90;
        }
        _sprintf(DAT_00b69d20, pcVar5, pcVar4, pcVar3, uVar13, iVar8);
        FUN_004db7d0(0xc, DAT_00b69d20, local_1c, 0, 1, (unsigned int)uVar9);
      }
      iVar14 = (int)((unsigned int)iVar14 + 1u);
      FUN_004d64a0(local_1c, 0, 0xf);
    } while (iVar14 < (int)((unsigned int)DAT_00664e8c + 10u));
  }
  FUN_004d6460(local_1c, 0x14a, 0x6f, 0x24e, 0x7d);
  piVar11 = local_1c;
  uVar15 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x28b);
  FUN_004db640(0xc, pcVar3, piVar11, uVar15);
  FUN_004d6460(local_1c, 0x24f, 0x7e, 600, 0x19f);
  DAT_00664e88 = FUN_004d9d70(local_1c, DAT_00664e88, DAT_00bb72a0, 0, 5, DAT_00bb2810, 2);
  FUN_004d6460(local_1c, 0x14a, 0x7e, 0x24e, 0x19f);
  FUN_004d67e0(local_1c, 0, 0, 0xffff, 0);
  FUN_004d64c0(local_1c, 2, 2);
  FUN_004d67e0(local_1c, 0, 0, 0, 0);
  local_1c[3] = (int)((unsigned int)local_1c[1] + 0xeu);
  FUN_004d64c0(local_1c, 2, 0);
  if (DAT_00664e88 < (int)((unsigned int)DAT_00664e88 + 0x13u)) {
    puVar10 = (unsigned char *)DAT_00bb2820 + (int)((unsigned int)DAT_00664e88 * 0x95u);
    iVar12 = DAT_00664e88;
    do {
      if (DAT_00bb72a0 <= iVar12) break;
      iVar14 = FUN_004d6b90(local_1c, local_4, local_8);
      if (iVar14 != 0) {
        FUN_004d6950(local_1c, 0, 0, (short)DAT_00b6a2a0);
        uVar2 = FUN_004dc220();
        if (((int)uVar2 != 0) ||
            (uVar2 = FUN_004dc240(), (int)uVar2 != 0)) {
          if ((iVar12 != DAT_00664e94) && (DAT_00bb73a7 != '\0')) {
            FUN_004de5a0(s_settingsound_0052ad80, 1, 0, 0xff);
          }
          DAT_00664e94 = iVar12;
          DAT_00bb7380[DAT_00664e90] = *puVar10;
        }
      }
      _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 2, puVar10 + 1, 1, puVar10 + 0x15);
      uVar15 = DAT_00b6a2c0;
      if (iVar12 != DAT_00664e94) {
        uVar15 = 0xffff;
      }
      puVar10 = puVar10 + 0x95;
      iVar12 = (int)((unsigned int)iVar12 + 1u);
      FUN_004db7d0(0xc, DAT_00b69d20, local_1c, 0, 1, uVar15);
      FUN_004d64a0(local_1c, 0, 0xf);
    } while (iVar12 < (int)((unsigned int)DAT_00664e88 + 0x13u));
  }
  iVar12 = 0;
  uVar15 = FUN_004bd770();
  if (0 < DAT_00bb72a0) {
    pcVar3 = DAT_00bb2820;
    iVar14 = DAT_00bb72a0;
    iVar8 = DAT_00664e90;
    do {
      if (((char)uVar15 != '\0') && (*pcVar3 == (char)uVar15)) {
        if ((iVar12 != DAT_00664e94) && (DAT_00bb73a7 != '\0')) {
          FUN_004de5a0(s_settingsound_0052ad80, 1, 0, 0xff);
          iVar14 = DAT_00bb72a0;
          iVar8 = DAT_00664e90;
        }
        DAT_00664e88 = iVar12;
        DAT_00664e94 = iVar12;
        DAT_00bb7380[iVar8] = *pcVar3;
      }
      pcVar3 = pcVar3 + 0x95;
      iVar12 = (int)((unsigned int)iVar12 + 1u);
    } while (iVar12 < iVar14);
  }
  return;
}
