/* SPDX-License-Identifier: MIT */
/* z_bop 004f1cc0 */
/* DAT_ names kept as placeholders. Packed globals are byte arrays accessed via RDP/WRP. */

extern int _sprintf(char *, const char *, ...);
extern void *_malloc(unsigned int);

extern int FUN_004cc9f0(int);
extern void FUN_004d2e50(void);
extern char *FUN_004ddc10(int);
extern int FUN_004dcb90(char *, char *, char *, char *, char *);
extern int FUN_004cfee0(void);
extern int FUN_004d11d0(void);
extern void FUN_004cef70(int, unsigned int *, unsigned int);
extern void FUN_004cd9c0(void);
extern void FUN_004cd9d0(void);
extern void FUN_004c9ee0(void);
extern void FUN_004dc330(void *, void *);
extern int FUN_004f4b80(void);
extern int FUN_004df5f0(int);
extern int FUN_004f4750(void);
extern void FUN_004de0b0(int, int);
extern void FUN_004e5000(int);
extern void FUN_004f70f0(void);
extern void FUN_004f36d0(void);
extern void FUN_004f3ea0(void);
extern int FUN_004d1400(void);
extern int FUN_004d11e0(void);
extern void FUN_004b4a60(char *, char *);
extern void FUN_004d4e20(void);
extern void FUN_004b5de0(char *, int, int);
extern void FUN_004b5570(char *, int, int);
extern void FUN_004b4d40(char *, int, int);
extern void FUN_004d6460(int *, int, int, int, int);
extern void FUN_004d64a0(int *, int, int);
extern void FUN_004d4ec0(int);
extern void FUN_004dc140(int);
extern void FUN_004e5590(void *);
extern void FUN_004d4cf0(int *);
extern void FUN_004d4d20(int *);
extern void FUN_004db640(int, char *, int *, unsigned int);
extern int FUN_004e0490(void);
extern void FUN_004f1680(void);
extern unsigned char FUN_004dc2e0(int);
extern void FUN_004dc1d0(void);
extern void FUN_004f32b0(unsigned int);
extern void FUN_004f3740(unsigned int);
extern void FUN_004f4f80(void);
extern void FUN_004d8690(unsigned int);
extern void FUN_004dad40(void);
extern void FUN_004dad70(char *);
extern int FUN_004da780(int *, char *, char *, char *, int, int, int, char *);
extern int FUN_004f3f70(void);
extern void FUN_004dad60(void);
extern void FUN_004f4710(void);
extern void FUN_004f4010(void);
extern int FUN_004bf9b0(int);
extern unsigned char FUN_004dc2a0(int);
extern unsigned char FUN_004dc2c0(int);
extern void FUN_004de090(int *, int *);
extern void FUN_004db4e0(int, char *, int, int, unsigned int);

/* handler functions / code labels passed to FUN_004dc330 */
extern void FUN_004be1b0(void);
extern void FUN_004d73c0(void);
extern void FUN_004f5170(void);
extern void FUN_004e15d0(void);
extern void FUN_004fbe90(void);
extern unsigned char LAB_004be050[];
extern unsigned char LAB_004d7370[];
extern unsigned char LAB_004f5100[];
extern unsigned char LAB_004e1470[];
extern unsigned char LAB_004fbe10[];

extern int DAT_0052bf54;
extern int DAT_006691d0;
extern int DAT_0052c208;
extern int DAT_00bb2818;
extern int DAT_00b69e30;
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6af4;
extern int DAT_00b6a250;
extern int DAT_0052c184;
extern int DAT_00a91b8c;
extern unsigned char DAT_00b6a606[];
extern unsigned char DAT_00b6a5aa[];
extern unsigned char DAT_00b6a5a6[];
extern unsigned char DAT_00b6a5ae[];
extern unsigned char DAT_00aa615e[];
extern char DAT_00aa6150[];
extern unsigned char DAT_00aa6162[];
extern unsigned char DAT_00b6bba6[];
extern unsigned char DAT_00b6bbae[];
extern unsigned char DAT_00b6d962[];
extern int DAT_00aa6120;
extern int DAT_00aa6124;
extern int DAT_00aa6128;
extern int DAT_00aa612c;
extern int DAT_00aa6130;
extern int DAT_00aa5e64;
extern int DAT_00aa5e38;
extern int DAT_00aa5e60;
extern int DAT_00aa5e40[8];
extern int DAT_00aa6114;
extern int DAT_00aa6134;
extern int DAT_00aa6138;
extern int DAT_00aa5cb0[];
extern int DAT_00aa5df0[];
extern int DAT_00aa5e10[];
extern int DAT_00aa5e68;
extern int DAT_0052c204;
extern void *DAT_00aa6118;
extern char DAT_00b69d20[];
extern char DAT_0052ada8[];
extern unsigned int DAT_00b69e38;
extern unsigned int DAT_00b6a2a0;
extern unsigned int DAT_00b6a2c0;
extern int DAT_0052c18c;
extern char DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern short DAT_00aa5aac;
extern char DAT_00a91d24;
extern char DAT_00a91d17[];
extern char DAT_00bb73a8;
extern char DAT_00bb7397;
extern unsigned char DAT_00b6a270[];

