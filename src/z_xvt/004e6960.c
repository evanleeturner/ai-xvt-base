/* SPDX-License-Identifier: MIT */
/* z_xvt 004e6960 */
extern int DAT_0081b850;
extern unsigned char DAT_0081bfba[4];
extern unsigned int DAT_0081bcd0;
extern char DAT_0081b740[];
extern char s_settingsound_0053199c[];
extern char s__d__s_0053221c[];
extern unsigned char DAT_0085e78f;
extern unsigned char DAT_0085e790;
extern unsigned char DAT_0085e7b4;
extern unsigned char DAT_0085e7b5;
extern unsigned char DAT_0085e7b6;
extern unsigned char DAT_0085e7b7;
extern unsigned char DAT_0085e7b8;
extern unsigned char DAT_0085e7b9;
extern unsigned char DAT_0085e7ba;
extern unsigned char DAT_0085e7bb;
extern unsigned char DAT_0085e7bc;
extern unsigned char DAT_0085e7bd;
extern unsigned char DAT_0085e7be;
extern unsigned char DAT_0085e7bf;
extern unsigned char DAT_0085e7c4;
extern unsigned int DAT_00758620;
extern unsigned int DAT_00758624;
extern unsigned int DAT_00758628;
extern unsigned int DAT_0075862c;
extern unsigned int DAT_00758630;
extern unsigned int DAT_00758634;
extern unsigned int DAT_00758638;
extern unsigned int DAT_0075863c;
extern unsigned int DAT_00758640;
extern unsigned int DAT_00758644;
extern unsigned int DAT_00758648;
extern unsigned int DAT_0075864c;
extern unsigned int DAT_00758650;
extern unsigned int DAT_00758654;
extern unsigned int DAT_00758658;
extern unsigned int DAT_0075865c;
extern unsigned int DAT_00758660;

char *FUN_004dcfa0(int param_1);
void FUN_004da7c0(int a, char *b, int c, int d, unsigned int e);
int FUN_004cc810(void);
int FUN_004db2f0(char *a, int b);
void FUN_004dcfd0(int *a, int b, int c, int d, int e);
int FUN_004d9950(int *a, char *b, int *c, unsigned int d, int e, char *f);
void FUN_004daac0(int a, char *b, int *c, int d, int e, unsigned int f);
int _sprintf(char *buf, const char *fmt, ...);
int _rand(void);
void FUN_004cb830(int a, unsigned int *b, int c);

