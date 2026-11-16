/* SPDX-License-Identifier: MIT */
/* z_bop 004fbe90 */
/* Globals */
extern int DAT_0066d898;
extern int DAT_0052c208;
extern int DAT_0052c184;
extern int DAT_0052c188;
extern int DAT_0052c18c;
extern int DAT_00b6a250;
extern int DAT_00b69e30;
extern int DAT_0066d888;
extern int DAT_0066d880;
extern int DAT_0066d894;
extern int DAT_0066d88c;
extern int DAT_00aa6114;
extern void *DAT_00aa6118;
extern uint DAT_00aa6134;
extern uint DAT_00aa6138;
extern uint DAT_00aa6af0;
extern int DAT_00aa6af4;
extern int DAT_00aa5cb0[];
extern uint DAT_00b69e38;
extern uint DAT_00b6a2a0;
extern uint DAT_00b6a2c0;
extern char DAT_00b6a2e0;
extern char DAT_00b6bc9e;
extern char DAT_00b69d20;
extern char DAT_00a91b90;
extern char DAT_00bb73a8;

/* Packed (unaligned) globals: accessed only via __builtin_memcpy */
extern unsigned char DAT_00b6a5aa[];
extern unsigned char DAT_00b6a5ae[];
extern unsigned char DAT_00b6a5a6[];
extern unsigned char DAT_00b6a5ba[];
extern unsigned char DAT_00b6bbaa[];
extern unsigned char DAT_00b6bba6[];
extern unsigned char DAT_00b6bbda[];
extern unsigned char DAT_00b6bc02[];
extern unsigned char DAT_00b6bbae[];
extern unsigned char DAT_00b6d962[];
extern unsigned char DAT_00aa615e;
extern unsigned char DAT_00aa6162;
extern unsigned char DAT_00aa62ae;

