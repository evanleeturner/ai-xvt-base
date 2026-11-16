/* SPDX-License-Identifier: MIT */
/* z_xvt 004ebad0 */
extern int DAT_0081b850;
extern char DAT_0085e7bb;
extern int DAT_00757c9c;
extern int DAT_00757cb4;
extern int DAT_00757c80;
extern int DAT_00757c88;
extern int DAT_00757c8c;
extern int DAT_00757cb8;
extern int DAT_00757c90;
extern int DAT_00757c84;
extern int DAT_00757cb0;
extern int DAT_00757ca4;
extern unsigned int DAT_00757c98;
extern unsigned int DAT_00757c94;
extern int DAT_00757cac;
extern int DAT_00757ca0;
extern int DAT_0064ff9c;
extern unsigned char DAT_007438d3[];
extern unsigned char DAT_007438e1[];
extern unsigned char DAT_00743dcc[];
extern unsigned char DAT_00743dd6[];
extern unsigned char DAT_00743dc2[];
extern unsigned char DAT_007438d2[];
extern unsigned int DAT_005333fc[];
extern unsigned int DAT_0053340c[];
extern int DAT_00533428[];
/* packed (unaligned) 4-byte globals */
extern unsigned char DAT_0081c012[4];
extern unsigned char DAT_0081bfba[4];
extern unsigned char DAT_0081d326[4];
extern unsigned char DAT_0081fb96[4];
extern unsigned int DAT_00758620;
extern unsigned int DAT_00758624;
extern unsigned int DAT_00758628;
extern unsigned int DAT_0075862c;
extern unsigned int DAT_00758630;
extern unsigned int DAT_00758634;
extern unsigned int DAT_00758638;
extern unsigned int DAT_0075863c;
extern unsigned int DAT_00758640;

extern char s_jewelsound_00531990[];
extern char s_craft5u_00533668[];
extern char s_craft5d_00533670[];
extern char s_craft4u_00533658[];
extern char s_craft4d_00533660[];
extern char s_craft3u_00533648[];
extern char s_craft3d_00533650[];
extern char s_craft2u_00533638[];
extern char s_craft2d_00533640[];
extern char s_craft1u_00533628[];
extern char s_craft1d_00533630[];
extern char s_background_00532a4c[];
extern char s_frontres_craftsi_bmp_005335c0[];
extern char s_frontres_craftmi_bmp_005335a8[];
extern char s_frontres_craftsr_bmp_005335d8[];
extern char s_frontres_craftmr_bmp_00533590[];
extern char s_frame_00531358[];
extern char s_allactive_00531ebc[];
extern char s_chatbox_00532a84[];
extern char s_regoverlay_00531e94[];

int FUN_004cc810(void);
unsigned int FUN_004ecce0(int a);
void FUN_004ddc70(int *a, int *b);
void FUN_004dcfd0(int *a, int b, int c, int d, int e);
int FUN_004dd7d0(int *a, int b, int c);
unsigned char FUN_004db530(void);
unsigned char FUN_004db550(void);
unsigned char FUN_004db570(void);
unsigned char FUN_004db590(void);
void FUN_004dd030(int *a, int b, int c);
void FUN_004d9ea0(int *a);
char *FUN_004dcfa0(int a);
int FUN_004d9a80(int *a, char *b, char *c, char *d, int e, int f, int g, char *h);
void FUN_004526c0(char *a);
void FUN_004d0430(char *a);
void FUN_004d0290(char *a, char *b);
void FUN_004d52b0(void);
void FUN_004d1750(char *a, int b, int c);
void FUN_004d0e60(char *a, int b, int c);
void FUN_004d0580(char *a, int b, int c);
void FUN_004d5350(int a);
void FUN_004cb830(int a, unsigned int *b, int c);

