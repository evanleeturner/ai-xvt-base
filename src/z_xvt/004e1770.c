/* SPDX-License-Identifier: MIT */
/* z_xvt 004e1770 */
/* Globals at unaligned addresses are declared as byte arrays and accessed with
   __builtin_memcpy through the LD32/ST32 macros. Aligned globals use plain types.
   Type guesses for DAT_00757cc8, DAT_00757ccc and DAT_0064ff38 are int; the
   decompile does not show signedness for them. */

extern unsigned char DAT_0081bfb6[];
extern unsigned char DAT_0081bfba[];
extern unsigned char DAT_0081bfbe[];
extern unsigned char DAT_0081c012[];
extern unsigned char DAT_0081c016[];
extern unsigned char DAT_0081fb96[];
extern unsigned char DAT_0081fb9e[];
extern unsigned char DAT_0081fba2[];
extern unsigned char DAT_0081fba6[];
extern unsigned char DAT_0081fbda[];
extern unsigned char DAT_0081fbde[];
extern unsigned char DAT_0083cbe6[];
extern unsigned char DAT_0083cbea[];
extern unsigned char DAT_0083cbee[];
extern unsigned char DAT_0083cc22[];
extern unsigned char DAT_0083cc26[];
extern unsigned char DAT_0081d302[];
extern unsigned char DAT_0081d402[];
extern unsigned char DAT_0081f7be[];
extern unsigned char DAT_0081fa7e[];
extern unsigned char DAT_00757cee[];
extern unsigned char DAT_00757cf2[];
extern unsigned char DAT_00757e3e[];
extern unsigned char DAT_0081e016[];
extern unsigned char DAT_0081dc0e[];
extern unsigned char DAT_0081d80e[];
extern unsigned char DAT_0081d406[];
extern unsigned char DAT_0081d32a[];
extern unsigned char DAT_0081d40a[];

extern int DAT_00743850;
extern int DAT_00757cd4;
extern int DAT_0081b850;
extern int DAT_0064ff44;
extern int DAT_0081b70c;
extern void *DAT_00757cc4;
extern int DAT_005330fc;
extern int DAT_0064ff30;
extern int DAT_005330f8;
extern int DAT_0081bcb8;
extern int DAT_0064ff38;
extern DWORD DAT_0064ff34;
extern int DAT_00757cc8;
extern int DAT_00757cc0;
extern int DAT_00757ccc;
extern int DAT_00757cd0;
extern int DAT_0053308c;
extern unsigned int DAT_00743860[];
extern char DAT_0081bcf0[];
extern char DAT_00757ce0[];
extern char DAT_0085e7bd;
extern char DAT_0085e7bb;
extern char DAT_0085e7c6;
extern char DAT_0085e6f0;
extern unsigned int DAT_00758828;
extern unsigned int DAT_00758618;
extern unsigned int DAT_00758620;
extern unsigned int DAT_00758624;
extern unsigned int DAT_00758628;
extern unsigned int DAT_0075862c;
extern unsigned int DAT_00758630;
extern int DAT_0081bcd0;
extern unsigned int DAT_00533090;
extern unsigned int DAT_00533094;
extern char DAT_0081bfd2[];
extern char DAT_0081b740[];
extern char DAT_0081e082[];

extern char s_background_00532a4c[];
extern char s__s__c_s_00532aa4[];
extern char s_v___d__d_00531e88[];
extern char s__c_s__c_s_00531334[];
extern char s_rebtiny_d_00531328[];
extern char s_imptiny_d_0053131c[];
extern char s_buttonsound_00531310[];
extern char s_quickflyup_00533128[];
extern char s_quickflydown_00533134[];
extern char s_leaveup_005312fc[];
extern char s_leavedown_00531304[];
extern char s_nextup_00532a58[];
extern char s_nextdown_00532a60[];
extern char s_flysound_00532a6c[];
extern char s_dropbtnup_00533110[];
extern char s_dropbtndown_0053311c[];
extern char s_bootu_00533100[];
extern char s_bootd_00533108[];