/* Strings / labels */
extern char s_frontres_soloteam_bmp_0052c890[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_gameoverlay_0052c374[];
extern char s__c_s__c_s_0052a6b0[];
extern char s_rebtiny_d_0052a6a4[];
extern char s_imptiny_d_0052a698[];
extern char s_buttonsound_0052a68c[];
extern char s_leaveup_0052a678[];
extern char s_leavedown_0052a680[];
extern char s_flysound_0052bf94[];
extern char s_nextup_0052bf80[];
extern char s_nextdown_0052bf88[];
extern char s_dropbtnup_0052c21c[];
extern char s_dropbtndown_0052c228[];
extern unsigned char LAB_004d7370;
extern unsigned char LAB_004e1470;
extern unsigned char LAB_004f5100;
extern unsigned char LAB_004fd1f0;

/* Prototypes */
void FUN_004de0b0(int, int);
void FUN_004b4a60(char *, char *);
void FUN_004e5000(int);
void FUN_004d4e20(void);
void FUN_004b5de0(char *, int, int);
void FUN_004b5570(char *, int, int);
void FUN_004b4d40(char *, int, int);
void FUN_004d4ec0(int);
void FUN_004dc140(int);
DWORD __stdcall GetTickCount(void);
void *_malloc(uint);
void FUN_004e5590(void *);
void FUN_004fcf30(void);
int FUN_004e0490(void);
void FUN_004cd9d0(void);
int FUN_004cfee0(void);
char *FUN_004ddc10(uint);
int FUN_004dcb90(char *, char *, char *, char *, char *);
void FUN_004dc330(void *, void *);
void FUN_004d73c0(void);
void FUN_004f5170(void);
void FUN_004e15d0(void);
void FUN_004fa420(void);
void FUN_004f70f0(void);
void FUN_004d6460(int *, int, int, int, int);
void FUN_004d4cf0(void *);
void FUN_004d4d20(void *);
void FUN_004db640(int, char *, int *, uint);
void FUN_004d64c0(int *, int, int);
void FUN_004db7d0(int, char *, int *, int, int, uint);
void FUN_004fcbf0(void);
void FUN_004d8690(uint);
int _sprintf(char *, const char *, ...);
void FUN_004c9bf0(int);
void FUN_004cef70(int, uint *, int);
int FUN_004d11e0(void);
int FUN_004d11d0(void);
void FUN_004d64a0(int *, int, int);
void FUN_004fcd70(uint);
void FUN_004dad40(void);
void FUN_004dad70(char *);
int FUN_004da780(int *, char *, char *, char *, int, uint, int, char *);
void FUN_004dad60(void);
void FUN_004dc380(void *, int *);
int FUN_004bf9b0(int);

bool FUN_004fbe90(uint param_1)
{
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  int *piVar4;
  bool bVar5;
  char *pcVar6;
  uint uVar7;
  char *pcVar8;
  uint uVar9;
  uint *puVar10;
  int iVar11;
  void *pcVar12;
  char *pcVar13;
  uint uVar14;
  void *puVar15;
  int local_20[4];
  uint local_10[4];
  unsigned char *pbVar;
  int tA;
  int tB;
  int tC;

  if (param_1 == 0) {
    DAT_0066d898 = 0;
    FUN_004de0b0(0x25, 0x1bd);
    DAT_0052c208 = 0;
    FUN_004b4a60(s_frontres_soloteam_bmp_0052c890, s_background_0052bf74);
    __builtin_memcpy(&tA, DAT_00b6a5aa, 4);
    FUN_004e5000(tA);
    if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
      piVar4 = (int *)(DAT_00aa6114 + 0x140);
      do {
        __builtin_memcpy(&tA, DAT_00b6a5aa, 4);
        __builtin_memcpy(&tB, DAT_00b6a5ae + (uint)tA * 4u, 4);
        if (*piVar4 == tB) break;
        piVar4 = piVar4 + 0x52;
        DAT_00aa6134 = DAT_00aa6134 + 1;
      } while (DAT_00aa6134 < DAT_00aa6138);
    }
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    if (DAT_00b6a250 == 0) {
      pcVar6 = s_clientactive_0052b69c;
    }
    else {
      pcVar6 = s_allactive_0052b6ac;
    }
    FUN_004b5570(pcVar6, 0, 0);
    if (DAT_00b69e30 != 2) {
      FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
    }
    FUN_004b4d40(s_gameoverlay_0052c374, 0, 0);
    FUN_004d4ec0(1);
    FUN_004dc140(0x14);
    DAT_0066d888 = 120000;
    DAT_0066d880 = GetTickCount();
    DAT_0066d894 = 0x78;
    DAT_0066d88c = DAT_0066d880;
    DAT_00aa6118 = _malloc(0x1000);
    FUN_004e5590(DAT_00aa6118);
    FUN_004fcf30();
  }
  if (DAT_00b69e30 != 2) {
    iVar1 = FUN_004e0490();
    if (iVar1 == 0x46) {
      FUN_004cd9d0();
      iVar1 = FUN_004cfee0();
      if (iVar1 == 0) {
        pcVar13 = (char *)0x0;
        pcVar8 = (char *)0x0;
        pcVar6 = FUN_004ddc10(0x279);
        pcVar2 = FUN_004ddc10(0x278);
        pcVar3 = FUN_004ddc10(0x277);
        FUN_004dcb90(pcVar3, pcVar2, pcVar6, pcVar8, pcVar13);
      }
      DAT_0052c184 = 1;
      DAT_00b69e30 = 3;
      FUN_004dc330((void *)FUN_004d73c0, &LAB_004d7370);
    }
    else if (iVar1 == 0x41) {
      FUN_004fa420();
    }
    else {
      if (iVar1 == 0x58) {
        bVar5 = DAT_00aa6114 != 0;
        __builtin_memcpy(DAT_00b6bbaa, &DAT_0052c18c, 4);
        __builtin_memcpy(&tA, DAT_00b6bba6, 4);
        __builtin_memcpy(DAT_00b6bbda + (uint)tA * 4u, &DAT_0052c18c, 4);
        if ((bVar5) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
          piVar4 = (int *)(DAT_00aa6114 + 0x140);
          do {
            if (*piVar4 == DAT_0052c18c) {
              __builtin_memcpy(&tA, DAT_00b6bba6, 4);
              __builtin_memcpy(DAT_00b6bc02 + (uint)tA * 4u, &DAT_00aa6134, 4);
              break;
            }
            piVar4 = piVar4 + 0x52;
            DAT_00aa6134 = DAT_00aa6134 + 1;
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
        FUN_004f70f0();
        FUN_004dc330((void *)FUN_004f5170, &LAB_004f5100);
        return 0;
      }
      if (iVar1 == 0x53) {
        DAT_0052c184 = 1;
        FUN_004dc330((void *)FUN_004e15d0, &LAB_004e1470);
        return 0;
      }
      if (iVar1 == 100) {
        if (DAT_0052c18c < DAT_0066d888) {
          DAT_0066d888 = DAT_0052c18c;
        }
      }
      else if (iVar1 == 0x75) {
        iVar1 = 0;
        pbVar = &DAT_00aa615e;
        do {
          __builtin_memcpy(&tA, pbVar, 4);
          if (tA == DAT_0052c188) {
            __builtin_memcpy(&DAT_00aa6162 + (uint)iVar1 * 0x2au, &DAT_0052c18c, 4);
            break;
          }
          pbVar = pbVar + 0x2a;
          iVar1 = iVar1 + 1;
        } while (pbVar < &DAT_00aa62ae);
      }
    }
  }
  FUN_004d6460(local_20, 0x9e, 0x34, 0x1eb, 0x44);
  FUN_004d4cf0(local_10);
  FUN_004d4d20(local_20);
  FUN_004db640(0xc, &DAT_00a91b90, local_20, 0xffff);
  FUN_004d4d20(local_10);
  FUN_004d6460(local_20, 0x54, 0x5a, 0x1b2, 0x6c);
  piVar4 = local_20;
  uVar14 = 0xffff;
  pcVar6 = FUN_004ddc10(0x32c);
  FUN_004db640(0xf, pcVar6, piVar4, uVar14);
  iVar1 = FUN_004cfee0();
  if ((iVar1 == 0) && (DAT_00b69e30 != 2)) {
    uVar9 = 0x1b2;
  }
  else {
    uVar9 = 0x1a0;
  }
  FUN_004d6460(local_20, 0x54, 0x70, uVar9, 0x82);
  if (DAT_00aa6134 < DAT_00aa6138) {
    FUN_004d64c0(local_20, 4, 0);
    FUN_004db7d0(0xf, (char *)(DAT_00aa6134 * 0x148 + DAT_00aa6114 + 0x40), local_20, 0, 1, DAT_00b6a2c0);
  }
  FUN_004fcbf0();
  if (DAT_00b69e30 != 2) {
    FUN_004d8690(param_1);
  }
  FUN_004d6460(local_20, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_00b6a2e0 != '\0') {
    _sprintf(&DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, &DAT_00b6bc9e, 1, &DAT_00b6a2e0);
    FUN_004db640(0xc, &DAT_00b69d20, local_20, DAT_00b6a2c0);
    uVar14 = (uint)((int)param_1 >> 0x1f);
    if (DAT_00b69e30 == 2) {
      __builtin_memcpy(&tA, DAT_00b6a5aa, 4);
      if ((tA == 1) || (tA == 2)) {
        iVar1 = (int)(((((param_1 ^ uVar14) - uVar14) & 0x1f) ^ uVar14) - uVar14);
        goto LAB_004fc401;
      }
      __builtin_memcpy(&tB, DAT_00b6d962, 4);
      if (tB == 0) {
        _sprintf(&DAT_00b69d20, s_rebtiny_d_0052a6a4,
                 (int)(((((param_1 ^ uVar14) - uVar14) & 0x1f) ^ uVar14) - uVar14) >> 1);
        FUN_004b5570(&DAT_00b69d20, 0xcc, 0x1c5);
      }
      else {
        _sprintf(&DAT_00b69d20, s_imptiny_d_0052a698,
                 (int)(((((param_1 ^ uVar14) - uVar14) & 0x1f) ^ uVar14) - uVar14) >> 1);
        FUN_004b5570(&DAT_00b69d20, 0xcc, 0x1c5);
      }
    }
    else {
      iVar1 = (int)(((((param_1 ^ uVar14) - uVar14) & 0x1f) ^ uVar14) - uVar14);
LAB_004fc401:
      _sprintf(&DAT_00b69d20, s_rebtiny_d_0052a6a4, iVar1 >> 1);
      FUN_004b5570(&DAT_00b69d20, 0xcc, 0x1c5);
      _sprintf(&DAT_00b69d20, s_imptiny_d_0052a698, iVar1 >> 1);
    }
    FUN_004b5570(&DAT_00b69d20, 0x1a4, 0x1c5);
  }
  if (DAT_00b69e30 != 2) {
    FUN_004d6460(local_20, 0x1fb, 0x1c4, 0x232, 0x1d0);
    FUN_004c9bf0(DAT_0066d888 / 1000);
    FUN_004db640(0xc, &DAT_00b69d20, local_20, 0xffff);
    if (DAT_00b69e30 != 2) {
      DAT_0066d880 = GetTickCount();
      DAT_0066d888 = (int)((uint)DAT_0066d888 + ((uint)DAT_0066d88c - (uint)DAT_0066d880));
      iVar1 = FUN_004cfee0();
      if ((iVar1 != 0) && (DAT_0066d894 != DAT_0066d888 / 1000)) {
        DAT_00aa6af4 = DAT_0066d888;
        DAT_00aa6af0 = 100;
        DAT_0066d894 = DAT_0066d888 / 1000;
        FUN_004cef70(0, &DAT_00aa6af0, 8);
      }
      if (DAT_0066d888 < 0) {
        DAT_0066d888 = 0;
        if (DAT_0066d898 == 0) {
          DAT_0066d898 = 1;
          __builtin_memcpy(&tA, DAT_00b6bba6, 4);
          __builtin_memcpy(&tB, DAT_00b6bbae + (uint)tA * 4u, 4);
          __builtin_memcpy(&tC, DAT_00b6a5a6, 4);
          if (tB != tC) {
            iVar1 = FUN_004d11e0();
            if (iVar1 == DAT_00aa5cb0[(uint)tC * 8u]) {
              uVar14 = 4;
              puVar10 = &DAT_00aa6af0;
              __builtin_memcpy(&DAT_00aa6af4, DAT_00b6a5ba, 4);
              DAT_00aa6af0 = 0x7d;
              iVar1 = FUN_004d11d0();
              FUN_004cef70(iVar1, puVar10, uVar14);
            }
          }
        }
        return 0;
      }
      DAT_0066d88c = DAT_0066d880;
    }
  }
  FUN_004d6460(local_20, 0x54, 400, 0x1b2, 0x19e);
  if (DAT_00b69e30 == 2) {
    piVar4 = local_20;
    uVar14 = DAT_00b6a2a0;
    pcVar6 = FUN_004ddc10(0x32d);
    FUN_004db640(0xc, pcVar6, piVar4, uVar14);
    FUN_004d64a0(local_20, 0, 0xf);
    uVar7 = 0x32e;
    uVar14 = DAT_00b6a2a0;
  }
  else {
    __builtin_memcpy(&tA, DAT_00b6bba6, 4);
    __builtin_memcpy(&tB, DAT_00b6bbae + (uint)tA * 4u, 4);
    __builtin_memcpy(&tC, DAT_00b6a5a6, 4);
    if (tB == tC) {
      piVar4 = local_20;
      uVar14 = DAT_00b69e38;
      pcVar6 = FUN_004ddc10(0x32f);
      FUN_004db640(0xc, pcVar6, piVar4, uVar14);
      FUN_004d64a0(local_20, 0, 0xf);
      uVar7 = 0x330;
      uVar14 = DAT_00b69e38;
    }
    else {
      iVar1 = FUN_004d11e0();
      if (iVar1 == DAT_00aa5cb0[(uint)tC * 8u]) {
        piVar4 = local_20;
        uVar14 = DAT_00b6a2a0;
        pcVar6 = FUN_004ddc10(0x331);
        FUN_004db640(0xc, pcVar6, piVar4, uVar14);
        FUN_004d64a0(local_20, 0, 0xf);
        uVar7 = 0x32e;
        uVar14 = DAT_00b6a2a0;
      }
      else {
        piVar4 = local_20;
        uVar14 = DAT_00b69e38;
        pcVar6 = FUN_004ddc10(0x331);
        FUN_004db640(0xc, pcVar6, piVar4, uVar14);
        FUN_004d64a0(local_20, 0, 0xf);
        uVar7 = 0x332;
        uVar14 = DAT_00b69e38;
      }
    }
  }
  piVar4 = local_20;
  pcVar6 = FUN_004ddc10(uVar7);
  FUN_004db640(0xc, pcVar6, piVar4, uVar14);
  FUN_004fcd70(param_1);
  FUN_004d6460(local_20, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_00bb73a8 != '\0') {
    FUN_004dad40();
  }
  if (DAT_00b69e30 == 2) {
    pcVar6 = FUN_004ddc10(0x239);
    FUN_004dad70(pcVar6);
    pcVar2 = s_buttonsound_0052a68c;
    iVar11 = 8;
    uVar9 = 0;
    iVar1 = 0xc;
    pcVar6 = FUN_004ddc10(0x104);
    iVar1 = FUN_004da780(local_20, s_leaveup_0052a678, s_leavedown_0052a680, pcVar6, iVar1, uVar9, iVar11, pcVar2);
    if (iVar1 == 0) goto LAB_004fc9d8;
    pcVar6 = FUN_004ddc10(0x13);
    pcVar2 = FUN_004ddc10(0x20b);
    pcVar3 = FUN_004ddc10(0x2ab);
    pcVar8 = FUN_004ddc10(0x2aa);
    pcVar13 = FUN_004ddc10(0x2a9);
    iVar1 = FUN_004dcb90(pcVar13, pcVar8, pcVar3, pcVar2, pcVar6);
    if (iVar1 == 0) goto LAB_004fc9d8;
    DAT_0052c184 = 1;
    puVar15 = &LAB_004e1470;
    pcVar12 = (void *)FUN_004e15d0;
  }
  else {
    iVar1 = FUN_004cfee0();
    if (iVar1 != 0) {
      pcVar6 = FUN_004ddc10(0x29c);
      FUN_004dad70(pcVar6);
      pcVar2 = s_buttonsound_0052a68c;
      iVar11 = 8;
      uVar9 = 0;
      iVar1 = 0xc;
      pcVar6 = FUN_004ddc10(0x104);
      iVar1 = FUN_004da780(local_20, s_leaveup_0052a678, s_leavedown_0052a680, pcVar6, iVar1, uVar9, iVar11, pcVar2);
      if (iVar1 != 0) {
        pcVar6 = FUN_004ddc10(0x13);
        pcVar2 = FUN_004ddc10(0x20b);
        pcVar3 = FUN_004ddc10(0x2f2);
        pcVar8 = FUN_004ddc10(0x2f1);
        pcVar13 = FUN_004ddc10(0x2f0);
        iVar1 = FUN_004dcb90(pcVar13, pcVar8, pcVar3, pcVar2, pcVar6);
        if (iVar1 != 0) {
          DAT_00aa6af0 = 0x53;
          FUN_004cef70(0, &DAT_00aa6af0, 4);
        }
      }
      goto LAB_004fc9d8;
    }
    pcVar6 = FUN_004ddc10(0xcc);
    FUN_004dad70(pcVar6);
    pcVar2 = s_buttonsound_0052a68c;
    iVar11 = 8;
    uVar9 = 0;
    iVar1 = 0xc;
    pcVar6 = FUN_004ddc10(0xcc);
    iVar1 = FUN_004da780(local_20, s_leaveup_0052a678, s_leavedown_0052a680, pcVar6, iVar1, uVar9, iVar11, pcVar2);
    if (iVar1 == 0) goto LAB_004fc9d8;
    pcVar6 = FUN_004ddc10(0x13);
    pcVar2 = FUN_004ddc10(0x20b);
    pcVar3 = FUN_004ddc10(0x22d);
    pcVar8 = FUN_004ddc10(0x22c);
    pcVar13 = FUN_004ddc10(0x22b);
    iVar1 = FUN_004dcb90(pcVar13, pcVar8, pcVar3, pcVar2, pcVar6);
    if (iVar1 == 0) goto LAB_004fc9d8;
    DAT_0052c184 = 1;
    DAT_00aa6af0 = 0x47;
    uVar14 = 4;
    puVar10 = &DAT_00aa6af0;
    iVar1 = FUN_004d11d0();
    FUN_004cef70(iVar1, puVar10, uVar14);
    FUN_004cd9d0();
    puVar15 = &LAB_004d7370;
    pcVar12 = (void *)FUN_004d73c0;
  }
  FUN_004dc330(pcVar12, puVar15);
LAB_004fc9d8:
  FUN_004d6460(local_20, 8, 0x195, 0x47, 0x1d6);
  if (DAT_00b69e30 == 2) {
    pcVar6 = FUN_004ddc10(0xd4);
    FUN_004dad70(pcVar6);
    pcVar2 = s_flysound_0052bf94;
    iVar11 = 7;
    uVar9 = 0;
    iVar1 = 0xc;
    pcVar6 = FUN_004ddc10(0x1db);
    iVar1 = FUN_004da780(local_20, s_nextup_0052bf80, s_nextdown_0052bf88, pcVar6, iVar1, uVar9, iVar11, pcVar2);
    if (iVar1 != 0) {
      FUN_004dc330((void *)FUN_004f5170, &LAB_004f5100);
      FUN_004dad60();
      return 0;
    }
  }
  else {
    __builtin_memcpy(&tA, DAT_00b6bba6, 4);
    __builtin_memcpy(&tB, DAT_00b6bbae + (uint)tA * 4u, 4);
    __builtin_memcpy(&tC, DAT_00b6a5a6, 4);
    if ((tB != tC) &&
        (iVar1 = FUN_004d11e0(), iVar1 == DAT_00aa5cb0[(uint)tC * 8u])) {
      pcVar6 = FUN_004ddc10(0xd4);
      FUN_004dad70(pcVar6);
      pcVar2 = s_flysound_0052bf94;
      iVar11 = 7;
      uVar9 = 0;
      iVar1 = 0xc;
      pcVar6 = FUN_004ddc10(0x1db);
      iVar1 = FUN_004da780(local_20, s_nextup_0052bf80, s_nextdown_0052bf88, pcVar6, iVar1, uVar9, iVar11, pcVar2);
      if (iVar1 != 0) {
        uVar14 = 4;
        puVar10 = &DAT_00aa6af0;
        __builtin_memcpy(&DAT_00aa6af4, DAT_00b6a5ba, 4);
        DAT_00aa6af0 = 0x7d;
        iVar1 = FUN_004d11d0();
        FUN_004cef70(iVar1, puVar10, uVar14);
      }
    }
  }
  bVar5 = 0;
  FUN_004dad60();
  if (DAT_00b69e30 == 2) {
    bVar5 = 1;
  }
  else {
    __builtin_memcpy(&tA, DAT_00b6bba6, 4);
    __builtin_memcpy(&tB, DAT_00b6bbae + (uint)tA * 4u, 4);
    __builtin_memcpy(&tC, DAT_00b6a5a6, 4);
    if ((tB != tC) &&
        (iVar1 = FUN_004d11e0(), iVar1 == DAT_00aa5cb0[(uint)tC * 8u])) {
      bVar5 = 1;
    }
  }
  if (bVar5) {
    FUN_004d6460(local_20, 0x1a1, 0x70, 0x1b2, 0x82);
    pcVar2 = s_buttonsound_0052a68c;
    iVar11 = 9;
    uVar9 = 0xffff;
    iVar1 = 0xf;
    pcVar6 = FUN_004ddc10(0x2ac);
    iVar1 = FUN_004da780(local_20, s_dropbtnup_0052c21c, s_dropbtndown_0052c228, pcVar6, iVar1, uVar9, iVar11, pcVar2);
    if (iVar1 != 0) {
      FUN_004d6460(local_20, 0, 0, 0x27f, 0x1df);
      FUN_004dc380(&LAB_004fd1f0, local_20);
    }
  }
  iVar1 = FUN_004bf9b0(1);
  return iVar1 == 1;
}
