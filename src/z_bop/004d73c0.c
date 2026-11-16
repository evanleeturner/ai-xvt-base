/* SPDX-License-Identifier: MIT */
/* z_bop 004d73c0 */
/* Prototypes of called functions */
extern void FUN_004be1b0(void);
extern char LAB_004be050;
extern void FUN_004b7f30(void);
extern void FUN_004d7e70(void);

void __cdecl FUN_004dc330(void *, void *);
unsigned int __cdecl FUN_004d8c20(unsigned int);
char * __cdecl FUN_004ddc10(unsigned int);
int __cdecl FUN_004dcb90(char *, char *, char *, char *, char *);
void __cdecl FUN_004d6460(int *, int, int, int, int);
void __cdecl FUN_004dc380(void *, void *);
void __cdecl FUN_004de0b0(int, int);
void __cdecl FUN_004d1420(void);
void * __cdecl _malloc(unsigned int);
void __cdecl FUN_004b4a60(char *, char *);
void __cdecl FUN_004d4e20(void);
void __cdecl FUN_004b5de0(char *, int, int);
void __cdecl FUN_004b5570(char *, int, int);
void __cdecl FUN_004b4d40(char *, int, int);
void __cdecl FUN_004d4ec0(int);
void __cdecl FUN_004dc140(int);
void __cdecl FUN_004db640(int, char *, int *, unsigned int);
void __cdecl FUN_004d92b0(void);
int __cdecl FUN_004e0490(void);
DWORD __stdcall GetTickCount(void);
void __cdecl FUN_004f1680(void);
void __cdecl FUN_004e5000(int);
void __cdecl FUN_004e5590(unsigned char *);
void __cdecl FUN_004cd9d0(void);
int __cdecl FUN_004d7020(unsigned int);
int __cdecl FUN_004d97f0(int);
void __cdecl FUN_004d9680(void);
void __cdecl FUN_004dc310(void);
void __cdecl FUN_004d8540(void);
void __cdecl FUN_004d8350(void);
void __cdecl FUN_004d8690(unsigned int);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004d9170(void);
int __cdecl FUN_004bf9b0(int);
void __cdecl FUN_004dad40(void);
void __cdecl FUN_004dad70(char *);
int __cdecl FUN_004da780(int *, char *, char *, char *, int, int, int, char *);
int __cdecl FUN_004d11d0(void);
void __cdecl FUN_004cef70(int, unsigned int *, int);
void __cdecl FUN_004dad60(void);

/* Globals */
extern int DAT_00b69ce0;
extern char DAT_00bb72d0;
extern char DAT_00bb72d1;
extern char DAT_00bb7311;
extern int DAT_0052c184;
extern int DAT_00b69e30;
extern unsigned int DAT_00aa6150[0x54];
extern unsigned int DAT_00665440[8];
extern unsigned int *DAT_00b69cd0;
extern int DAT_00b6a2c4;
extern unsigned int DAT_00aa62b0[0x1f0];
extern int DAT_00aa6cf4;
extern int DAT_0052c208;
extern int DAT_00aa613c;
extern int DAT_00aa6cf8;
extern unsigned int DAT_00aa6ae8;
extern int DAT_0052bf50;
extern int DAT_00b6a2a8;
extern unsigned char *DAT_00aa6118;
extern int DAT_00b6a250;
extern int DAT_00aa6114;
extern int DAT_00aa6134;
extern int DAT_00aa6138;
extern int DAT_00aa6144;
extern unsigned int DAT_00aa6a70;
extern unsigned char DAT_00aa6a74;
extern unsigned char DAT_00aa62e0;
extern unsigned char DAT_00aa62e4;
extern unsigned char DAT_00aa62e8;
extern unsigned char DAT_00aa62ec;
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6a5ae[];
extern unsigned int DAT_00aa6cf0;
extern char DAT_00b69d20[];
extern char DAT_00b6a2e0;
extern char DAT_00b6bc9e;
extern unsigned int DAT_00b6a2c0;
extern char DAT_00bb73a8;
extern char DAT_00b6a5c6[];
extern unsigned char DAT_00bb735f[4];
extern unsigned char DAT_00bb7363[4];
extern unsigned char DAT_00bb7367[4];
extern unsigned char DAT_00bb736b[4];
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6af4;
extern unsigned int DAT_00aa6af8;
extern unsigned int DAT_00aa6afc;
extern unsigned int DAT_00aa6b00;
extern unsigned int DAT_00aa6b04;

