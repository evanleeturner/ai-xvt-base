/* SPDX-License-Identifier: MIT */
/* z_xvt 004d66c0 */
/* Prototypes */
typedef unsigned int uint;
void __cdecl FUN_004bef20(void);
void LAB_004bee00(void);
void __cdecl FUN_004b8a50(void);
void __cdecl FUN_004d7150(void);
void __cdecl FUN_004db670(void *, void *);
uint __cdecl FUN_004d7e90(uint);
char * __cdecl FUN_004dcfa0(int);
int __cdecl FUN_004dbf00(void);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);
void __cdecl FUN_004db6c0(void *, int *);
void __cdecl FUN_004ddc90(int, int);
void __cdecl FUN_004cde20(void);
void * __cdecl _malloc(uint);
void __cdecl FUN_004d0290(char *, char *);
void __cdecl FUN_004d52b0(void);
void __cdecl FUN_004d1750(char *, int, int);
void __cdecl FUN_004d0e60(char *, int, int);
void __cdecl FUN_004d0580(char *, int, int);
void __cdecl FUN_004d5350(int);
void __cdecl FUN_004db490(int);
void __cdecl FUN_004da920(int, char *, int *, int);
void __cdecl FUN_004d8520(void);
int __cdecl FUN_004e0750(void);
void __cdecl FUN_004ee3d0(void);
void __cdecl FUN_004e43d0(int);
void __cdecl FUN_004e4820(unsigned char *);
void __cdecl FUN_004ca200(void);
int __cdecl FUN_004d6320(uint);
int __cdecl FUN_004d8a70(int);
void __cdecl FUN_004d8900(void);
void __cdecl FUN_004db650(void);
void __cdecl FUN_004d77b0(void);
void __cdecl FUN_004d75c0(void);
void __cdecl FUN_004d7900(uint);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004d83f0(void);
int __cdecl FUN_004c0750(int);
void __cdecl FUN_004da030(void);
void __cdecl FUN_004da060(char *);
int __cdecl FUN_004d9a80(int *, char *, char *, char *, int, int, int, char *);
int __cdecl FUN_004cdbb0(void);
void __cdecl FUN_004cb830(int, uint *, int);
void __cdecl FUN_004da050(void);

/* Globals */
extern int DAT_0081b70c;
extern char DAT_0085e6f0;
extern int DAT_0053308c;
extern int DAT_0081b850;
extern unsigned char DAT_0085e731;
extern unsigned char DAT_0085e6f1;
extern uint DAT_00757ce0[];
extern uint DAT_0064f658[];
extern uint *DAT_0081b700;
extern int DAT_0081bcd4;
extern uint DAT_00757e40[];
extern int DAT_00758824;
extern int DAT_005330fc;
extern int DAT_00757cd0;
extern int DAT_00758828;
extern uint DAT_00758618;
extern int DAT_00532a34;
extern int DAT_0081bcb8;
extern unsigned char *DAT_00757cc4;
extern char s_frontres_joinback_bmp_00532a8c[];
extern char s_background_00532a4c[];
extern char s_frame_00531358[];
extern int DAT_0081bc68;
extern char s_clientactive_00531eac[];
extern char s_allactive_00531ebc[];
extern char s_chatbox_00532a84[];
extern char s_joinoverlay_00532a78[];
extern unsigned char DAT_00757e70[];   /* player entry +0x30, stride 0x3b */
extern unsigned char DAT_00757e74[];   /* player entry +0x34 */
extern unsigned char DAT_00757e78[];   /* player entry +0x38 */
extern unsigned char DAT_00757e79[];   /* player entry +0x39 */
extern int DAT_007585a0;
extern unsigned char DAT_007585a4;
extern unsigned char DAT_0081bfba[];   /* unaligned int */
extern unsigned char DAT_0081bfbe[];   /* unaligned int array */
extern unsigned char *DAT_00757cc0;
extern int DAT_00757cc8;
extern int DAT_00757ccc;
extern int DAT_00758820;
extern char DAT_0081b740[];
extern char s_v___d__d_00531e88[];
extern char DAT_0081bcf0;
extern char DAT_0081e082[];
extern char s__c_s__c_s_00531334[];
extern int DAT_0081bcd0;
extern char s_rebtiny_d_00531328[];
extern char s_imptiny_d_0053131c[];
extern char DAT_0085e7c6;
extern char s_buttonsound_00531310[];
extern char s_leaveup_005312fc[];
extern char s_leavedown_00531304[];
extern char s_flysound_00532a6c[];
extern char s_nextup_00532a58[];
extern char s_nextdown_00532a60[];
extern int DAT_00757cd8;
extern char DAT_0081bfd2[];
extern uint DAT_00758620;
extern int DAT_00758624;
extern unsigned char DAT_00758628[];
extern unsigned char DAT_0075862c[];
extern unsigned char DAT_00758630[];
extern unsigned char DAT_00758634[];
extern unsigned char DAT_0085e77f[];   /* unaligned */
extern unsigned char DAT_0085e783[];
extern unsigned char DAT_0085e787[];
extern unsigned char DAT_0085e78b[];

