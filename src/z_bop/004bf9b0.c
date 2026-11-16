/* SPDX-License-Identifier: MIT */
/* z_bop 004bf9b0 */
/* Prototypes */
void FUN_004de090(int *a, int *b);
void FUN_004d6460(int *r, int a, int b, int c, int d);
void FUN_004b5570(char *s, int a, int b);
char *FUN_004ddc10(unsigned int id);
int FUN_004da780(int *r, char *a, char *b, char *c, int d, unsigned int e, int f, char *g);
void FUN_004dad40(void);
void FUN_004dad70(char *s);
unsigned int FUN_004dcb10(void);
void FUN_004dcb50(void);
int FUN_004cfee0(void);
int FUN_004dcb90(char *a, char *b, char *c, char *d, char *e);
void FUN_004ba3c0(void);
void FUN_004cd9c0(void);
void FUN_004dad60(void);
int FUN_004d11d0(void);
void FUN_004cef70(int a, unsigned int *b, unsigned int c);
void FUN_004dc380(void *cb, void *arg);
void FUN_004dad80(void);
void FUN_004daa20(int *r, char *a, char *b, int c);
int FUN_004d6b90(int *r, int a, int b);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
void FUN_004de5a0(char *s, int a, int b, int c);
void FUN_004dcb30(void);
void FUN_004dc7e0(void);
void FUN_004dc1d0(void);
void FUN_004b4bf0(char *s);
void FUN_004dc170(void);
void FUN_004d9d40(void);
int _rand(void);
void FUN_004cd9d0(void);
void FUN_004dc330(void *cb, void *arg);
void _free(void *p);
void FUN_0042b1b0(void);
void FUN_004b7f30(void);
void FUN_004e96b0(void);
void FUN_004d73c0(void);
void FUN_004dfd30(void);
void FUN_004e15d0(void);
void FUN_004be1b0(void);
void LAB_004d7370(void);
void LAB_004dfd10(void);
void LAB_004e1470(void);
void LAB_004be050(void);

/* Globals */
extern char s_helpup_0052b428[];
extern char s_helpdown_0052b430[];
extern char s_buttonsound_0052a68c[];
extern char s_configup_0052b410[];
extern char s_exitup_0052b408[];
extern char s_exitdown_0052b41c[];
extern char s_configdown_0052b3fc[];
extern char s_backconfig_0052a6f4[];
extern char s_joinindown_0052b3f0[];
extern char s_creategamedown_0052b3e0[];
extern char DAT_0052b3cc[];
extern char s_flysolodown_0052b3d4[];
extern char s_reviewcraftdown_0052b3bc[];
extern char s_backreview_0052b3b0[];
extern char s_pilotregdown_0052b3a0[];

extern int DAT_00bb73a8;
extern char DAT_00bb73a7;
extern int DAT_00aa613c;
extern int DAT_00b69e30;
extern int DAT_00b6a606;
extern int DAT_00b6a5aa;
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6af4;
extern unsigned int DAT_00aa6af8;
extern unsigned int DAT_00aa6afc;
extern unsigned int DAT_00aa6b00;
extern unsigned int DAT_00aa6b04;
extern unsigned int DAT_00aa6b08;
extern unsigned int DAT_00aa6b0c;
extern unsigned int DAT_00aa6b10;
extern unsigned int DAT_00aa6b14;
extern unsigned int DAT_00aa6b18;
extern unsigned int DAT_00aa6b1c;
extern unsigned int DAT_00aa6b20;
extern unsigned int DAT_00aa6b24;
extern unsigned int DAT_00aa6b28;
extern unsigned int DAT_00aa6b2c;
extern unsigned int DAT_00aa6b30;
extern unsigned int DAT_00aa6b34;
extern unsigned int DAT_00aa6b38;
extern unsigned char DAT_00bb7394;
extern unsigned char DAT_00bb7395;
extern unsigned char DAT_00bb7396;
extern unsigned char DAT_00bb7397;
extern unsigned char DAT_00bb7398;
extern unsigned char DAT_00bb7399;
extern unsigned char DAT_00bb739a;
extern unsigned char DAT_00bb739b;
extern unsigned char DAT_00bb739c;
extern unsigned char DAT_00bb739d;
extern unsigned char DAT_00bb739e;
extern unsigned char DAT_00bb739f;
extern unsigned char DAT_00bb736f;
extern unsigned char DAT_00bb73a4;
extern unsigned char DAT_00bb7370;
extern unsigned char DAT_00bb73a5;
extern unsigned char DAT_00bb73a6;
extern int DAT_0052c000;
extern int DAT_00bb72d0;
extern char DAT_00b69d20[];
extern unsigned int DAT_00b6a2e0;
extern int DAT_00aa6144;
extern int DAT_00b6a2a8;
extern int DAT_00b6a250;
extern unsigned int DAT_00aa6150[];
extern int DAT_00aa60d0;
extern void *DAT_00aa60f4;
extern void *DAT_00aa6110;

