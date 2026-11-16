/* SPDX-License-Identifier: MIT */
/* z_xvt 00436f10 */
/* Evidence gaps:
 * - Global element types are inferred from the decompile and P-code sizes,
 *   otherwise guessed (int / byte).
 * - DAT_0070a174 is called as __cdecl; its real convention is unconfirmed.
 * - FUN_0045ba90 takes its argument in ECX (Ghidra's extraout_ECX). It is declared
 *   __fastcall with one pointer parameter. The original sets no ECX before the call,
 *   so the local is left uninitialised and read through its address, mirroring the
 *   leftover ECX without tripping -Wuninitialized.
 * - DAT_0071b740 lies inside DAT_0071b730's range in the original; declared separately.
 * - The FILE* at DAT_00732f60 is declared void* since FILE is not known to be in xvt.h.
 */

typedef unsigned char byte_t;

/* prototypes (callee-cleanup per stack-check findings: RET 0x0 => __cdecl) */
void  __stdcall FUN_004a5320(void);
void  __stdcall FUN_00409b60(void);
void  __stdcall FUN_0049b2c0(void);
DWORD __stdcall timeGetTime(void);
void  __stdcall FUN_0048f520(void);
int   __stdcall FUN_00432cb0(void);
int   __cdecl   FUN_00432c10(int);
int   __stdcall FUN_00431cf0(void);
void  __stdcall FUN_00425280(void);
void  __cdecl   FUN_004810f0(char *);
void  __stdcall FUN_004812d0(void);
void  __stdcall FUN_004a6a80(void);
void  __stdcall FUN_004378f0(void);
void  __stdcall FUN_004a6f90(void);
void  __stdcall FUN_00462400(void);
void  __stdcall FUN_00461db0(void);
void  __stdcall FUN_004a7120(void);
void  __stdcall FUN_004a7040(void);
void  __cdecl   FUN_00405fd0(int);
void  __cdecl   FUN_0047bfe0(char *, byte_t *);
void  __stdcall FUN_00462410(void);
void  __stdcall FUN_0047c090(void);
void  __stdcall FUN_0047cb30(void);
int   __cdecl   FUN_0047ed40(char *, void *, int, int);
void  __cdecl   FUN_0047ee70(int);
void  __stdcall FUN_00405c50(void);
int   __cdecl   _rand(void);
void  __cdecl   FUN_0041baf0(char *);
void  __stdcall FUN_00409d90(void);
void  __stdcall FUN_0047d810(void);
void  __stdcall FUN_00438dd0(void);
void  __stdcall FUN_0041d620(void);
int   __stdcall FUN_004390b0(void);
void  __cdecl   FUN_004395a0(unsigned int);
unsigned int __stdcall FUN_004784d0(void);
void  __cdecl   FUN_004392c0(int, unsigned int, unsigned int);
int   __cdecl   FUN_00439530(int);
void  __stdcall FUN_00401250(void);
void  __cdecl   FUN_00401960(unsigned short);
void  __cdecl   FUN_004a1d70(int, int);
int   __stdcall FUN_0048f530(void);
void  __stdcall FUN_004230f0(void);
int   __stdcall FUN_0042bc90(void);
void  __stdcall thunk_FUN_00491150(void);
void  __stdcall FUN_004398e0(void);
void  __stdcall FUN_0047ce40(void);
void  __stdcall FUN_0048f580(void);
void  __cdecl   FUN_004395b0(unsigned int, unsigned int, int);
void  __stdcall FUN_00439400(void);
void  __stdcall FUN_00498fc0(void);
void  __stdcall FUN_0049c1f0(void);
void  __stdcall FUN_00499090(void);
void  __fastcall FUN_0045ba90(void *);
void  __stdcall FUN_00433140(void);
int   __stdcall FUN_0042c130(void);
void  __stdcall FUN_00437ab0(void);
void  __stdcall FUN_00499040(void);
int   __cdecl   _fclose(void *);
void  __stdcall FUN_00478830(void);
void  __cdecl   FUN_004c04d0(int);