extern char s_frontres_joinback_bmp_0052bfb4[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_joinoverlay_0052bfa0[];
extern char s_v___d__d_0052b1b4[];
extern char s__c_s__c_s_0052a6b0[];
extern char s_rebtiny_d_0052a6a4[];
extern char s_imptiny_d_0052a698[];
extern char s_buttonsound_0052a68c[];
extern char s_leaveup_0052a678[];
extern char s_leavedown_0052a680[];
extern char s_flysound_0052bf94[];
extern char s_nextup_0052bf80[];
extern char s_nextdown_0052bf88[];

unsigned int __cdecl FUN_004d73c0(unsigned int param_1)
{
  char cVar1;
  unsigned char uVar2;
  unsigned int uVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  char *pcVar7;
  char *pcVar8;
  int iVar9;
  DWORD DVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  int *piVar13;
  unsigned int *puVar14;
  unsigned char *pbVar15;
  int iVar16;
  int local_20[4];
  int local_10[4];
  int t;
  int t2;
  unsigned int ut;

  DAT_00b69ce0 = 0;
  if (DAT_00bb72d0 != '\0') {
    if (DAT_0052c184 != 0) {
      DAT_00b69e30 = 0;
      FUN_004dc330((void *)FUN_004be1b0, &LAB_004be050);
      return 0;
    }
    if (DAT_00b69e30 == 3) {
      if (DAT_00bb72d0 == '\x01') {
        if ((char)DAT_00bb7311 == '\0') {
          pcVar4 = FUN_004ddc10(0x13);
          pcVar5 = FUN_004ddc10(0x20b);
          pcVar6 = FUN_004ddc10(0x2b7);
          pcVar7 = FUN_004ddc10(0x2b6);
          pcVar8 = FUN_004ddc10(0x2b5);
          iVar9 = FUN_004dcb90(pcVar8, pcVar7, pcVar6, pcVar5, pcVar4);
          if (iVar9 == 0) {
            DAT_00b69e30 = 0;
            FUN_004dc330((void *)FUN_004be1b0, &LAB_004be050);
            return 0;
          }
          FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
          FUN_004dc380((void *)FUN_004b7f30, local_10);
          return 0;
        }
      }
      else if ((DAT_00bb72d0 == '\x02') && ((char)DAT_00bb72d1 == '\0')) {
        pcVar4 = FUN_004ddc10(0x13);
        pcVar5 = FUN_004ddc10(0x20b);
        pcVar6 = FUN_004ddc10(0x2ba);
        pcVar7 = FUN_004ddc10(0x2b9);
        pcVar8 = FUN_004ddc10(0x2b8);
        iVar9 = FUN_004dcb90(pcVar8, pcVar7, pcVar6, pcVar5, pcVar4);
        if (iVar9 == 0) {
          DAT_00b69e30 = 0;
          FUN_004dc330((void *)FUN_004be1b0, &LAB_004be050);
          return 0;
        }
        FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
        FUN_004dc380((void *)FUN_004b7f30, local_10);
        return 0;
      }
    }
    uVar3 = FUN_004d8c20(param_1);
    return uVar3;
  }
  if (param_1 == 0) {
    FUN_004de0b0(0x19f, 0x79);
    puVar14 = DAT_00aa6150;
    for (iVar9 = 0x54; iVar9 != 0; iVar9 = iVar9 + -1) {
      *puVar14 = 0;
      puVar14 = puVar14 + 1;
    }
    FUN_004d1420();
    puVar14 = DAT_00665440;
    for (iVar9 = 8; iVar9 != 0; iVar9 = iVar9 + -1) {
      *puVar14 = 0;
      puVar14 = puVar14 + 1;
    }
    if (DAT_00b69cd0 != (unsigned int *)0x0) {
      puVar14 = DAT_00b69cd0;
      for (iVar9 = 0x100; iVar9 != 0; iVar9 = iVar9 + -1) {
        *puVar14 = 0;
        puVar14 = puVar14 + 1;
      }
      DAT_00b6a2c4 = 0;
    }
    if (DAT_0052c184 == 0) {
      puVar14 = DAT_00aa62b0;
      for (iVar9 = 0x1f0; iVar9 != 0; iVar9 = iVar9 + -1) {
        *puVar14 = 0;
        puVar14 = puVar14 + 1;
      }
      DAT_00aa6cf4 = 0;
    }
    DAT_0052c208 = 0;
    DAT_00b69ce0 = 0;
    DAT_00aa613c = 0;
    DAT_00aa6cf8 = 0;
    DAT_00aa6ae8 = 0xffffffff;
    DAT_0052bf50 = -1;
    DAT_00b6a2a8 = 0;
    DAT_0052c184 = 0;
    if (DAT_00aa6118 == (unsigned char *)0) {
      DAT_00aa6118 = (unsigned char *)_malloc(0x1000);
    }
    FUN_004b4a60(s_frontres_joinback_bmp_0052bfb4, s_background_0052bf74);
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
    FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
    FUN_004b4d40(s_joinoverlay_0052bfa0, 0, 0);
    FUN_004d4ec0(1);
    FUN_004dc140(0x14);
  }
  FUN_004d6460(local_20, 0x9e, 0x34, 0x1eb, 0x44);
  FUN_004db640(0xc, (char *)DAT_00665440, local_20, 0xffff);
  uVar12 = (unsigned int)((int)param_1 >> 0x1f);
  if ((int)param_1 % 0xa0 == 0) {
    if (DAT_0052bf50 == -1) {
      FUN_004d92b0();
      goto LAB_004d7733;
    }
  }
  else {
LAB_004d7733:
    if (DAT_0052bf50 == -1) goto LAB_004d7a1e;
  }
  iVar9 = FUN_004e0490();
  if (iVar9 == 0x41) {
    DVar10 = GetTickCount();
    iVar9 = DAT_0052bf50;
    __builtin_memcpy(&DAT_00aa62e4 + (unsigned int)iVar9 * 0x3eu, &DVar10, 4);
    uVar2 = DAT_00aa6a74;
    __builtin_memcpy(&DAT_00aa62e0 + (unsigned int)iVar9 * 0x3eu, &DAT_00aa6a70, 4);
    (&DAT_00aa62ec)[(unsigned int)iVar9 * 0x3eu] = uVar2;
    FUN_004f1680();
    __builtin_memcpy(DAT_00b6a5aa, &DAT_00aa6cf8, 4);
    __builtin_memcpy(DAT_00b6a5ae + (unsigned int)DAT_00aa6cf8 * 4u, &DAT_00aa6ae8, 4);
    __builtin_memcpy(&t, DAT_00b6a5aa, 4);
    FUN_004e5000(t);
    FUN_004e5590(DAT_00aa6118);
    if (DAT_00aa6114 != 0) {
      DAT_00aa6134 = 0;
      if (DAT_00aa6138 != 0) {
        piVar13 = (int *)(DAT_00aa6114 + 0x140);
        do {
          __builtin_memcpy(&t, DAT_00b6a5aa, 4);
          __builtin_memcpy(&t2, DAT_00b6a5ae + (unsigned int)t * 4u, 4);
          if (*piVar13 == t2) break;
          piVar13 = piVar13 + 0x52;
          DAT_00aa6134 = (int)((unsigned int)DAT_00aa6134 + 1u);
        } while (DAT_00aa6134 < DAT_00aa6138);
      }
    }
    DAT_0052c208 = 0;
    goto LAB_004d7a1e;
  }
  if ((((iVar9 == 0x46) || (iVar9 == 0x49)) || (iVar9 == 0x4b)) ||
      (((iVar9 == 0x4c || (iVar9 == 0x5a)) || ((iVar9 == 0x66 || (iVar9 == 0x76)))))) {
    FUN_004cd9d0();
    FUN_004d92b0();
    DAT_00aa6cf0 = 0;
    DAT_0052bf50 = -1;
    DAT_00aa6ae8 = 0xffffffff;
    goto LAB_004d7a03;
  }
  if (iVar9 == 0x69) {
    FUN_004cd9d0();
    DAT_00aa6cf0 = 0;
    DAT_0052bf50 = -1;
    DAT_00aa6ae8 = 0xffffffff;
    FUN_004d92b0();
    goto LAB_004d7a03;
  }
  if (((iVar9 != 0x44) && (iVar9 != 0x5e)) && ((iVar9 != 0x5f && (iVar9 != 0x60))))
    goto LAB_004d7a1e;
  FUN_004cd9d0();
  switch (iVar9) {
  case 0x44:
    pcVar7 = (char *)0;
    pcVar6 = (char *)0;
    pcVar4 = FUN_004ddc10(0x221);
    pcVar5 = FUN_004ddc10(0x220);
    uVar11 = 0x21f;
    break;
  default:
    goto switchD_004d78ca_caseD_45;
  case 0x5e:
    pcVar7 = (char *)0;
    pcVar6 = (char *)0;
    pcVar4 = FUN_004ddc10(0x224);
    pcVar5 = FUN_004ddc10(0x223);
    uVar11 = 0x222;
    break;
  case 0x5f:
    pcVar8 = (char *)0;
    pcVar7 = (char *)0;
    pcVar4 = FUN_004ddc10(0x227);
    pcVar5 = FUN_004ddc10(0x226);
    pcVar6 = FUN_004ddc10(0x225);
    FUN_004dcb90(pcVar6, pcVar5, pcVar4, pcVar7, pcVar8);
    FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
    FUN_004dc380((void *)FUN_004b7f30, local_10);
    goto switchD_004d78ca_caseD_45;
  case 0x60:
    pcVar7 = (char *)0;
    pcVar6 = (char *)0;
    pcVar4 = FUN_004ddc10(0x22a);
    pcVar5 = FUN_004ddc10(0x229);
    uVar11 = 0x228;
  }
  pcVar8 = FUN_004ddc10(uVar11);
  FUN_004dcb90(pcVar8, pcVar5, pcVar4, pcVar6, pcVar7);
switchD_004d78ca_caseD_45:
  DAT_00aa6cf0 = 0;
  DAT_0052bf50 = -1;
  DAT_00aa6ae8 = 0xffffffff;
  FUN_004d92b0();
LAB_004d7a03:
  puVar14 = DAT_00665440;
  for (iVar9 = 8; iVar9 != 0; iVar9 = iVar9 + -1) {
    *puVar14 = 0;
    puVar14 = puVar14 + 1;
  }
  pbVar15 = DAT_00aa6118;
  for (iVar9 = 0x400; iVar9 != 0; iVar9 = iVar9 + -1) {
    pbVar15[0] = 0;
    pbVar15[1] = 0;
    pbVar15[2] = 0;
    pbVar15[3] = 0;
    pbVar15 = pbVar15 + 4;
  }
LAB_004d7a1e:
  iVar9 = FUN_004d7020(param_1);
  if (iVar9 != -1) {
    if (DAT_0052bf50 == iVar9) {
      DAT_0052bf50 = -1;
      DAT_00aa6ae8 = 0xffffffff;
      DAT_00aa6cf0 = 0;
      puVar14 = DAT_00665440;
      for (iVar9 = 8; iVar9 != 0; iVar9 = iVar9 + -1) {
        *puVar14 = 0;
        puVar14 = puVar14 + 1;
      }
      pbVar15 = DAT_00aa6118;
      for (iVar9 = 0x400; iVar9 != 0; iVar9 = iVar9 + -1) {
        pbVar15[0] = 0;
        pbVar15[1] = 0;
        pbVar15[2] = 0;
        pbVar15[3] = 0;
        pbVar15 = pbVar15 + 4;
      }
      FUN_004cd9d0();
    }
    else {
      DAT_0052bf50 = iVar9;
      iVar9 = FUN_004d97f0(iVar9);
      if (iVar9 == 0) {
        FUN_004cd9d0();
        DAT_00aa6cf0 = 0;
        DAT_0052bf50 = -1;
        DAT_00aa6ae8 = 0xffffffff;
        FUN_004d92b0();
        puVar14 = DAT_00665440;
        for (iVar9 = 8; iVar9 != 0; iVar9 = iVar9 + -1) {
          *puVar14 = 0;
          puVar14 = puVar14 + 1;
        }
        pbVar15 = DAT_00aa6118;
        for (iVar9 = 0x400; iVar9 != 0; iVar9 = iVar9 + -1) {
          pbVar15[0] = 0;
          pbVar15[1] = 0;
          pbVar15[2] = 0;
          pbVar15[3] = 0;
          pbVar15 = pbVar15 + 4;
        }
      }
      else {
        FUN_004d9680();
        __builtin_memcpy(&t, &DAT_00aa62e0 + (unsigned int)DAT_0052bf50 * 0x3eu, 4);
        if (t != 0) {
          FUN_004dc310();
          FUN_004de0b0(0x25, 0x1bd);
        }
      }
    }
  }
  FUN_004d8540();
  FUN_004d8350();
  if (DAT_0052bf50 != -1) {
    FUN_004d8690(param_1);
  }
  FUN_004d6460(local_20, 0x1fb, 0x1c4, 0x232, 0x1d0);
  _sprintf(DAT_00b69d20, s_v___d__d_0052b1b4, 2, 0);
  FUN_004db640(0xc, DAT_00b69d20, local_20, 0xffff);
  FUN_004d6460(local_20, 200, 0x1c4, 0x1b4, 0x1d0);
  if ((char)DAT_00b6a2e0 != '\0') {
    _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, &DAT_00b6bc9e, 1, &DAT_00b6a2e0);
    FUN_004db640(0xc, DAT_00b69d20, local_20, DAT_00b6a2c0);
    iVar9 = (int)(((((param_1 ^ uVar12) - uVar12) & 0x1f) ^ uVar12) - uVar12) >> 1;
    _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, iVar9);
    FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
    _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, iVar9);
    FUN_004b5570(DAT_00b69d20, 0x1a4, 0x1c5);
  }
  FUN_004d9170();
  iVar9 = FUN_004bf9b0(0);
  if (iVar9 == 1) {
    return 1;
  }
  FUN_004d6460(local_20, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_00bb73a8 != '\0') {
    FUN_004dad40();
  }
  pcVar4 = FUN_004ddc10(0x239);
  FUN_004dad70(pcVar4);
  pcVar5 = s_buttonsound_0052a68c;
  iVar16 = 8;
  uVar3 = 0;
  iVar9 = 0xc;
  pcVar4 = FUN_004ddc10(0x102);
  iVar9 = FUN_004da780(local_20, s_leaveup_0052a678, s_leavedown_0052a680, pcVar4, iVar9, uVar3,
                       iVar16, pcVar5);
  if (iVar9 != 0) {
    DAT_00b69e30 = 0;
    FUN_004cd9d0();
    FUN_004dc330((void *)FUN_004be1b0, &LAB_004be050);
  }
  FUN_004d6460(local_20, 8, 0x195, 0x47, 0x1d6);
  if (DAT_0052bf50 != -1) {
    __builtin_memcpy(&t, &DAT_00aa62e0 + (unsigned int)DAT_0052bf50 * 0x3eu, 4);
    if (t != 0) {
      __builtin_memcpy(&t2, &DAT_00aa62e8 + (unsigned int)DAT_0052bf50 * 0x3eu, 4);
      if (t2 == 0x65) {
        pcVar4 = FUN_004ddc10(0x12);
        FUN_004dad70(pcVar4);
        pcVar5 = s_flysound_0052bf94;
        iVar16 = 7;
        uVar3 = 0;
        iVar9 = 0xc;
        pcVar4 = FUN_004ddc10(0x12);
        iVar9 = FUN_004da780(local_20, s_nextup_0052bf80, s_nextdown_0052bf88, pcVar4, iVar9,
                             uVar3, iVar16, pcVar5);
        if (iVar9 != 0) {
          uVar12 = 0xffffffff;
          DAT_00aa6144 = 0;
          pcVar4 = (char *)DAT_00aa62b0 + (unsigned int)DAT_0052bf50 * 0x3eu;
          do {
            pcVar5 = pcVar4;
            if (uVar12 == 0) break;
            uVar12 = uVar12 - 1;
            pcVar5 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar5;
          } while (cVar1 != '\0');
          uVar12 = ~uVar12;
          pcVar4 = pcVar5 + -uVar12;
          pcVar5 = DAT_00b6a5c6;
          for (uVar11 = uVar12 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
            __builtin_memcpy(&ut, pcVar4, 4);
            __builtin_memcpy(pcVar5, &ut, 4);
            pcVar4 = pcVar4 + 4;
            pcVar5 = pcVar5 + 4;
          }
          for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
            *pcVar5 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar5 = pcVar5 + 1;
          }
          iVar9 = FUN_004d11d0();
          DAT_00aa6af0 = 0x42;
          DAT_00aa6af4 = 0x65;
          __builtin_memcpy(&DAT_00aa6af8, DAT_00bb735f, 4);
          __builtin_memcpy(&DAT_00aa6afc, DAT_00bb7363, 4);
          __builtin_memcpy(&DAT_00aa6b00, DAT_00bb7367, 4);
          __builtin_memcpy(&DAT_00aa6b04, DAT_00bb736b, 4);
          FUN_004cef70(iVar9, &DAT_00aa6af0, 0x18);
          FUN_004dc330((void *)FUN_004d7e70, (void *)0);
        }
      }
    }
  }
  FUN_004dad60();
  return 0;
}