extern void FUN_004d66c0(void);
extern void LAB_004d6670(void);
extern void FUN_004bef20(void);
extern void LAB_004bee00(void);
extern void DAT_004ee9b0(void);
extern void LAB_004ee950(void);
extern void DAT_004e56b0(void);

void __cdecl FUN_004ddc90(int a, int b);
void __cdecl FUN_004ee3d0(void);
void *__cdecl _malloc(unsigned int size);
DWORD __stdcall GetTickCount(void);
void __cdecl FUN_004f2990(void);
void __cdecl FUN_004e4820(void *p);
void __cdecl FUN_004f0100(void);
void __cdecl FUN_004e78b0(void);
void __cdecl FUN_004db490(int a);
int __cdecl FUN_004cc810(void);
void __cdecl FUN_004e5600(int a);
void __cdecl FUN_004e5120(void);
int __cdecl FUN_004e0750(void);
void __cdecl FUN_004d0430(char *s);
void __cdecl FUN_004ca200(void);
char *__cdecl FUN_004dcfa0(int id);
int __cdecl FUN_004dbf00(void);
void __cdecl FUN_004db670(void (*a)(void), void (*b)(void));
int __cdecl FUN_004cdbb0(void);
void __cdecl FUN_004cb830(int a, unsigned int *b, unsigned int c);
void __cdecl FUN_004dcfd0(int *r, int a, int b, int c, int d);
void __cdecl FUN_004da920(int a, char *s, int *r, int c);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void __cdecl FUN_004dd070(int *r, int a, int b);
void __cdecl FUN_004daac0(int a, char *s, int *r, int c, int d, int e);
void __cdecl FUN_004e4a20(void);
void __cdecl FUN_004d7900(unsigned int a);
void __cdecl FUN_004d0e60(char *s, int x, int y);
void __cdecl FUN_004e4b80(unsigned int a);
void __cdecl FUN_004e6960(void);
void __cdecl FUN_004da030(void);
void __cdecl FUN_004da060(char *s);
int __cdecl FUN_004d9a80(int *r, char *up, char *down, char *txt, int a, int b, int c, char *snd);
int __cdecl FUN_004e6630(void);
void __cdecl FUN_004da050(void);
int __cdecl FUN_004cde00(void);
void __cdecl FUN_004e5320(int a);
void __cdecl FUN_004db6c0(void (*f)(void), int *r);
int __cdecl FUN_004e2910(void);
int __cdecl FUN_004c0750(int a);
void __cdecl FUN_004cddb0(int a);

#define LD32(p) ({ unsigned int _t; __builtin_memcpy(&_t, (p), 4); _t; })
#define ST32(p, v) do { unsigned int _t = (unsigned int)(v); __builtin_memcpy((p), &_t, 4); } while (0)
#define SEL(base, idx) ((base) + (unsigned int)(idx) * 4u)
#define V_BFBA LD32(DAT_0081bfba)
#define V_C012 LD32(DAT_0081c012)
#define V_FB96 LD32(DAT_0081fb96)