uint __cdecl FUN_004d66c0(uint param_1)
{
  char cVar1;
  uint uVar2;
  int iVar3;
  DWORD DVar4;
  uint uVar5;
  uint uVar6;
  int *piVar7;
  uint *puVar8;
  unsigned char *pbVar9;
  char *pcVar10;
  int iVar11;
  char *pcVar12;
  int local_20[4];
  int local_10[4];
  int tmp;

  DAT_0081b70c = 0;
  if (DAT_0085e6f0 != '\0') {
    if (DAT_0053308c != 0) {
      DAT_0081b850 = 0;
      FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
      return 0;
    }
    if (DAT_0081b850 == 3) {
      if (DAT_0085e6f0 == '\x01') {
        if (DAT_0085e731 == '\0') {
          FUN_004dcfa0(0x13);
          FUN_004dcfa0(0x1fd);
          FUN_004dcfa0(0x2a9);
          FUN_004dcfa0(0x2a8);
          FUN_004dcfa0(0x2a7);
          iVar3 = FUN_004dbf00();
          if (iVar3 == 0) {
            DAT_0081b850 = 0;
            FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
            return 0;
          }
          FUN_004dcfd0(local_10, 0, 0, 0x280, 0x1e0);
          FUN_004db6c0((void *)FUN_004b8a50, local_10);
          return 0;
        }
      }
      else if ((DAT_0085e6f0 == '\x02') && (DAT_0085e6f1 == '\0')) {
        FUN_004dcfa0(0x13);
        FUN_004dcfa0(0x1fd);
        FUN_004dcfa0(0x2ac);
        FUN_004dcfa0(0x2ab);
        FUN_004dcfa0(0x2aa);
        iVar3 = FUN_004dbf00();
        if (iVar3 == 0) {
          DAT_0081b850 = 0;
          FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
          return 0;
        }
        FUN_004dcfd0(local_10, 0, 0, 0x280, 0x1e0);
        FUN_004db6c0((void *)FUN_004b8a50, local_10);
        return 0;
      }
    }
    uVar2 = FUN_004d7e90(param_1);
    return uVar2;
  }
  if (param_1 == 0) {
    FUN_004ddc90(0x19f, 0x79);
    puVar8 = DAT_00757ce0;
    for (iVar3 = 0x54; iVar3 != 0; iVar3 = iVar3 + -1) {
      *puVar8 = 0;
      puVar8 = puVar8 + 1;
    }
    FUN_004cde20();
    puVar8 = DAT_0064f658;
    for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
      *puVar8 = 0;
      puVar8 = puVar8 + 1;
    }
    if (DAT_0081b700 != (uint *)0) {
      puVar8 = DAT_0081b700;
      for (iVar3 = 0x100; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar8 = 0;
        puVar8 = puVar8 + 1;
      }
      DAT_0081bcd4 = 0;
    }
    if (DAT_0053308c == 0) {
      puVar8 = DAT_00757e40;
      for (iVar3 = 0x1d8; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar8 = 0;
        puVar8 = puVar8 + 1;
      }
      DAT_00758824 = 0;
    }
    DAT_005330fc = 0;
    DAT_0081b70c = 0;
    DAT_00757cd0 = 0;
    DAT_00758828 = 0;
    DAT_00758618 = 0xffffffff;
    DAT_00532a34 = -1;
    DAT_0081bcb8 = 0;
    DAT_0053308c = 0;
    if (DAT_00757cc4 == (unsigned char *)0) {
      DAT_00757cc4 = (unsigned char *)_malloc(0x400);
    }
    FUN_004d0290(s_frontres_joinback_bmp_00532a8c, s_background_00532a4c);
    FUN_004d52b0();
    FUN_004d1750(s_background_00532a4c, 0, 0);
    FUN_004d0e60(s_frame_00531358, 0, 0);
    if (DAT_0081bc68 == 0) {
      pcVar10 = s_clientactive_00531eac;
    }
    else {
      pcVar10 = s_allactive_00531ebc;
    }
    FUN_004d0e60(pcVar10, 0, 0);
    FUN_004d0580(s_chatbox_00532a84, 0, 0);
    FUN_004d0580(s_joinoverlay_00532a78, 0, 0);
    FUN_004d5350(1);
    FUN_004db490(0x14);
  }
  FUN_004dcfd0(local_20, 0x9e, 0x34, 0x1eb, 0x44);
  FUN_004da920(0xc, (char *)DAT_0064f658, local_20, 0xffff);
  uVar6 = (uint)((int)param_1 >> 0x1f);
  if ((int)param_1 % 0xa0 == 0) {
    if (DAT_00532a34 == -1) {
      FUN_004d8520();
      if (DAT_00532a34 == -1) goto LAB_004d6cf7;
    }
  }
  else {
    if (DAT_00532a34 == -1) goto LAB_004d6cf7;
  }
  iVar3 = FUN_004e0750();
  if (iVar3 == 0x41) {
    DVar4 = GetTickCount();
    __builtin_memcpy(DAT_00757e74 + (uint)DAT_00532a34 * 0x3b, &DVar4, 4);
    iVar3 = DAT_00532a34;
    __builtin_memcpy(DAT_00757e70 + (uint)DAT_00532a34 * 0x3b, &DAT_007585a0, 4);
    DAT_00757e79[(uint)iVar3 * 0x3b] = DAT_007585a4;
    FUN_004ee3d0();
    __builtin_memcpy(DAT_0081bfba, &DAT_00758828, 4);
    __builtin_memcpy(DAT_0081bfbe + (uint)DAT_00758828 * 4, &DAT_00758618, 4);
    __builtin_memcpy(&tmp, DAT_0081bfba, 4);
    FUN_004e43d0(tmp);
    FUN_004e4820(DAT_00757cc4);
    if ((DAT_00757cc0 != (unsigned char *)0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
      piVar7 = (int *)(DAT_00757cc0 + 0x140);
      do {
        int a, b;
        __builtin_memcpy(&a, DAT_0081bfba, 4);
        __builtin_memcpy(&b, DAT_0081bfbe + (uint)a * 4, 4);
        if (*piVar7 == b) break;
        piVar7 = piVar7 + 0x51;
        DAT_00757cc8 = DAT_00757cc8 + 1;
      } while (DAT_00757cc8 < DAT_00757ccc);
    }
    DAT_005330fc = 0;
    goto LAB_004d6cf7;
  }
  if ((((iVar3 == 0x46) || (iVar3 == 0x49)) || (iVar3 == 0x4b)) ||
     (((iVar3 == 0x4c || (iVar3 == 0x5a)) || ((iVar3 == 0x66 || (iVar3 == 0x76)))))) {
    FUN_004ca200();
    FUN_004d8520();
    DAT_00758820 = 0;
    DAT_00532a34 = -1;
    DAT_00758618 = 0xffffffff;
  }
  else {
    if (iVar3 == 0x69) {
      FUN_004ca200();
      DAT_00758820 = 0;
      DAT_00532a34 = -1;
      DAT_00758618 = 0xffffffff;
      FUN_004d8520();
      goto LAB_004d6cdc;
    }
    if (((iVar3 != 0x44) && (iVar3 != 0x5e)) && ((iVar3 != 0x5f && (iVar3 != 0x60))))
      goto LAB_004d6cf7;
    switch (iVar3) {
    case 0x44:
      FUN_004dcfa0(0x213);
      FUN_004dcfa0(0x212);
      uVar5 = 0x211;
      break;
    default:
      goto switchD_004d6be2_caseD_45;
    case 0x5e:
      FUN_004dcfa0(0x216);
      FUN_004dcfa0(0x215);
      uVar5 = 0x214;
      break;
    case 0x5f:
      FUN_004dcfa0(0x219);
      FUN_004dcfa0(0x218);
      uVar5 = 0x217;
      break;
    case 0x60:
      FUN_004dcfa0(0x21c);
      FUN_004dcfa0(0x21b);
      uVar5 = 0x21a;
    }
    FUN_004dcfa0((int)uVar5);
    FUN_004dbf00();
switchD_004d6be2_caseD_45:
    FUN_004ca200();
    DAT_00758820 = 0;
    DAT_00532a34 = -1;
    DAT_00758618 = 0xffffffff;
    FUN_004d8520();
  }
LAB_004d6cdc:
  puVar8 = DAT_0064f658;
  for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar8 = 0;
    puVar8 = puVar8 + 1;
  }
  pbVar9 = DAT_00757cc4;
  for (iVar3 = 0x100; iVar3 != 0; iVar3 = iVar3 + -1) {
    pbVar9[0] = 0;
    pbVar9[1] = 0;
    pbVar9[2] = 0;
    pbVar9[3] = 0;
    pbVar9 = pbVar9 + 4;
  }