unsigned int __stdcall FUN_004e6960(void)
{
  char cVar1;
  _Bool bVar2;
  char *pcVar3;
  unsigned int uVar4;
  int iVar5;
  int iVar6;
  _Bool bVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  unsigned int uVar11;
  char *pcVar12;
  int local_1c;
  int local_10[4];
  int local_fba;

  uVar11 = 0xffff;
  iVar10 = 0x137;
  iVar9 = 0x58;
  iVar5 = 0x58;
  pcVar3 = FUN_004dcfa0(0x284);
  iVar6 = 0x14a;
  FUN_004da7c0(0xf, pcVar3, iVar9, iVar10, uVar11);
  local_1c = 0;
  bVar7 = 0;
  iVar9 = FUN_004cc810();
  if ((iVar9 == 0) && (DAT_0081b850 != 2)) {
    bVar2 = 0;
  }
  else {
    bVar2 = 1;
  }
  if (DAT_0081b850 == 2) {
LAB_004e6aaf:
    uVar11 = 0xffff;
    iVar10 = 0x58;
    iVar9 = iVar6;
    pcVar3 = FUN_004dcfa0(0x198);
    FUN_004da7c0(10, pcVar3, iVar10, iVar9, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x198);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + 0x5a, iVar6, 0xfd, iVar6 + 0xd);
    if (bVar2) {
      pcVar12 = s_settingsound_0053199c;
      iVar9 = 0x14;
      uVar11 = 0xffff;
      piVar8 = (int *)0xa;
      pcVar3 = FUN_004dcfa0(DAT_0085e7b4 + 0x1a4);
      iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
      if (iVar9 != 0) {
        bVar7 = 1;
        DAT_0085e7b4 = DAT_0085e7b4 + 1;
        if (2 < DAT_0085e7b4) {
          DAT_0085e7b4 = 0;
        }
      }
    }
    else {
      piVar8 = local_10;
      iVar10 = 1;
      iVar9 = 0;
      uVar11 = DAT_0081bcd0;
      pcVar3 = FUN_004dcfa0(DAT_0085e7b4 + 0x1a4);
      FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
    }
    iVar6 = iVar6 + 0xe;
    local_1c = local_1c + 1;
    if (local_1c == 6) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
  }
  else {
    if (bVar2) {
      uVar11 = 0xffff;
      iVar6 = 0x14a;
      iVar9 = 0x58;
      pcVar3 = FUN_004dcfa0(0x19d);
      FUN_004da7c0(10, pcVar3, iVar9, iVar6, uVar11);
      iVar9 = 10;
      pcVar3 = FUN_004dcfa0(0x19d);
      iVar9 = FUN_004db2f0(pcVar3, iVar9);
      FUN_004dcfd0(local_10, iVar9 + 0x5a, 0x14a, 0xfd, 0x157);
      pcVar12 = s_settingsound_0053199c;
      iVar9 = 0x15;
      uVar11 = 0xffff;
      piVar8 = (int *)0xa;
      pcVar3 = FUN_004dcfa0(DAT_0085e7b9 + 0x1aa);
      iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
      bVar7 = iVar9 != 0;
      if (bVar7) {
        DAT_0085e7b9 = DAT_0085e7b9 ^ 1;
      }
      local_1c = 1;
      iVar6 = 0x158;
    }
    __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
    if ((DAT_0081b850 == 2) || ((local_fba != 3 && (local_fba != 4)))) goto LAB_004e6aaf;
  }
  if (DAT_0081b850 != 2) {
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x1a2);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x1a2);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
    if (DAT_0085e7be == 0) {
      pcVar3 = FUN_004dcfa0(0x1b0);
      uVar11 = 0xffffffff;
      do {
        pcVar12 = pcVar3;
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        pcVar12 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar12;
      } while (cVar1 != '\0');
      uVar11 = ~uVar11;
      pcVar3 = pcVar12 + -uVar11;
      pcVar12 = DAT_0081b740;
      for (uVar4 = uVar11 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        __builtin_memcpy(pcVar12, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
        *pcVar12 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar12 = pcVar12 + 1;
      }
    }
    else if (DAT_0085e7be == 0xff) {
      pcVar3 = FUN_004dcfa0(0x1b1);
      uVar11 = 0xffffffff;
      do {
        pcVar12 = pcVar3;
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        pcVar12 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar12;
      } while (cVar1 != '\0');
      uVar11 = ~uVar11;
      pcVar3 = pcVar12 + -uVar11;
      pcVar12 = DAT_0081b740;
      for (uVar4 = uVar11 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        __builtin_memcpy(pcVar12, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
        *pcVar12 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar12 = pcVar12 + 1;
      }
    }
    else {
      pcVar3 = FUN_004dcfa0(0x1b3);
      _sprintf(DAT_0081b740, s__d__s_0053221c, (unsigned int)DAT_0085e7be, pcVar3);
    }
    if (bVar2) {
      iVar9 = FUN_004d9950(local_10, DAT_0081b740, (int *)0xa, 0xffff, 0x16,
                           s_settingsound_0053199c);
      if (iVar9 == 1) {
        bVar7 = 1;
        if (DAT_0085e7be == 0) {
          DAT_0085e7be = 0xff;
        }
        else if (DAT_0085e7be == 0xff) {
          DAT_0085e7be = 1;
        }
        else {
          DAT_0085e7be = DAT_0085e7be + 1;
          if (0x14 < DAT_0085e7be) {
            DAT_0085e7be = 0;
          }
        }
      }
      if (iVar9 == 2) {
        bVar7 = 1;
        if (DAT_0085e7be == 0xff) {
          DAT_0085e7be = 0;
        }
        else if (DAT_0085e7be == 0) {
          DAT_0085e7be = 0x14;
        }
        else {
          DAT_0085e7be = DAT_0085e7be - 1;
          if (DAT_0085e7be == 0) {
            DAT_0085e7be = 0xff;
          }
        }
      }
    }
    else {
      FUN_004daac0(10, DAT_0081b740, local_10, 0, 1, DAT_0081bcd0);
    }
    iVar6 = iVar6 + 0xe;
    if (local_1c == 5) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x1a3);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x1a3);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
    if (DAT_0085e7bf == 0) {
      pcVar3 = FUN_004dcfa0(0x1b0);
      uVar11 = 0xffffffff;
      do {
        pcVar12 = pcVar3;
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        pcVar12 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar12;
      } while (cVar1 != '\0');
      uVar11 = ~uVar11;
      pcVar3 = pcVar12 + -uVar11;
      pcVar12 = DAT_0081b740;
      for (uVar4 = uVar11 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        __builtin_memcpy(pcVar12, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
        *pcVar12 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar12 = pcVar12 + 1;
      }
    }
    else {
      pcVar3 = FUN_004dcfa0(0x1b3);
      _sprintf(DAT_0081b740, s__d__s_0053221c, (unsigned int)DAT_0085e7bf, pcVar3);
    }
    if (bVar2) {
      iVar9 = FUN_004d9950(local_10, DAT_0081b740, (int *)0xa, 0xffff, 0x17,
                           s_settingsound_0053199c);
      if (iVar9 == 1) {
        bVar7 = 1;
        DAT_0085e7bf = DAT_0085e7bf + 1;
        if (10 < DAT_0085e7bf) {
          DAT_0085e7bf = 0;
        }
      }
      if (iVar9 == 2) {
        bVar7 = 1;
        if (DAT_0085e7bf == 0) {
          DAT_0085e7bf = 10;
        }
        else {
          DAT_0085e7bf = DAT_0085e7bf - 1;
        }
      }
    }
    else {
      FUN_004daac0(10, DAT_0081b740, local_10, 0, 1, DAT_0081bcd0);
    }
    iVar6 = iVar6 + 0xe;
    local_1c = local_1c + 2;
    if (local_1c == 6) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
  }
  __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
  if (local_fba == 4) {
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x19c);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x19c);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
    if (bVar2) {
      pcVar12 = s_settingsound_0053199c;
      iVar9 = 0x1a;
      uVar11 = 0xffff;
      piVar8 = (int *)0xa;
      pcVar3 = FUN_004dcfa0(DAT_0085e7b8 + 0x1a7);
      iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
      if (iVar9 != 0) {
        bVar7 = 1;
        DAT_0085e7b8 = DAT_0085e7b8 + 1;
        if (2 < DAT_0085e7b8) {
          DAT_0085e7b8 = 0;
        }
      }
    }
    else {
      piVar8 = local_10;
      iVar10 = 1;
      iVar9 = 0;
      uVar11 = DAT_0081bcd0;
      pcVar3 = FUN_004dcfa0(DAT_0085e7b8 + 0x1a7);
      FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
    }
    iVar6 = iVar6 + 0xe;
    local_1c = local_1c + 1;
    if (local_1c == 6) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
    __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
    if (local_fba != 4) goto LAB_004e7061;
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x2d2);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    uVar11 = 0x2d2;
  }
  else {
LAB_004e7061:
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x19b);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    uVar11 = 0x19b;
  }
  iVar9 = 10;
  pcVar3 = FUN_004dcfa0(uVar11);
  iVar9 = FUN_004db2f0(pcVar3, iVar9);
  FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
  if (bVar2) {
    pcVar12 = s_settingsound_0053199c;
    iVar9 = 0x18;
    uVar11 = 0xffff;
    piVar8 = (int *)0xa;
    pcVar3 = FUN_004dcfa0(DAT_0085e7b7 + 0xdf);
    iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
    if (iVar9 != 0) {
      bVar7 = 1;
      DAT_0085e7b7 = DAT_0085e7b7 ^ 1;
    }
  }
  else {
    piVar8 = local_10;
    iVar10 = 1;
    iVar9 = 0;
    uVar11 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(DAT_0085e7b7 + 0xdf);
    FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
  }
  iVar6 = iVar6 + 0xe;
  iVar9 = local_1c + 1;
  if (iVar9 == 6) {
    iVar5 = 0x106;
    iVar6 = 0x14a;
  }
  if (DAT_0081b850 != 2) {
    __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
    if ((local_fba == 1) || (local_fba == 2)) {
      uVar11 = 0xffff;
      iVar9 = iVar5;
      iVar10 = iVar6;
      pcVar3 = FUN_004dcfa0(0x22e);
      FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
      iVar9 = 10;
      pcVar3 = FUN_004dcfa0(0x22e);
      iVar9 = FUN_004db2f0(pcVar3, iVar9);
      FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
      if (bVar2) {
        pcVar12 = s_settingsound_0053199c;
        iVar9 = 0x19;
        uVar11 = 0xffff;
        piVar8 = (int *)0xa;
        pcVar3 = FUN_004dcfa0(DAT_0085e7c4 + 0xdf);
        iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
        if (iVar9 != 0) {
          bVar7 = 1;
          DAT_0085e7c4 = DAT_0085e7c4 ^ 1;
        }
      }
      else {
        piVar8 = local_10;
        iVar10 = 1;
        iVar9 = 0;
        uVar11 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(DAT_0085e7c4 + 0xdf);
        FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
      }
      iVar6 = iVar6 + 0xe;
      iVar9 = local_1c + 2;
      if (iVar9 == 6) {
        iVar5 = 0x106;
        iVar6 = 0x14a;
      }
    }
    local_1c = iVar9;
    if (DAT_0081b850 == 2) goto LAB_004e74a8;
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x19f);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x19f);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
    if (bVar2) {
      pcVar12 = s_settingsound_0053199c;
      iVar9 = 0x1b;
      uVar11 = 0xffff;
      piVar8 = (int *)0xa;
      pcVar3 = FUN_004dcfa0(DAT_0085e7bb + 0x20a);
      iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
      if (iVar9 != 0) {
        bVar7 = 1;
        DAT_0085e7bb = DAT_0085e7bb + 1;
        if (2 < DAT_0085e7bb) {
          DAT_0085e7bb = 0;
        }
      }
    }
    else {
      piVar8 = local_10;
      iVar10 = 1;
      iVar9 = 0;
      uVar11 = DAT_0081bcd0;
      pcVar3 = FUN_004dcfa0(DAT_0085e7bb + 0x20a);
      FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
    }
    __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
    if (((DAT_0085e7bb == 2) && (local_fba != 1)) && (local_fba != 2)) {
      DAT_0085e7bb = 0;
    }
    iVar6 = iVar6 + 0xe;
    iVar9 = local_1c + 1;
    if (iVar9 == 6) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
  }
  local_1c = iVar9;
  if (DAT_0081b850 != 2) {
    uVar11 = 0xffff;
    iVar9 = iVar5;
    iVar10 = iVar6;
    pcVar3 = FUN_004dcfa0(0x1a0);
    FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
    iVar9 = 10;
    pcVar3 = FUN_004dcfa0(0x1a0);
    iVar9 = FUN_004db2f0(pcVar3, iVar9);
    FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
    if (bVar2) {
      pcVar12 = s_settingsound_0053199c;
      iVar9 = 0x1c;
      uVar11 = 0xffff;
      piVar8 = (int *)0xa;
      pcVar3 = FUN_004dcfa0(DAT_0085e7bc + 0x1ae);
      iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
      if (iVar9 != 0) {
        bVar7 = 1;
        DAT_0085e7bc = DAT_0085e7bc ^ 1;
      }
    }
    else {
      piVar8 = local_10;
      iVar10 = 1;
      iVar9 = 0;
      uVar11 = DAT_0081bcd0;
      pcVar3 = FUN_004dcfa0(DAT_0085e7bc + 0x1ae);
      FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
    }
    iVar6 = iVar6 + 0xe;
    local_1c = local_1c + 1;
    if (local_1c == 6) {
      iVar5 = 0x106;
      iVar6 = 0x14a;
    }
  }
