/* SPDX-License-Identifier: MIT */
/* z_xvt 004fa230 */
/* Globals whose addresses are not multiples of their width are read through
   memcpy (packed). The macros below only wrap __builtin_memcpy and wrapping
   unsigned arithmetic; they are not helper functions. */
#define LD32(p) ({ int _t; __builtin_memcpy(&_t, (const void *)(p), 4); _t; })
#define EL(base, i, s) LD32((char *)&(base) + (unsigned int)(i) * ((s) * 4U))
#define ADD(a, b) ((int)((unsigned int)(a) + (unsigned int)(b)))
#define SUB(a, b) ((int)((unsigned int)(a) - (unsigned int)(b)))

extern char DAT_00534304;
extern char DAT_0081b740;
extern char DAT_005321b8;
extern char DAT_0075637a;
extern char DAT_007438e4;
extern char DAT_007438d8;
extern char DAT_00743880;
extern char DAT_0081bc80;

extern char s__c_s__c_s_00531334[];
extern char s__c_s__c_d__s__d__s_005342f0[];
extern char s__c_d___c_s_005342e4[];
extern char s__d___d__005321bc[];
extern char s__c_d___c_s__c_s_005342d4[];
extern char s___s__s__005342cc[];
extern char s__c_d___c__s__s__005342bc[];
extern char s__d__d__005321a8[];

extern int DAT_0081c012;
extern int DAT_0081bfba;
extern int DAT_0081bfb6;
extern int DAT_0081d32a;
extern int DAT_0081d326;
extern int DAT_0081d402;
extern int DAT_0081d40a;
extern int DAT_0081d40e;
extern unsigned int DAT_0081bcd0;
extern unsigned int _DAT_0081b6fc;
extern int DAT_0081b850;
extern unsigned int DAT_00654c18;
extern unsigned int DAT_00654c40;
extern int DAT_00654c60;
extern int DAT_00654bd0;
extern int DAT_00654c88;
extern int DAT_00654ca8;
extern int DAT_00654cb8;
extern int DAT_00654cd8;
extern int DAT_00654d44;
extern short DAT_0075766c;
extern int DAT_0081fa7e;
extern int DAT_0081fa8e;
extern int DAT_0081fa92;
extern int DAT_0081fa96;
extern int DAT_0081fa9e;
extern int DAT_0081e426;
extern unsigned int DAT_0081f7cc;
extern int DAT_0081f7da;
extern int DAT_0081f7de;
extern int DAT_0081f7e2;
extern int DAT_0081f7e6;
extern int DAT_0081f7ea;
extern int DAT_0081f7ee;
extern int DAT_0081f7fa;
extern int DAT_0081f812;
extern int DAT_0081e0a6;
extern int DAT_0081e0c6;
extern int DAT_0081e0e6;
extern int DAT_0081e1a6;
extern int DAT_0081e266;
extern int DAT_0081e286;
extern int DAT_0081e2a6;
extern int DAT_0081e366;

unsigned char __cdecl FUN_004dbe00(int param_1);
void __cdecl FUN_004d0e60(char *param_1, int param_2, int param_3);
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *__cdecl FUN_004dcfa0(unsigned int param_1);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
void __cdecl FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
void __cdecl FUN_004d5180(void *param_1);
void __cdecl FUN_004d51b0(void *param_1);
int __cdecl FUN_004cdbc0(void);