extern char s_frontres_soloteam_bmp_0052c890[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_teamoverlay_0052c884[];
extern char s_captoverlay_0052c878[];
extern char s_slotoverlay_0052c86c[];
extern char s__c_s__c_s_0052a6b0[];
extern char s_rebtiny_d_0052a6a4[];
extern char s_imptiny_d_0052a698[];
extern char s_buttonsound_0052a68c[];
extern char s_leaveup_0052a678[];
extern char s_leavedown_0052a680[];
extern char s_flysound_0052bf94[];
extern char s_nextup_0052bf80[];
extern char s_nextdown_0052bf88[];

#define RDP(p) ({ int t_; __builtin_memcpy(&t_, (p), 4); t_; })
#define WRP(p, v) do { int t_ = (v); __builtin_memcpy((p), &t_, 4); } while (0)

#define TINY_IDX() ((int)(((param_1 ^ uVar22) - uVar22 & 0x1f ^ uVar22) - uVar22) >> 1)

unsigned int FUN_004f1cc0(unsigned int param_1)
{
  char cVar1;
  int iVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  char *pcVar7;
  char *pcVar8;
  int iVar9;
  int *piVar10;
  int *piVar11;
  int iVar12;
  int iVar13;
  int *puVar14;
  int *piVar15;
  int *puVar16;
  unsigned char *puVar24;
  int bVar17;
  unsigned int uVar18;
  unsigned int uVar19;
  unsigned int *puVar20;
  void *pcVar21;
  unsigned int uVar22;
  void *puVar23;
  int local_2c;
  int local_28[4];
  int local_18;
  int local_14;
  int local_10[4];

  if (param_1 == 0) {
    DAT_0052bf54 = 0;
    DAT_006691d0 = 0;
    DAT_0052c208 = 0;
    while (iVar3 = FUN_004cc9f0(DAT_00bb2818), iVar3 == 0) {
      FUN_004d2e50();
      pcVar4 = FUN_004ddc10(0x13);
      pcVar5 = FUN_004ddc10(0x20b);
      pcVar6 = FUN_004ddc10(0x2c4);
      pcVar7 = FUN_004ddc10(0x2c3);
      pcVar8 = FUN_004ddc10(0x2c2);
      iVar3 = FUN_004dcb90(pcVar8, pcVar7, pcVar6, pcVar5, pcVar4);
      if (iVar3 == 0) {
        if (DAT_00b69e30 != 2) {
          iVar3 = FUN_004cfee0();
          if (iVar3 == 0) {
            DAT_00aa6af0 = 0x47;
            uVar22 = 4;
            puVar20 = &DAT_00aa6af0;
            iVar3 = FUN_004d11d0();
          }
          else {
            DAT_00aa6af0 = 0x46;
            uVar22 = 4;
            puVar20 = &DAT_00aa6af0;
            iVar3 = 0;
          }
          FUN_004cef70(iVar3, puVar20, uVar22);
          FUN_004cd9c0();
        }
        return 1;
      }
    }
    FUN_004c9ee0();
    if ((DAT_00b6a250 == 0) && (DAT_00b69e30 != 3)) {
      pcVar8 = (char *)0;
      pcVar7 = (char *)0;
      pcVar4 = FUN_004ddc10(0x2ef);
      pcVar5 = FUN_004ddc10(0x2ee);
      pcVar6 = FUN_004ddc10(0x2ed);
      FUN_004dcb90(pcVar6, pcVar5, pcVar4, pcVar7, pcVar8);
      if (DAT_00b69e30 != 2) {
        DAT_00aa6af0 = 0x46;
        FUN_004cef70(0, &DAT_00aa6af0, 4);
        FUN_004cd9d0();
      }
      DAT_00b69e30 = 0;
      FUN_004dc330((void *)FUN_004be1b0, LAB_004be050);
      return 0;
    }
    if (((DAT_0052c184 == 0) && (DAT_00a91b8c != 1)) && (RDP(DAT_00b6a606) == 1)) {
      if (RDP(DAT_00b6a5aa) == 0) {
        iVar3 = FUN_004f4b80();
        if (iVar3 == 0) {
          DAT_00aa6120 = 0;
          DAT_00aa6124 = 0;
          DAT_00aa6128 = 0;
          DAT_00aa612c = 0;
          DAT_00aa6130 = 0;
        }
        iVar3 = FUN_004df5f0(0);
        if ((DAT_00b69e30 != 2) && (iVar3 == 0)) {
          DAT_00aa6af0 = 0x47;
          uVar22 = 4;
          puVar20 = &DAT_00aa6af0;
          iVar3 = FUN_004d11d0();
          FUN_004cef70(iVar3, puVar20, uVar22);
          FUN_004cd9d0();
          FUN_004dc330((void *)FUN_004d73c0, LAB_004d7370);
          return 0;
        }
      }
      else if ((RDP(DAT_00b6a5aa) == 3) && (iVar3 = FUN_004f4750(), iVar3 == 0)) {
        DAT_00aa6120 = 0;
        DAT_00aa6124 = 0;
        DAT_00aa6128 = 0;
        DAT_00aa612c = 0;
        DAT_00aa6130 = 0;
      }
    }
    if (DAT_00a91b8c != 0) {
      DAT_00a91b8c = 0;
      DAT_00aa5e64 = 1;
      FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
      return 0;
    }
    FUN_004de0b0(0x25, 0x1bd);
    DAT_00aa5e38 = 0;
    DAT_00a91b8c = 0;
    DAT_00aa5e60 = 0;
    puVar14 = DAT_00aa5e40;
    for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
      *puVar14 = 0;
      puVar14 = puVar14 + 1;
    }
    FUN_004e5000(RDP(DAT_00b6a5aa));
    if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
      piVar11 = (int *)(DAT_00aa6114 + 0x140);
      do {
        if (*piVar11 == RDP(DAT_00b6a5ae + RDP(DAT_00b6a5aa) * 4)) break;
        piVar11 = piVar11 + 0x52;
        DAT_00aa6134 = DAT_00aa6134 + 1;
      } while (DAT_00aa6134 < DAT_00aa6138);
    }
    FUN_004f70f0();
    if (DAT_0052c184 == 0) {
      puVar14 = DAT_00aa5cb0;
      for (iVar3 = 0x50; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar14 = 0;
        puVar14 = puVar14 + 1;
      }
      puVar14 = DAT_00aa5df0;
      for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar14 = 0;
        puVar14 = puVar14 + 1;
      }
      FUN_004f36d0();
    }
    else {
      FUN_004f3ea0();
    }
    if (DAT_00aa5e68 == 1) {
      iVar3 = 1;
      if (DAT_00b69e30 != 2) {
        iVar3 = FUN_004d1400();
      }
      if (iVar3 == 1) {
        if (DAT_0052c184 != 1) {
          DAT_00aa5cb0[0] = RDP(DAT_00aa615e);
          DAT_00aa5df0[0] = RDP(DAT_00aa615e);
          WRP(DAT_00b6a5a6, 0);
          FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
          DAT_00aa5e64 = 1;
          DAT_0052c184 = 0;
          return 0;
        }
        FUN_004dc330((void *)FUN_004e15d0, LAB_004e1470);
        DAT_0052c184 = 0;
        return 0;
      }
      iVar13 = 0;
      if (DAT_0052c184 == 0) {
        local_2c = 0;
        if (DAT_00aa5e68 != 0) {
          puVar16 = DAT_00aa5df0;
          puVar24 = DAT_00aa615e;
          do {
            iVar12 = 0;
            if (0 < DAT_00aa5e10[iVar13]) {
              piVar11 = DAT_00aa5cb0 + iVar13 * 8;
              do {
                if (*piVar11 == 0) {
                  __builtin_memcpy(&uVar19, puVar24, 4);
                  *puVar16 = (int)uVar19;
                  puVar16 = puVar16 + 1;
                  puVar24 = puVar24 + 0x2a;
                  DAT_00aa5cb0[iVar12 + iVar13 * 8] = (int)uVar19;
                  local_2c = local_2c + 1;
                  break;
                }
                piVar11 = piVar11 + 1;
                iVar12 = iVar12 + 1;
              } while (iVar12 < DAT_00aa5e10[iVar13]);
            }
            iVar13 = iVar13 + 1;
            if (DAT_00aa5e68 <= iVar13) {
              iVar13 = 0;
            }
          } while (local_2c < iVar3);
        }
      }
    }
    else {
      if (DAT_00b69e30 == 2) {
        if (DAT_0052c184 == 0) {
          if ((RDP(DAT_00b6a5aa) == 3) || (RDP(DAT_00b6a5aa) == 4)) {
            uVar22 = (unsigned int)RDP(DAT_00b6d962) ^ 1;
            DAT_00aa5df0[0] = RDP(DAT_00aa615e);
            WRP(DAT_00b6a5a6, (int)uVar22);
            DAT_00aa5cb0[uVar22 * 8] = RDP(DAT_00aa615e);
            bVar17 = 0;
            if ((RDP(DAT_00b6a5aa) == 3) &&
                ((((RDP(DAT_00b6a606) == 1 && (DAT_00bb7397 == '\x02')) && (RDP(DAT_00b6bba6) != 0)) &&
                  ((unsigned int)RDP(DAT_00b6bbae + RDP(DAT_00b6bba6) * 4) != uVar22)))) {
              bVar17 = 1;
            }
            if (bVar17) {
              puVar23 = LAB_004fbe10;
              pcVar21 = (void *)FUN_004fbe90;
            }
            else {
              puVar23 = LAB_004f5100;
              pcVar21 = (void *)FUN_004f5170;
            }
            FUN_004dc330(pcVar21, puVar23);
            DAT_00aa5e64 = 1;
            DAT_0052c184 = 0;
            return 0;
          }
        }
        else if (DAT_00aa5e64 != 0) {
          FUN_004dc330((void *)FUN_004e15d0, LAB_004e1470);
          DAT_0052c184 = 0;
          return 0;
        }
      }
      if (DAT_0052c184 == 0) {
        iVar3 = 1;
        if (DAT_00b69e30 != 2) {
          iVar3 = FUN_004d1400();
        }
        iVar13 = 0;
        local_2c = 0;
        if (DAT_00aa5e68 != 0) {
          puVar14 = DAT_00aa5df0;
          puVar24 = DAT_00aa615e;
          do {
            iVar12 = 0;
            if (0 < DAT_00aa5e10[iVar13]) {
              piVar11 = DAT_00aa5cb0 + iVar13 * 8;
              do {
                if (*piVar11 == 0) {
                  __builtin_memcpy(&uVar19, puVar24, 4);
                  *puVar14 = (int)uVar19;
                  puVar14 = puVar14 + 1;
                  puVar24 = puVar24 + 0x2a;
                  DAT_00aa5cb0[iVar12 + iVar13 * 8] = (int)uVar19;
                  local_2c = local_2c + 1;
                  break;
                }
                piVar11 = piVar11 + 1;
                iVar12 = iVar12 + 1;
              } while (iVar12 < DAT_00aa5e10[iVar13]);
            }
            iVar13 = iVar13 + 1;
            if (DAT_00aa5e68 <= iVar13) {
              iVar13 = 0;
            }
          } while (local_2c < iVar3);
        }
      }
    }
    iVar13 = 0;
    DAT_0052c184 = 0;
    DAT_00aa5e64 = 0;
    iVar3 = RDP(DAT_00b6a5a6);
    if ((DAT_00b69e30 == 2) && (0 < DAT_00aa5e68)) {
      piVar11 = DAT_00aa5cb0;
      do {
        iVar3 = iVar13;
        if (*piVar11 == 1) break;
        piVar11 = piVar11 + 8;
        iVar13 = iVar13 + 1;
        iVar3 = RDP(DAT_00b6a5a6);
      } while (iVar13 < DAT_00aa5e68);
    }
    WRP(DAT_00b6a5a6, iVar3);
    if (RDP(DAT_00b6a5aa) == 1) {
      iVar3 = 0;
      bVar17 = DAT_00aa5e68 == 0;
      if (0 < DAT_00aa5e68) {
        piVar11 = DAT_00aa5e10;
        do {
          if (1 < *piVar11) break;
          piVar11 = piVar11 + 1;
          iVar3 = iVar3 + 1;
        } while (iVar3 < DAT_00aa5e68);
        bVar17 = iVar3 == DAT_00aa5e68;
      }
      if (bVar17) {
        iVar3 = 0;
        WRP(DAT_00b6a5a6, 0);
        if ((DAT_00b69e30 != 2) && (0 < DAT_00aa5e68)) {
          piVar10 = DAT_00aa5cb0;
          piVar11 = DAT_00aa5e10;
          do {
            iVar13 = 0;
            piVar15 = piVar10;
            if (0 < *piVar11) {
              do {
                iVar12 = FUN_004d11e0();
                if (iVar12 == *piVar15) {
                  WRP(DAT_00b6a5a6, iVar3);
                }
                iVar13 = iVar13 + 1;
                piVar15 = piVar15 + 1;
              } while (iVar13 < *piVar11);
            }
            piVar11 = piVar11 + 1;
            iVar3 = iVar3 + 1;
            piVar10 = piVar10 + 8;
          } while (iVar3 < DAT_00aa5e68);
        }
        DAT_0052c184 = 0;
        FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
        DAT_00aa5e64 = 1;
        return 0;
      }
    }
    if ((DAT_00b69e30 == 2) && (DAT_0052c204 == 1)) {
      FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
      DAT_00aa5e64 = 1;
      DAT_0052c184 = 0;
      return 0;
    }
    if (((RDP(DAT_00b6a5aa) == 0) && (DAT_00aa5e68 == 1)) && (DAT_00aa5e10[0] == 8)) {
      iVar3 = 0;
      piVar11 = DAT_00aa5cb0;
      do {
        iVar13 = 0;
        piVar15 = piVar11;
        do {
          iVar9 = FUN_004d11e0();
          iVar12 = iVar3;
          if (iVar9 == *piVar15) break;
          piVar15 = piVar15 + 1;
          iVar13 = iVar13 + 1;
          iVar12 = RDP(DAT_00b6a5a6);
        } while (iVar13 < 8);
        WRP(DAT_00b6a5a6, iVar12);
        piVar11 = piVar11 + 8;
        iVar3 = iVar3 + 1;
        if (DAT_00aa5e68 <= iVar3) {
          FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
          DAT_00aa5e64 = 1;
          DAT_0052c184 = 0;
          return 0;
        }
      } while (1);
    }
    FUN_004b4a60(s_frontres_soloteam_bmp_0052c890, s_background_0052bf74);
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    if (DAT_00b6a250 == 0) {
      pcVar4 = s_clientactive_0052b69c;
    }
    else {
      pcVar4 = s_allactive_0052b6ac;
    }
    FUN_004b5570(pcVar4, 0, 0);
    if (DAT_00b69e30 != 2) {
      FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
    }
    FUN_004b4d40(s_teamoverlay_0052c884, 0, 0);
    iVar3 = 0;
    FUN_004d6460(local_28, 0x58, 0xcf, 0x100, 0xdd);
    if (0 < DAT_00aa5e68) {
      do {
        if (((DAT_00aa5e68 == 8) || (DAT_00aa5e68 == 7)) && (iVar3 == 4)) {
          FUN_004d6460(local_28, 0x104, 0xcf, 0x1ac, 0xdd);
        }
        iVar13 = 0;
        FUN_004d64a0(local_28, 0, 0xf);
        if (0 < DAT_00aa5e10[iVar3]) {
          do {
            if (iVar13 == 0) {
              pcVar4 = s_captoverlay_0052c878;
            }
            else {
              pcVar4 = s_slotoverlay_0052c86c;
            }
            iVar13 = iVar13 + 1;
            FUN_004b4d40(pcVar4, local_28[0], local_28[1]);
            FUN_004d64a0(local_28, 0, 0xf);
          } while (iVar13 < DAT_00aa5e10[iVar3]);
        }
        iVar3 = iVar3 + 1;
      } while (iVar3 < DAT_00aa5e68);
    }
    FUN_004d4ec0(1);
    FUN_004dc140(0x14);
    DAT_00aa6118 = _malloc(0x1000);
    FUN_004e5590(DAT_00aa6118);
  }
  FUN_004d6460(local_28, 0x9e, 0x34, 0x1eb, 0x44);
  FUN_004d4cf0(local_10);
  FUN_004d4d20(local_28);
  _sprintf(DAT_00b69d20, DAT_0052ada8, 4,
           (int)((unsigned int)DAT_00aa6134 * 0x148u + (unsigned int)DAT_00aa6114 + 0x40u));
  uVar22 = 0xffffffff;
  pcVar4 = DAT_00b69d20;
  do {
    if (uVar22 == 0) break;
    uVar22 = uVar22 - 1;
    cVar1 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar1 != '\0');
  for (iVar3 = (int)(~uVar22 - 2); 0 < iVar3; iVar3 = iVar3 + -1) {
    if (DAT_00b69d20[iVar3] == '(') {
      DAT_00b69d20[iVar3] = 0;
      break;
    }
  }
  FUN_004db640(0xc, DAT_00b69d20, local_28, 0xffff);
  FUN_004d4d20(local_10);
  FUN_004d6460(local_28, 0x54, 0x5a, 0x1b2, 0x6c);
  piVar11 = local_28;
  uVar22 = 0xffff;
  if (RDP(DAT_00b6a5aa) == 3) {
    uVar18 = 0x1d2;
  }
  else {
    uVar18 = 0x1d1;
  }
  pcVar4 = FUN_004ddc10((int)uVar18);
  FUN_004db640(0xf, pcVar4, piVar11, uVar22);
  if (DAT_006691d0 == 0) {
    FUN_004d6460(local_28, 0x54, 400, 0x1b2, 0x19e);
    iVar3 = FUN_004cfee0();
    if (iVar3 == 0) {
      if (DAT_00b69e30 == 2) goto LAB_004f27c6;
      iVar3 = 0;
      if (0 < DAT_00aa5e68) {
        piVar11 = DAT_00aa5cb0;
        do {
          iVar13 = FUN_004d11e0();
          if (iVar13 == *piVar11) {
            piVar15 = local_28;
            uVar22 = DAT_00b69e38;
            pcVar4 = FUN_004ddc10(0x2e5);
            FUN_004db640(0xc, pcVar4, piVar15, uVar22);
          }
          piVar11 = piVar11 + 8;
          iVar3 = iVar3 + 1;
        } while (iVar3 < DAT_00aa5e68);
      }
      FUN_004d64a0(local_28, 0, 0xf);
      uVar18 = 0x1d5;
      uVar22 = DAT_00b69e38;
    }
    else {
      if (DAT_00b69e30 != 2) {
        piVar11 = local_28;
        uVar22 = DAT_00b6a2a0;
        pcVar4 = FUN_004ddc10(0x1d4);
        FUN_004db640(0xc, pcVar4, piVar11, uVar22);
      }
LAB_004f27c6:
      FUN_004d64a0(local_28, 0, 0xf);
      uVar18 = 0x1d3;
      uVar22 = DAT_00b6a2a0;
    }
    piVar11 = local_28;
    pcVar4 = FUN_004ddc10((int)uVar18);
    FUN_004db640(0xc, pcVar4, piVar11, uVar22);
  }
  if (DAT_00b69e30 != 2) {
    iVar13 = FUN_004e0490();
    iVar3 = DAT_00aa5e60;
    if (iVar13 == 0x46) {
      iVar3 = FUN_004cfee0();
      if (iVar3 == 0) {
        pcVar8 = (char *)0;
        pcVar7 = (char *)0;
        pcVar4 = FUN_004ddc10(0x279);
        pcVar5 = FUN_004ddc10(0x278);
        pcVar6 = FUN_004ddc10(0x277);
        FUN_004dcb90(pcVar6, pcVar5, pcVar4, pcVar7, pcVar8);
      }
      DAT_00b69e30 = 3;
      DAT_0052c184 = 1;
      FUN_004dc330((void *)FUN_004d73c0, LAB_004d7370);
      FUN_004cd9d0();
    }
    else if (iVar13 == 0x41) {
      FUN_004f1680();
      FUN_004f3ea0();
    }
    else {
      if (iVar13 == 0x48) {
        iVar3 = 0;
        if (0 < DAT_00aa5e68) {
          piVar11 = DAT_00aa5cb0;
          do {
            iVar13 = 0;
            piVar15 = piVar11;
            do {
              iVar9 = FUN_004d11e0();
              iVar12 = iVar3;
              if (iVar9 == *piVar15) break;
              piVar15 = piVar15 + 1;
              iVar13 = iVar13 + 1;
              iVar12 = RDP(DAT_00b6a5a6);
            } while (iVar13 < 8);
            WRP(DAT_00b6a5a6, iVar12);
            piVar11 = piVar11 + 8;
            iVar3 = iVar3 + 1;
          } while (iVar3 < DAT_00aa5e68);
        }
        if (((RDP(DAT_00b6a5aa) == 3) && (RDP(DAT_00b6a606) == 1)) &&
            ((DAT_00bb7397 == '\x02' && (RDP(DAT_00b6bba6) != 0)))) {
          if ((DAT_00b69e30 != 2) && (iVar3 = FUN_004d1400(), iVar3 != 1)) {
            FUN_004dc330((void *)FUN_004fbe90, LAB_004fbe10);
            return 0;
          }
          if (RDP(DAT_00b6bbae + RDP(DAT_00b6bba6) * 4) != RDP(DAT_00b6a5a6)) {
            FUN_004dc330((void *)FUN_004fbe90, LAB_004fbe10);
            return 0;
          }
        }
        FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
        return 0;
      }
      if (iVar13 == 0x53) {
        DAT_0052c184 = 1;
        FUN_004dc330((void *)FUN_004e15d0, LAB_004e1470);
        return 0;
      }
      if (iVar13 == 0x67) {
        iVar3 = 0;
        if (0 < DAT_00aa5e60) {
          piVar11 = DAT_00aa5e40;
          do {
            if (*piVar11 == DAT_0052c18c) {
              DAT_00aa5e60 = DAT_00aa5e60 + -1;
              break;
            }
            piVar11 = piVar11 + 1;
            iVar3 = iVar3 + 1;
          } while (iVar3 < DAT_00aa5e60);
        }
        if (iVar3 < 7) {
          puVar14 = DAT_00aa5e40 + iVar3;
          do {
            puVar16 = puVar14 + 1;
            *puVar14 = *puVar16;
            puVar14 = puVar16;
          } while (puVar16 < &DAT_00aa5e40[7]);
        }
        DAT_00aa5e40[7] = 0; /* _DAT_00aa5e5c */
      }
      else if (iVar13 == 0x68) {
        iVar13 = 0;
        bVar17 = DAT_00aa5e60 == 0;
        if (0 < DAT_00aa5e60) {
          piVar11 = DAT_00aa5e40;
          do {
            if (*piVar11 == DAT_0052c18c) break;
            piVar11 = piVar11 + 1;
            iVar13 = iVar13 + 1;
          } while (iVar13 < DAT_00aa5e60);
          bVar17 = iVar13 == DAT_00aa5e60;
        }
        if (bVar17) {
          DAT_00aa5e60 = DAT_00aa5e60 + 1;
          DAT_00aa5e40[iVar3] = DAT_0052c18c;
        }
      }
      else if ((iVar13 == 0x55) || (iVar13 == 0x54)) {
        DAT_00aa5e60 = 0;
        puVar14 = DAT_00aa5e40;
        for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
          *puVar14 = 0;
          puVar14 = puVar14 + 1;
        }
        DAT_00aa5e38 = 0;
        bVar17 = FUN_004dc2e0(2);
        if (bVar17 != 0) {
          FUN_004dc1d0();
        }
      }
    }
  }
  FUN_004f32b0(param_1);
  if (DAT_006691d0 == 0) {
    FUN_004f3740(param_1);
  }
  else {
    FUN_004f4f80();
  }
  if (DAT_00b69e30 != 2) {
    FUN_004d8690(param_1);
  }
  FUN_004d6460(local_28, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_00b6a2e0[0] != '\0') {
    _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, DAT_00b6bc9e, 1, DAT_00b6a2e0);
    FUN_004db640(0xc, DAT_00b69d20, local_28, DAT_00b6a2c0);
    uVar22 = (unsigned int)((int)param_1 >> 0x1f);
    if ((RDP(DAT_00b6a5aa) == 1) || (RDP(DAT_00b6a5aa) == 2)) {
LAB_004f2ce6:
      iVar3 = TINY_IDX();
      _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, iVar3);
      FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
      _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, iVar3);
    }
    else {
      if (RDP(DAT_00b6a5aa) == 0) {
        iVar3 = 0;
        if (0 < DAT_00aa5aac) {
          pcVar4 = &DAT_00a91d24;
          do {
            if (*pcVar4 != '\0') {
              if (DAT_00a91d17[iVar3 * 0x562] == '\0') {
                _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, TINY_IDX());
                FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
              }
              else {
                if (DAT_00a91d17[iVar3 * 0x562] != '\x01') goto LAB_004f2ce6;
                _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, TINY_IDX());
                FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
              }
              goto LAB_004f2d30;
            }
            pcVar4 = pcVar4 + 0x562;
            iVar3 = iVar3 + 1;
          } while (iVar3 < DAT_00aa5aac);
        }
        goto LAB_004f2d4a;
      }
      iVar13 = 0;
      iVar3 = FUN_004d11e0();
      bVar17 = DAT_00aa5e68 == 0;
      if (0 < DAT_00aa5e68) {
        piVar11 = DAT_00aa5cb0;
        piVar15 = DAT_00aa5e10;
        do {
          iVar9 = 0;
          iVar12 = *piVar15;
          piVar10 = piVar11;
          if (0 < iVar12) {
            do {
              if (*piVar10 == iVar3) break;
              iVar9 = iVar9 + 1;
              piVar10 = piVar10 + 1;
            } while (iVar9 < iVar12);
            if (iVar9 < iVar12) break;
          }
          piVar11 = piVar11 + 8;
          piVar15 = piVar15 + 1;
          iVar13 = iVar13 + 1;
        } while (iVar13 < DAT_00aa5e68);
        bVar17 = iVar13 == DAT_00aa5e68;
      }
      if (bVar17) goto LAB_004f2ce6;
      if (iVar13 == 0) {
        _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, TINY_IDX());
        FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
      }
      else {
        _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, TINY_IDX());
        FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
      }
    }
