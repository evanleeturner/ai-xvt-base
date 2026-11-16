/* SPDX-License-Identifier: MIT */
/* z_xvt 0042f120 */
/* prototypes */
void FUN_004ce3a0(int **a, unsigned int *b, unsigned int *c, int *d, int *e,
                  unsigned int *f, void *g, int *h, int *i, int *j,
                  unsigned int *k, int *l, int *m, unsigned int *n, int *o, int *p,
                  int *q, unsigned int *r, int *s, int *t, int *u, int *v);
char *__cdecl _strncpy(char *dst, const char *src, unsigned int n);
DWORD __stdcall timeGetTime(void);
void FUN_00431110(int target, unsigned int *buf, int size);
int *FUN_00431d70(undefined4 *a, undefined4 *b);
void FUN_00430980(int a, int b);
void FUN_00432df0(int a);
undefined4 FUN_00438a10(void);
void LAB_0042fe30(void); /* callback, convention unverified */

typedef void (__stdcall *PFN_VT_0x38)(int *, undefined4 *);
typedef void (__stdcall *PFN_VT_0x30_4)(int *, int, void *, int);
typedef void (__stdcall *PFN_VT_0x30_5)(int *, int, void *, int, int);
typedef void (__stdcall *PFN_VT_0x0c)(int *, int, int);
typedef void (__stdcall *PFN_VT_0x1c)(int *, int, int);

#define VT_SLOT(obj, off) (*(unsigned int *)((unsigned int)*(obj) + (off)))

/* globals */
extern char s_Dial_a_New_Number__005251c0[];
extern unsigned int DAT_00733070[];
extern char DAT_0071e6c8;
extern int _DAT_00539a00;
extern int _DAT_0073307c;
extern int DAT_00733380;
extern int DAT_00733588;
extern unsigned int DAT_00733384;
extern int DAT_00733584;
extern int DAT_0073358c;
extern int DAT_00733794;
extern unsigned int DAT_00733590;
extern int DAT_00733790;
extern int DAT_007330d4;
extern unsigned int DAT_0073379c[];
extern unsigned int DAT_0073905c[];
extern int DAT_005251a8;
extern unsigned int DAT_005ce218[];
extern int DAT_005251b4;
extern int DAT_00739290;
extern int *DAT_00733074;
extern unsigned int DAT_00733080[];
extern unsigned int DAT_00733090[];
extern int DAT_007330a0;
extern int DAT_007330a8;
extern unsigned int DAT_007330d8[];
extern int DAT_005251b0;
extern int DAT_005251ac;
extern unsigned int DAT_00733798[];
extern int DAT_005bda18;
extern int DAT_005251a4;
extern int DAT_007330f8;
extern int _DAT_007330fc;
extern char DAT_007330e8[];
extern unsigned int DAT_00733100[];
extern int DAT_007330a4;
extern int DAT_007330ac;
extern unsigned int _DAT_00539800;
extern int _DAT_00539804;
extern int DAT_00539808;
extern DWORD DAT_0053980c;
extern unsigned int DAT_00539810[];
extern int DAT_00733120[];
extern int DAT_00733124[];
extern char DAT_00733110[];
extern unsigned char DAT_0073311c[];
extern unsigned char DAT_0073310c[];
extern unsigned int DAT_007337a0[];
extern unsigned int DAT_007337a4[];
extern unsigned int DAT_007337a8[];
extern unsigned int DAT_007337ac[];
extern unsigned int DAT_007337b0[];
extern unsigned int DAT_007337b4[];
extern unsigned char DAT_007337b8[];
extern unsigned int DAT_007339b8[];
extern unsigned int DAT_007339bc[];
extern unsigned int DAT_007339c0[];
extern unsigned int DAT_007339c4[];