unsigned int __cdecl FUN_004fa230(int param_1)
{
  short sVar2;
  char *pcVar3;
  int iVar4;
  char *pcVar5;
  char *pcVar6;
  int *piVar7;
  int iVar8;
  unsigned int *puVar9;
  int iVar10;
  int *piVar11;
  int iVar12;
  int iVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned int uVar16;
  unsigned int *puVar17;
  int local_38;
  int local_34;
  int *local_30[3];
  int local_24;
  int local_20[4];
  unsigned int local_10[4];

  if ((FUN_004dbe00(0x12) != 0) && (FUN_004dbe00(0x10) != 0) && (FUN_004dbe00(0x11) != 0)) {
    FUN_004d0e60(&DAT_00534304, 0xb8, 0x16a);
  }
  FUN_004dcfd0(local_20, 0x54, 0x5a, 0x1b2, 0x6a);
  if (LD32(&DAT_0081c012) == 0) {
    piVar7 = local_20;
    uVar16 = 0xffff;
    pcVar3 = FUN_004dcfa0(0x12a);
    goto LAB_004fa430;
  }
  if (LD32(&DAT_0081bfba) == 1) {
    if (SUB(LD32(&DAT_0081d32a), LD32(&DAT_0081d326)) == 1) {
      pcVar3 = FUN_004dcfa0(0xc1);
      uVar16 = 0x2a1;
LAB_004fa404:
      uVar14 = 4;
      pcVar5 = FUN_004dcfa0(uVar16);
      _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 1, pcVar5, uVar14, pcVar3);
    }
    else {
      pcVar3 = FUN_004dcfa0(0x147);
      iVar10 = LD32(&DAT_0081d32a);
      pcVar5 = FUN_004dcfa0(0x142);
      iVar12 = ADD(LD32(&DAT_0081d326), 1);
      uVar14 = 4;
      pcVar6 = FUN_004dcfa0(0x2a1);
      _sprintf(&DAT_0081b740, s__c_s__c_d__s__d__s_005342f0, 1, pcVar6, uVar14, iVar12, pcVar5,
               iVar10, pcVar3);
    }
  }
  else {
    local_30[0] = (int *)0x0;
    local_30[1] = (int *)0x0;
    local_30[2] = (int *)0x0;
    if (-1 < LD32(&DAT_0081d402)) {
      piVar7 = &DAT_0081d40e;
      iVar10 = ADD(LD32(&DAT_0081d402), 1);
      do {
        __builtin_memcpy(&iVar12, piVar7, 4);
        piVar7 = piVar7 + 1;
        iVar10 = SUB(iVar10, 1);
        local_30[iVar12] = (int *)((unsigned int)local_30[iVar12] + 1);
      } while (iVar10 != 0);
    }
    if (LD32(&DAT_0081bfb6) == 0) {
      iVar10 = 0;
    }
    else if (LD32(&DAT_0081bfb6) == 1) {
      iVar10 = 1;
    }
    else {
      iVar10 = 0;
    }
    if ((local_30[0] == (int *)LD32(&DAT_0081d40a)) || (local_30[1] == (int *)LD32(&DAT_0081d40a))) {
      pcVar3 = FUN_004dcfa0(0xc1);
      uVar16 = 0x2a3;
      goto LAB_004fa404;
    }
    pcVar3 = FUN_004dcfa0(0x2a4);
    piVar11 = (int *)LD32(&DAT_0081d40a);
    pcVar5 = FUN_004dcfa0(0x142);
    piVar7 = local_30[iVar10];
    uVar14 = 4;
    pcVar6 = FUN_004dcfa0(0x2a3);
    _sprintf(&DAT_0081b740, s__c_s__c_d__s__d__s_005342f0, 1, pcVar6, uVar14, piVar7, pcVar5,
             piVar11, pcVar3);
  }
  piVar7 = local_20;
  uVar16 = 0xffff;
  pcVar3 = &DAT_0081b740;
