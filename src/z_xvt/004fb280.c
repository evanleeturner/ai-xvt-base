/* SPDX-License-Identifier: MIT */
/* z_xvt 004fb280 */
extern int DAT_0081b850;
extern int DAT_00654c10;
extern int DAT_00654e6c;
extern int DAT_00654e68;
extern char DAT_0075633b;
extern int DAT_00654d48;
extern int DAT_00654d4c;
extern int DAT_00654d58;
extern int DAT_00654d5c;
extern int DAT_00654ce0;
extern int DAT_00654c50;
extern int DAT_00654d38;
extern int DAT_00654c40;
extern int DAT_00654bc0;
extern int DAT_00654c00;
extern int DAT_00654cac;
extern int DAT_00654cec;
extern int DAT_00654ca8;
extern int DAT_00654cdc;
extern int DAT_00654d44;
extern int DAT_00654bfc;
extern int DAT_00654bf8;
extern int DAT_00859c2c;
extern unsigned int DAT_0081bcd0;
extern unsigned int DAT_0081b854;
extern char DAT_0081b740[];

/* packed (4-byte values at addresses == 2 mod 4): accessed via memcpy only */
extern unsigned char DAT_0081bfba;
extern unsigned char DAT_0081c022;
extern unsigned char DAT_0081fb96;
extern unsigned char DAT_0081bfb6;
extern unsigned char DAT_0081e016;
extern unsigned char DAT_0081e50a;
extern unsigned char DAT_0081f5de;
extern unsigned char DAT_0081f5ea;
extern unsigned char DAT_0081f5f6;
extern unsigned char DAT_0081f602;
extern unsigned char DAT_0081f60e;
extern unsigned char DAT_0081f61a;
extern unsigned char DAT_0081f626;
extern unsigned char DAT_0081f632;
extern unsigned char DAT_0081f63e;
extern unsigned char DAT_0081fc42;
extern unsigned char DAT_007438d8;
extern unsigned char DAT_0081f7da;
extern unsigned char DAT_0081f7de;
extern unsigned char DAT_0081f7e2;
extern unsigned char DAT_0081f7e6;
extern unsigned char DAT_0081fa82;
extern unsigned char DAT_0081fa8a;
extern unsigned char DAT_0081fa9e;
extern unsigned char DAT_0081e0a6;
extern unsigned char DAT_0081e0c6;
extern unsigned char DAT_0081e266;
extern unsigned char DAT_0081e286;
extern unsigned char DAT_0081e522;
extern unsigned char DAT_0081e682;
extern unsigned char DAT_0081ed62;
extern unsigned char DAT_0081f182;
extern unsigned char DAT_0081f506;
extern unsigned char DAT_0081f64a;
extern unsigned char DAT_0081f776;

extern char s__c_s__c_s__c_s__c_d__c_s_00534320[];
extern char s__c_s__c_s_00531334[];
extern char s__c_s__c_s__s__00534310[];
extern char s__c_s__c_d_005333b4[];
extern char s_rank_d_00532234[];
extern char s_rcitlvl_d_00532210[];
extern char s_citlvl_d_00532204[];
extern char s_medlvl_d_005321f8[];
extern char s__s____s_00534308[];
extern char s__d___d__005321bc[];
extern char DAT_005321c4[];
extern char DAT_005321b8[];
extern char DAT_005321b0[];
extern char s__d__d__005321a8[];

void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004dcfa0(unsigned int param_1);
void FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
int FUN_004cdbc0(void);
int FUN_004d9010(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);
int _sprintf(char *param_1, const char *param_2, ...);
void FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
void FUN_004c9300(unsigned int param_1);
void FUN_004d0e60(char *param_1, int param_2, int param_3);
int FUN_004db2f0(char *param_1, int param_2);

#define RD32(dst, src) __builtin_memcpy(&(dst), (src), 4)