unsigned int __cdecl FUN_004bf9b0(int param_1)
{
  char cVar1;
  int bVar2;
  unsigned char uVar3;
  char *pcVar4;
  unsigned int uVar5;
  char *pcVar6;
  char *pcVar7;
  unsigned int uVar8;
  unsigned int *puVar9;
  int iVar10;
  unsigned int uVar11;
  int iVar12;
  unsigned int *puVar13;
  char *pcVar14;
  char *pcVar15;
  int local_28[4];
  int local_18;
  int local_14;
  int local_10[4];

  bVar2 = 0;
  FUN_004de090(&local_14, &local_18);
  FUN_004d6460(local_28, 0x262, 0x1bd, 0x27a, 0x1d5);
  if (DAT_00bb73a8 == 0) {
    FUN_004b5570(s_helpup_0052b428, 0x262, 0x1bd);
    pcVar14 = s_buttonsound_0052a68c;
    iVar12 = 0x23;
    uVar11 = 0;
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0x2bf);
  } else {
    FUN_004b5570(s_helpdown_0052b430, 0x262, 0x1bd);
    pcVar14 = s_buttonsound_0052a68c;
    iVar12 = 0x23;
    uVar11 = 0;
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0x2c0);
  }
  iVar10 = FUN_004da780(local_28, (char *)0, (char *)0, pcVar4, iVar10, uVar11, iVar12, pcVar14);
  if (iVar10 != 0) {
    DAT_00bb73a8 = DAT_00bb73a8 ^ 1;
  }
  if (DAT_00bb73a8 != 0) {
    FUN_004dad40();
  }
  FUN_004d6460(local_28, 0x24a, 4, 0x279, 0x47);
  pcVar4 = FUN_004ddc10(0x238);
  FUN_004dad70(pcVar4);
  if ((param_1 < 2) || (3 < param_1)) {
    if (param_1 == 4) {
      FUN_004b5570(s_configup_0052b410, 0, 0);
      pcVar15 = s_buttonsound_0052a68c;
      iVar12 = 6;
      uVar11 = 0;
      iVar10 = 0xc;
      pcVar4 = FUN_004ddc10(6);
      pcVar14 = s_exitup_0052b408;
    } else {
      pcVar15 = s_buttonsound_0052a68c;
      iVar12 = 6;
      uVar11 = 0;
      iVar10 = 0xc;
      pcVar4 = FUN_004ddc10(6);
      pcVar14 = (char *)0;
    }
    iVar10 = FUN_004da780(local_28, pcVar14, s_exitdown_0052b41c, pcVar4, iVar10, uVar11, iVar12, pcVar15);
    uVar5 = FUN_004dcb10();
    if ((char)uVar5 == '\x1b') {
      iVar10 = 1;
      FUN_004dcb50();
    }
    if (iVar10 != 0) {
      if (DAT_00aa613c == 0) {
LAB_004bfcac:
        pcVar4 = FUN_004ddc10(0x13);
        pcVar14 = FUN_004ddc10(0x20b);
        pcVar15 = FUN_004ddc10(0x27c);
        pcVar6 = FUN_004ddc10(0x27b);
        uVar5 = 0x27a;
      } else if (DAT_00b69e30 == 2) {
        if (DAT_00b6a606 != 1) goto LAB_004bfcac;
        if (DAT_00b6a5aa == 1) {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar14 = FUN_004ddc10(0x20b);
          pcVar15 = FUN_004ddc10(0x2a8);
          pcVar6 = FUN_004ddc10(0x2a7);
          uVar5 = 0x2a6;
        } else if (DAT_00b6a5aa == 3) {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar14 = FUN_004ddc10(0x20b);
          pcVar15 = FUN_004ddc10(0x2ab);
          pcVar6 = FUN_004ddc10(0x2aa);
          uVar5 = 0x2a9;
        } else {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar14 = FUN_004ddc10(0x20b);
          pcVar15 = FUN_004ddc10(0x30d);
          pcVar6 = FUN_004ddc10(0x30c);
          uVar5 = 0x30b;
        }
      } else {
        iVar10 = FUN_004cfee0();
        if (iVar10 == 0) {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar14 = FUN_004ddc10(0x20b);
          pcVar15 = FUN_004ddc10(0x22d);
          pcVar6 = FUN_004ddc10(0x22c);
          uVar5 = 0x22b;
        } else {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar14 = FUN_004ddc10(0x20b);
          pcVar15 = FUN_004ddc10(0x230);
          pcVar6 = FUN_004ddc10(0x22f);
          uVar5 = 0x22e;
        }
      }
      pcVar7 = FUN_004ddc10(uVar5);
      iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar15, pcVar14, pcVar4);
      if (iVar10 != 0) {
        if (param_1 == 1) {
          if (DAT_00b69e30 != 2) {
            iVar10 = FUN_004cfee0();
            if (iVar10 == 0) {
              DAT_00aa6af0 = 0x47;
              uVar5 = 4;
              puVar13 = &DAT_00aa6af0;
              iVar10 = FUN_004d11d0();
            } else {
              DAT_00aa6af0 = 0x46;
              uVar5 = 4;
              puVar13 = &DAT_00aa6af0;
              iVar10 = 0;
            }
            FUN_004cef70(iVar10, puVar13, uVar5);
          }
          FUN_004ba3c0();
          FUN_004cd9c0();
          FUN_004dad60();
          return 1;
        }
        if (param_1 != 4) {
          FUN_004ba3c0();
          FUN_004cd9c0();
          FUN_004dad60();
          return 1;
        }
        if (DAT_00b6a606 == 0) {
          if (DAT_00b69e30 == 2) goto LAB_004bfe13;
          iVar10 = FUN_004cfee0();
          if (iVar10 != 0) {
            DAT_00aa6af0 = 0x5c;
            goto LAB_004bfe02;
          }
          DAT_00aa6af0 = 0x47;
          uVar5 = 4;
          puVar13 = &DAT_00aa6af0;
          iVar10 = FUN_004d11d0();
        } else {
          iVar10 = FUN_004cfee0();
          if (iVar10 == 0) {
            if (DAT_00b69e30 == 2) goto LAB_004bfe13;
            DAT_00aa6af0 = 0x47;
            uVar5 = 4;
            puVar13 = &DAT_00aa6af0;
            iVar10 = FUN_004d11d0();
          } else {
            if (DAT_00b69e30 == 2) goto LAB_004bfe13;
            DAT_00aa6af0 = 0x46;
LAB_004bfe02:
            uVar5 = 4;
            puVar13 = &DAT_00aa6af0;
            iVar10 = 0;
          }
        }
        FUN_004cef70(iVar10, puVar13, uVar5);
LAB_004bfe13:
        FUN_004ba3c0();
        FUN_004cd9c0();
        FUN_004dad60();
        return 1;
      }
    }
  }
  FUN_004d6460(local_28, 0x1f7, 4, 0x245, 0x38);
  pcVar4 = FUN_004ddc10(0x237);
  FUN_004dad70(pcVar4);
  if ((DAT_00b69e30 == 5) || (param_1 == 2)) {
    FUN_004dad80();
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0x23a);
    FUN_004daa20(local_28, s_configdown_0052b3fc, pcVar4, iVar10);
    iVar10 = FUN_004d6b90(local_28, local_14, local_18);
    if ((iVar10 != 0) &&
        (((uVar3 = FUN_004dc220()) != 0) || ((uVar3 = FUN_004dc240()) != 0))) {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_buttonsound_0052a68c, 1, 0, 0xff);
      }
      FUN_004ba3c0();
      DAT_0052c000 = 0;
      FUN_004dcb30();
      FUN_004dc7e0();
      FUN_004dc1d0();
      FUN_004b4bf0(s_backconfig_0052a6f4);
      FUN_004dc170();
      FUN_004d9d40();
      if (DAT_00b69e30 == 4) {
        DAT_00aa6af0 = 0x59;
        DAT_00aa6af4 = (unsigned int)DAT_00bb7394;
        DAT_00aa6af8 = (unsigned int)DAT_00bb7395;
        DAT_00aa6afc = (unsigned int)DAT_00bb7396;
        DAT_00aa6b00 = (unsigned int)DAT_00bb7397;
        DAT_00aa6b04 = (unsigned int)DAT_00bb7398;
        DAT_00aa6b08 = (unsigned int)DAT_00bb7399;
        DAT_00aa6b0c = (unsigned int)DAT_00bb739a;
        DAT_00aa6b10 = (unsigned int)DAT_00bb739b;
        DAT_00aa6b14 = (unsigned int)DAT_00bb739c;
        DAT_00aa6b18 = (unsigned int)DAT_00bb739d;
        DAT_00aa6b1c = (unsigned int)DAT_00bb739e;
        DAT_00aa6b20 = (unsigned int)DAT_00bb739f;
        DAT_00aa6b24 = (unsigned int)_rand();
        DAT_00aa6b28 = (unsigned int)DAT_00bb736f;
        DAT_00aa6b2c = (unsigned int)DAT_00bb73a4;
        DAT_00aa6b30 = (unsigned int)DAT_00bb7370;
        DAT_00aa6b34 = (unsigned int)DAT_00bb73a5;
        DAT_00aa6b38 = (unsigned int)DAT_00bb73a6;
        FUN_004cef70(0, &DAT_00aa6af0, 0x4c);
      }
    }
  } else if ((param_1 < 3) || (3 < param_1)) {
    pcVar14 = s_buttonsound_0052a68c;
    iVar12 = 5;
    uVar11 = 0;
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(5);
    iVar10 = FUN_004da780(local_28, (char *)0, s_configdown_0052b3fc, pcVar4, iVar10, uVar11, iVar12, pcVar14);
    if (((iVar10 != 0) && (-1 < param_1)) && ((param_1 < 2 || (param_1 == 4)))) {
      FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
      FUN_004dc380((void *)FUN_004b7f30, local_10);
    }
  }
  if (param_1 < 2) {
    FUN_004d6460(local_28, 0x186, 4, 0x1f0, 0x2b);
    pcVar4 = FUN_004ddc10(3);
    FUN_004dad70(pcVar4);
    switch (DAT_00bb72d0) {
    case 0:
      if (DAT_00bb736f == 0) {
        uVar5 = 0x2d8;
      } else {
        uVar5 = 0x2d7;
      }
      break;
    case 1:
      if (DAT_00bb736f == 0) {
        uVar5 = 0x2da;
      } else {
        uVar5 = 0x2d9;
      }
      break;
    case 2:
      uVar5 = 0x2db;
      break;
    case 3:
      uVar5 = 0x2dc;
      break;
    default:
      uVar5 = 3;
    }
    pcVar4 = FUN_004ddc10(uVar5);
    uVar5 = 0xffffffff;
    do {
      pcVar14 = pcVar4;
      if (uVar5 == 0) break;
      uVar5 = uVar5 - 1;
      pcVar14 = pcVar4 + 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar14;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar4 = pcVar14 + -uVar5;
    pcVar14 = DAT_00b69d20;
    for (uVar8 = uVar5 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
      *(unsigned int *)pcVar14 = *(unsigned int *)pcVar4;
      pcVar4 = pcVar4 + 4;
      pcVar14 = pcVar14 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
      *pcVar14 = *pcVar4;
      pcVar4 = pcVar4 + 1;
      pcVar14 = pcVar14 + 1;
    }
    if (DAT_00b69e30 == 3) {
      FUN_004dad80();
      FUN_004daa20(local_28, s_joinindown_0052b3f0, DAT_00b69d20, 0xc);
    } else {
      iVar10 = FUN_004da780(local_28, (char *)0, s_joinindown_0052b3f0, DAT_00b69d20, 0xc, 0, 3, s_buttonsound_0052a68c);
      if (iVar10 != 0) {
        if (DAT_00aa613c != 0) {
          if (DAT_00b69e30 == 2) {
            if (DAT_00b6a606 != 1) goto LAB_004c0345;
            if (DAT_00b6a5aa == 1) {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x2a8);
              pcVar6 = FUN_004ddc10(0x2a7);
              uVar5 = 0x2a6;
            } else if (DAT_00b6a5aa == 3) {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x2ab);
              pcVar6 = FUN_004ddc10(0x2aa);
              uVar5 = 0x2a9;
            } else {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x30d);
              pcVar6 = FUN_004ddc10(0x30c);
              uVar5 = 0x30b;
            }
          } else {
            iVar10 = FUN_004cfee0();
            if (iVar10 == 0) {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x22d);
              pcVar6 = FUN_004ddc10(0x22c);
              uVar5 = 0x22b;
            } else {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x230);
              pcVar6 = FUN_004ddc10(0x22f);
              uVar5 = 0x22e;
            }
          }
          pcVar7 = FUN_004ddc10(uVar5);
          iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar15, pcVar14, pcVar4);
        }