LAB_004f2d30:
    FUN_004b5570(DAT_00b69d20, 0x1a4, 0x1c5);
  }
LAB_004f2d4a:
  FUN_004d6460(local_28, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_00bb73a8 != '\0') {
    FUN_004dad40();
  }
  iVar3 = FUN_004cfee0();
  if ((iVar3 == 0) && (DAT_00b69e30 != 2)) {
    pcVar4 = FUN_004ddc10(0x239);
    FUN_004dad70(pcVar4);
    pcVar5 = s_buttonsound_0052a68c;
    iVar13 = 8;
    uVar19 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0xcc);
    iVar3 = FUN_004da780(local_28, s_leaveup_0052a678, s_leavedown_0052a680, pcVar4, iVar3,
                         (int)uVar19, iVar13, pcVar5);
    if (iVar3 == 0) goto LAB_004f2f58;
    pcVar4 = FUN_004ddc10(0x13);
    pcVar5 = FUN_004ddc10(0x20b);
    pcVar6 = FUN_004ddc10(0x22d);
    pcVar7 = FUN_004ddc10(0x22c);
    pcVar8 = FUN_004ddc10(0x22b);
    iVar3 = FUN_004dcb90(pcVar8, pcVar7, pcVar6, pcVar5, pcVar4);
    if (iVar3 == 0) goto LAB_004f2f58;
    DAT_00aa6af0 = 0x47;
    uVar22 = 4;
    puVar20 = &DAT_00aa6af0;
    iVar3 = FUN_004d11d0();
    FUN_004cef70(iVar3, puVar20, uVar22);
    FUN_004cd9d0();
    DAT_0052c184 = 1;
    puVar23 = LAB_004d7370;
    pcVar21 = (void *)FUN_004d73c0;
  }
  else {
    pcVar4 = FUN_004ddc10(0x239);
    FUN_004dad70(pcVar4);
    pcVar5 = s_buttonsound_0052a68c;
    iVar13 = 8;
    uVar19 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x104);
    iVar3 = FUN_004da780(local_28, s_leaveup_0052a678, s_leavedown_0052a680, pcVar4, iVar3,
                         (int)uVar19, iVar13, pcVar5);
    if (iVar3 == 0) goto LAB_004f2f58;
    if ((RDP(DAT_00b6a606) == 1) && (RDP(DAT_00b6a5aa) == 0)) {
      pcVar4 = FUN_004ddc10(0x13);
      pcVar5 = FUN_004ddc10(0x20b);
      pcVar6 = FUN_004ddc10(0x30d);
      pcVar7 = FUN_004ddc10(0x30c);
      pcVar8 = FUN_004ddc10(0x30b);
      FUN_004dcb90(pcVar8, pcVar7, pcVar6, pcVar5, pcVar4);
    }
    if (DAT_00b69e30 != 2) {
      DAT_00aa6af0 = 0x53;
      FUN_004cef70(0, &DAT_00aa6af0, 4);
      if (DAT_00b69e30 != 2) goto LAB_004f2f58;
    }
    puVar23 = LAB_004e1470;
    pcVar21 = (void *)FUN_004e15d0;
  }
  FUN_004dc330(pcVar21, puVar23);