unsigned int __stdcall FUN_004ebad0(void)
{
  int iVar1;
  unsigned char bVar2;
  unsigned char bVar3;
  int bVar4;
  int bVar5;
  int bVar6;
  unsigned char uVar7;
  int iVar8;
  unsigned int uVar9;
  char *pcVar10;
  int *piVar11;
  unsigned int uVar12;
  int iVar13;
  char *pcVar14;
  int aiStack_34[4];
  int aiStack_24[8];
  int iStack_4;
  int iC012;
  int iBFBA;
  int iD326;
  int iFB96;

  if (DAT_0081b850 == 2) {
LAB_004ebaf5:
    bVar4 = 1;
  }
  else {
    iVar8 = FUN_004cc810();
    bVar4 = 0;
    if (iVar8 != 0) goto LAB_004ebaf5;
  }
  bVar6 = 0;
  if (DAT_0081b850 == 2) {
LAB_004ebb28:
    bVar5 = 1;
  }
  else {
    bVar5 = 0;
    if (DAT_0085e7bb == '\0') {
      bVar5 = 0;
    }
    else if ((DAT_0085e7bb == '\x01') || ((DAT_0085e7bb == '\x02' && (bVar4)))) goto LAB_004ebb28;
  }
  if (((DAT_00757c9c < 2) && (DAT_00757cb4 < 2)) || (!bVar5)) {
    aiStack_24[0] = 0;
  }
  else {
    aiStack_24[0] = 1;
  }
  if ((DAT_00757c80 < 2) || (!bVar5)) {
    aiStack_24[2] = 0;
  }
  else {
    aiStack_24[2] = 1;
  }
  aiStack_24[1] = aiStack_24[0];
  if ((DAT_00757c88 < 2) || (!bVar5)) {
switchD_004ebba4_caseD_1:
    aiStack_24[3] = 0;
  }
  else {
    aiStack_24[3] = 1;
    uVar9 = FUN_004ecce0(-1);
    switch (uVar9) {
    case 1:
    case 2:
    case 3:
    case 5:
    case 0xe:
      goto switchD_004ebba4_caseD_1;
    }
  }
  if ((DAT_00757c8c < 2) || (!bVar5)) {
    aiStack_24[4] = 0;
  }
  else {
    aiStack_24[4] = 1;
  }
  aiStack_24[5] = 0;
  aiStack_24[6] = 0;
  FUN_004ddc70(&iStack_4, aiStack_24 + 7);
  FUN_004dcfd0(aiStack_34, 0x16, 0x72, 0x2a, 0x8a);
  piVar11 = aiStack_24;
  do {
    if (((*piVar11 != 0) && (iVar8 = FUN_004dd7d0(aiStack_34, iStack_4, aiStack_24[7]), iVar8 != 0))
        && ((uVar7 = FUN_004db530(), uVar7 != 0 ||
             (((uVar7 = FUN_004db550(), uVar7 != 0 ||
                (uVar7 = FUN_004db570(), uVar7 != 0)) ||
               (uVar7 = FUN_004db590(), uVar7 != 0)))))) {
      *piVar11 = 2;
    }
    piVar11 = piVar11 + 1;
    FUN_004dd030(aiStack_34, 0, 0x1c);
  } while (piVar11 < aiStack_24 + 5);
  FUN_004d9ea0(aiStack_24);
  FUN_004dcfd0(aiStack_34, 0x16, 0xe2, 0x2a, 0xfa);
  if (aiStack_24[4] != 0) {
    pcVar14 = s_jewelsound_00531990;
    iVar13 = 0xf;
    uVar12 = 0;
    iVar8 = 0xc;
    pcVar10 = FUN_004dcfa0(0x102);
    iVar8 = FUN_004d9a80(aiStack_34, s_craft5u_00533668, s_craft5d_00533670, pcVar10, iVar8, uVar12,
                         iVar13, pcVar14);
    if (iVar8 != 0) {
      uVar7 = FUN_004db570();
      if (uVar7 == 0) {
        if (DAT_00757cb8 == 0) {
          DAT_00757cb8 = (int)((unsigned int)DAT_00757c8c - 1u);
        }
        else {
          DAT_00757cb8 = (int)((unsigned int)DAT_00757cb8 - 1u);
        }
      }
      else {
        DAT_00757cb8 = (int)((unsigned int)DAT_00757cb8 + 1u);
        if (DAT_00757c8c <= DAT_00757cb8) {
          DAT_00757cb8 = 0;
        }
      }
      bVar6 = 1;
    }
  }
  FUN_004dd030(aiStack_34, 0, -0x1c);
  if (aiStack_24[3] != 0) {
    pcVar14 = s_jewelsound_00531990;
    iVar13 = 0xe;
    uVar12 = 0;
    iVar8 = 0xc;
    pcVar10 = FUN_004dcfa0(0x103);
    iVar8 = FUN_004d9a80(aiStack_34, s_craft4u_00533658, s_craft4d_00533660, pcVar10, iVar8, uVar12,
                         iVar13, pcVar14);
    if (iVar8 != 0) {
      uVar7 = FUN_004db570();
      if (uVar7 == 0) {
        if (DAT_00757c90 == 0) {
          DAT_00757c90 = (int)((unsigned int)DAT_00757c88 - 1u);
        }
        else {
          DAT_00757c90 = (int)((unsigned int)DAT_00757c90 - 1u);
        }
      }
      else {
        DAT_00757c90 = (int)((unsigned int)DAT_00757c90 + 1u);
        if (DAT_00757c88 <= DAT_00757c90) {
          DAT_00757c90 = 0;
        }
      }
      bVar6 = 1;
    }
  }
  FUN_004dd030(aiStack_34, 0, -0x1c);
  if (aiStack_24[2] != 0) {
    pcVar14 = s_jewelsound_00531990;
    iVar13 = 0xd;
    uVar12 = 0;
    iVar8 = 0xc;
    pcVar10 = FUN_004dcfa0(0x101);
    iVar8 = FUN_004d9a80(aiStack_34, s_craft3u_00533648, s_craft3d_00533650, pcVar10, iVar8, uVar12,
                         iVar13, pcVar14);
    if (iVar8 != 0) {
      uVar7 = FUN_004db570();
      if (uVar7 == 0) {
        if (DAT_00757c84 == 0) {
          DAT_00757c84 = (int)((unsigned int)DAT_00757c80 - 1u);
        }
        else {
          DAT_00757c84 = (int)((unsigned int)DAT_00757c84 - 1u);
        }
      }
      else {
        DAT_00757c84 = (int)((unsigned int)DAT_00757c84 + 1u);
        if (DAT_00757c80 <= DAT_00757c84) {
          DAT_00757c84 = 0;
        }
      }
      bVar6 = 1;
    }
  }
  FUN_004dd030(aiStack_34, 0, -0x1c);
  if (aiStack_24[1] != 0) {
    pcVar14 = s_jewelsound_00531990;
    iVar13 = 0xc;
    uVar12 = 0;
    iVar8 = 0xc;
    pcVar10 = FUN_004dcfa0(0x100);
    iVar8 = FUN_004d9a80(aiStack_34, s_craft2u_00533638, s_craft2d_00533640, pcVar10, iVar8, uVar12,
                         iVar13, pcVar14);
    if (iVar8 != 0) {
      bVar6 = 1;
      iVar8 = DAT_00757cb0;
      iVar13 = DAT_00757cb4;
      do {
        bVar4 = 0;
        if (iVar13 == 0) {
          DAT_00757ca4 = (int)((unsigned int)DAT_00757ca4 - 1u);
          if (DAT_00757ca4 < 0) {
            DAT_00757ca4 = (int)((unsigned int)DAT_00757c9c - 1u);
          }
          if (DAT_00757ca4 == 0) {
            bVar2 = DAT_007438d3[iVar8 * 0x562];
            bVar3 = DAT_007438e1[iVar8 * 0x562];
          }
          else {
            iVar1 = iVar8 * 0x562 + DAT_00757ca4;
            bVar2 = DAT_00743dcc[iVar1];
            bVar3 = DAT_00743dd6[iVar1];
          }
          DAT_00757c98 = (unsigned int)bVar2;
          DAT_00757c94 = (unsigned int)bVar3;
        }
        else {
          DAT_00757cac = (int)((unsigned int)DAT_00757cac - 1u);
          if (DAT_00757cac < 0) {
            DAT_00757cac = (int)((unsigned int)iVar13 - 1u);
          }
          if (DAT_00757cac != 0) {
            iVar1 = iVar8 * 0x562;
            bVar2 = DAT_00743dc2[iVar1];
            if (bVar2 != 0) {
              if (bVar2 < 3) {
                if (DAT_005333fc[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar1]) {
                  DAT_00757cac = (int)((unsigned int)DAT_00757cac - 1u);
                }
              }
              else if ((bVar2 == 3) &&
                       (DAT_0053340c[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar1])) {
                DAT_00757cac = (int)((unsigned int)DAT_00757cac - 1u);
              }
            }
          }
        }
        __builtin_memcpy(&iC012, DAT_0081c012, 4);
        __builtin_memcpy(&iBFBA, DAT_0081bfba, 4);
        __builtin_memcpy(&iD326, DAT_0081d326, 4);
        if (((iC012 == 1) && ((iBFBA == 1 || (iBFBA == 2)))) &&
            (iD326 != 0)) {
          uVar9 = FUN_004ecce0(-1);
          iVar8 = DAT_00757cb0;
          iVar13 = DAT_00757cb4;
          __builtin_memcpy(&iFB96, DAT_0081fb96, 4);
          if (((int)uVar9 < 1) || ((3 < (int)uVar9 && (uVar9 != 0xe)))) {
            if (iFB96 == 0) goto LAB_004ec008;
          }
          else if (iFB96 == 1) {
LAB_004ec008:
            bVar4 = 1;
          }
        }
      } while (bVar4);
      uVar9 = FUN_004ecce0(-1);
      FUN_004526c0((char *)(DAT_00757ca0 + DAT_00533428[uVar9] * 0x44));
      __builtin_memcpy(&iBFBA, DAT_0081bfba, 4);
      if ((iBFBA == 1) || (iBFBA == 2)) {
        uVar9 = FUN_004ecce0(-1);
        if (((int)uVar9 < 1) || ((3 < (int)uVar9 && (uVar9 != 0xe)))) {
          if (DAT_0064ff9c == 0) {
            FUN_004d0430(s_background_00532a4c);
            DAT_0064ff9c = 1;
            if (DAT_0081b850 == 2) {
              pcVar10 = s_frontres_craftsi_bmp_005335c0;
            }
            else {
              pcVar10 = s_frontres_craftmi_bmp_005335a8;
            }
            goto LAB_004ec0f2;
          }
        }
        else if (DAT_0064ff9c == 1) {
          FUN_004d0430(s_background_00532a4c);
          DAT_0064ff9c = 0;
          if (DAT_0081b850 == 2) {
            pcVar10 = s_frontres_craftsr_bmp_005335d8;
          }
          else {
            pcVar10 = s_frontres_craftmr_bmp_00533590;
          }
LAB_004ec0f2:
          FUN_004d0290(pcVar10, s_background_00532a4c);
        }
        FUN_004d52b0();
        FUN_004d1750(s_background_00532a4c, 0, 0);
        FUN_004d0e60(s_frame_00531358, 0, 0);
        FUN_004d0e60(s_allactive_00531ebc, 0, 0);
        if (DAT_0081b850 != 2) {
          FUN_004d0580(s_chatbox_00532a84, 0, 0);
        }
        FUN_004d0580(s_regoverlay_00531e94, 0, 0);
        FUN_004d5350(1);
      }
    }
  }
  FUN_004dd030(aiStack_34, 0, -0x1c);
  if (aiStack_24[0] == 0) goto LAB_004ec4a2;
  pcVar14 = s_jewelsound_00531990;
  iVar13 = 0xb;
  uVar12 = 0;
  iVar8 = 0xc;
  pcVar10 = FUN_004dcfa0(0xff);
  iVar8 = FUN_004d9a80(aiStack_34, s_craft1u_00533628, s_craft1d_00533630, pcVar10, iVar8, uVar12,
                       iVar13, pcVar14);
  if (iVar8 == 0) goto LAB_004ec4a2;
  bVar6 = 1;
  iVar8 = DAT_00757cb0;
  do {
    bVar4 = 0;
    if (DAT_00757cb4 == 0) {
      DAT_00757ca4 = (int)((unsigned int)DAT_00757ca4 + 1u);
      if (DAT_00757c9c <= DAT_00757ca4) {
        DAT_00757ca4 = 0;
      }
      if (DAT_00757ca4 == 0) {
        bVar2 = DAT_007438d3[iVar8 * 0x562];
        bVar3 = DAT_007438e1[iVar8 * 0x562];
      }
      else {
        iVar13 = iVar8 * 0x562 + DAT_00757ca4;
        bVar2 = DAT_00743dcc[iVar13];
        bVar3 = DAT_00743dd6[iVar13];
      }
      DAT_00757c98 = (unsigned int)bVar2;
      DAT_00757c94 = (unsigned int)bVar3;
    }
    else {
      DAT_00757cac = (int)((unsigned int)DAT_00757cac + 1u);
      if (DAT_00757cb4 <= DAT_00757cac) {
        DAT_00757cac = 0;
      }
      if (DAT_00757cac != 0) {
        iVar13 = iVar8 * 0x562;
        bVar2 = DAT_00743dc2[iVar13];
        if (bVar2 != 0) {
          if (bVar2 < 3) {
            if (DAT_005333fc[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar13])
              goto LAB_004ec243;
          }
          else if ((bVar2 == 3) &&
                   (DAT_0053340c[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar13])) {
LAB_004ec243:
            DAT_00757cac = (int)((unsigned int)DAT_00757cac + 1u);
          }
        }
      }
      if (DAT_00757cb4 <= DAT_00757cac) {
        DAT_00757cac = 0;
      }
    }
    __builtin_memcpy(&iC012, DAT_0081c012, 4);
    __builtin_memcpy(&iBFBA, DAT_0081bfba, 4);
    __builtin_memcpy(&iD326, DAT_0081d326, 4);
    if ((iC012 == 1) && (((iBFBA == 1 || (iBFBA == 2)) && (iD326 != 0)))) {
      uVar9 = FUN_004ecce0(-1);
      iVar8 = DAT_00757cb0;
      __builtin_memcpy(&iFB96, DAT_0081fb96, 4);
      if (((int)uVar9 < 1) || ((3 < (int)uVar9 && (uVar9 != 0xe)))) {
        if (iFB96 == 0) goto LAB_004ec31c;
      }
      else if (iFB96 == 1) {
LAB_004ec31c:
        bVar4 = 1;
      }
    }
  } while (bVar4);
  uVar9 = FUN_004ecce0(-1);
  FUN_004526c0((char *)(DAT_00757ca0 + DAT_00533428[uVar9] * 0x44));
  __builtin_memcpy(&iBFBA, DAT_0081bfba, 4);
  if ((iBFBA != 1) && (iBFBA != 2)) goto LAB_004ec4a2;
  uVar9 = FUN_004ecce0(-1);
  if (((int)uVar9 < 1) || ((3 < (int)uVar9 && (uVar9 != 0xe)))) {
    if (DAT_0064ff9c == 0) {
      FUN_004d0430(s_background_00532a4c);
      DAT_0064ff9c = 1;
      if (DAT_0081b850 == 2) {
        pcVar10 = s_frontres_craftsi_bmp_005335c0;
      }
      else {
        pcVar10 = s_frontres_craftmi_bmp_005335a8;
      }
      goto LAB_004ec41a;
    }
  }
  else if (DAT_0064ff9c == 1) {
    FUN_004d0430(s_background_00532a4c);
    DAT_0064ff9c = 0;
    if (DAT_0081b850 == 2) {
      pcVar10 = s_frontres_craftsr_bmp_005335d8;
    }
    else {
      pcVar10 = s_frontres_craftmr_bmp_00533590;
    }
LAB_004ec41a:
    FUN_004d0290(pcVar10, s_background_00532a4c);
  }
  FUN_004d52b0();
  FUN_004d1750(s_background_00532a4c, 0, 0);
  FUN_004d0e60(s_frame_00531358, 0, 0);
  FUN_004d0e60(s_allactive_00531ebc, 0, 0);
  if (DAT_0081b850 != 2) {
    FUN_004d0580(s_chatbox_00532a84, 0, 0);
  }
  FUN_004d0580(s_regoverlay_00531e94, 0, 0);
  FUN_004d5350(1);
LAB_004ec4a2:
  if ((bVar6) && (DAT_0081b850 != 2)) {
    DAT_00758620 = 0x62;
    DAT_00758628 = (unsigned int)DAT_00757cac;
    DAT_0075862c = (unsigned int)DAT_00757ca4;
    DAT_00758630 = (unsigned int)DAT_00757c84;
    DAT_00758634 = (unsigned int)DAT_00757c90;
    DAT_0075863c = DAT_00757c94;
    DAT_00758624 = (unsigned int)DAT_00743dc2[DAT_00757cb0 * 0x562];
    DAT_00758640 = DAT_00757c98;
    DAT_00758638 = (unsigned int)DAT_00757cb8;
    FUN_004cb830(0, &DAT_00758620, 0x24);
  }
  return 0;
}