unsigned int __stdcall FUN_004fb280(void)
{
  char cVar1;
  char *pcVar2;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  int *piVar6;
  unsigned char *pbVar6;
  unsigned char *pbVar9;
  unsigned char *pbEntry;
  unsigned int uVar7;
  int iVar8;
  int iVar10;
  int iVar11;
  int iVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  int iVar16;
  unsigned int uVar17;
  unsigned int uVar18;
  unsigned int uVar19;
  int local_28;
  int local_24[4];
  char local_14[20];
  int iTmp0;
  int iTmp1;
  int iTmp2;
  unsigned int uTmp0;

  iVar11 = 0;
  FUN_004dcfd0(local_24, 0x54, 0x5a, 0x1b2, 0x6a);
  uVar19 = 0xffff;
  pcVar2 = FUN_004dcfa0(299);
  FUN_004da920(0xf, pcVar2, local_24, uVar19);
  if (DAT_0081b850 == 2) {
    local_28 = 0;
  }
  else {
    local_28 = 0;
    pbVar9 = &DAT_0081f7de;
    do {
      iVar8 = FUN_004cdbc0();
      RD32(iTmp0, pbVar9);
      if (iVar8 == iTmp0) break;
      pbVar9 = pbVar9 + 0x58;
      local_28 = local_28 + 1;
    } while (pbVar9 < &DAT_0081fa9e);
  }
  if (local_28 == 8) {
    return 0;
  }
  if (DAT_00654c10 != 0) {
    DAT_00654e6c = 0xd;
    DAT_00654e68 = 0;
    DAT_00654c10 = 0;
    if (DAT_0075633b == '\0') {
      RD32(iTmp0, &DAT_0081bfba);
      if (iTmp0 != 1) {
        RD32(iTmp0, &DAT_0081bfba);
        if (iTmp0 != 2) {
          DAT_00654e6c = 0xe;
        }
      }
    }
    if (DAT_0081b850 != 2) {
      DAT_00654e6c = DAT_00654e6c + 5;
    }
    RD32(iTmp0, &DAT_0081bfba);
    if (iTmp0 == 1) {
      DAT_00654e6c = DAT_00654e6c + 1;
    }
    RD32(iTmp0, &DAT_0081c022);
    if (iTmp0 != 0) {
      DAT_00654e6c = DAT_00654e6c + 1;
    }
    RD32(iTmp0, &DAT_0081fb96);
    pbVar9 = &DAT_0081fc42 + (unsigned int)iTmp0 * 0xe824u;
    iVar8 = 4;
    do {
      RD32(iTmp0, pbVar9);
      if (iTmp0 != 0) {
        DAT_00654e6c = DAT_00654e6c + 1;
      }
      pbVar9 = pbVar9 + 4;
      iVar8 = iVar8 + -1;
    } while (iVar8 != 0);
    DAT_00654d48 = 0;
    DAT_00654d58 = 0;
    DAT_00654ce0 = 0;
    DAT_00654c50 = 0;
    DAT_00654d38 = 0;
    DAT_00654c40 = 0;
    DAT_00654bc0 = 0;
    DAT_00654c00 = 0;
    DAT_00654cac = 0;
    pbVar9 = &DAT_0081e522;
    do {
      iVar8 = 1;
      DAT_00654cec = 0;
      pbVar6 = pbVar9;
      do {
        RD32(iTmp0, pbVar6);
        if (iTmp0 == 0) {
          RD32(iTmp1, pbVar6 + 0x420);
          if (iTmp1 == 0) goto skip_a;
        }
        DAT_00654cec = 1;
        DAT_00654cac = 1;
skip_a:
        pbVar6 = pbVar6 + 0x160;
        iVar8 = iVar8 + -1;
      } while (iVar8 != 0);
      if (DAT_00654cec != 0) {
        DAT_00654e6c = DAT_00654e6c + 1;
      }
      pbVar9 = pbVar9 + 4;
    } while (pbVar9 < &DAT_0081e682);
    if (DAT_00654cac != 0) {
      DAT_00654e6c = DAT_00654e6c + 2;
    }
    DAT_00654ca8 = 0;
    if (DAT_0081b850 != 2) {
      pbVar9 = &DAT_0081e0a6;
      do {
        RD32(iTmp0, pbVar9);
        if (iTmp0 == 0) {
          RD32(iTmp1, pbVar9 + 0x20);
          if (iTmp1 == 0) goto skip_b;
        }
        DAT_00654ca8 = 1;
        DAT_00654e6c = DAT_00654e6c + 1;
skip_b:
        pbVar9 = pbVar9 + 4;
      } while (pbVar9 < &DAT_0081e0c6);
      if (DAT_00654ca8 != 0) {
        DAT_00654e6c = DAT_00654e6c + 2;
      }
    }
    DAT_00654cdc = 0;
    if (DAT_0081b850 != 2) {
      pbVar9 = &DAT_0081e266;
      do {
        RD32(iTmp0, pbVar9);
        if (iTmp0 == 0) {
          RD32(iTmp1, pbVar9 + 0x20);
          if (iTmp1 == 0) goto skip_c;
        }
        DAT_00654cdc = 1;
        DAT_00654e6c = DAT_00654e6c + 1;
skip_c:
        pbVar9 = pbVar9 + 4;
      } while (pbVar9 < &DAT_0081e286);
      if (DAT_00654cdc != 0) {
        DAT_00654e6c = DAT_00654e6c + 2;
      }
    }
    iVar8 = 0;
    pbVar9 = &DAT_0081ed62;
    do {
      iVar10 = 0x58;
      do {
        RD32(iVar12, pbVar9);
        RD32(iVar16, pbVar9 - 0x420);
        pbVar9 = pbVar9 + 4;
        *(int *)((char *)&DAT_00654c00 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654c00 + iVar8) + (unsigned int)iVar12);
        iVar10 = iVar10 + -1;
        *(int *)((char *)&DAT_00654bc0 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654bc0 + iVar8) + (unsigned int)iVar16);
      } while (iVar10 != 0);
      iVar8 = iVar8 + 4;
    } while (iVar8 < 4);
    iVar8 = 0;
    pbVar9 = &DAT_0081f182;
    do {
      iVar10 = 0x19;
      do {
        RD32(iVar12, pbVar9);
        RD32(iVar16, pbVar9 + 0x12c);
        pbVar9 = pbVar9 + 4;
        *(int *)((char *)&DAT_00654c40 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654c40 + iVar8) + (unsigned int)iVar12);
        iVar10 = iVar10 + -1;
        *(int *)((char *)&DAT_00654d38 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654d38 + iVar8) + (unsigned int)iVar16);
      } while (iVar10 != 0);
      iVar8 = iVar8 + 4;
    } while (iVar8 < 4);
    iVar8 = 0;
    pbVar9 = &DAT_0081f506;
    do {
      iVar10 = 6;
      do {
        RD32(iVar12, pbVar9);
        RD32(iVar16, pbVar9 + 0x48);
        pbVar9 = pbVar9 + 4;
        *(int *)((char *)&DAT_00654c50 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654c50 + iVar8) + (unsigned int)iVar12);
        iVar10 = iVar10 + -1;
        *(int *)((char *)&DAT_00654ce0 + iVar8) =
            (int)((unsigned int)*(int *)((char *)&DAT_00654ce0 + iVar8) + (unsigned int)iVar16);
      } while (iVar10 != 0);
      iVar8 = iVar8 + 4;
    } while (iVar8 < 4);
    piVar6 = &DAT_00654d58;
    pbVar9 = &DAT_0081f64a;
    do {
      iVar8 = 0x19;
      do {
        RD32(iVar10, pbVar9);
        pbVar9 = pbVar9 + 4;
        iVar8 = iVar8 + -1;
        *piVar6 = (int)((unsigned int)*piVar6 + (unsigned int)iVar10);
      } while (iVar8 != 0);
      piVar6 = piVar6 + 1;
    } while (piVar6 < &DAT_00654d5c);
    piVar6 = &DAT_00654d48;
    pbVar9 = &DAT_0081f776;
    do {
      iVar8 = 6;
      do {
        RD32(iVar10, pbVar9);
        pbVar9 = pbVar9 + 4;
        iVar8 = iVar8 + -1;
        *piVar6 = (int)((unsigned int)*piVar6 + (unsigned int)iVar10);
      } while (iVar8 != 0);
      piVar6 = piVar6 + 1;
    } while (piVar6 < &DAT_00654d4c);
  }
  FUN_004dcfd0(local_24, 0x1a9, 0x6b, 0x1b2, 0x1b1);
  if (0x15 < DAT_00654e6c) {
    DAT_00654e68 = FUN_004d9010(local_24, DAT_00654e68, DAT_00654e6c, 0, 5, DAT_00859c2c, 10);
  }
  iVar8 = 0x6f;
  RD32(iTmp0, &DAT_0081bfba);
  if ((DAT_0081b850 != 2) && (iTmp0 == 1)) {
    if ((DAT_00654e68 < 1) && (-0x15 < DAT_00654e68)) {
      if (DAT_00654d44 == 0) {
        uVar19 = 0x2a5;
      }
      else {
        uVar19 = 0x163;
      }
      pcVar2 = FUN_004dcfa0(uVar19);
      uVar17 = 4;
      uVar15 = 1;
      uVar18 = DAT_00654bfc;
      pcVar3 = FUN_004dcfa0(0x142);
      uVar14 = 4;
      pcVar4 = FUN_004dcfa0(DAT_00654bf8 + 0x131);
      uVar13 = 1;
      pcVar5 = FUN_004dcfa0(0x162);
      _sprintf(DAT_0081b740, s__c_s__c_s__c_s__c_d__c_s_00534320, 4, pcVar5, uVar13, pcVar4,
               uVar14, pcVar3, uVar15, uVar18, uVar17, pcVar2);
      iVar8 = 0x7e;
      FUN_004da7c0(0xc, DAT_0081b740, 0x58, 0x6f, 0xffff);
    }
    iVar11 = 1;
  }
  iVar10 = iVar8;
  if (DAT_0075633b != '\0') goto LAB_004fb8aa;
  RD32(iTmp0, &DAT_0081bfba);
  if (iTmp0 == 1) goto LAB_004fb8aa;
  RD32(iTmp0, &DAT_0081bfba);
  if (iTmp0 == 2) goto LAB_004fb8aa;
  if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
    RD32(iTmp0, &DAT_0081bfb6);
    RD32(uVar19, &DAT_0081fa82 + (unsigned int)iTmp0 * 28u);
    RD32(iTmp1, &DAT_0081bfba);
    if (iTmp1 == 0) {
      uVar7 = (unsigned int)(uVar19 == 0);
    }
    else {
      RD32(iTmp0, &DAT_0081bfb6);
      RD32(uVar7, &DAT_0081fa82 + (unsigned int)(iTmp0 ^ 1) * 28u);
    }
    if (uVar7 == uVar19) {
      uVar19 = 0x14e;
LAB_004fb85d:
      pcVar2 = FUN_004dcfa0(uVar19);
      uVar18 = 1;
      pcVar3 = FUN_004dcfa0(0x164);
      _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 4, pcVar3, uVar18, pcVar2);
    }
    else {
      if (uVar19 != 1) {
        uVar19 = 0x166;
        goto LAB_004fb85d;
      }
      RD32(iTmp0, &DAT_0081f7da + (unsigned int)local_28 * 0x58u);
      uTmp0 = *(&DAT_007438d8 + (unsigned int)iTmp0 * 0x562u);
      RD32(uVar7, &DAT_0081fa8a + uTmp0 * 28u);
      FUN_004c9300(uVar7);
      uVar19 = 0xffffffff;
      pcVar2 = DAT_0081b740;
      do {
        pcVar3 = pcVar2;
        if (uVar19 == 0) break;
        uVar19 = uVar19 - 1;
        pcVar3 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar3;
      } while (cVar1 != '\0');
      uVar19 = ~uVar19;
      pcVar2 = pcVar3 + -uVar19;
      pcVar3 = local_14;
      for (uVar7 = uVar19 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        __builtin_memcpy(pcVar3, pcVar2, 4);
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 4;
      }
      for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
        *pcVar3 = *pcVar2;
        pcVar2 = pcVar2 + 1;
        pcVar3 = pcVar3 + 1;
      }
      pcVar2 = local_14;
      pcVar3 = FUN_004dcfa0(0x165);
      uVar18 = 1;
      pcVar4 = FUN_004dcfa0(0x164);
      _sprintf(DAT_0081b740, s__c_s__c_s__s__00534310, 4, pcVar4, uVar18, pcVar3, pcVar2);
    }
    iVar10 = iVar8 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar8, 0xffff);
  }
  iVar11 = iVar11 + 1;