LAB_004d6cf7:
  iVar3 = FUN_004d6320(param_1);
  if (iVar3 != -1) {
    if (iVar3 == DAT_00532a34) {
      DAT_00532a34 = -1;
      DAT_00758618 = 0xffffffff;
      DAT_00758820 = 0;
      puVar8 = DAT_0064f658;
      for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar8 = 0;
        puVar8 = puVar8 + 1;
      }
      pbVar9 = DAT_00757cc4;
      for (iVar3 = 0x100; iVar3 != 0; iVar3 = iVar3 + -1) {
        pbVar9[0] = 0;
        pbVar9[1] = 0;
        pbVar9[2] = 0;
        pbVar9[3] = 0;
        pbVar9 = pbVar9 + 4;
      }
      FUN_004ca200();
    }
    else {
      DAT_00532a34 = iVar3;
      iVar3 = FUN_004d8a70(iVar3);
      if (iVar3 == 0) {
        FUN_004ca200();
        DAT_00758820 = 0;
        DAT_00532a34 = -1;
        DAT_00758618 = 0xffffffff;
        FUN_004d8520();
        puVar8 = DAT_0064f658;
        for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
          *puVar8 = 0;
          puVar8 = puVar8 + 1;
        }
        pbVar9 = DAT_00757cc4;
        for (iVar3 = 0x100; iVar3 != 0; iVar3 = iVar3 + -1) {
          pbVar9[0] = 0;
          pbVar9[1] = 0;
          pbVar9[2] = 0;
          pbVar9[3] = 0;
          pbVar9 = pbVar9 + 4;
        }
      }
      else {
        FUN_004d8900();
        {
          int v;
          __builtin_memcpy(&v, DAT_00757e70 + (uint)DAT_00532a34 * 0x3b, 4);
          if (v != 0) {
            FUN_004db650();
            FUN_004ddc90(0x25, 0x1bd);
          }
        }
      }
    }
  }
  FUN_004d77b0();
  FUN_004d75c0();
  if (DAT_00532a34 != -1) {
    FUN_004d7900(param_1);
  }
  FUN_004dcfd0(local_20, 0x1fb, 0x1c4, 0x232, 0x1d0);
  _sprintf(DAT_0081b740, s_v___d__d_00531e88, 1, 1);
  FUN_004da920(0xc, DAT_0081b740, local_20, 0xffff);
  FUN_004dcfd0(local_20, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_0081bcf0 != '\0') {
    _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, DAT_0081e082, 1, &DAT_0081bcf0);
    FUN_004da920(0xc, DAT_0081b740, local_20, DAT_0081bcd0);
    iVar3 = (int)(((((param_1 ^ uVar6) - uVar6) & 0x1f) ^ uVar6) - uVar6) >> 1;
    _sprintf(DAT_0081b740, s_rebtiny_d_00531328, iVar3);
    FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
    _sprintf(DAT_0081b740, s_imptiny_d_0053131c, iVar3);
    FUN_004d0e60(DAT_0081b740, 0x1a4, 0x1c5);
  }
  FUN_004d83f0();
  iVar3 = FUN_004c0750(0);
  if (iVar3 == 1) {
    return 1;
  }
  FUN_004dcfd0(local_20, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_0085e7c6 != '\0') {
    FUN_004da030();
  }
  pcVar10 = FUN_004dcfa0(0x22b);
  FUN_004da060(pcVar10);
  pcVar12 = s_buttonsound_00531310;
  iVar11 = 8;
  uVar2 = 0;
  iVar3 = 0xc;
  pcVar10 = FUN_004dcfa0(0xf5);
  iVar3 = FUN_004d9a80(local_20, s_leaveup_005312fc, s_leavedown_00531304, pcVar10, iVar3,
                       (int)uVar2, iVar11, pcVar12);
  if (iVar3 != 0) {
    DAT_0081b850 = 0;
    FUN_004ca200();
    FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
  }
  FUN_004dcfd0(local_20, 8, 0x195, 0x47, 0x1d6);
  if (DAT_00532a34 != -1) {
    int v;
    __builtin_memcpy(&v, DAT_00757e70 + (uint)DAT_00532a34 * 0x3b, 4);
    if ((v != 0) && (DAT_00757e78[(uint)DAT_00532a34 * 0x3b] == '\x02')) {
      pcVar10 = FUN_004dcfa0(0x12);
      FUN_004da060(pcVar10);
      pcVar12 = s_flysound_00532a6c;
      iVar11 = 7;
      uVar2 = 0;
      iVar3 = 0xc;
      pcVar10 = FUN_004dcfa0(0x12);
      iVar3 = FUN_004d9a80(local_20, s_nextup_00532a58, s_nextdown_00532a60, pcVar10, iVar3,
                           (int)uVar2, iVar11, pcVar12);
      if (iVar3 != 0) {
        DAT_00757cd8 = 0;
        uVar6 = 0xffffffff;
        pcVar10 = (char *)DAT_00757e40 + (uint)DAT_00532a34 * 0x3b;
        do {
          pcVar12 = pcVar10;
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          pcVar12 = pcVar10 + 1;
          cVar1 = *pcVar10;
          pcVar10 = pcVar12;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        pcVar10 = pcVar12 + -uVar6;
        pcVar12 = DAT_0081bfd2;
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          __builtin_memcpy(pcVar12, pcVar10, 4);
          pcVar10 = pcVar10 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *pcVar12 = *pcVar10;
          pcVar10 = pcVar10 + 1;
          pcVar12 = pcVar12 + 1;
        }
        iVar3 = FUN_004cdbb0();
        DAT_00758620 = 0x42;
        DAT_00758624 = 2;
        __builtin_memcpy(DAT_00758628, DAT_0085e77f, 4);
        __builtin_memcpy(DAT_0075862c, DAT_0085e783, 4);
        __builtin_memcpy(DAT_00758630, DAT_0085e787, 4);
        __builtin_memcpy(DAT_00758634, DAT_0085e78b, 4);
        FUN_004cb830(iVar3, &DAT_00758620, 0x18);
        FUN_004db670((void *)FUN_004d7150, (void *)0);
      }
    }
  }
  FUN_004da050();
  return 0;
}