/* globals */
extern int DAT_006db57c, DAT_0071e51c, DAT_00525544, DAT_0052556c;
extern int DAT_0070aaf0[];
extern int DAT_005257e8, DAT_005257ec;
extern byte_t DAT_0071f2eb;
extern int DAT_0052ae54, DAT_0052551c, DAT_00525520, DAT_0052552c;
extern int DAT_00525550, _DAT_00732f40, _DAT_007307e4, DAT_0071ecf4, DAT_0072f444, DAT_00732e48;
extern unsigned int DAT_006da500[], DAT_00730850[], DAT_006fb7d0[];
extern int _DAT_00732ff0, _DAT_00732ff4, DAT_00525570, DAT_00525568;
extern int DAT_007092e0, DAT_007092e4;
extern short DAT_0081bd0e, DAT_0081bfba, DAT_0081c012;
extern byte_t DAT_007092d4, DAT_0085e7b4, DAT_007092d5, DAT_0085e7b5, DAT_007092d6, DAT_0085e7b6;
extern byte_t DAT_007092d7, DAT_0085e7b7, DAT_007092d8, DAT_0085e7b8, DAT_007092d9, DAT_0085e7bc;
extern byte_t DAT_007092db, DAT_0085e7bd, DAT_007092dc, DAT_007092dd, DAT_007092da;
extern byte_t DAT_0085e7be, DAT_0085e7bf, DAT_0085e7c4, DAT_007092df;
extern unsigned short DAT_00703abc, DAT_0085e7c0, DAT_006d9d90, DAT_0070a128;
extern unsigned int DAT_0071b730[];
extern int DAT_006db730[], DAT_00732920[], DAT_0070a150[];
extern byte_t DAT_0071bcbc[], DAT_0071bce9[], DAT_0071bce5[];
extern byte_t DAT_0071b740[];
extern int DAT_0052555c, DAT_00525560, DAT_0071e690, DAT_006fb7c0;
extern char s_paiplan_005257fc[], s_newpal_act_005257f0[], s_DEMO_TIE_00525588[];
extern int DAT_0070acd0, DAT_00525734, _DAT_0071b710, DAT_0052575c;
extern void *DAT_00732f60;
extern void (*DAT_0070a174)(byte_t *, int, int);
extern int DAT_00525518;
extern unsigned int DAT_00525574;
extern int DAT_0071b014;
extern byte_t *DAT_006da9f4;
extern int DAT_0052ae20;
extern byte_t DAT_007092c2, DAT_007092c3, DAT_007092c1;
extern byte_t DAT_006ff9e0[], DAT_006ff9e1[];
extern unsigned short DAT_00525714, DAT_00525710, DAT_0052f0e0, DAT_0052f0e4, DAT_0052f0ec, DAT_0052f0f0;
extern int DAT_0072f440, DAT_00703c6c, DAT_00703c70, DAT_0052571c, DAT_00703c84;
extern int DAT_00655140[], DAT_006559b0[];
extern int DAT_00732588, DAT_0073257c;
extern char DAT_0085e798;
extern byte_t DAT_0085e79e;
extern int DAT_0071e528, DAT_0072f4e4, DAT_006da844;
extern byte_t DAT_00525784[];
extern unsigned int DAT_00525788[];
extern int _DAT_00732950;
extern unsigned int DAT_007092c4;
extern int DAT_006db5a8;
extern int DAT_006d4988, DAT_00732a38;
extern void (*DAT_006db6c4)(void);
extern int DAT_00525530;
extern byte_t DAT_0071ed10[];