unsigned int __cdecl FUN_004e1770(unsigned int param_1)
{
  char cVar1;
  unsigned char bVar2;
  DWORD DVar3;
  int *piVar4;
  char *pcVar5;
  char *pcVar6;
  int iVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned char *puVar10;
  unsigned char *puVar11;
  unsigned int uVar12;
  int iVar13;
  void (*pcVar14)(void);
  unsigned int *puVar15;
  void (*puVar16)(void);
  int aiStack_10[4];
  unsigned int uKey;
  unsigned char *pbVar;

  if (param_1 == 0) {
    FUN_004ddc90(0x25, 0x1bd);
    FUN_004ee3d0();
    DAT_00743850 = 0;
    DAT_00757cd4 = 0;
    if (DAT_0081b850 == 2) {
      DAT_0064ff44 = 0;
      uKey = V_FB96 * 0x3a09u;
      ST32(DAT_0081bfb6, LD32(SEL(DAT_0081fb9e, uKey)));
      ST32(DAT_0081bfba, LD32(SEL(DAT_0081fba2, uKey)));
      puVar10 = SEL(DAT_0081fba6, uKey);
      puVar11 = DAT_0081bfbe;
      for (iVar7 = 5; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar11, puVar10, 4);
        puVar10 = puVar10 + 4;
        puVar11 = puVar11 + 4;
      }
      ST32(DAT_0081c012, LD32(SEL(DAT_0081fbda, uKey)));
      ST32(SEL(DAT_0081fbda, V_FB96 * 0x3a09u), 0);
      ST32(DAT_0081c016, LD32(SEL(DAT_0081fbde, V_FB96 * 0x3a09u)));
    }
    else {
      ST32(DAT_0081bfb6, LD32(DAT_0083cbe6));
      ST32(DAT_0081bfba, LD32(DAT_0083cbea));
      DAT_0064ff44 = 1;
      puVar10 = DAT_0083cbee;
      puVar11 = DAT_0081bfbe;
      for (iVar7 = 5; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar11, puVar10, 4);
        puVar10 = puVar10 + 4;
        puVar11 = puVar11 + 4;
      }
      ST32(DAT_0081c012, LD32(DAT_0083cc22));
      ST32(DAT_0081c016, LD32(DAT_0083cc26));
      ST32(DAT_0083cc22, 0);
    }
    DAT_0081b70c = 0;
    DAT_00757cc4 = _malloc(0x400);
    DAT_005330fc = 0;
    DAT_0064ff30 = 0;
    DAT_005330f8 = 0;
    DAT_0081bcb8 = 0;
    DAT_0064ff38 = -1;
    DAT_0064ff34 = GetTickCount();
    DAT_00757cc8 = 0;
    if ((DAT_00757cc0 != 0) && (DAT_00757ccc != 0)) {
      piVar4 = (int *)(DAT_00757cc0 + 0x140);
      do {
        if ((unsigned int)*piVar4 == LD32(SEL(DAT_0081bfbe, V_BFBA))) break;
        piVar4 = piVar4 + 0x51;
        DAT_00757cc8 = DAT_00757cc8 + 1;
      } while (DAT_00757cc8 < DAT_00757ccc);
    }
    if (V_C012 == 1) {
      ST32(SEL(DAT_0081bfbe, V_BFBA), LD32(DAT_0081c016));
    }
    puVar10 = DAT_0081d302;
    for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
      ST32(puVar10, 0);
      puVar10 = puVar10 + 4;
    }
    puVar10 = DAT_0081d402;
    for (iVar7 = 0x305; iVar13 = (int)V_BFBA, iVar7 != 0; iVar7 = iVar7 + -1) {
      ST32(puVar10, 0);
      puVar10 = puVar10 + 4;
    }
    if (V_C012 == 1) {
      ST32(DAT_0081bfba, 2);
      if (iVar13 != 1) {
        ST32(DAT_0081bfba, 4);
      }
    }
    ST32(DAT_0081c012, 0);
    FUN_004f2990();
    FUN_004e4820(DAT_00757cc4);
    FUN_004f0100();
    if ((DAT_0081b850 != 2) &&
        (((V_BFBA == 3 || (V_BFBA == 4)) && (DAT_0085e7bd == '\x02')))) {
      DAT_0085e7bd = '\x01';
    }
    if ((DAT_0085e7bb == '\x02') &&
        ((DAT_0081b850 == 2 || ((V_BFBA != 1 && (V_BFBA != 2)))))) {
      DAT_0085e7bb = '\x01';
    }
    DAT_0053308c = 0;
    puVar10 = DAT_0081f7be;
    do {
      puVar11 = puVar10;
      for (iVar7 = 0x16; iVar7 != 0; iVar7 = iVar7 + -1) {
        ST32(puVar11, 0);
        puVar11 = puVar11 + 4;
      }
      ST32(puVar10 + 0x40, 0);
      puVar11 = puVar10 + 0x58;
      ST32(puVar10 + 0x44, 0xffffffffu);
      ST32(puVar10 + 0x48, 0xffffffffu);
      ST32(puVar10 + 0x4c, 0xffffffffu);
      ST32(puVar10 + 0x50, 0xffffffffu);
      puVar10 = puVar11;
    } while (puVar11 < DAT_0081fa7e);
    for (iVar7 = 0; iVar7 < 8; iVar7 = iVar7 + 1) {
      DAT_00743860[iVar7] = 0;
    }
    if (DAT_0081b850 == 2) {
      uVar8 = 0xffffffff;
      pcVar5 = DAT_0081bcf0;
      do {
        pcVar6 = pcVar5;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar6 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar6;
      } while (cVar1 != '\0');
      uVar8 = ~uVar8;
      pcVar5 = pcVar6 + -uVar8;
      pcVar6 = DAT_00757ce0;
      for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar6, pcVar5, 4);
        pcVar5 = pcVar5 + 4;
        pcVar6 = pcVar6 + 4;
      }
      for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar6 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar6 = pcVar6 + 1;
      }
      ST32(DAT_00757cee, 1);
      ST32(DAT_00757cf2, LD32(DAT_0081e016));
    }
    FUN_004e78b0();
    FUN_004db490(0x14);
  }
  if (DAT_0081b850 != 2) {
    iVar7 = FUN_004cc810();
    if ((iVar7 != 0) && (DVar3 = GetTickCount(), 5000 < DVar3 - DAT_0064ff34)) {
      FUN_004e5600(0);
      FUN_004e5120();
      DAT_0064ff34 = DVar3;
    }
    iVar7 = FUN_004e0750();
    if (iVar7 == 0x41) {
      ST32(DAT_0081bfba, DAT_00758828);
      ST32(SEL(DAT_0081bfbe, DAT_00758828), DAT_00758618);
      FUN_004f2990();
      FUN_004d0430(s_background_00532a4c);
      FUN_004e78b0();
      FUN_004e4820(DAT_00757cc4);
      FUN_004f0100();
      if ((DAT_00757cc0 != 0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
        piVar4 = (int *)(DAT_00757cc0 + 0x140);
        do {
          if ((unsigned int)*piVar4 == LD32(SEL(DAT_0081bfbe, V_BFBA))) break;
          piVar4 = piVar4 + 0x51;
          DAT_00757cc8 = DAT_00757cc8 + 1;
        } while (DAT_00757cc8 < DAT_00757ccc);
      }
      DAT_005330fc = 0;
      DAT_0064ff38 = -1;
      if ((DAT_0081b850 != 2) &&
          (((V_BFBA == 3 || (V_BFBA == 4)) && (DAT_0085e7bd == '\x02')))) {
        DAT_0085e7bd = '\x01';
      }
      if (DAT_0085e7bb == '\x02') {
        if (DAT_0081b850 == 2) {
          DAT_0085e7bb = '\x01';
        }
        else if ((V_BFBA != 1) && (V_BFBA != 2)) {
          DAT_0085e7bb = '\x01';
        }
      }
    }
    else if (iVar7 == 0x46) {
      FUN_004ca200();
      iVar7 = FUN_004cc810();
      if (iVar7 == 0) {
        FUN_004dcfa0(0x26b);
        FUN_004dcfa0(0x26a);
        FUN_004dcfa0(0x269);
        FUN_004dbf00();
      }
      DAT_0081b850 = 3;
      if (DAT_0085e6f0 == '\0') {
        FUN_004db670(FUN_004d66c0, LAB_004d6670);
      }
      else {
        FUN_004db670(FUN_004bef20, LAB_004bee00);
      }
    }
    else if (iVar7 == 0x69) {
      FUN_004ca200();
      FUN_004dcfa0(0x297);
      FUN_004dcfa0(0x296);
      FUN_004dcfa0(0x295);
      FUN_004dbf00();
      DAT_0081b850 = 3;
      if (DAT_0085e6f0 == '\0') {
        FUN_004db670(FUN_004d66c0, LAB_004d6670);
      }
      else {
        FUN_004db670(FUN_004bef20, LAB_004bee00);
      }
    }
    else {
      if (iVar7 == 0x49) {
        if ((DAT_0081b850 != 2) &&
            (((V_BFBA == 3 || (V_BFBA == 4)) && (DAT_0085e7bd == '\x02')))) {
          DAT_0085e7bd = '\x01';
        }
        if (((DAT_0085e7bb == '\x02') && (V_BFBA != 1)) && (V_BFBA != 2)) {
          DAT_0085e7bb = '\x01';
        }
        if ((V_BFBA == 3) && (V_C012 == 1)) {
          ST32(DAT_0081dc0e, DAT_00757cc8);
          ST32(DAT_0081d80e, DAT_00757cc8);
          if (DAT_00757cc0 != 0) {
            ST32(DAT_0081d406,
                 *(unsigned int *)((unsigned int)DAT_00757cc8 * 0x144u + 0x140u +
                                   (unsigned int)DAT_00757cc0));
          }
        }
        DAT_0081bcb8 = 1;
        FUN_004db670(DAT_004ee9b0, LAB_004ee950);
        return 0;
      }
      if (iVar7 == 0x5b) {
        DAT_0053308c = 1;
        DAT_00758620 = 0x47;
        uVar8 = 4;
        puVar15 = &DAT_00758620;
        iVar7 = FUN_004cdbb0();
        FUN_004cb830(iVar7, puVar15, uVar8);
        FUN_004ca200();
        FUN_004dcfa0(0x226);
        FUN_004dcfa0(0x225);
        FUN_004dcfa0(0x224);
        FUN_004dbf00();
        if (DAT_0085e6f0 == '\0') {
          FUN_004db670(FUN_004d66c0, LAB_004d6670);
          return 0;
        }
        FUN_004db670(FUN_004bef20, LAB_004bee00);
        return 0;
      }
      if (iVar7 == 0x75) {
        iVar7 = 0;
        pbVar = DAT_00757cee;
        do {
          if (LD32(pbVar) == DAT_00533090) {
            ST32(DAT_00757cf2 + (unsigned int)iVar7 * 0x2au, DAT_00533094);
            break;
          }
          pbVar = pbVar + 0x2a;
          iVar7 = iVar7 + 1;
        } while (pbVar < DAT_00757e3e);
      }
    }
    if (DAT_0081b850 != 2) {
      FUN_004dcfd0(aiStack_10, 0x9e, 0x34, 0x1eb, 0x44);
      FUN_004da920(0xc, DAT_0081bfd2, aiStack_10, 0xffff);
    }
  }
  pcVar5 = FUN_004dcfa0((int)(V_BFBA + 0xb6u));
  uVar12 = 4;
  pcVar6 = FUN_004dcfa0(0xb5);
  _sprintf(DAT_0081b740, s__s__c_s_00532aa4, pcVar6, uVar12, pcVar5);
  FUN_004dcfd0(aiStack_10, 0x54, 0x5a, 0x1b2, 0x6c);
  FUN_004da920(0xf, DAT_0081b740, aiStack_10, 0xffff);
  iVar7 = FUN_004cc810();
  if ((iVar7 == 0) && (DAT_0081b850 != 2)) {
    uVar12 = 0x1b2;
  }
  else {
    uVar12 = 0x1a0;
  }
  FUN_004dcfd0(aiStack_10, 0x54, 0x70, (int)uVar12, 0x82);
  if (DAT_00757cc8 < DAT_00757ccc) {
    FUN_004dd070(aiStack_10, 4, 0);
    FUN_004daac0(0xf,
                 (char *)((unsigned int)DAT_00757cc8 * 0x144u + (unsigned int)DAT_00757cc0 + 0x40u),
                 aiStack_10, 0, 1, DAT_0081bcd0);
  }
  FUN_004e4a20();
  if (DAT_0081b850 != 2) {
    FUN_004d7900(param_1);
  }
  FUN_004dcfd0(aiStack_10, 0x1fb, 0x1c4, 0x232, 0x1d0);
  _sprintf(DAT_0081b740, s_v___d__d_00531e88, 1, 1);
  FUN_004da920(0xc, DAT_0081b740, aiStack_10, 0xffff);
  FUN_004dcfd0(aiStack_10, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_0081bcf0[0] != '\0') {
    _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, DAT_0081e082, 1, DAT_0081bcf0);
    FUN_004da920(0xc, DAT_0081b740, aiStack_10, DAT_0081bcd0);
    uVar8 = (unsigned int)((int)param_1 >> 0x1f);
    if (((DAT_0081b850 == 2) && (V_BFBA != 1)) && (V_BFBA != 2)) {
      if (V_FB96 == 0) {
        _sprintf(DAT_0081b740, s_rebtiny_d_00531328,
                 (int)((((((param_1 ^ uVar8) - uVar8) & 0x1f) ^ uVar8) - uVar8)) >> 1);
        FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
      }
      else {
        _sprintf(DAT_0081b740, s_imptiny_d_0053131c,
                 (int)((((((param_1 ^ uVar8) - uVar8) & 0x1f) ^ uVar8) - uVar8)) >> 1);
        FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
      }
    }
    else {
      iVar7 = (int)((((((param_1 ^ uVar8) - uVar8) & 0x1f) ^ uVar8) - uVar8)) >> 1;
      _sprintf(DAT_0081b740, s_rebtiny_d_00531328, iVar7);
      FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
      _sprintf(DAT_0081b740, s_imptiny_d_0053131c, iVar7);
    }
    FUN_004d0e60(DAT_0081b740, 0x1a4, 0x1c5);
  }
  if ((DAT_0064ff30 == 1) || (DAT_0081b850 == 2)) {
    FUN_004e6960();
  }
  else {
    FUN_004e4b80(param_1);
  }
  FUN_004dcfd0(aiStack_10, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_0085e7c6 != '\0') {
    FUN_004da030();
  }
  if (DAT_0081b850 == 2) {
    pcVar5 = FUN_004dcfa0(0x285);
    FUN_004da060(pcVar5);
    pcVar6 = s_buttonsound_00531310;
    iVar13 = 8;
    uVar12 = 0;
    iVar7 = 0xc;
    pcVar5 = FUN_004dcfa0(0x285);
    iVar7 = FUN_004d9a80(aiStack_10, s_quickflyup_00533128, s_quickflydown_00533134, pcVar5, iVar7,
                         (int)uVar12, iVar13, pcVar6);
    if (iVar7 != 0) {
      if (((V_BFBA == 2) || (V_BFBA == 4)) && (iVar7 = FUN_004e6630(), iVar7 == 0)) {
        FUN_004da050();
        return 0;
      }
      DAT_005330f8 = 1;
      FUN_004db670(DAT_004ee9b0, LAB_004ee950);
      FUN_004da050();
      return 0;
    }
    goto LAB_004e24aa;
  }
  iVar7 = FUN_004cc810();
  if (iVar7 == 0) {
    pcVar5 = FUN_004dcfa0(0x22b);
    FUN_004da060(pcVar5);
    pcVar6 = s_buttonsound_00531310;
    iVar13 = 8;
    uVar12 = 0;
    iVar7 = 0xc;
    pcVar5 = FUN_004dcfa0(0xf6);
    iVar7 = FUN_004d9a80(aiStack_10, s_leaveup_005312fc, s_leavedown_00531304, pcVar5, iVar7,
                         (int)uVar12, iVar13, pcVar6);
    if (iVar7 == 0) goto LAB_004e24aa;
    FUN_004dcfa0(0x13);
    FUN_004dcfa0(0x1fd);
    FUN_004dcfa0(0x21f);
    FUN_004dcfa0(0x21e);
    FUN_004dcfa0(0x21d);
    iVar7 = FUN_004dbf00();
    if (iVar7 == 0) goto LAB_004e24aa;
    DAT_0053308c = 1;
    DAT_00758620 = 0x47;
    uVar8 = 4;
    puVar15 = &DAT_00758620;
    iVar7 = FUN_004cdbb0();
    FUN_004cb830(iVar7, puVar15, uVar8);
    FUN_004ca200();
    puVar16 = LAB_004d6670;
    pcVar14 = FUN_004d66c0;
  }
  else {
    pcVar5 = FUN_004dcfa0(0x22b);
    FUN_004da060(pcVar5);
    pcVar6 = s_buttonsound_00531310;
    iVar13 = 8;
    uVar12 = 0;
    iVar7 = 0xc;
    pcVar5 = FUN_004dcfa0(0xf5);
    iVar7 = FUN_004d9a80(aiStack_10, s_leaveup_005312fc, s_leavedown_00531304, pcVar5, iVar7,
                         (int)uVar12, iVar13, pcVar6);
    if (iVar7 == 0) goto LAB_004e24aa;
    if ((DAT_00757cd0 != 0) && (DAT_0081b850 != 2)) {
      FUN_004dcfa0(0x13);
      FUN_004dcfa0(0x1fd);
      FUN_004dcfa0(0x222);
      FUN_004dcfa0(0x221);
      FUN_004dcfa0(0x220);
      iVar7 = FUN_004dbf00();
    }
    if (iVar7 == 0) goto LAB_004e24aa;
    DAT_0053308c = 1;
    DAT_00758620 = 0x46;
    FUN_004cb830(0, &DAT_00758620, 4);
    FUN_004ca200();
    DAT_0081b850 = 0;
    puVar16 = LAB_004bee00;
    pcVar14 = FUN_004bef20;
  }
  FUN_004db670(pcVar14, puVar16);
