/* SPDX-License-Identifier: MIT */
/* z_bop 004baaf0 */
extern int DAT_00664e9c;
extern int DAT_00664e98;
extern int DAT_00664e94;
extern int DAT_00664e90;
extern int DAT_00b69e30;
extern int DAT_0052c000;
extern int DAT_00bb72a0;
extern char DAT_00bb2820[];

extern unsigned char DAT_00bb7370;
extern unsigned char DAT_00bb736f;
extern unsigned char DAT_00bb72d0;
extern unsigned char DAT_00bb72b0, DAT_00bb72b1, DAT_00bb72b2, DAT_00bb72b3;
extern unsigned char DAT_00bb72b4, DAT_00bb72b5, DAT_00bb72b6, DAT_00bb72b7;
extern unsigned char DAT_00bb72b8, DAT_00bb72b9, DAT_00bb72ba, DAT_00bb72bb;
extern unsigned char DAT_00bb72bc, DAT_00bb72bd, DAT_00bb72be, DAT_00bb72bf;
extern unsigned char DAT_00bb72c0, DAT_00bb72c1, DAT_00bb72c2, DAT_00bb72c3;
extern unsigned char DAT_00bb72c4, DAT_00bb72c5, DAT_00bb72c6, DAT_00bb72c7;
extern unsigned char DAT_00bb72c8, DAT_00bb72c9, DAT_00bb72ca, DAT_00bb72cb;
extern unsigned char DAT_00bb72cc, DAT_00bb72cd, DAT_00bb72ce, DAT_00bb72cf;
extern unsigned char DAT_00bb7371, DAT_00bb7372, DAT_00bb7373, DAT_00bb73a7;
extern unsigned char DAT_00bb7374, DAT_00bb7375, DAT_00bb7376, DAT_00bb7377;
extern unsigned char DAT_00bb7378, DAT_00bb7379, DAT_00bb737a, DAT_00bb737b;
extern unsigned char DAT_00bb737c, DAT_00bb737d, DAT_00bb737e;
extern char DAT_00bb737f;
extern unsigned char DAT_00bb7380[20];
extern char DAT_00bb73a9[];
extern char DAT_00bb73ef[];
extern char DAT_00bb7435[];
extern char DAT_00bb747b[];

extern char s_jewelsound_0052ad74[];
extern char s_config6u_0052ad5c[];
extern char s_config6d_0052ad68[];
extern char s_config8d_0052ad50[];
extern char s_config8u_0052ad44[];
extern char s_config5d_0052ad38[];
extern char s_config5u_0052ad2c[];
extern char s_config4d_0052ad20[];
extern char s_config4u_0052ad14[];
extern char s_config3d_0052ad08[];
extern char s_config3u_0052acfc[];
extern char s_config2d_0052acf0[];
extern char s_config2u_0052ace4[];
extern char s_config1d_0052acd8[];
extern char s_config1u_0052accc[];
extern char s_backconfig_0052a6f4[];
extern char s_frame_0052a6d4[];
extern char s_alloff_0052a6cc[];
extern char s_configoverlay_0052a6bc[];

void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004de090(int *param_1, int *param_2);
int FUN_004d6b90(int *param_1, int param_2, int param_3);
unsigned int FUN_004dc1e0(void);
unsigned int FUN_004dc200(void);
unsigned int FUN_004dc220(void);
unsigned int FUN_004dc240(void);
void FUN_004daba0(int *param_1);
char *FUN_004ddc10(int param_1);
int FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4, int param_5,
                 unsigned int param_6, int param_7, char *param_8);
int FUN_004dcb90(char *param_1, char *param_2, char *param_3, char *param_4, char *param_5);
unsigned char FUN_004d5e60(void);
void FUN_004d34a0(int param_1);
void FUN_004d3020(int param_1, int param_2, int param_3);
unsigned int FUN_004d6300(int param_1);
void FUN_004daa20(int *param_1, char *param_2, char *param_3, int param_4);
void FUN_004d4e20(void);
void FUN_004b5de0(char *param_1, int param_2, int param_3);
void FUN_004b5570(char *param_1, int param_2, int param_3);
void FUN_004b4d40(char *param_1, int param_2, int param_3);
void FUN_004d4ec0(int param_1);
unsigned int FUN_004d6e30(int param_1, int param_2, int param_3);
void FUN_004d6550(int *param_1, int param_2, int param_3, unsigned int param_4);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004dcb30(void);