undefined4 __cdecl
FUN_0042f120(char *param_1, char *param_2, int param_3, undefined4 param_4,
             undefined4 param_5, int param_6)
{
  char cVar1;
  int iVar2;
  unsigned int *puVar3;
  DWORD DVar4;
  DWORD DVar5;
  int *piVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned int *puVar12;
  char *pcVar13;
  int iVar14;
  int *piVar15;
  char *pcVar16;
  int iVar17;
  undefined4 local_78[4];   /* stack0xffffff88 / 90 / 94 slots */
  undefined4 local_48[10];
  /* frame bytes -0x20 .. -1: local_20..local_1 (packed tail) */
  char local_20[32];
  unsigned int local_zero;
  unsigned int local_tail;

  __builtin_memcpy(local_20 + 0, s_Dial_a_New_Number__005251c0 + 0, 4);
  __builtin_memcpy(local_20 + 4, s_Dial_a_New_Number__005251c0 + 4, 4);
  __builtin_memcpy(local_20 + 8, s_Dial_a_New_Number__005251c0 + 8, 4);
  __builtin_memcpy(local_20 + 12, s_Dial_a_New_Number__005251c0 + 12, 4);
  __builtin_memcpy(local_20 + 16, s_Dial_a_New_Number__005251c0 + 16, 2);
  local_20[0x12] = s_Dial_a_New_Number__005251c0[0x12];
  local_zero = 0;
  __builtin_memcpy(local_20 + 0x13, &local_zero, 4);
  __builtin_memcpy(local_20 + 0x17, &local_zero, 4);
  __builtin_memcpy(local_20 + 0x1b, &local_zero, 4);
  local_20[0x1f] = 0;
  puVar3 = DAT_00733070;
  for (iVar7 = 0x190e; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  DAT_0071e6c8 = 1;
  _DAT_00539a00 = 0;
  _DAT_0073307c = param_5;
  DAT_00733380 = 0;
  DAT_00733588 = 1;
  *(unsigned char *)&DAT_00733384 = 0x39;
  DAT_00733584 = 1;
  DAT_0073358c = 0;
  DAT_00733794 = 1;
  *(unsigned char *)&DAT_00733590 = 0x39;
  DAT_00733790 = 1;
  DAT_007330d4 = 0;
  puVar3 = DAT_0073379c;
  do {
    *puVar3 = 0x7f;
    puVar12 = puVar3 + 0x8e;
    puVar3[2] = 0x7f;
    puVar3[4] = 0x7f;
    puVar3[1] = 0x7f;
    puVar3[3] = 0x7f;
    puVar3[5] = 0x7f;
    puVar3[6] = 0;
    puVar3[-1] = 0;
    *(unsigned char *)(puVar3 + 7) = 0x39;
    puVar3[0x87] = 1;
    puVar3[0x88] = 0;
    puVar3[0x89] = 0;
    puVar3[0x8a] = 0;
    puVar3 = puVar12;
  } while (puVar12 < DAT_0073905c);
  DAT_005251a8 = 0;
  puVar3 = DAT_005ce218;
  for (iVar7 = 0x8400; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  DAT_005251b4 = 0;
  DAT_00739290 = 0;
  if ((param_6 == 1) && (param_3 == 1)) {
    FUN_004ce3a0(&DAT_00733074, DAT_00733080, DAT_00733090, &DAT_007330a0,
                 &DAT_007330a8, DAT_007330d8, (void *)0x539a08, &DAT_005251b0,
                 &DAT_005251b4, &DAT_005251ac, DAT_00733798, &DAT_00739290,
                 &DAT_00733380, &DAT_00733384, &DAT_00733584, &DAT_00733588,
                 &DAT_0073358c, &DAT_00733590, &DAT_00733790, &DAT_00733794,
                 &DAT_005bda18, &DAT_005251a4);
    if (DAT_00733074 == (int *)0) {
      DAT_007330f8 = 1;
      _DAT_007330fc = 1;
    } else {
      puVar3 = local_48;
      for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar3 = 0;
        puVar3 = puVar3 + 1;
      }
      local_48[0] = 0x28;
      ((PFN_VT_0x38)VT_SLOT(DAT_00733074, 0x38))(DAT_00733074, local_48);
    }
    _strncpy((char *)DAT_007330d8, param_1, 0x10);
    _strncpy(DAT_007330e8, param_2, 0x10);
    puVar3 = DAT_007330d8;
    puVar12 = DAT_00733100;
    for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
      *puVar12 = *puVar3;
      puVar3 = puVar3 + 1;
      puVar12 = puVar12 + 1;
    }
    DAT_007330a4 = param_3;
    DAT_007330ac = 1;
    return 1;
  }
  FUN_004ce3a0(&DAT_00733074, DAT_00733080, DAT_00733090, &DAT_007330a0,
               &DAT_007330a8, DAT_007330d8, (void *)0x539a08, &DAT_005251b0,
               &DAT_005251b4, &DAT_005251ac, DAT_00733798, &DAT_00739290,
               &DAT_00733380, &DAT_00733384, &DAT_00733584, &DAT_00733588,
               &DAT_0073358c, &DAT_00733590, &DAT_00733790, &DAT_00733794,
               &DAT_005bda18, &DAT_005251a4);
  puVar3 = local_48;
  for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  iVar17 = 0;
  local_48[0] = 0x28;
  ((PFN_VT_0x38)VT_SLOT(DAT_00733074, 0x38))(DAT_00733074, local_48);
  iVar7 = 0;
  DAT_007330a4 = param_3;
  DAT_007330ac = 0;
  ((PFN_VT_0x30_4)VT_SLOT(DAT_00733074, 0x30))(DAT_00733074, 0, (void *)&LAB_0042fe30, 0);
  DAT_0071e6c8 = 1;
  timeGetTime();
  _DAT_00539800 = 0xd;
  FUN_00431110(DAT_007330a8, &_DAT_00539800, 4);
  _DAT_00539800 = 0x39;
  FUN_00431110(DAT_007330a8, &_DAT_00539800, 4);
  if (DAT_007330a4 != 0) {
    iVar14 = param_6;
    if (param_6 != 0) {
      do {
        DVar4 = timeGetTime();
LAB_0042f492:
        DVar5 = timeGetTime();
        if (DVar5 - DVar4 < 0xea61) goto LAB_0042f4a2;
        piVar6 = (int *)0;
LAB_0042f4d6:
        if (piVar6 == (int *)0) {
          DAT_00733074 = (int *)0;
          return 0;
        }
        if (*piVar6 == 0xd) {
          iVar14 = (int)((unsigned int)iVar14 - 1u);
        }
        if (iVar14 == 0) break;
      } while (1);
    }
    if (DAT_007330a4 != 0) {
      if (param_6 < 2) {
        return 1;
      }
      FUN_00432df0(0);
      _DAT_00539800 = 0x39;
      FUN_00431110(0, &_DAT_00539800, 4);
      return 1;
    }
  }
  __builtin_memcpy(&local_tail, local_20 + 0x1c, 4);
  if (local_tail != 0) {
    return 1;
  }
  do {
    DVar4 = timeGetTime();
LAB_0042f54c:
    DVar5 = timeGetTime();
    if (DVar5 - DVar4 < 0xea61) {
      while (piVar6 = FUN_00431d70(&local_78[0], &local_78[2]), piVar6 != (int *)0) {
        if (iVar7 != 0) {
          if (piVar6 != (int *)0) goto LAB_0042f590;
          break;
        }
        FUN_00430980(*piVar6, (int)piVar6);
      }
      goto LAB_0042f54c;
    }
    piVar6 = (int *)0;
LAB_0042f590:
    if (piVar6 == (int *)0) {
      DAT_00733074 = (int *)0;
      return 0;
    }
    if (*piVar6 == 0xe) {
      iVar7 = 0;
      DAT_007330ac = piVar6[1];
      if (DAT_007330ac < 1) {
        return 1;
      }
      do {
        DVar4 = timeGetTime();
LAB_0042f5c0:
        DVar5 = timeGetTime();
        if (DVar5 - DVar4 < 0xea61) {
LAB_0042f5d6:
          piVar6 = FUN_00431d70(&local_78[3], &local_78[2]);
          if (piVar6 != (int *)0) {
            if (iVar17 != 0) {
              if (piVar6 != (int *)0) goto LAB_0042fd48;
              goto LAB_0042f5c0;
            }
            iVar14 = *piVar6;
            if (iVar14 == 3) {
              if (DAT_007330a4 != 0) {
                if (DAT_0071e6c8 == '\0') {
                  if (piVar6[1] == 1) {
                    if (piVar6[2] != DAT_007330f8) {
                      _DAT_00539804 = 0;
                      _DAT_00539800 = 0x3b;
                      DAT_00539808 = DAT_00739290;
                      DAT_0053980c = timeGetTime();
                      iVar14 = 0;
                      if (0 < DAT_00739290) {
                        puVar3 = DAT_00539810;
                        puVar12 = DAT_00733798;
                        do {
                          iVar14 = (int)((unsigned int)iVar14 + 1u);
                          *puVar3 = *puVar12;
                          *(unsigned char *)(puVar3 + 1) = *(unsigned char *)(puVar12 + 3);
                          *((unsigned char *)puVar3 + 5) = *(unsigned char *)(puVar12 + 5);
                          *((unsigned char *)puVar3 + 6) = *(unsigned char *)(puVar12 + 4);
                          *((unsigned char *)puVar3 + 7) = *(unsigned char *)(puVar12 + 6);
                          puVar3 = puVar3 + 2;
                          puVar12 = puVar12 + 0x8e;
                        } while (iVar14 < DAT_00739290);
                      }
                      FUN_00431110(piVar6[2], &_DAT_00539800,
                                   (int)((unsigned int)DAT_00739290 * 8u + 0x10u));
                      _DAT_00539804 = 0;
                      _DAT_00539800 = 0x3a;
                      FUN_00431110(piVar6[2], &_DAT_00539800, 8);
                    }
                    DAT_007330ac = 0;
                    iVar14 = 0;
                    ((PFN_VT_0x30_5)VT_SLOT(DAT_00733074, 0x30))(DAT_00733074, 0, (void *)&LAB_0042fe30, 0, 0);
                    if (0 < DAT_007330ac) {
                      piVar6 = DAT_00733120;
                      do {
                        if (DAT_007330f8 != *piVar6) {
                          ((PFN_VT_0x0c)VT_SLOT(DAT_00733074, 0xc))(DAT_00733074, DAT_007330a0, *piVar6);
                        }
                        piVar6 = piVar6 + 10;
                        iVar14 = (int)((unsigned int)iVar14 + 1u);
                      } while (iVar14 < DAT_007330ac);
                    }
                  }
                } else if ((piVar6[1] == 1) && (piVar6[2] != DAT_007330f8)) {
                  _DAT_00539804 = 0;
                  _DAT_00539800 = 0x3b;
                  DAT_00539808 = DAT_00739290;
                  DAT_0053980c = timeGetTime();
                  iVar14 = 0;
                  if (0 < DAT_00739290) {
                    puVar3 = DAT_00539810;
                    puVar12 = DAT_00733798;
                    do {
                      iVar14 = (int)((unsigned int)iVar14 + 1u);
                      *puVar3 = *puVar12;
                      *(unsigned char *)(puVar3 + 1) = *(unsigned char *)(puVar12 + 3);
                      *((unsigned char *)puVar3 + 5) = *(unsigned char *)(puVar12 + 5);
                      *((unsigned char *)puVar3 + 6) = *(unsigned char *)(puVar12 + 4);
                      *((unsigned char *)puVar3 + 7) = *(unsigned char *)(puVar12 + 6);
                      puVar3 = puVar3 + 2;
                      puVar12 = puVar12 + 0x8e;
                    } while (iVar14 < DAT_00739290);
                  }
                  FUN_00431110(piVar6[2], &_DAT_00539800,
                               (int)((unsigned int)DAT_00739290 * 8u + 0x10u));
                  _DAT_00539800 = 0x3a;
                  _DAT_00539804 = (int)FUN_00438a10();
                  DAT_0053980c = 0;
                  DAT_00539808 = 2;
                  FUN_00431110(piVar6[2], &_DAT_00539800, 0x10);
                }
              }
            } else if (iVar14 == 5) {
              if (DAT_007330a4 != 0) {
                if (DAT_0071e6c8 == '\0') {
                  if (piVar6[1] == 1) {
                    DAT_007330ac = 0;
                    ((PFN_VT_0x30_5)VT_SLOT(DAT_00733074, 0x30))(DAT_00733074, 0, (void *)&LAB_0042fe30, 0, 0);
                    ((PFN_VT_0x1c)VT_SLOT(DAT_00733074, 0x1c))(DAT_00733074, DAT_007330a0, piVar6[2]);
                    iVar14 = 0;
                    if (0 < DAT_00739290) {
                      piVar9 = (int *)DAT_00733798;
                      while (*piVar9 != piVar6[2]) {
                        piVar9 = piVar9 + 0x8e;
                        iVar14 = (int)((unsigned int)iVar14 + 1u);
                        if (DAT_00739290 <= iVar14) goto LAB_0042f5d6;
                      }
                      DAT_00739290 = (int)((unsigned int)DAT_00739290 - 1u);
                      puVar3 = DAT_00733798 + (unsigned int)DAT_00739290 * 0x8eu;
                      puVar12 = DAT_00733798 + (unsigned int)iVar14 * 0x8eu;
                      for (iVar8 = 0x8e; iVar8 != 0; iVar8 = iVar8 + -1) {
                        *puVar12 = *puVar3;
                        puVar3 = puVar3 + 1;
                        puVar12 = puVar12 + 1;
                      }
                      DAT_00733798[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                      DAT_0073379c[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337a4[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337ac[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337a0[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337a8[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337b0[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                      DAT_007337b4[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                      DAT_007337b8[(unsigned int)DAT_00739290 * 0x238u] = 0x39;
                      DAT_007339b8[(unsigned int)DAT_00739290 * 0x8eu] = 1;
                      DAT_007339bc[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                      DAT_007339c0[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                      DAT_007339c4[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                    }
                  }
                } else if (piVar6[1] == 1) {
                  iVar14 = 0;
                  if (0 < DAT_007330ac) {
                    piVar9 = DAT_00733120;
                    do {
                      if (*piVar9 == piVar6[2]) {
                        if (DAT_00733124[(unsigned int)iVar14 * 10u] != 0) {
                          _DAT_00539800 = 0xd;
                          FUN_00431110(DAT_007330f8, &_DAT_00539800, 4);
                        }
                        ((PFN_VT_0x1c)VT_SLOT(DAT_00733074, 0x1c))(DAT_00733074, DAT_007330a0, piVar6[2]);
                        DAT_00733124[(unsigned int)iVar14 * 10u] = 0;
                        break;
                      }
                      piVar9 = piVar9 + 10;
                      iVar14 = (int)((unsigned int)iVar14 + 1u);
                    } while (iVar14 < DAT_007330ac);
                  }
                  iVar14 = 0;
                  if (0 < DAT_00739290) {
                    piVar9 = (int *)DAT_00733798;
                    while (*piVar9 != piVar6[2]) {
                      piVar9 = piVar9 + 0x8e;
                      iVar14 = (int)((unsigned int)iVar14 + 1u);
                      if (DAT_00739290 <= iVar14) goto LAB_0042f5d6;
                    }
                    DAT_00739290 = (int)((unsigned int)DAT_00739290 - 1u);
                    puVar3 = DAT_00733798 + (unsigned int)DAT_00739290 * 0x8eu;
                    puVar12 = DAT_00733798 + (unsigned int)iVar14 * 0x8eu;
                    for (iVar8 = 0x8e; iVar8 != 0; iVar8 = iVar8 + -1) {
                      *puVar12 = *puVar3;
                      puVar3 = puVar3 + 1;
                      puVar12 = puVar12 + 1;
                    }
                    DAT_00733798[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                    DAT_0073379c[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337a4[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337ac[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337a0[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337a8[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337b0[(unsigned int)DAT_00739290 * 0x8eu] = 0x7f;
                    DAT_007337b4[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                    DAT_007337b8[(unsigned int)DAT_00739290 * 0x238u] = 0x39;
                    DAT_007339b8[(unsigned int)DAT_00739290 * 0x8eu] = 1;
                    DAT_007339bc[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                    DAT_007339c0[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                    DAT_007339c4[(unsigned int)DAT_00739290 * 0x8eu] = 0;
                  }
                }
              }
            } else if (((iVar14 == 0x103) && (piVar6[1] == 1)) &&
                       (iVar14 = 0, 0 < DAT_007330ac)) {
              piVar9 = DAT_00733120;
              do {
                if (*piVar9 == piVar6[2]) {
                  uVar10 = 0xffffffff;
                  iVar8 = (int)((unsigned int)iVar14 * 0x28u);
                  piVar9 = piVar6 + 7;
                  goto code_r0x0042fcd6;
                }
                piVar9 = piVar9 + 10;
                iVar14 = (int)((unsigned int)iVar14 + 1u);
              } while (iVar14 < DAT_007330ac);
            }
            goto LAB_0042f5d6;
          }
          goto LAB_0042f5c0;
        }
        piVar6 = (int *)0;
LAB_0042fd48:
        if (piVar6 == (int *)0) {
          return 0;
        }
        if (*piVar6 == 0xf) {
          piVar9 = piVar6 + 2;
          piVar6 = (int *)&DAT_00733100[(unsigned int)piVar6[1] * 10u];
          for (iVar14 = 10; iVar14 != 0; iVar14 = iVar14 + -1) {
            *piVar6 = *piVar9;
            piVar9 = piVar9 + 1;
            piVar6 = piVar6 + 1;
          }
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          if (DAT_007330ac <= iVar7) {
            return 1;
          }
        }
      } while (1);
    }
  } while (1);

LAB_0042f4a2:
  while (piVar6 = FUN_00431d70(&local_78[0], &local_78[2]), piVar6 != (int *)0) {
    if (iVar7 != 0) {
      if (piVar6 != (int *)0) goto LAB_0042f4d6;
      break;
    }
    FUN_00430980(*piVar6, (int)piVar6);
  }
  goto LAB_0042f492;

  while (1) {
    uVar10 = uVar10 - 1;
    piVar15 = (int *)((int)piVar9 + 1);
    iVar2 = *(char *)piVar9;
    piVar9 = piVar15;
    if ((char)iVar2 == '\0') break;
code_r0x0042fcd6:
    piVar15 = piVar9;
    if (uVar10 == 0) break;
  }
  uVar10 = ~uVar10;
  puVar3 = (unsigned int *)((int)piVar15 - uVar10);
  puVar12 = (unsigned int *)&DAT_00733110[iVar8];
  for (uVar11 = uVar10 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
    *puVar12 = *puVar3;
    puVar3 = puVar3 + 1;
    puVar12 = puVar12 + 1;
  }
  for (uVar10 = uVar10 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
    *(unsigned char *)puVar12 = *(unsigned char *)puVar3;
    puVar3 = (unsigned int *)((int)puVar3 + 1);
    puVar12 = (unsigned int *)((int)puVar12 + 1);
  }
  uVar10 = 0xffffffff;
  pcVar13 = &DAT_00733110[iVar8];
  do {
    if (uVar10 == 0) break;
    uVar10 = uVar10 - 1;
    cVar1 = *pcVar13;
    pcVar13 = pcVar13 + 1;
  } while (cVar1 != '\0');
  uVar11 = 0xffffffff;
  pcVar13 = (char *)(~uVar10 + 0x1c + (int)piVar6);
  do {
    pcVar16 = pcVar13;
    if (uVar11 == 0) break;
    uVar11 = uVar11 - 1;
    pcVar16 = pcVar13 + 1;
    cVar1 = *pcVar13;
    pcVar13 = pcVar16;
  } while (cVar1 != '\0');
  uVar11 = ~uVar11;
  pcVar13 = pcVar16 + -uVar11;
  pcVar16 = (char *)&DAT_00733100[(unsigned int)iVar14 * 10u];
  for (uVar10 = uVar11 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
    *(unsigned int *)pcVar16 = *(unsigned int *)pcVar13;
    pcVar13 = pcVar13 + 4;
    pcVar16 = pcVar16 + 4;
  }
  for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
    *pcVar16 = *pcVar13;
    pcVar13 = pcVar13 + 1;
    pcVar16 = pcVar16 + 1;
  }
  DAT_0073311c[iVar8] = 0;
  DAT_0073310c[iVar8] = 0;
  goto LAB_0042f5d6;
}