LAB_004e24aa:
  if (DAT_00757cd4 != 0) {
    DAT_00757cd4 = DAT_00757cd4 + -1;
  }
  FUN_004dcfd0(aiStack_10, 8, 0x195, 0x47, 0x1d6);
  if (DAT_0081b850 == 2) {
    pcVar5 = FUN_004dcfa0(0x2b3);
    FUN_004da060(pcVar5);
    pcVar6 = s_flysound_00532a6c;
    iVar13 = 7;
    uVar12 = 0;
    iVar7 = 0xc;
    pcVar5 = FUN_004dcfa0(0x2b3);
    iVar7 = FUN_004d9a80(aiStack_10, s_nextup_00532a58, s_nextdown_00532a60, pcVar5, iVar7,
                         (int)uVar12, iVar13, pcVar6);
    if (iVar7 != 0) {
      if (((V_BFBA == 2) || (V_BFBA == 4)) && (iVar7 = FUN_004e6630(), iVar7 == 0)) {
        FUN_004da050();
        return 0;
      }
      FUN_004db670(DAT_004ee9b0, LAB_004ee950);
      FUN_004da050();
      return 0;
    }
  }
  else {
    iVar7 = FUN_004cc810();
    if ((iVar7 != 0) && (DAT_00757cd4 == 0)) {
      pcVar5 = FUN_004dcfa0(0x2b3);
      FUN_004da060(pcVar5);
      pcVar6 = s_flysound_00532a6c;
      iVar13 = 7;
      uVar12 = 0;
      iVar7 = 0xc;
      pcVar5 = FUN_004dcfa0(0x2b3);
      iVar7 = FUN_004d9a80(aiStack_10, s_nextup_00532a58, s_nextdown_00532a60, pcVar5, iVar7,
                           (int)uVar12, iVar13, pcVar6);
      if (iVar7 != 0) {
        bVar2 = *(unsigned char *)((unsigned int)DAT_00757cc8 * 0x144u + (unsigned int)DAT_00757cc0);
        iVar7 = FUN_004cde00();
        if ((int)(bVar2 - 0x30) < iVar7) {
          bVar2 = *(unsigned char *)((unsigned int)DAT_00757cc8 * 0x144u +
                                     (unsigned int)DAT_00757cc0);
          if (bVar2 == 0x31) {
            FUN_004dcfa0(0x271);
            FUN_004dcfa0(0x270);
            FUN_004dcfa0(0x26f);
            FUN_004dbf00();
          }
          else {
            iVar7 = bVar2 - 0x30;
            pcVar5 = FUN_004dcfa0(0x206);
            _sprintf(DAT_0081b740, pcVar5, iVar7);
            FUN_004dcfa0(0x207);
            FUN_004dbf00();
          }
        }
        else {
          if (((V_BFBA == 2) || (V_BFBA == 4)) && (iVar7 = FUN_004e6630(), iVar7 == 0)) {
            FUN_004da050();
            return 0;
          }
          FUN_004e5320(0);
          DAT_00758628 = V_BFBA;
          DAT_00758624 = LD32(SEL(DAT_0081bfbe, V_BFBA));
          DAT_0075862c = V_C012;
          DAT_0081bcb8 = 1;
          DAT_00758620 = 0x49;
          uVar12 = LD32(DAT_0081d32a);
          if ((V_BFBA == 1) || (uVar12 = LD32(DAT_0081d40a), V_BFBA == 3)) {
            DAT_00758630 = uVar12;
          }
          FUN_004cb830(0, &DAT_00758620, 0x14);
        }
      }
    }
  }
  FUN_004da050();
  iVar7 = FUN_004cc810();
  if ((iVar7 != 0) || (DAT_0081b850 == 2)) {
    FUN_004dcfd0(aiStack_10, 0x1a1, 0x70, 0x1b2, 0x82);
    pcVar6 = s_buttonsound_00531310;
    iVar13 = 9;
    uVar12 = 0xffff;
    iVar7 = 0xf;
    pcVar5 = FUN_004dcfa0(0x29e);
    iVar7 = FUN_004d9a80(aiStack_10, s_dropbtnup_00533110, s_dropbtndown_0053311c, pcVar5, iVar7,
                         (int)uVar12, iVar13, pcVar6);
    if (iVar7 != 0) {
      FUN_004dcfd0(aiStack_10, 0, 0, 0x27f, 0x1df);
      FUN_004db6c0(DAT_004e56b0, aiStack_10);
    }
  }
  iVar7 = FUN_004e2910();
  if (((DAT_0081b850 != 2) && (iVar13 = FUN_004cc810(), iVar13 != 0)) && (iVar7 != 0)) {
    FUN_004e5320(0);
  }
  iVar7 = FUN_004c0750(1);
  if (iVar7 != 1) {
    if (((DAT_0081b850 == 4) && (iVar7 = FUN_004cc810(), iVar7 != 0)) && (DAT_0064ff30 == 0)) {
      FUN_004dcfd0(aiStack_10, 0x1a1, 0x135, 0x1b0, 0x150);
      pcVar6 = s_buttonsound_00531310;
      iVar13 = 10;
      uVar12 = 0;
      iVar7 = 0xc;
      pcVar5 = FUN_004dcfa0(0x205);
      iVar7 = FUN_004d9a80(aiStack_10, s_bootu_00533100, s_bootd_00533108, pcVar5, iVar7,
                           (int)uVar12, iVar13, pcVar6);
      if ((iVar7 != 0) && (DAT_0064ff38 != -1)) {
        DAT_00758620 = 0x5b;
        FUN_004cb830((int)LD32(DAT_00757cee + (unsigned int)DAT_0064ff38 * 0x2au),
                     &DAT_00758620, 4);
        FUN_004cddb0((int)LD32(DAT_00757cee + (unsigned int)DAT_0064ff38 * 0x2au));
        FUN_004e5320(0);
      }
    }
    return 0;
  }
  return 1;
}