LAB_004e74a8:
  uVar11 = 0xffff;
  iVar9 = iVar5;
  iVar10 = iVar6;
  pcVar3 = FUN_004dcfa0(0x1a1);
  FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
  iVar9 = 10;
  pcVar3 = FUN_004dcfa0(0x1a1);
  iVar9 = FUN_004db2f0(pcVar3, iVar9);
  FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
  if (bVar2) {
    pcVar12 = s_settingsound_0053199c;
    iVar9 = 0x1d;
    uVar11 = 0xffff;
    piVar8 = (int *)0xa;
    pcVar3 = FUN_004dcfa0(DAT_0085e7bd + 0x1b0);
    iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
    if (iVar9 != 0) {
      bVar7 = 1;
      DAT_0085e7bd = DAT_0085e7bd + 1;
      if (2 < DAT_0085e7bd) {
        DAT_0085e7bd = 0;
      }
      __builtin_memcpy(&local_fba, &DAT_0081bfba, 4);
      if (((DAT_0081b850 != 2) && ((local_fba == 3 || (local_fba == 4)))) &&
          (DAT_0085e7bd == 2)) {
        DAT_0085e7bd = 0;
      }
    }
  }
  else {
    piVar8 = local_10;
    iVar10 = 1;
    iVar9 = 0;
    uVar11 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(DAT_0085e7bd + 0x1b0);
    FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar10, uVar11);
  }
  iVar6 = iVar6 + 0xe;
  if (local_1c == 5) {
    iVar5 = 0x106;
    iVar6 = 0x14a;
  }
  uVar11 = 0xffff;
  iVar9 = iVar5;
  iVar10 = iVar6;
  pcVar3 = FUN_004dcfa0(0x199);
  FUN_004da7c0(10, pcVar3, iVar9, iVar10, uVar11);
  iVar9 = 10;
  pcVar3 = FUN_004dcfa0(0x199);
  iVar9 = FUN_004db2f0(pcVar3, iVar9);
  FUN_004dcfd0(local_10, iVar9 + iVar5 + 2, iVar6, iVar5 + 0xa5, iVar6 + 0xd);
  if (bVar2) {
    pcVar12 = s_settingsound_0053199c;
    iVar9 = 0x1f;
    uVar11 = 0xffff;
    piVar8 = (int *)0xa;
    pcVar3 = FUN_004dcfa0(DAT_0085e7b5 + 0xdf);
    iVar9 = FUN_004d9950(local_10, pcVar3, piVar8, uVar11, iVar9, pcVar12);
    if (iVar9 != 0) {
      bVar7 = 1;
      DAT_0085e7b5 = DAT_0085e7b5 ^ 1;
    }
  }
  else {
    piVar8 = local_10;
    iVar5 = 1;
    iVar9 = 0;
    uVar11 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(DAT_0085e7b5 + 0xdf);
    FUN_004daac0(10, pcVar3, piVar8, iVar9, iVar5, uVar11);
  }
  if ((DAT_0081b850 != 2) && (bVar7)) {
    DAT_00758620 = 0x59;
    DAT_00758624 = (unsigned int)DAT_0085e7b4;
    DAT_00758628 = (unsigned int)DAT_0085e7b5;
    DAT_0075862c = (unsigned int)DAT_0085e7b6;
    DAT_00758630 = (unsigned int)DAT_0085e7b7;
    DAT_00758634 = (unsigned int)DAT_0085e7b8;
    DAT_00758638 = (unsigned int)DAT_0085e7b9;
    DAT_0075863c = (unsigned int)DAT_0085e7ba;
    DAT_00758640 = (unsigned int)DAT_0085e7bb;
    DAT_00758644 = (unsigned int)DAT_0085e7bc;
    DAT_00758648 = (unsigned int)DAT_0085e7bd;
    DAT_0075864c = (unsigned int)DAT_0085e7be;
    DAT_00758650 = (unsigned int)DAT_0085e7bf;
    DAT_00758654 = (unsigned int)_rand();
    DAT_00758658 = (unsigned int)DAT_0085e78f;
    DAT_0075865c = (unsigned int)DAT_0085e7c4;
    DAT_00758660 = (unsigned int)DAT_0085e790;
    FUN_004cb830(0, &DAT_00758620, 0x44);
  }
  return 1;
}