LAB_004fb8aa:
  iVar8 = iVar10;
  if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
    RD32(uVar18, &DAT_0081f7e6 + (unsigned int)local_28 * 0x58u);
    uVar13 = 1;
    pcVar2 = FUN_004dcfa0(0x140);
    _sprintf(DAT_0081b740, s__c_s__c_d_005333b4, 4, pcVar2, uVar13, uVar18);
    iVar8 = iVar10 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar10, 0xffff);
  }
  iVar12 = iVar11 + 1;
  iVar10 = iVar8;
  RD32(iTmp0, &DAT_0081c022);
  if (iTmp0 != 0) {
    if ((DAT_00654e68 <= iVar12) && (iVar12 - DAT_00654e68 < 0x15)) {
      RD32(iTmp0, &DAT_0081e016);
      _sprintf(DAT_0081b740, s_rank_d_00532234, iTmp0);
      FUN_004d0e60(DAT_0081b740, 0x58, iVar8 + 1);
      RD32(iTmp0, &DAT_0081e016);
      pcVar2 = FUN_004dcfa0((unsigned int)iTmp0 + 0x6eu);
      uVar18 = 6;
      RD32(iTmp1, &DAT_0081c022);
      pcVar3 = FUN_004dcfa0((unsigned int)iTmp1 + 0x16au);
      _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 4, pcVar3, uVar18, pcVar2);
      iVar10 = iVar8 + 0xf;
      FUN_004da7c0(0xc, DAT_0081b740, 0x91, iVar8, 0xffff);
    }
    iVar12 = iVar11 + 2;
  }
  DAT_00654cec = 0;
  iVar11 = 0;
  do {
    iVar8 = iVar10;
    RD32(iTmp2, &DAT_0081fb96);
    pbEntry = &DAT_0081fc42 + ((unsigned int)iTmp2 * 0xe824u + (unsigned int)iVar11 * 4u);
    RD32(iTmp0, pbEntry);
    if (iTmp0 != 0) {
      if ((DAT_00654e68 <= iVar12) && (iVar12 - DAT_00654e68 < 0x15)) {
        if (DAT_00654cec == 0) {
          iVar16 = 0x58;
          uVar19 = DAT_0081bcd0;
          pcVar2 = FUN_004dcfa0(0x16c);
          FUN_004da7c0(0xc, pcVar2, iVar16, iVar8, uVar19);
          DAT_00654cec = 1;
        }
        iVar8 = 0xc;
        pcVar2 = FUN_004dcfa0(0x16c);
        iVar16 = FUN_004db2f0(pcVar2, iVar8);
        if (iVar11 == 2) {
          RD32(iTmp1, &DAT_0081fb96);
          if (iTmp1 == 0) {
            pcVar2 = s_rcitlvl_d_00532210;
            RD32(uVar18, &DAT_0081fc42 + 8);
          }
          else {
            RD32(iTmp2, &DAT_0081fb96);
            RD32(uVar18, &DAT_0081fc42 + ((unsigned int)iTmp2 * 0xe824u + 8u));
            pcVar2 = s_citlvl_d_00532204;
          }
        }
        else {
          RD32(iTmp2, &DAT_0081fb96);
          RD32(uVar18, &DAT_0081fc42 + ((unsigned int)iTmp2 * 0xe824u + (unsigned int)iVar11 * 4u));
          pcVar2 = s_medlvl_d_005321f8;
        }
        _sprintf(DAT_0081b740, pcVar2, uVar18);
        FUN_004d0e60(DAT_0081b740, iVar16 + 0x62, iVar10);
        RD32(iTmp2, &DAT_0081fb96);
        if (iVar11 == 2) {
          RD32(iTmp1, &DAT_0081fc42 + ((unsigned int)iTmp2 * 0xe824u + 8u));
          uVar19 = (unsigned int)iTmp1 + 0x285u;
        }
        else {
          RD32(iTmp1, &DAT_0081fc42 + ((unsigned int)iTmp2 * 0xe824u + (unsigned int)iVar11 * 4u));
          uVar19 = (unsigned int)iTmp1 + 0x170u;
        }
        pcVar2 = FUN_004dcfa0(uVar19);
        pcVar3 = FUN_004dcfa0(iVar11 + 0x16d);
        _sprintf(DAT_0081b740, s__s____s_00534308, pcVar3, pcVar2);
        iVar8 = iVar10 + 0xf;
        FUN_004da7c0(0xc, DAT_0081b740, iVar16 + 0x8a, iVar10, 0xffff);
      }
      iVar12 = iVar12 + 1;
    }
    iVar11 = iVar11 + 1;
    iVar10 = iVar8;
  } while (iVar11 < 4);
  if ((DAT_00654e68 <= iVar12) && (iVar12 - DAT_00654e68 < 0x15)) {
    iVar8 = iVar8 + 0xf;
  }
  iVar11 = iVar8;
  if ((DAT_00654e68 <= iVar12 + 1) && ((iVar12 + 1) - DAT_00654e68 < 0x15)) {
    iVar10 = 0x58;
    iVar11 = iVar8 + 0xf;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x151);
    FUN_004da7c0(0xc, pcVar2, iVar10, iVar8, uVar19);
  }
  iVar8 = iVar11;
  if ((DAT_00654e68 <= iVar12 + 2) && ((iVar12 + 2) - DAT_00654e68 < 0x15)) {
    iVar10 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x11c);
    FUN_004da7c0(0xc, pcVar2, iVar10, iVar8, uVar19);
    RD32(iTmp0, &DAT_0081e50a);
    _sprintf(DAT_0081b740, s__d___d__005321bc, iTmp0, DAT_00654bc0);
    iVar8 = iVar11 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar11, 0xffff);
  }
  iVar11 = iVar12 + 3;
  if (DAT_0081b850 != 2) {
    iVar10 = iVar8;
    if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
      iVar10 = 0x58;
      iVar11 = iVar8;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x152);
      FUN_004da7c0(0xc, pcVar2, iVar10, iVar11, uVar19);
      RD32(iTmp0, &DAT_0081bfba);
      if (iTmp0 == 0) {
        _sprintf(DAT_0081b740, DAT_005321c4);
      }
      else {
        _sprintf(DAT_0081b740, s__d___d__005321bc, DAT_00654c40, DAT_00654d38);
      }
      iVar10 = iVar8 + 0xf;
      FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
    }
    iVar8 = iVar10;
    if ((DAT_00654e68 <= iVar12 + 4) && ((iVar12 + 4) - DAT_00654e68 < 0x15)) {
      iVar8 = 0x58;
      iVar11 = iVar10;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x153);
      FUN_004da7c0(0xc, pcVar2, iVar8, iVar11, uVar19);
      _sprintf(DAT_0081b740, s__d___d__005321bc, DAT_00654c50, DAT_00654ce0);
      iVar8 = iVar10 + 0xf;
      FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar10, 0xffff);
    }
    iVar11 = iVar12 + 5;
  }
  iVar10 = iVar8;
  if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x154);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar10, uVar19);
    _sprintf(DAT_0081b740, DAT_005321b8, DAT_00654c00);
    iVar10 = iVar8 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
  }
  iVar8 = iVar10;
  if ((DAT_00654e68 <= iVar11 + 1) && ((iVar11 + 1) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x155);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar8, uVar19);
    RD32(iTmp0, &DAT_0081f5de);
    _sprintf(DAT_0081b740, DAT_005321b8, iTmp0);
    iVar8 = iVar10 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar10, 0xffff);
  }
  iVar10 = iVar8;
  if ((DAT_00654e68 <= iVar11 + 2) && ((iVar11 + 2) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x156);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar10, uVar19);
    uVar19 = 0;
    RD32(uTmp0, &DAT_0081f5f6);
    if (uTmp0 != 0) {
      RD32(uVar7, &DAT_0081f5ea);
      RD32(uTmp0, &DAT_0081f5f6);
      uVar19 = (uVar7 * 100u) / uTmp0;
    }
    _sprintf(DAT_0081b740, DAT_005321b0, uVar19);
    iVar10 = iVar8 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
  }
  iVar8 = iVar10;
  if ((DAT_00654e68 <= iVar11 + 3) && ((iVar11 + 3) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x157);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar8, uVar19);
    uVar19 = 0;
    RD32(uTmp0, &DAT_0081f60e);
    if (uTmp0 != 0) {
      RD32(uVar7, &DAT_0081f602);
      RD32(uTmp0, &DAT_0081f60e);
      uVar19 = (uVar7 * 100u) / uTmp0;
    }
    _sprintf(DAT_0081b740, DAT_005321b0, uVar19);
    iVar8 = iVar10 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar10, 0xffff);
  }
  iVar10 = iVar11 + 4;
  if (DAT_00654ca8 != 0) {
    iVar12 = iVar8;
    if ((DAT_00654e68 <= iVar10) && (iVar10 - DAT_00654e68 < 0x15)) {
      iVar12 = iVar8 + 0xf;
    }
    iVar8 = iVar12;
    if ((DAT_00654e68 <= iVar11 + 5) && ((iVar11 + 5) - DAT_00654e68 < 0x15)) {
      iVar10 = 0x58;
      iVar8 = iVar12 + 0xf;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x158);
      FUN_004da7c0(0xc, pcVar2, iVar10, iVar12, uVar19);
    }
    local_28 = 0x18;
    iVar10 = iVar11 + 6;
    do {
      pbVar9 = &DAT_0081f7e2;
      pbVar6 = &DAT_0081e0a6;
      iVar11 = iVar8;
      do {
        iVar8 = iVar11;
        RD32(iTmp0, pbVar9);
        if (iTmp0 == local_28) {
          RD32(iTmp1, pbVar6);
          if (iTmp1 == 0) {
            RD32(iTmp1, pbVar6 + 0x20);
            if (iTmp1 == 0) goto skip_d;
          }
          if ((DAT_00654e68 <= iVar10) && (iVar10 - DAT_00654e68 < 0x15)) {
            iVar8 = (int)(pbVar9 - 0x16);
            uVar18 = 1;
            pcVar2 = FUN_004dcfa0(local_28 + 0x8e);
            _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar2, uVar18, (char *)(pbVar9 - 0x16));
            FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar11, DAT_0081bcd0);
            RD32(iTmp1, pbVar6);
            RD32(iTmp2, pbVar6 + 0x20);
            _sprintf(DAT_0081b740, s__d__d__005321a8, iTmp1, iTmp2);
            iVar8 = iVar11 + 0xf;
            FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar11, 0xffff);
          }
          iVar10 = iVar10 + 1;
        }