LAB_004fa430:
  FUN_004da920(0xf, pcVar3, piVar7, uVar16);
  iVar12 = 0x72;
  iVar10 = 0xe4;
  uVar16 = DAT_0081bcd0;
  pcVar3 = FUN_004dcfa0(0x13d);
  FUN_004da7c0(0xc, pcVar3, iVar10, iVar12, uVar16);
  iVar12 = 0x72;
  iVar10 = 0x12a;
  uVar16 = DAT_0081bcd0;
  pcVar3 = FUN_004dcfa0(0x143);
  FUN_004da7c0(0xc, pcVar3, iVar10, iVar12, uVar16);
  iVar13 = 0x72;
  iVar12 = 0x16b;
  iVar10 = 0x86;
  uVar16 = DAT_0081bcd0;
  pcVar3 = FUN_004dcfa0(0x20d);
  FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar16);
  puVar9 = &DAT_00654c18;
  local_34 = 0;
  local_24 = 0;
  local_38 = EL(DAT_0081fa7e, DAT_00654c18, 7);
  do {
    iVar12 = 0x58;
    uVar16 = *puVar9;
    if (uVar16 == 0xffffffff) break;
    if ((&DAT_00654c60)[uVar16] != 0) {
      if (local_38 != EL(DAT_0081fa7e, uVar16, 7)) {
        local_34 = local_24;
        local_38 = EL(DAT_0081fa7e, uVar16, 7);
      }
      sVar2 = (short)((local_34 < 3) + 1);
      if (DAT_00654d44 == 0) {
        if ((LD32(&DAT_0081bfba) == 1) || (LD32(&DAT_0081bfba) == 2)) {
          _sprintf(&DAT_0081b740, s__c_d___c_s_005342e4, sVar2, ADD(local_34, 1), 1,
                   &DAT_0075637a + uVar16 * 0x1e5);
          FUN_004da7c0(0xc, &DAT_0081b740, 0x58, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa7e, *puVar9, 7));
          FUN_004da7c0(0xc, &DAT_0081b740, 0xe4, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, s__d___d__005321bc, EL(DAT_0081fa8e, *puVar9, 7),
                   EL(DAT_0081fa92, *puVar9, 7));
          FUN_004da7c0(0xc, &DAT_0081b740, 0x12a, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa96, *puVar9, 7));
          iVar12 = 0x67;
          FUN_004da7c0(0xc, &DAT_0081b740, 0x16b, iVar10, 0xffff);
          iVar10 = ADD(iVar10, 0xf);
        }
        else if ((&DAT_00654bd0)[uVar16] == 0) {
          _sprintf(&DAT_0081b740, s__c_d___c_s_005342e4, sVar2, ADD(local_34, 1), 1,
                   &DAT_0075637a + uVar16 * 0x1e5);
          iVar12 = 0x67;
          FUN_004da7c0(0xc, &DAT_0081b740, 0x58, iVar10, 0xffff);
          iVar10 = ADD(iVar10, 0xf);
        }
        else {
          _sprintf(&DAT_0081b740, s__c_d___c_s_005342e4, sVar2, ADD(local_34, 1), 1,
                   &DAT_0075637a + uVar16 * 0x1e5);
          FUN_004da7c0(0xc, &DAT_0081b740, 0x58, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa7e, *puVar9, 7));
          FUN_004da7c0(0xc, &DAT_0081b740, 0xe4, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, s__d___d__005321bc, EL(DAT_0081fa8e, *puVar9, 7),
                   EL(DAT_0081fa92, *puVar9, 7));
          FUN_004da7c0(0xc, &DAT_0081b740, 0x12a, iVar10, 0xffff);
          _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa96, *puVar9, 7));
          iVar12 = 0x67;
          FUN_004da7c0(0xc, &DAT_0081b740, 0x16b, iVar10, 0xffff);
          iVar10 = ADD(iVar10, 0xf);
        }
      }
      else if (((&DAT_00654bd0)[uVar16] != 0) && (iVar13 = 0, 0 < DAT_0075766c)) {
        local_30[0] = &DAT_0081e426;
        pcVar3 = &DAT_007438e4;
        iVar8 = iVar10;
        do {
          iVar10 = iVar8;
          if ((*pcVar3 != '\0') && ((unsigned int)(unsigned char)pcVar3[-0xc] == *puVar9)) {
            pcVar5 = pcVar3 + -100;
            uVar14 = 1;
            pcVar6 = FUN_004dcfa0(ADD(LD32(local_30[0]), 0x8e));
            _sprintf(&DAT_0081b740, s__c_d___c_s__c_s_005342d4, sVar2, ADD(local_34, 1), 6, pcVar6,
                     uVar14, pcVar5);
            FUN_004da7c0(0xc, &DAT_0081b740, iVar12, iVar8, 0xffff);
            _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa7e, *puVar9, 7));
            FUN_004da7c0(0xc, &DAT_0081b740, 0xe4, iVar8, 0xffff);
            _sprintf(&DAT_0081b740, s__d___d__005321bc, EL(DAT_0081fa8e, *puVar9, 7),
                     EL(DAT_0081fa92, *puVar9, 7));
            iVar12 = 0x16b;
            FUN_004da7c0(0xc, &DAT_0081b740, 0x12a, iVar8, 0xffff);
            _sprintf(&DAT_0081b740, &DAT_005321b8, EL(DAT_0081fa96, *puVar9, 7));
            iVar10 = ADD(iVar8, 0xf);
            FUN_004da7c0(0xc, &DAT_0081b740, 0x16b, iVar8, 0xffff);
          }
          pcVar3 = pcVar3 + 0x562;
          local_30[0] = local_30[0] + 1;
          iVar13 = ADD(iVar13, 1);
          iVar8 = iVar10;
        } while (iVar13 < DAT_0075766c);
      }
      if ((&DAT_00654bd0)[*puVar9] == 0) {
        piVar7 = &DAT_00654c88;
        iVar13 = iVar10;
        do {
          iVar8 = *piVar7;
          iVar10 = iVar13;
          if (iVar8 == -1) break;
          if ((unsigned int)(unsigned char)(&DAT_007438d8)[(int)((unsigned int)EL(DAT_0081f7da, iVar8, 0x16) * 0x562U)] == *puVar9) {
            FUN_004dcfd0(local_20, iVar12, iVar13, 0xe1, ADD(iVar13, 0xe));
            FUN_004d5180(local_10);
            FUN_004d51b0(local_20);
            if (DAT_00654d44 == 0) {
              puVar17 = &DAT_0081f7cc + (unsigned int)iVar8 * 0x16U;
              if (EL(DAT_0081f812, iVar8, 0x16) == 0) {
                uVar14 = 1;
                pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar8, 0x16), 0x8e));
                _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, uVar14, puVar17);
              }
              else {
                pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar8, 0x16), 0x8e));
                _sprintf(&DAT_0081b740, s___s__s__005342cc, pcVar3);
              }
            }
            else {
              puVar17 = &DAT_0081f7cc + (unsigned int)iVar8 * 0x16U;
              if (EL(DAT_0081f812, iVar8, 0x16) == 0) {
                uVar14 = 1;
                pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar8, 0x16), 0x8e));
                _sprintf(&DAT_0081b740, s__c_d___c_s__c_s_005342d4, sVar2, ADD(local_34, 1), 6,
                         pcVar3, uVar14, puVar17);
              }
              else {
                pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar8, 0x16), 0x8e));
                _sprintf(&DAT_0081b740, s__c_d___c__s__s__005342bc, sVar2, ADD(local_34, 1), 1,
                         pcVar3);
              }
            }
            uVar16 = _DAT_0081b6fc;
            if ((EL(DAT_0081f812, iVar8, 0x16) == 0) &&
                ((iVar10 = FUN_004cdbc0(), iVar10 == EL(DAT_0081f7de, iVar8, 0x16)) ||
                 (uVar16 = DAT_0081bcd0, DAT_0081b850 == 2))) {
              uVar16 = *(unsigned int *)(&DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
            }
            FUN_004da7c0(0xc, &DAT_0081b740, iVar12, iVar13, uVar16);
            FUN_004d51b0(local_10);
            if (((LD32(&DAT_0081bfba) == 1) || (LD32(&DAT_0081bfba) == 2)) && (DAT_00654d44 != 0)) {
              uVar14 = EL(DAT_0081fa7e, *puVar9, 7);
            }
            else {
              uVar14 = EL(DAT_0081f7e6, iVar8, 0x16);
            }
            _sprintf(&DAT_0081b740, &DAT_005321b8, uVar14);
            FUN_004da7c0(0xc, &DAT_0081b740, 0xe4, iVar13, 0xffff);
            if (DAT_00654d44 == 0) {
              _sprintf(&DAT_0081b740, s__d___d__005321bc, EL(DAT_0081f7ea, iVar8, 0x16),
                       EL(DAT_0081f7ee, iVar8, 0x16));
              FUN_004da7c0(0xc, &DAT_0081b740, 0x12a, iVar13, 0xffff);
              uVar14 = EL(DAT_0081f7fa, iVar8, 0x16);
            }
            else {
              _sprintf(&DAT_0081b740, s__d___d__005321bc, EL(DAT_0081fa8e, *puVar9, 7),
                       EL(DAT_0081fa92, *puVar9, 7));
              FUN_004da7c0(0xc, &DAT_0081b740, 0x12a, iVar13, 0xffff);
              uVar14 = EL(DAT_0081fa96, *puVar9, 7);
            }
            _sprintf(&DAT_0081b740, &DAT_005321b8, uVar14);
            iVar10 = ADD(iVar13, 0xf);
            FUN_004da7c0(0xc, &DAT_0081b740, 0x16b, iVar13, 0xffff);
            iVar12 = ADD((-(unsigned int)(DAT_00654d44 == 0) & 0xf), 0x58);
          }
          piVar7 = piVar7 + 1;
          iVar13 = iVar10;
        } while (piVar7 < &DAT_00654ca8);
      }
      if (((LD32(&DAT_0081bfba) == 1) || (LD32(&DAT_0081bfba) == 2)) &&
          ((DAT_00654d44 == 0 && (iVar13 = 0, 0 < DAT_0075766c)))) {
        local_30[0] = &DAT_0081e426;
        pcVar3 = &DAT_007438e4;
        iVar8 = iVar10;
        do {
          iVar10 = iVar8;
          if ((*pcVar3 != '\0') && ((unsigned int)(unsigned char)pcVar3[-0xc] == *puVar9)) {
            iVar4 = 0;
            piVar7 = &DAT_0081f7de;
            do {
              if ((LD32(piVar7) != 0) && (LD32(piVar7 - 1) == iVar13)) break;
              piVar7 = piVar7 + 0x16;
              iVar4 = ADD(iVar4, 1);
            } while (piVar7 < &DAT_0081fa9e);
            if (iVar4 == 8) {
              pcVar5 = pcVar3 + -100;
              uVar14 = 1;
              pcVar6 = FUN_004dcfa0(ADD(LD32(local_30[0]), 0x8e));
              _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar6, uVar14, pcVar5);
              iVar10 = ADD(iVar8, 0xf);
              FUN_004da7c0(0xc, &DAT_0081b740, iVar12, iVar8, 0xffff);
            }
          }
          local_30[0] = local_30[0] + 1;
          pcVar3 = pcVar3 + 0x562;
          iVar13 = ADD(iVar13, 1);
          iVar8 = iVar10;
        } while (iVar13 < DAT_0075766c);
      }
    }
    puVar9 = puVar9 + 1;
    local_24 = ADD(local_24, 1);
  } while (puVar9 < &DAT_00654c40);
  iVar8 = ADD(iVar10, 0xf);
  iVar10 = ADD(iVar10, 0x1e);
  iVar13 = iVar8;
  uVar16 = DAT_0081bcd0;
  pcVar3 = FUN_004dcfa0(0x210);
  FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar16);
  local_30[0] = &DAT_00654cb8;
  iVar12 = iVar10;
  do {
    iVar13 = *local_30[0];
    if (iVar13 == -1) break;
    if (iVar13 < 8) {
      iVar4 = FUN_004cdbc0();
      if ((iVar4 != EL(DAT_0081f7de, iVar13, 0x16)) &&
          ((EL(DAT_0081e0a6, iVar13, 1) != 0 || (EL(DAT_0081e0c6, iVar13, 1) != 0)))) {
        FUN_004dcfd0(local_20, 0x58, iVar12, 0xce, ADD(iVar12, 0xe));
        FUN_004d5180(local_10);
        FUN_004d51b0(local_20);
        if (EL(DAT_0081f812, iVar13, 0x16) == 0) {
          puVar17 = &DAT_0081f7cc + (unsigned int)iVar13 * 0x16U;
          uVar14 = 4;
          pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar13, 0x16), 0x8e));
          _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, uVar14, puVar17);
          uVar16 = 0xffff;
        }
        else {
          puVar17 = &DAT_0081f7cc + (unsigned int)iVar13 * 0x16U;
          pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar13, 0x16), 0x8e));
          _sprintf(&DAT_0081b740, s___s__s__005342cc, pcVar3, puVar17);
          uVar16 = _DAT_0081b6fc;
        }
        FUN_004da7c0(0xc, &DAT_0081b740, 0x58, iVar12, uVar16);
        FUN_004d51b0(local_10);
        uVar14 = EL(DAT_0081e0c6, iVar13, 1);
        uVar15 = EL(DAT_0081e0a6, iVar13, 1);