unsigned int __stdcall FUN_004baaf0(void)
{
  char cVar1;
  unsigned char uVar2;
  int iVar3;
  unsigned int extraout_var;
  unsigned int extraout_var_00;
  unsigned int extraout_var_01;
  unsigned int extraout_var_02;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  char *pcVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  int iVar11;
  char *pcVar12;
  unsigned int uEax;
  int local_38[4];
  int local_28[9];
  int local_4;

  local_28[0] = 1;
  local_28[1] = 1;
  local_28[2] = 1;
  local_28[3] = 1;
  local_28[4] = 1;
  if (DAT_00664e9c == 0) {
    if ((DAT_00b69e30 == 4) || (DAT_00b69e30 == 3)) {
      local_28[5] = 0;
    }
    else {
      local_28[5] = 1;
    }
  }
  else {
    local_28[5] = 1;
  }
  local_28[6] = 0;
  local_28[7] = 1;
  if (DAT_00664e9c == 5) {
    local_28[7] = 2;
  }
  else {
    local_28[DAT_00664e9c] = 2;
  }
  FUN_004d6460(local_38, 0x16, 0x132, 0x2a, 0x14a);
  FUN_004de090(&local_4, local_28 + 8);
  if (((local_28[5] != 0) && (iVar3 = FUN_004d6b90(local_38, local_4, local_28[8]), iVar3 != 0)) &&
      ((uEax = FUN_004dc1e0(), uVar2 = (unsigned char)uEax, extraout_var = uEax >> 8,
        ((extraout_var << 8) | uVar2) != 0) ||
       ((((uEax = FUN_004dc200(), uVar2 = (unsigned char)uEax, extraout_var_00 = uEax >> 8,
           ((extraout_var_00 << 8) | uVar2) != 0) ||
          (uEax = FUN_004dc220(), uVar2 = (unsigned char)uEax, extraout_var_01 = uEax >> 8,
           ((extraout_var_01 << 8) | uVar2) != 0)) ||
         (uEax = FUN_004dc240(), uVar2 = (unsigned char)uEax, extraout_var_02 = uEax >> 8,
          ((extraout_var_02 << 8) | uVar2) != 0))))) {
    local_28[5] = 2;
  }
  FUN_004daba0(local_28);
  if (local_28[5] != 0) {
    FUN_004d6460(local_38, 0x16, 0x132, 0x2a, 0x14a);
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0x10;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x1a4);
    iVar3 = FUN_004da780(local_38, s_config6u_0052ad5c, s_config6d_0052ad68, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      pcVar4 = FUN_004ddc10(0x13);
      pcVar12 = FUN_004ddc10(0x20b);
      pcVar5 = FUN_004ddc10(0x2a2);
      pcVar6 = FUN_004ddc10(0x2a1);
      pcVar7 = FUN_004ddc10(0x2a0);
      iVar3 = FUN_004dcb90(pcVar7, pcVar6, pcVar5, pcVar12, pcVar4);
      cVar1 = DAT_00bb737f;
      if (iVar3 != 0) {
        switch (DAT_00664e9c) {
        case 0:
          DAT_00bb7370 = 8;
          DAT_00bb72d0 = 0;
          DAT_00bb736f = 0;
          break;
        case 1:
          DAT_00bb72b0 = 1;
          DAT_00bb72b2 = 2;
          DAT_00bb72b4 = 1;
          DAT_00bb72b6 = 1;
          DAT_00bb72b8 = 1;
          DAT_00bb72ba = 1;
          DAT_00bb72bc = 1;
          DAT_00bb72be = 2;
          DAT_00bb72c0 = 10;
          DAT_00bb72c2 = 10;
          DAT_00bb72c4 = 2;
          DAT_00bb72c6 = 2;
          DAT_00bb72ca = 2;
          DAT_00bb72c8 = 0;
          DAT_00bb72cc = FUN_004d5e60();
          DAT_00bb72ce = 1;
          break;
        case 2:
          DAT_00bb72b1 = 1;
          DAT_00bb72b3 = 2;
          DAT_00bb72b5 = 1;
          DAT_00bb72b7 = 1;
          DAT_00bb72b9 = 1;
          DAT_00bb72bb = 1;
          DAT_00bb72bd = 1;
          DAT_00bb72bf = 2;
          DAT_00bb72c1 = 10;
          DAT_00bb72c3 = 10;
          DAT_00bb72c5 = 2;
          DAT_00bb72c7 = 2;
          DAT_00bb72c9 = 0;
          DAT_00bb72cb = 2;
          DAT_00bb72cd = 0;
          DAT_00bb72cf = 1;
          break;
        case 3:
          DAT_00bb7371 = 1;
          DAT_00bb7372 = 1;
          DAT_00bb7373 = 1;
          DAT_00bb73a7 = 1;
          DAT_00bb7374 = 2;
          DAT_00bb7375 = 2;
          DAT_00bb7376 = 1;
          DAT_00bb7377 = 1;
          DAT_00bb7378 = 1;
          DAT_00bb737f = '\x01';
          DAT_00bb7379 = 9;
          DAT_00bb737a = 9;
          DAT_00bb737b = 9;
          DAT_00bb737c = 9;
          DAT_00bb737d = 9;
          DAT_00bb737e = 5;
          if (cVar1 != '\x01') {
            FUN_004d34a0(0x8e38);
            FUN_004d3020(7, 0, 0);
          }
          break;
        case 4:
          iVar3 = 0;
          *(unsigned int *)&DAT_00bb7380[0] = 0;
          *(unsigned int *)&DAT_00bb7380[4] = 0;
          *(unsigned int *)&DAT_00bb7380[8] = 0;
          *(unsigned int *)&DAT_00bb7380[12] = 0;
          *(unsigned int *)&DAT_00bb7380[16] = 0;
          uEax = FUN_004d6300(0);
          if ((uEax != 0) && (-1 < (int)uEax)) {
            do {
              if (0xf < iVar3) break;
              switch (iVar3) {
              case 0:
                DAT_00bb7380[iVar3] = 0x9c;
                break;
              case 1:
                DAT_00bb7380[iVar3] = 0x9d;
                break;
              case 2:
                DAT_00bb7380[iVar3] = 0x72;
                break;
              case 3:
                DAT_00bb7380[iVar3] = 0x2e;
                break;
              case 4:
                DAT_00bb7380[iVar3] = 0x65;
                break;
              case 5:
                DAT_00bb7380[iVar3] = 0x69;
                break;
              case 6:
                DAT_00bb7380[iVar3] = 0x5b;
                break;
              case 7:
                DAT_00bb7380[iVar3] = 8;
                break;
              case 8:
                DAT_00bb7380[iVar3] = 0xd;
                break;
              case 9:
                DAT_00bb7380[iVar3] = 0x5d;
              }
              iVar3 = iVar3 + 1;
            } while (iVar3 < (int)uEax);
          }
          iVar3 = 0;
          if (0 < DAT_00bb72a0) {
            pcVar4 = DAT_00bb2820;
            do {
              if (*pcVar4 == (char)DAT_00bb7380[DAT_00664e90]) {
                DAT_00664e94 = iVar3;
              }
              pcVar4 = pcVar4 + 0x95;
              iVar3 = iVar3 + 1;
            } while (iVar3 < DAT_00bb72a0);
          }
          break;
        case 5:
          pcVar4 = FUN_004ddc10(0x316);
          uVar8 = 0xffffffff;
          do {
            pcVar12 = pcVar4;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar12 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar12;
          } while (cVar1 != '\0');
          uVar8 = ~uVar8;
          pcVar4 = pcVar12 + -uVar8;
          pcVar12 = DAT_00bb73a9;
          for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar12, pcVar4, 4);
            pcVar4 = pcVar4 + 4;
            pcVar12 = pcVar12 + 4;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar12 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar12 = pcVar12 + 1;
          }
          pcVar4 = FUN_004ddc10(0x317);
          uVar8 = 0xffffffff;
          do {
            pcVar12 = pcVar4;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar12 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar12;
          } while (cVar1 != '\0');
          uVar8 = ~uVar8;
          pcVar4 = pcVar12 + -uVar8;
          pcVar12 = DAT_00bb73ef;
          for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar12, pcVar4, 4);
            pcVar4 = pcVar4 + 4;
            pcVar12 = pcVar12 + 4;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar12 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar12 = pcVar12 + 1;
          }
          pcVar4 = FUN_004ddc10(0x318);
          uVar8 = 0xffffffff;
          do {
            pcVar12 = pcVar4;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar12 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar12;
          } while (cVar1 != '\0');
          uVar8 = ~uVar8;
          pcVar4 = pcVar12 + -uVar8;
          pcVar12 = DAT_00bb7435;
          for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar12, pcVar4, 4);
            pcVar4 = pcVar4 + 4;
            pcVar12 = pcVar12 + 4;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar12 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar12 = pcVar12 + 1;
          }
          pcVar4 = FUN_004ddc10(0x319);
          uVar8 = 0xffffffff;
          do {
            pcVar12 = pcVar4;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar12 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar12;
          } while (cVar1 != '\0');
          uVar8 = ~uVar8;
          pcVar4 = pcVar12 + -uVar8;
          pcVar12 = DAT_00bb747b;
          for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar12, pcVar4, 4);
            pcVar4 = pcVar4 + 4;
            pcVar12 = pcVar12 + 4;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar12 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar12 = pcVar12 + 1;
          }
        }
      }
    }
  }
  FUN_004d6460(local_38, 0x16, 0xfe, 0x2a, 0x116);
  if (DAT_00664e9c == 5) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x31a);
    FUN_004daa20(local_38, s_config8d_0052ad50, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0x12;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x31a);
    iVar3 = FUN_004da780(local_38, s_config8u_0052ad44, s_config8u_0052ad44, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      DAT_00664e98 = 1;
      DAT_00664e9c = 5;
      DAT_0052c000 = 0;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d4ec0(1);
    }
  }
  FUN_004d6460(local_38, 0x16, 0xe2, 0x2a, 0xfa);
  if (DAT_00664e9c == 4) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x19f);
    FUN_004daa20(local_38, s_config5d_0052ad38, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xf;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x19f);
    iVar3 = FUN_004da780(local_38, s_config5u_0052ad2c, s_config5u_0052ad2c, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      DAT_00664e98 = 1;
      DAT_00664e9c = 4;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d6460(local_38, 0x14a, 0x6f, 0x24e, 0x19f);
      uVar8 = FUN_004d6e30(0, 0, 0x40);
      FUN_004d6550(local_38, 0, 0, uVar8);
      FUN_004d4ec0(1);
    }
  }
  FUN_004d64a0(local_38, 0, -0x1c);
  if (DAT_00664e9c == 3) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x136);
    FUN_004daa20(local_38, s_config4d_0052ad20, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xe;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x136);
    iVar3 = FUN_004da780(local_38, s_config4u_0052ad14, s_config4u_0052ad14, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      DAT_00664e98 = 1;
      DAT_00664e9c = 3;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d4ec0(1);
    }
  }
  FUN_004d64a0(local_38, 0, -0x1c);
  if (DAT_00664e9c == 2) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x281);
    FUN_004daa20(local_38, s_config3d_0052ad08, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xd;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x281);
    iVar3 = FUN_004da780(local_38, s_config3u_0052acfc, s_config3u_0052acfc, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      DAT_00664e98 = 1;
      DAT_00664e9c = 2;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d4ec0(1);
    }
  }
  FUN_004d64a0(local_38, 0, -0x1c);
  if (DAT_00664e9c == 1) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0xdb);
    FUN_004daa20(local_38, s_config2d_0052acf0, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xc;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0xdb);
    iVar3 = FUN_004da780(local_38, s_config2u_0052ace4, s_config2u_0052ace4, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      DAT_00664e98 = 1;
      DAT_00664e9c = 1;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d4ec0(1);
    }
  }
  FUN_004d64a0(local_38, 0, -0x1c);
  if (DAT_00664e9c == 0) {
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x130);
    FUN_004daa20(local_38, s_config1d_0052acd8, pcVar4, iVar3);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xb;
    uVar10 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004ddc10(0x130);
    iVar3 = FUN_004da780(local_38, s_config1u_0052accc, s_config1u_0052accc, pcVar4, iVar3,
                         uVar10, iVar11, pcVar12);
    if (iVar3 != 0) {
      FUN_004dcb30();
      DAT_00664e9c = 0;
      DAT_0052c000 = 0;
      DAT_00664e98 = 1;
      FUN_004d4e20();
      FUN_004b5de0(s_backconfig_0052a6f4, 0, 0);
      FUN_004b5570(s_frame_0052a6d4, 0, 0);
      FUN_004b5570(s_alloff_0052a6cc, 0, 0);
      FUN_004d6460(local_38, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
      FUN_004d4ec0(1);
    }
  }
  return 1;
}