skip_d:
        pbVar9 = pbVar9 + 0x58;
        pbVar6 = pbVar6 + 4;
        iVar11 = iVar8;
      } while (pbVar6 < &DAT_0081e0c6);
      local_28 = local_28 + -1;
    } while (-1 < local_28);
  }
  if (DAT_00654cac != 0) {
    iVar11 = iVar8;
    if ((DAT_00654e68 <= iVar10) && (iVar10 - DAT_00654e68 < 0x15)) {
      iVar11 = iVar8 + 0xf;
    }
    iVar8 = iVar11;
    if ((DAT_00654e68 <= iVar10 + 1) && ((iVar10 + 1) - DAT_00654e68 < 0x15)) {
      iVar12 = 0x58;
      iVar8 = iVar11 + 0xf;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x159);
      FUN_004da7c0(0xc, pcVar2, iVar12, iVar11, uVar19);
    }
    iVar10 = iVar10 + 2;
  }
  iVar11 = 0;
  pbVar9 = &DAT_0081e522;
  do {
    iVar12 = 1;
    DAT_00654cec = 0;
    pbVar6 = pbVar9;
    do {
      RD32(iTmp0, pbVar6);
      if (iTmp0 == 0) {
        RD32(iTmp1, pbVar6 + 0x420);
        if (iTmp1 == 0) goto skip_e;
      }
      DAT_00654cec = 1;
skip_e:
      pbVar6 = pbVar6 + 0x160;
      iVar12 = iVar12 + -1;
    } while (iVar12 != 0);
    iVar12 = iVar8;
    if (DAT_00654cec != 0) {
      if ((DAT_00654e68 <= iVar10) && (iVar10 - DAT_00654e68 < 0x15)) {
        iVar16 = 0x58;
        uVar19 = DAT_0081b854;
        pcVar2 = FUN_004dcfa0(iVar11 + 0x15);
        FUN_004da7c0(0xc, pcVar2, iVar16, iVar12, uVar19);
        RD32(iTmp0, pbVar9);
        RD32(iTmp1, pbVar9 + 0x420);
        _sprintf(DAT_0081b740, s__d___d__005321bc, iTmp0, iTmp1);
        iVar12 = iVar8 + 0xf;
        FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
      }
      iVar10 = iVar10 + 1;
    }
    pbVar9 = pbVar9 + 4;
    iVar11 = iVar11 + 1;
    iVar8 = iVar12;
  } while (pbVar9 < &DAT_0081e682);
  if ((DAT_00654e68 <= iVar10) && (iVar10 - DAT_00654e68 < 0x15)) {
    iVar12 = iVar12 + 0xf;
  }
  iVar11 = iVar12;
  if ((DAT_00654e68 <= iVar10 + 1) && ((iVar10 + 1) - DAT_00654e68 < 0x15)) {
    iVar8 = 0x58;
    iVar11 = iVar12 + 0xf;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x15b);
    FUN_004da7c0(0xc, pcVar2, iVar8, iVar12, uVar19);
  }
  iVar8 = iVar11;
  if ((DAT_00654e68 <= iVar10 + 2) && ((iVar10 + 2) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x15c);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar8, uVar19);
    RD32(iTmp0, &DAT_0081f61a);
    _sprintf(DAT_0081b740, DAT_005321b8, iTmp0);
    iVar8 = iVar11 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar11, 0xffff);
  }
  iVar11 = iVar10 + 3;
  if (DAT_0081b850 != 2) {
    iVar12 = iVar8;
    if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
      iVar12 = 0x58;
      iVar11 = iVar8;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x15d);
      FUN_004da7c0(0xc, pcVar2, iVar12, iVar11, uVar19);
      _sprintf(DAT_0081b740, DAT_005321b8, DAT_00654d58);
      iVar12 = iVar8 + 0xf;
      FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
    }
    iVar8 = iVar12;
    if ((DAT_00654e68 <= iVar10 + 4) && ((iVar10 + 4) - DAT_00654e68 < 0x15)) {
      iVar8 = 0x58;
      iVar11 = iVar12;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x15e);
      FUN_004da7c0(0xc, pcVar2, iVar8, iVar11, uVar19);
      _sprintf(DAT_0081b740, DAT_005321b8, DAT_00654d48);
      iVar8 = iVar12 + 0xf;
      FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar12, 0xffff);
    }
    iVar11 = iVar10 + 5;
  }
  iVar10 = iVar8;
  if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x15f);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar10, uVar19);
    RD32(iTmp0, &DAT_0081f632);
    _sprintf(DAT_0081b740, DAT_005321b8, iTmp0);
    iVar10 = iVar8 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
  }
  iVar8 = iVar10;
  if ((DAT_00654e68 <= iVar11 + 1) && ((iVar11 + 1) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x160);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar8, uVar19);
    RD32(iTmp0, &DAT_0081f63e);
    _sprintf(DAT_0081b740, DAT_005321b8, iTmp0);
    iVar8 = iVar10 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar10, 0xffff);
  }
  iVar10 = iVar8;
  if ((DAT_00654e68 <= iVar11 + 2) && ((iVar11 + 2) - DAT_00654e68 < 0x15)) {
    iVar12 = 0x58;
    uVar19 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x161);
    FUN_004da7c0(0xc, pcVar2, iVar12, iVar10, uVar19);
    RD32(iTmp0, &DAT_0081f626);
    _sprintf(DAT_0081b740, DAT_005321b8, iTmp0);
    iVar10 = iVar8 + 0xf;
    FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar8, 0xffff);
  }
  if (DAT_00654cdc != 0) {
    if ((DAT_00654e68 <= iVar11 + 3) && ((iVar11 + 3) - DAT_00654e68 < 0x15)) {
      iVar10 = iVar10 + 0xf;
    }
    iVar8 = iVar10;
    if ((DAT_00654e68 <= iVar11 + 4) && ((iVar11 + 4) - DAT_00654e68 < 0x15)) {
      iVar12 = 0x58;
      iVar8 = iVar10 + 0xf;
      uVar19 = DAT_0081bcd0;
      pcVar2 = FUN_004dcfa0(0x177);
      FUN_004da7c0(0xc, pcVar2, iVar12, iVar10, uVar19);
    }
    local_28 = 0x18;
    iVar11 = iVar11 + 5;
    do {
      pbVar6 = &DAT_0081f7e2;
      pbVar9 = &DAT_0081e266;
      iVar10 = iVar8;
      do {
        iVar8 = iVar10;
        RD32(iTmp0, pbVar6);
        if (iTmp0 == local_28) {
          RD32(iTmp1, pbVar9);
          if (iTmp1 == 0) {
            RD32(iTmp1, pbVar9 + 0x20);
            if (iTmp1 == 0) goto skip_f;
          }
          if ((DAT_00654e68 <= iVar11) && (iVar11 - DAT_00654e68 < 0x15)) {
            iVar8 = (int)(pbVar6 - 0x16);
            uVar18 = 1;
            pcVar2 = FUN_004dcfa0(local_28 + 0x8e);
            _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar2, uVar18, (char *)(pbVar6 - 0x16));
            FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar10, DAT_0081bcd0);
            RD32(iTmp1, pbVar9);
            RD32(iTmp2, pbVar9 + 0x20);
            _sprintf(DAT_0081b740, s__d__d__005321a8, iTmp1, iTmp2);
            iVar8 = iVar10 + 0xf;
            FUN_004da7c0(0xc, DAT_0081b740, 0x10c, iVar10, 0xffff);
          }
          iVar11 = iVar11 + 1;
        }
skip_f:
        pbVar6 = pbVar6 + 0x58;
        pbVar9 = pbVar9 + 4;
        iVar10 = iVar8;
      } while (pbVar9 < &DAT_0081e286);
      local_28 = local_28 + -1;
    } while (-1 < local_28);
  }
  return 1;
}