void __stdcall FUN_00436f10(void)
{
  char cVar1;
  char cVar2;
  char cVar3;
  byte_t bVar4;
  short sVar5;
  DWORD DVar6;
  int iVar7;
  unsigned int uVar8;
  short sVar9;
  void *extraout_ECX;
  int iVar10;
  unsigned int *puVar11;
  char *pcVar12;
  byte_t abStack_400[1024];
  unsigned int zero;
  int tmp;

  FUN_004a5320();
  sVar5 = 0;
  DAT_006db57c = 0;
  DAT_0071e51c = 0;
  DAT_00525544 = 0;
  DAT_0052556c = 0;
  do {
    iVar10 = (int)sVar5;
    sVar5 = (short)(sVar5 + 1);
    DAT_0070aaf0[iVar10] = 0;
  } while (sVar5 < 8);
  FUN_00409b60();
  FUN_0049b2c0();
  DAT_005257e8 = 0;
  DAT_005257ec = (int)timeGetTime();
  FUN_0048f520();
  DAT_0071f2eb = 1;
  DAT_0052ae54 = 1;
  if (DAT_0052551c == 0) {
    if (DAT_00525520 == 0x140) {
      DAT_0052552c = 0x13;
      goto LAB_00436fc8;
    }
    if (DAT_00525520 == 0x1e0) {
      DAT_0052552c = 0x112;
      goto LAB_00436fc8;
    }
  }
  DAT_0052552c = 0x101;
LAB_00436fc8:
  DAT_00525550 = 0;
  _DAT_00732f40 = 0;
  _DAT_007307e4 = 0;
  iVar10 = FUN_00432cb0();
  DAT_0071ecf4 = FUN_00432c10(iVar10);
  DAT_0072f444 = FUN_00431cf0();
  puVar11 = DAT_006da500;
  DAT_00732e48 = DAT_0072f444;
  for (iVar10 = 0xc; iVar10 != 0; iVar10 = iVar10 + -1) {
    *puVar11 = 0;
    puVar11 = puVar11 + 1;
  }
  puVar11 = DAT_00730850;
  for (iVar10 = 0xc; iVar10 != 0; iVar10 = iVar10 + -1) {
    *puVar11 = 0;
    puVar11 = puVar11 + 1;
  }
  puVar11 = DAT_006fb7d0;
  for (iVar10 = 0xc0; iVar10 != 0; iVar10 = iVar10 + -1) {
    *puVar11 = 0;
    puVar11 = puVar11 + 1;
  }
  _DAT_00732ff0 = 0;
  _DAT_00732ff4 = 0;
  DAT_00525570 = DAT_00525568;
  DAT_007092e0 = DAT_0072f444;
  DAT_007092e4 = DAT_0072f444;
  if ((DAT_0081bd0e < 2) || (DAT_0081bfba < 3)) {
    DAT_007092d4 = DAT_0085e7b4;
  }
  else {
    DAT_007092d4 = 1;
  }
  DAT_007092d5 = DAT_0085e7b5;
  DAT_007092d6 = DAT_0085e7b6;
  DAT_007092d7 = DAT_0085e7b7;
  DAT_007092d8 = DAT_0085e7b8;
  DAT_007092d9 = DAT_0085e7bc;
  DAT_007092db = DAT_0085e7bd;
  if (DAT_0081bd0e < 2) {
    DAT_007092dc = 0xff;
    DAT_007092dd = 0;
    DAT_007092da = 1;
  }
  else {
    DAT_007092dc = DAT_0085e7be;
    DAT_007092dd = DAT_0085e7bf;
    DAT_007092da = DAT_0085e7c4;
  }
  DAT_007092df = 1;
  if ((2 < DAT_0081bfba) && (DAT_0081c012 == 1)) {
    DAT_007092d7 = 0;
  }
  if (DAT_0072f444 == 1) {
    DVar6 = timeGetTime();
    DAT_00703abc = (unsigned short)((unsigned short)DVar6 ^ 0xbeef);
  }
  else {
    DAT_00703abc = DAT_0085e7c0;
  }
  DVar6 = timeGetTime();
  DAT_006d9d90 = 0xaced;
  DAT_0070a128 = (unsigned short)((unsigned int)DAT_00703abc + (unsigned int)(int)(short)DVar6);
  puVar11 = DAT_0071b730;
  for (iVar10 = 0xb7a; iVar10 != 0; iVar10 = iVar10 + -1) {
    *puVar11 = 0;
    puVar11 = puVar11 + 1;
  }
  sVar5 = 0;
  zero = 0;
  do {
    iVar10 = (int)sVar5;
    sVar5 = (short)(sVar5 + 1);
    DAT_006db730[iVar10] = 0;
    DAT_00732920[iVar10] = 1;
    DAT_0070a150[iVar10] = 0;
    __builtin_memcpy(DAT_0071bcbc + (unsigned int)iVar10 * 0x5bd, &zero, 4);
    __builtin_memcpy(DAT_0071bce9 + (unsigned int)iVar10 * 0x5bd, &zero, 4);
    __builtin_memcpy(DAT_0071bce5 + (unsigned int)iVar10 * 0x5bd, &zero, 4);
  } while (sVar5 < 8);
  sVar5 = 0;
  do {
    iVar10 = (int)sVar5;
    sVar5 = (short)(sVar5 + 1);
    DAT_0071b740[iVar10 * 0x5bd] = 0;
  } while (sVar5 < 8);
  sVar5 = 0;
  if (0 < DAT_0072f444) {
    do {
      iVar10 = DAT_0072f444;
      iVar7 = (int)sVar5;
      sVar5 = (short)(sVar5 + 1);
      DAT_0071b740[iVar7 * 0x5bd] = 1;
    } while (sVar5 < iVar10);
  }
  DAT_0052555c = 0;
  DAT_00525560 = 0;
  DAT_0071e690 = -1;
  if (DAT_006fb7c0 == 0) {
    FUN_00425280();
  }
  sVar5 = 0;
  FUN_004810f0(s_paiplan_005257fc);
  FUN_004812d0();
  DAT_0070acd0 = 0;
  DAT_00525734 = 0;
  _DAT_0071b710 = 0;
  DAT_00732f60 = (void *)0;
  DAT_0052575c = 1;
  FUN_004a6a80();
  FUN_004378f0();
  FUN_004a6f90();
  FUN_00462400();
  FUN_004a6a80();
  FUN_00461db0();
  FUN_004a6f90();
  FUN_004a7120();
  FUN_004a7040();
  FUN_00405fd0(3);
  FUN_0047bfe0(s_newpal_act_005257f0, abStack_400);
  do {
    iVar10 = (int)sVar5;
    sVar5 = (short)(sVar5 + 3);
    bVar4 = abStack_400[iVar10];
    abStack_400[iVar10] = (byte_t)(abStack_400[765 - iVar10] >> 2);
    abStack_400[765 - iVar10] = (byte_t)(bVar4 >> 2);
    bVar4 = abStack_400[iVar10 + 1];
    abStack_400[iVar10 + 1] = (byte_t)(abStack_400[765 + 1 - iVar10] >> 2);
    abStack_400[765 + 1 - iVar10] = (byte_t)(bVar4 >> 2);
    bVar4 = abStack_400[iVar10 + 2];
    abStack_400[iVar10 + 2] = (byte_t)(abStack_400[765 + 2 - iVar10] >> 2);
    abStack_400[765 + 2 - iVar10] = (byte_t)(bVar4 >> 2);
  } while (sVar5 < 0x180);
  (*DAT_0070a174)(abStack_400, 0, 0x100);
  FUN_00462410();
  FUN_004a6a80();
  FUN_0047c090();
  FUN_004a6f90();
  FUN_0047cb30();
  if (DAT_00525518 == 1) {
    iVar10 = -1;
    pcVar12 = s_DEMO_TIE_00525588;
    do {
      if (iVar10 == 0) break;
      iVar10 = iVar10 + -1;
      cVar1 = *pcVar12;
      pcVar12 = pcVar12 + 1;
    } while (cVar1 != '\0');
    iVar7 = (int)(short)(unsigned short)(~(unsigned short)iVar10 - 4);
    cVar1 = s_DEMO_TIE_00525588[iVar7];
    cVar2 = s_DEMO_TIE_00525588[iVar7 + 1];
    cVar3 = s_DEMO_TIE_00525588[iVar7 + 2];
    s_DEMO_TIE_00525588[iVar7] = 'p';
    s_DEMO_TIE_00525588[iVar7 + 1] = 'a';
    s_DEMO_TIE_00525588[iVar7 + 2] = 'l';
    iVar10 = FUN_0047ed40(s_DEMO_TIE_00525588, &DAT_00525574, 0, 0);
    if (iVar10 == 0) {
      DAT_0071b014 = 1;
    }
    else {
      DAT_0071b014 = 0;
      FUN_0047ee70(0);
      FUN_0047bfe0(s_DEMO_TIE_00525588, DAT_006da9f4);
      (*DAT_0070a174)(DAT_006da9f4, 0x40, 0xc0);
    }
    s_DEMO_TIE_00525588[iVar7] = cVar1;
    s_DEMO_TIE_00525588[iVar7 + 1] = cVar2;
    s_DEMO_TIE_00525588[iVar7 + 2] = cVar3;
  }
  if (DAT_0052ae20 == 0) {
    DAT_007092c2 = 0;
    DAT_007092c3 = 0;
  }
  else {
    DAT_007092c2 = 2;
    DAT_007092c3 = 4;
  }
  FUN_004a6a80();
  FUN_00405c50();
  FUN_004a6f90();
  for (sVar5 = 0; sVar5 < 0x1ff; sVar5 = (short)(sVar5 + 2)) {
    do {
      iVar10 = _rand();
      bVar4 = (byte_t)iVar10 & 0x7f;
      DAT_006ff9e0[sVar5] = bVar4;
    } while (0x7c < bVar4);
    iVar10 = _rand();
    DAT_006ff9e1[sVar5] = (byte_t)iVar10 & 3;
  }
  DAT_00525714 = 0;
  DAT_0072f440 = 0;
  DAT_00703c6c = 0x49d4;
  DAT_00703c70 = 0x49d4;
  DAT_0052571c = -1;
  DAT_00703c84 = 0x49d4;
  DAT_00525710 = 0xffff;
  DAT_0052f0e0 = 0xffff;
  DAT_0052f0e4 = 0xffff;
  DAT_0052f0ec = 0xffff;
  DAT_0052f0f0 = 0xffff;
  sVar5 = 0;
  do {
    sVar9 = (short)(sVar5 + 1);
    DAT_00655140[sVar5] = 0;
    DAT_006559b0[sVar5] = 0;
    sVar5 = sVar9;
  } while (sVar9 < 8);
  DAT_00732588 = 0;
  FUN_004a6a80();
  FUN_0041baf0(s_DEMO_TIE_00525588);
  FUN_004a6f90();
  FUN_00409d90();
  FUN_0047d810();
  if (DAT_0071b014 != 0) {
    iVar10 = -1;
    pcVar12 = s_DEMO_TIE_00525588;
    do {
      if (iVar10 == 0) break;
      iVar10 = iVar10 + -1;
      cVar1 = *pcVar12;
      pcVar12 = pcVar12 + 1;
    } while (cVar1 != '\0');
    iVar10 = (int)(short)(unsigned short)(~(unsigned short)iVar10 - 4);
    cVar1 = s_DEMO_TIE_00525588[iVar10];
    cVar2 = s_DEMO_TIE_00525588[iVar10 + 1];
    cVar3 = s_DEMO_TIE_00525588[iVar10 + 2];
    s_DEMO_TIE_00525588[iVar10] = 'p';
    s_DEMO_TIE_00525588[iVar10 + 1] = 'a';
    s_DEMO_TIE_00525588[iVar10 + 2] = 'l';
    s_DEMO_TIE_00525588[iVar10] = cVar1;
    s_DEMO_TIE_00525588[iVar10 + 1] = cVar2;
    s_DEMO_TIE_00525588[iVar10 + 2] = cVar3;
  }
  bVar4 = (byte_t)DAT_005257e8;
  while ((bVar4 & 0x7f) != 0x7f) {
    FUN_00438dd0();
    bVar4 = (byte_t)DAT_005257e8;
  }
  FUN_00438dd0();
  FUN_004a6a80();
  FUN_0041d620();
  FUN_004a6f90();
  DAT_0073257c = 0;
  if (((DAT_0085e798 == '\0') || (DAT_0085e79e == 0)) || (iVar10 = FUN_004390b0(), iVar10 == 0)) {
    DAT_0071e528 = 0x7fffffff;
    DAT_0072f4e4 = 0;
  }
  else {
    FUN_004395a0((((unsigned int)DAT_0085e79e * 0xffff) / 9) & 0xffff);
    uVar8 = FUN_004784d0();
    uVar8 = uVar8 & 3;
    FUN_004392c0(2, (unsigned int)DAT_00525784[uVar8], DAT_00525788[uVar8]);
    iVar10 = FUN_00439530(2);
    DAT_0071e528 = (int)((unsigned int)iVar10 +
                         (unsigned int)DAT_00525784[uVar8] * (unsigned int)-60000 +
                         DAT_00525788[uVar8] * (unsigned int)-1000);
    DAT_006da844 = (int)timeGetTime();
    DAT_0072f4e4 = 2;
  }
  if (DAT_007092c1 != 0) {
    FUN_00401250();
    FUN_00401960((unsigned short)DAT_007092c3);
    if ((DAT_007092c1 != 0) && (1 < DAT_007092c3)) {
      _DAT_00732950 = (int)DAT_007092c3 - 1;
      FUN_004a1d70(0xc4, DAT_0071ecf4);
      DAT_007092c4 = ((unsigned int)DAT_007092c3 * 5 + (unsigned int)-5) * 2000;
    }
  }
  do {
    iVar10 = FUN_0048f530();
    DAT_006db5a8 = (int)((unsigned int)DAT_006db5a8 + (unsigned int)iVar10);
  } while (DAT_006db5a8 == 0);
  FUN_004230f0();
  iVar10 = FUN_0042bc90();
  if (iVar10 != 0) {
    sVar5 = 0;
    _DAT_0071b710 = 0;
    if (0 < DAT_00732a38) {
      do {
        __builtin_memcpy(&iVar10,
                         (char *)(unsigned int)((unsigned int)((int)sVar5 * 0x23 + 0x1f) +
                                                (unsigned int)DAT_006d4988),
                         4);
        if (iVar10 != 0) {
          tmp = 0;
          __builtin_memcpy((char *)(unsigned int)((unsigned int)iVar10 + 2), &tmp, 4);
        }
        sVar5 = (short)(sVar5 + 1);
      } while (sVar5 < DAT_00732a38);
    }
    FUN_00498fc0();
    FUN_0049c1f0();
    FUN_00499090();
    FUN_0045ba90(*(void **)&extraout_ECX);
    FUN_00433140();
    iVar10 = FUN_0042c130();
    if (iVar10 != 0) {
      FUN_00437ab0();
    }
    FUN_00499040();
    if (DAT_00732f60 != (void *)0) {
      _fclose(DAT_00732f60);
    }
    (*DAT_006db6c4)();
    FUN_00478830();
    DAT_0071f2eb = 0;
    thunk_FUN_00491150();
    FUN_0047ce40();
    if (DAT_00525530 != DAT_0052552c) {
      FUN_0048f580();
    }
    FUN_004c04d0(0);
    if ((DAT_0085e798 != '\0') && (DAT_0085e79e != 0)) {
      uVar8 = ((unsigned int)DAT_0085e79e * 0xffff) / 9;
      FUN_004395b0(uVar8 & 0xffff, uVar8 >> 3 & 0x1fff, 1000);
    }
    FUN_00439400();
    return;
  }
  DAT_0071f2eb = 0;
  thunk_FUN_00491150();
  FUN_004398e0();
  FUN_0047ce40();
  if (DAT_00525530 != DAT_0052552c) {
    FUN_0048f580();
  }
  __builtin_memcpy(DAT_0071ed10,
                   (char *)DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd,
                   0x5bd);
  if ((DAT_0085e798 != '\0') && (DAT_0085e79e != 0)) {
    uVar8 = ((unsigned int)DAT_0085e79e * 0xffff) / 9;
    FUN_004395b0(uVar8 & 0xffff, uVar8 >> 3 & 0x1fff, 1000);
  }
  FUN_00439400();
  return;
}