LAB_004f2f58:
  FUN_004d6460(local_28, 8, 0x195, 0x47, 0x1d6);
  if (DAT_00b69e30 == 2) {
    iVar3 = FUN_004f3f70();
    if (iVar3 != 0) {
      pcVar4 = FUN_004ddc10(0xd4);
      FUN_004dad70(pcVar4);
      pcVar5 = s_flysound_0052bf94;
      iVar13 = 7;
      uVar19 = 0;
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x1db);
      iVar3 = FUN_004da780(local_28, s_nextup_0052bf80, s_nextdown_0052bf88, pcVar4, iVar3,
                           (int)uVar19, iVar13, pcVar5);
      if (iVar3 != 0) {
        iVar3 = 0;
        if (0 < DAT_00aa5e68) {
          piVar11 = DAT_00aa5cb0;
          do {
            iVar13 = 0;
            piVar15 = piVar11;
            do {
              iVar12 = iVar3;
              if (*piVar15 == 1) break;
              piVar15 = piVar15 + 1;
              iVar13 = iVar13 + 1;
              iVar12 = RDP(DAT_00b6a5a6);
            } while (iVar13 < 8);
            WRP(DAT_00b6a5a6, iVar12);
            piVar11 = piVar11 + 8;
            iVar3 = iVar3 + 1;
          } while (iVar3 < DAT_00aa5e68);
        }
        if ((((RDP(DAT_00b6a5aa) == 3) && (RDP(DAT_00b6a606) == 1)) && (DAT_00bb7397 == '\x02')) &&
            ((RDP(DAT_00b6bba6) != 0 &&
              (RDP(DAT_00b6bbae + RDP(DAT_00b6bba6) * 4) != RDP(DAT_00b6a5a6))))) {
          FUN_004dc330((void *)FUN_004fbe90, LAB_004fbe10);
          FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
          return 0;
        }
        FUN_004dc330((void *)FUN_004f5170, LAB_004f5100);
        FUN_004dad60();
        return 0;
      }
    }
  }
  else {
    iVar3 = FUN_004cfee0();
    if ((iVar3 != 0) && (iVar3 = FUN_004f3f70(), iVar3 != 0)) {
      pcVar4 = FUN_004ddc10(0xd4);
      FUN_004dad70(pcVar4);
      pcVar5 = s_flysound_0052bf94;
      iVar13 = 7;
      uVar19 = 0;
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x1db);
      iVar3 = FUN_004da780(local_28, s_nextup_0052bf80, s_nextdown_0052bf88, pcVar4, iVar3,
                           (int)uVar19, iVar13, pcVar5);
      if (iVar3 != 0) {
        FUN_004f4710();
      }
    }
  }
  FUN_004dad60();
  FUN_004f4010();
  iVar3 = FUN_004bf9b0(1);
  if (iVar3 == 1) {
    return 1;
  }
  bVar17 = FUN_004dc2e0(2);
  if (bVar17 != 0) {
    if ((FUN_004dc2a0(2) == 0) && (FUN_004dc2c0(2) == 0)) {
      iVar13 = 0;
      iVar3 = (int)(((0u - (unsigned int)(DAT_00b69e30 == 2)) & 0xfffffff9u) + 8u);
      if (0 < iVar3) {
        puVar24 = DAT_00aa615e;
        for (;;) {
          int t_;
          __builtin_memcpy(&t_, puVar24, 4);
          if (t_ == DAT_00aa5e38) break;
          puVar24 = puVar24 + 0x2a;
          iVar13 = iVar13 + 1;
          if (iVar3 <= iVar13) {
            return 0;
          }
        }
        FUN_004de090(&local_14, &local_18);
        iVar3 = (int)(DAT_00aa6150 + (unsigned int)iVar13 * 0x2au);
        uVar19 = 1;
        pcVar4 = FUN_004ddc10(RDP(DAT_00aa6162 + (unsigned int)iVar13 * 0x2au) + 0x9a);
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar4, uVar19, iVar3);
        iVar3 = FUN_004d11e0();
        if ((iVar3 != RDP(DAT_00aa615e + (unsigned int)iVar13 * 0x2au)) && (DAT_00b69e30 != 2)) {
          FUN_004db4e0(0xc, DAT_00b69d20, local_14 + -7, local_18 + -7, DAT_00b6a2c0);
          return 0;
        }
        FUN_004db4e0(0xc, DAT_00b69d20, local_14 + -7, local_18 + -7,
                     *(unsigned int *)(DAT_00b6a270 +
                                       ((((int)param_1 % 0x18) & 0xfffffffeU) * 2)));
        return 0;
      }
    }
    else {
      if (DAT_00b69e30 != 2) {
        DAT_00aa6af4 = (unsigned int)DAT_00aa5e38;
        DAT_00aa6af0 = 0x67;
        FUN_004cef70(0, &DAT_00aa6af0, 8);
      }
      FUN_004dc1d0();
      DAT_00aa5e38 = 0;
    }
  }
  return 0;
}