LAB_004c0345:
        if (iVar10 != 0) {
          if ((char)DAT_00b6a2e0 == '\0') {
            pcVar7 = (char *)0;
            pcVar6 = (char *)0;
            pcVar4 = FUN_004ddc10(0x20e);
            pcVar14 = FUN_004ddc10(0x20d);
            pcVar15 = FUN_004ddc10(0x20c);
            FUN_004dcb90(pcVar15, pcVar14, pcVar4, pcVar6, pcVar7);
          } else {
            DAT_00aa6144 = 0;
            DAT_00b6a2a8 = 0;
            if (param_1 == 1) {
              bVar2 = 1;
              DAT_00b69e30 = 3;
              FUN_004dc330((void *)FUN_004d73c0, (void *)LAB_004d7370);
            } else {
              FUN_004cd9d0();
              DAT_00b69e30 = 3;
              FUN_004dc330((void *)FUN_004d73c0, (void *)LAB_004d7370);
            }
          }
        }
      }
    }
    if (DAT_00b6a250 != 0) {
      FUN_004d6460(local_28, 0x101, 4, 0x185, 0x2a);
      pcVar4 = FUN_004ddc10(4);
      FUN_004dad70(pcVar4);
      switch (DAT_00bb72d0) {
      case 0:
        if (DAT_00bb736f == 0) {
          uVar5 = 0x2d2;
        } else {
          uVar5 = 0x2d1;
        }
        break;
      case 1:
        if (DAT_00bb736f == 0) {
          uVar5 = 0x2d4;
        } else {
          uVar5 = 0x2d3;
        }
        break;
      case 2:
        uVar5 = 0x2d5;
        break;
      case 3:
        uVar5 = 0x2d6;
        break;
      default:
        uVar5 = 4;
      }
      pcVar4 = FUN_004ddc10(uVar5);
      uVar5 = 0xffffffff;
      do {
        pcVar14 = pcVar4;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar14 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar14;
      } while (cVar1 != '\0');
      uVar5 = ~uVar5;
      pcVar4 = pcVar14 + -uVar5;
      pcVar14 = DAT_00b69d20;
      for (uVar8 = uVar5 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
        *(unsigned int *)pcVar14 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar14 = pcVar14 + 4;
      }
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar14 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar14 = pcVar14 + 1;
      }
      if (DAT_00b69e30 == 4) {
        FUN_004dad80();
        FUN_004daa20(local_28, s_creategamedown_0052b3e0, DAT_00b69d20, 0xc);
      } else {
        iVar10 = FUN_004da780(local_28, (char *)0, s_creategamedown_0052b3e0, DAT_00b69d20, 0xc, 0, 4, s_buttonsound_0052a68c);
        if (iVar10 != 0) {
          if (DAT_00aa613c != 0) {
            if (DAT_00b69e30 == 2) {
              if (DAT_00b6a606 != 1) goto LAB_004c0672;
              if (DAT_00b6a5aa == 1) {
                pcVar4 = FUN_004ddc10(0x13);
                pcVar14 = FUN_004ddc10(0x20b);
                pcVar15 = FUN_004ddc10(0x2a8);
                pcVar6 = FUN_004ddc10(0x2a7);
                uVar5 = 0x2a6;
              } else if (DAT_00b6a5aa == 3) {
                pcVar4 = FUN_004ddc10(0x13);
                pcVar14 = FUN_004ddc10(0x20b);
                pcVar15 = FUN_004ddc10(0x2ab);
                pcVar6 = FUN_004ddc10(0x2aa);
                uVar5 = 0x2a9;
              } else {
                pcVar4 = FUN_004ddc10(0x13);
                pcVar14 = FUN_004ddc10(0x20b);
                pcVar15 = FUN_004ddc10(0x30d);
                pcVar6 = FUN_004ddc10(0x30c);
                uVar5 = 0x30b;
              }
            } else {
              iVar10 = FUN_004cfee0();
              if (iVar10 == 0) {
                pcVar4 = FUN_004ddc10(0x13);
                pcVar14 = FUN_004ddc10(0x20b);
                pcVar15 = FUN_004ddc10(0x22d);
                pcVar6 = FUN_004ddc10(0x22c);
                uVar5 = 0x22b;
              } else {
                pcVar4 = FUN_004ddc10(0x13);
                pcVar14 = FUN_004ddc10(0x20b);
                pcVar15 = FUN_004ddc10(0x230);
                pcVar6 = FUN_004ddc10(0x22f);
                uVar5 = 0x22e;
              }
            }
            pcVar7 = FUN_004ddc10(uVar5);
            iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar15, pcVar14, pcVar4);
          }
LAB_004c0672:
          if (iVar10 != 0) {
            if ((char)DAT_00b6a2e0 == '\0') {
              pcVar7 = (char *)0;
              pcVar6 = (char *)0;
              pcVar4 = FUN_004ddc10(0x20e);
              pcVar14 = FUN_004ddc10(0x20d);
              pcVar15 = FUN_004ddc10(0x20c);
              FUN_004dcb90(pcVar15, pcVar14, pcVar4, pcVar6, pcVar7);
            } else if (param_1 == 1) {
              bVar2 = 1;
              DAT_00b69e30 = 4;
              FUN_004dc330((void *)FUN_004dfd30, (void *)LAB_004dfd10);
            } else {
              FUN_004cd9d0();
              DAT_00b69e30 = 4;
              FUN_004dc330((void *)FUN_004dfd30, (void *)LAB_004dfd10);
            }
          }
        }
      }
      FUN_004d6460(local_28, 0x99, 4, 0xfc, 0x2b);
      pcVar4 = FUN_004ddc10(2);
      FUN_004dad70(pcVar4);
      if (DAT_00b69e30 == 2) {
        FUN_004dad80();
        iVar10 = 0xc;
        pcVar4 = FUN_004ddc10(2);
        FUN_004daa20(local_28, s_flysolodown_0052b3d4, pcVar4, iVar10);
      } else {
        pcVar14 = s_buttonsound_0052a68c;
        iVar12 = 2;
        uVar11 = 0;
        iVar10 = 0xc;
        pcVar4 = FUN_004ddc10(2);
        iVar10 = FUN_004da780(local_28, DAT_0052b3cc, s_flysolodown_0052b3d4, pcVar4, iVar10, uVar11, iVar12, pcVar14);
        if (iVar10 != 0) {
          if ((DAT_00aa613c != 0) && (DAT_00b69e30 != 2)) {
            iVar10 = FUN_004cfee0();
            if (iVar10 == 0) {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x22d);
              pcVar6 = FUN_004ddc10(0x22c);
              uVar5 = 0x22b;
            } else {
              pcVar4 = FUN_004ddc10(0x13);
              pcVar14 = FUN_004ddc10(0x20b);
              pcVar15 = FUN_004ddc10(0x230);
              pcVar6 = FUN_004ddc10(0x22f);
              uVar5 = 0x22e;
            }
            pcVar7 = FUN_004ddc10(uVar5);
            iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar15, pcVar14, pcVar4);
          }
          if (iVar10 != 0) {
            if ((char)DAT_00b6a2e0 == '\0') {
              pcVar7 = (char *)0;
              pcVar6 = (char *)0;
              pcVar4 = FUN_004ddc10(0x20e);
              pcVar14 = FUN_004ddc10(0x20d);
              pcVar15 = FUN_004ddc10(0x20c);
              FUN_004dcb90(pcVar15, pcVar14, pcVar4, pcVar6, pcVar7);
            } else {
              DAT_00aa6144 = 0;
              DAT_00b6a2a8 = 0;
              if (param_1 == 1) {
                bVar2 = 1;
                DAT_00b69e30 = 2;
                FUN_004dc330((void *)FUN_004e15d0, (void *)LAB_004e1470);
              } else {
                FUN_004cd9d0();
                puVar9 = DAT_00aa6150;
                for (iVar10 = 0x54; iVar10 != 0; iVar10 = iVar10 + -1) {
                  *puVar9 = 0;
                  puVar9 = puVar9 + 1;
                }
                DAT_00b69e30 = 2;
                FUN_004dc330((void *)FUN_004e15d0, (void *)LAB_004e1470);
              }
            }
          }
        }
      }
    }
  }
  FUN_004d6460(local_28, 0x3f, 4, 0x93, 0x39);
  pcVar4 = FUN_004ddc10(0x236);
  FUN_004dad70(pcVar4);
  if (param_1 == 3) {
    FUN_004dad80();
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0x23b);
    FUN_004daa20(local_28, s_reviewcraftdown_0052b3bc, pcVar4, iVar10);
    iVar10 = FUN_004d6b90(local_28, local_14, local_18);
    if ((iVar10 != 0) &&
        (((uVar3 = FUN_004dc220()) != 0) || ((uVar3 = FUN_004dc240()) != 0))) {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_buttonsound_0052a68c, 1, 0, 0xff);
      }
      DAT_0052c000 = 0;
      FUN_004dcb30();
      FUN_004dc7e0();
      FUN_004dc1d0();
      FUN_004b4bf0(s_backreview_0052b3b0);
      if (DAT_00aa60d0 == 0) {
        if (DAT_00aa60f4 != (void *)0) {
          _free(DAT_00aa60f4);
          DAT_00aa60f4 = (void *)0;
        }
      } else {
        FUN_0042b1b0();
      }
      if (DAT_00aa6110 != (void *)0) {
        _free(DAT_00aa6110);
        DAT_00aa6110 = (void *)0;
      }
      FUN_004dc170();
    }
  } else {
    if (1 < param_1) goto LAB_004c0d3a;
    pcVar14 = s_buttonsound_0052a68c;
    iVar12 = 1;
    uVar11 = 0;
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(1);
    iVar10 = FUN_004da780(local_28, (char *)0, s_reviewcraftdown_0052b3bc, pcVar4, iVar10, uVar11, iVar12, pcVar14);
    if (iVar10 != 0) {
      FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
      FUN_004dc380((void *)FUN_004e96b0, local_10);
    }
  }
  if (1 < param_1) goto LAB_004c0d3a;
  FUN_004d6460(local_28, 9, 4, 0x3e, 0x47);
  pcVar4 = FUN_004ddc10(0x235);
  FUN_004dad70(pcVar4);
  if (DAT_00b69e30 == 0) {
    FUN_004dad80();
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0);
    FUN_004daa20(local_28, s_pilotregdown_0052b3a0, pcVar4, iVar10);
  } else {
    pcVar14 = s_buttonsound_0052a68c;
    iVar12 = 0;
    uVar11 = 0;
    iVar10 = 0xc;
    pcVar4 = FUN_004ddc10(0);
    iVar10 = FUN_004da780(local_28, (char *)0, s_pilotregdown_0052b3a0, pcVar4, iVar10, uVar11, iVar12, pcVar14);
    if (iVar10 != 0) {
      if (DAT_00aa613c != 0) {
        if (DAT_00b69e30 == 2) {
          if (DAT_00b6a606 != 1) goto LAB_004c0c9c;
          if (DAT_00b6a5aa == 1) {
            pcVar4 = FUN_004ddc10(0x13);
            pcVar14 = FUN_004ddc10(0x20b);
            pcVar15 = FUN_004ddc10(0x2a8);
            pcVar6 = FUN_004ddc10(0x2a7);
            uVar5 = 0x2a6;
          } else if (DAT_00b6a5aa == 3) {
            pcVar4 = FUN_004ddc10(0x13);
            pcVar14 = FUN_004ddc10(0x20b);
            pcVar15 = FUN_004ddc10(0x2ab);
            pcVar6 = FUN_004ddc10(0x2aa);
            uVar5 = 0x2a9;
          } else {
            pcVar4 = FUN_004ddc10(0x13);
            pcVar14 = FUN_004ddc10(0x20b);
            pcVar15 = FUN_004ddc10(0x30d);
            pcVar6 = FUN_004ddc10(0x30c);
            uVar5 = 0x30b;
          }
        } else {
          iVar10 = FUN_004cfee0();
          if (iVar10 == 0) {
            pcVar4 = FUN_004ddc10(0x13);
            pcVar14 = FUN_004ddc10(0x20b);
            pcVar15 = FUN_004ddc10(0x22d);
            pcVar6 = FUN_004ddc10(0x22c);
            uVar5 = 0x22b;
          } else {
            pcVar4 = FUN_004ddc10(0x13);
            pcVar14 = FUN_004ddc10(0x20b);
            pcVar15 = FUN_004ddc10(0x230);
            pcVar6 = FUN_004ddc10(0x22f);
            uVar5 = 0x22e;
          }
        }
        pcVar7 = FUN_004ddc10(uVar5);
        iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar15, pcVar14, pcVar4);
      }
LAB_004c0c9c:
      if (iVar10 != 0) {
        DAT_00aa6144 = 0;
        DAT_00b6a2a8 = 0;
        if (param_1 == 1) {
          bVar2 = 1;
        } else {
          FUN_004cd9d0();
        }
        DAT_00b69e30 = 0;
        FUN_004dc330((void *)FUN_004be1b0, (void *)LAB_004be050);
      }
    }
  }
  if ((param_1 == 1) && (bVar2)) {
    iVar10 = FUN_004cfee0();
    if (iVar10 == 0) {
      DAT_00aa6af0 = 0x47;
      uVar5 = 4;
      puVar13 = &DAT_00aa6af0;
      iVar10 = FUN_004d11d0();
    } else {
      DAT_00aa6af0 = 0x46;
      uVar5 = 4;
      puVar13 = &DAT_00aa6af0;
      iVar10 = 0;
    }
    FUN_004cef70(iVar10, puVar13, uVar5);
    FUN_004cd9d0();
    puVar9 = DAT_00aa6150;
    for (iVar10 = 0x54; iVar10 != 0; iVar10 = iVar10 + -1) {
      *puVar9 = 0;
      puVar9 = puVar9 + 1;
    }
  }
LAB_004c0d3a:
  FUN_004dad60();
  return 0;
}