LAB_004faff6:
        _sprintf(&DAT_0081b740, s__d__d__005321a8, uVar15, uVar14);
        FUN_004da7c0(0xc, &DAT_0081b740, 0xd0, iVar12, 0xffff);
        iVar12 = ADD(iVar12, 0xf);
      }
    }
    else {
      iVar4 = SUB(iVar13, 8);
      iVar13 = ADD((unsigned int)iVar4 * 0x52U,
                   (unsigned int)SUB((unsigned int)iVar13 * 9U, 0x48) * 0x90U);
      if (((&DAT_00654bd0)[(unsigned char)(&DAT_007438d8)[iVar13]] != 0) &&
          ((EL(DAT_0081e0e6, iVar4, 1) != 0 || (EL(DAT_0081e1a6, iVar4, 1) != 0)))) {
        iVar13 = ADD((int)&DAT_00743880, iVar13);
        uVar14 = 4;
        pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081e426, iVar4, 1), 0x8e));
        _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, uVar14, iVar13);
        FUN_004da7c0(0xc, &DAT_0081b740, 0x58, iVar12, 0xffff);
        uVar14 = EL(DAT_0081e1a6, iVar4, 1);
        uVar15 = EL(DAT_0081e0e6, iVar4, 1);
        goto LAB_004faff6;
      }
    }
    local_30[0] = local_30[0] + 1;
  } while (local_30[0] < &DAT_00654cd8);
  iVar12 = 0x103;
  piVar7 = &DAT_00654cb8;
  uVar16 = DAT_0081bcd0;
  pcVar3 = FUN_004dcfa0(0x20f);
  FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar16);
  do {
    iVar12 = *piVar7;
    if (iVar12 == -1) {
      return 1;
    }
    if (iVar12 < 8) {
      iVar13 = FUN_004cdbc0();
      if ((iVar13 != EL(DAT_0081f7de, iVar12, 0x16)) &&
          ((EL(DAT_0081e266, iVar12, 1) != 0 || (EL(DAT_0081e286, iVar12, 1) != 0)))) {
        FUN_004dcfd0(local_20, 0x103, iVar10, 0x179, ADD(iVar10, 0xe));
        FUN_004d5180(local_10);
        FUN_004d51b0(local_20);
        puVar17 = &DAT_0081f7cc + (unsigned int)iVar12 * 0x16U;
        if (EL(DAT_0081f812, iVar12, 0x16) == 0) {
          uVar14 = 4;
          pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar12, 0x16), 0x8e));
          _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, uVar14, puVar17);
          uVar16 = 0xffff;
        }
        else {
          pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081f7e2, iVar12, 0x16), 0x8e));
          _sprintf(&DAT_0081b740, s___s__s__005342cc, pcVar3);
          uVar16 = _DAT_0081b6fc;
        }
        FUN_004da7c0(0xc, &DAT_0081b740, 0x103, iVar10, uVar16);
        FUN_004d51b0(local_10);
        uVar14 = EL(DAT_0081e286, iVar12, 1);
        uVar15 = EL(DAT_0081e266, iVar12, 1);
LAB_004fb231:
        _sprintf(&DAT_0081b740, s__d__d__005321a8, uVar15, uVar14);
        FUN_004da7c0(0xc, &DAT_0081b740, 0x17b, iVar10, 0xffff);
        iVar10 = ADD(iVar10, 0xf);
      }
    }
    else {
      iVar13 = SUB(iVar12, 8);
      iVar12 = ADD((unsigned int)iVar13 * 0x52U,
                   (unsigned int)SUB((unsigned int)iVar12 * 9U, 0x48) * 0x90U);
      if (((&DAT_00654bd0)[(unsigned char)(&DAT_007438d8)[iVar12]] != 0) &&
          ((EL(DAT_0081e2a6, iVar13, 1) != 0 || (EL(DAT_0081e366, iVar13, 1) != 0)))) {
        iVar12 = ADD((int)&DAT_00743880, iVar12);
        uVar14 = 4;
        pcVar3 = FUN_004dcfa0(ADD(EL(DAT_0081e426, iVar13, 1), 0x8e));
        _sprintf(&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, uVar14, iVar12);
        FUN_004da7c0(0xc, &DAT_0081b740, 0x103, iVar10, 0xffff);
        uVar14 = EL(DAT_0081e366, iVar13, 1);
        uVar15 = EL(DAT_0081e2a6, iVar13, 1);
        goto LAB_004fb231;
      }
    }
    piVar7 = piVar7 + 1;
    if (&DAT_00654cd8 <= piVar7) {
      return 1;
    }
  } while (1);
}
