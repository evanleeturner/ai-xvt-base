/* SPDX-License-Identifier: MIT */
/* z_bop 004e7ce0 */
char *FUN_004ddc10(uint a);
void  FUN_004db4e0(int a, char *b, int c, int d, uint e);
int   FUN_004cfee0(void);
int   FUN_004dbfa0(char *a, int b);
void  FUN_004d6460(int *a, int b, int c, int d, int e);
int   FUN_004da650(int *a, char *b, int c, uint d, int e, char *f);
void  FUN_004db7d0(int a, char *b, int *c, int d, int e, uint f);
int   _sprintf(char *buf, const char *fmt, ...);
int   _rand(void);
void  FUN_004cef70(int a, uint *b, int c);

extern int  DAT_00b69e30;
extern unsigned char DAT_00b6a5aa[4];   /* unaligned 4-byte value */
extern int  DAT_00aa6114;
extern int  DAT_00aa6134;
extern int  DAT_00aa6120;
extern int  DAT_00aa6124;
extern uint DAT_00b6a2c0;
extern char DAT_00b69d20[];

extern unsigned char DAT_00bb0106[];
extern unsigned char DAT_00bb10a6[];
extern unsigned char DAT_00bb2046[];
extern unsigned char DAT_00bb260e[];
extern unsigned char DAT_00bb242e[];
extern unsigned char DAT_00bb010a[];
extern unsigned char DAT_00bb010e[];
extern unsigned char DAT_00bb10aa[];
extern unsigned char DAT_00bb10ae[];
extern unsigned char DAT_00bb204a[];
extern unsigned char DAT_00bb2612[];
extern unsigned char DAT_00bb2432[];

extern unsigned char DAT_00bb736f;
extern unsigned char DAT_00bb7370;
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
extern unsigned char DAT_00bb73a4;
extern unsigned char DAT_00bb73a5;
extern unsigned char DAT_00bb73a6;

extern char s_settingsound_0052ad80[];
extern char s__d__s_0052b4c8[];

extern uint DAT_00aa6af0;
extern uint DAT_00aa6af4;
extern uint DAT_00aa6af8;
extern uint DAT_00aa6afc;
extern uint DAT_00aa6b00;
extern uint DAT_00aa6b04;
extern uint DAT_00aa6b08;
extern uint DAT_00aa6b0c;
extern uint DAT_00aa6b10;
extern uint DAT_00aa6b14;
extern uint DAT_00aa6b18;
extern uint DAT_00aa6b1c;
extern uint DAT_00aa6b20;
extern uint DAT_00aa6b24;
extern uint DAT_00aa6b28;
extern uint DAT_00aa6b2c;
extern uint DAT_00aa6b30;
extern uint DAT_00aa6b34;
extern uint DAT_00aa6b38;

#define A5AA ({ int t_; __builtin_memcpy(&t_, DAT_00b6a5aa, 4); t_; })
#define RD4(base, off) ({ int t_; __builtin_memcpy(&t_, (base) + (off), 4); t_; })
#define SEL (*(int *)((uint)DAT_00aa6114 + 0x140u + (uint)DAT_00aa6134 * 0x148u))

uint __stdcall FUN_004e7ce0(void)
{
  char cVar1;
  _Bool bVar2;
  _Bool bVar3;
  _Bool bVar4;
  char *pcVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  int iVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  uint uVar14;
  char *pcVar15;
  int local_20;
  int local_14[4];
  uint local_4;

  uVar14 = 0xffff;
  iVar12 = 0x137;
  iVar10 = 0x58;
  iVar7 = 0x58;
  pcVar5 = FUN_004ddc10(0x292);
  iVar8 = 0x14a;
  FUN_004db4e0(0xf, pcVar5, iVar10, iVar12, uVar14);
  local_20 = 0;
  bVar4 = 0;
  iVar10 = FUN_004cfee0();
  if ((iVar10 == 0) && (DAT_00b69e30 != 2)) {
    bVar3 = 0;
  } else {
    bVar3 = 1;
  }
  if ((A5AA == 4) || (A5AA == 5)) {
    if (bVar3) {
      local_4 = 0;
      if (A5AA == 4) {
        iVar10 = SEL;
        if (DAT_00b69e30 == 2) {
          iVar10 = RD4(DAT_00bb0106, (uint)iVar10 * 0xa0u);
        } else {
          iVar10 = RD4(DAT_00bb10a6, (uint)iVar10 * 0xa0u);
        }
        local_4 = (uint)(iVar10 == 1);
        uVar14 = 0x305;
      } else {
        if (DAT_00b69e30 == 2) {
          iVar10 = RD4(DAT_00bb2046, (uint)SEL * 0x28u);
        } else {
          iVar12 = FUN_004cfee0();
          iVar10 = SEL;
          if (iVar12 == 0) {
            iVar10 = RD4(DAT_00bb260e, (uint)iVar10 * 0x28u);
          } else {
            iVar10 = RD4(DAT_00bb242e, (uint)iVar10 * 0x28u);
          }
        }
        if (iVar10 == 1) {
          local_4 = 1;
        }
        uVar14 = 0x30a;
      }
      pcVar5 = FUN_004ddc10(uVar14);
      uVar14 = 0xffffffff;
      do {
        pcVar15 = pcVar5;
        if (uVar14 == 0) break;
        uVar14 = uVar14 - 1;
        pcVar15 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar15;
      } while (cVar1 != '\0');
      uVar14 = ~uVar14;
      pcVar5 = pcVar15 - uVar14;
      pcVar15 = DAT_00b69d20;
      for (uVar6 = uVar14 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(pcVar15, pcVar5, 4);
        pcVar5 = pcVar5 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
        *pcVar15 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar15 = pcVar15 + 1;
      }
      if (local_4 == 0) goto LAB_004e80c8;
      FUN_004db4e0(10, DAT_00b69d20, 0x58, 0x14a, 0xffff);
      iVar8 = FUN_004dbfa0(DAT_00b69d20, 10);
      FUN_004d6460(local_14, iVar8 + 0x5a, 0x14a, 0xfd, 0x157);
      pcVar15 = s_settingsound_0052ad80;
      iVar10 = 0x21;
      uVar11 = 0xffff;
      iVar8 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb73a6 + 0x306);
      iVar8 = FUN_004da650(local_14, pcVar5, iVar8, uVar11, iVar10, pcVar15);
      if (iVar8 != 0) {
        bVar4 = 1;
        DAT_00bb73a6 = DAT_00bb73a6 ^ 1;
        if (DAT_00bb73a6 != 0) {
          if (A5AA == 4) {
            piVar9 = (int *)((uint)DAT_00aa6114 + 0x140u + (uint)DAT_00aa6134 * 0x148u);
            if (DAT_00b69e30 == 2) {
              DAT_00bb7398 = DAT_00bb010a[(uint)*piVar9 * 0xa0u];
              DAT_00bb7397 = DAT_00bb010e[(uint)*piVar9 * 0xa0u];
            } else {
              DAT_00bb7398 = DAT_00bb10aa[(uint)*piVar9 * 0xa0u];
              DAT_00bb7397 = DAT_00bb10ae[(uint)*piVar9 * 0xa0u];
            }
          } else if (A5AA == 5) {
            if (DAT_00b69e30 == 2) {
              DAT_00bb7397 = DAT_00bb204a[(uint)SEL * 0x28u];
            } else {
              iVar10 = FUN_004cfee0();
              iVar8 = SEL;
              if (iVar10 == 0) {
                DAT_00bb7397 = DAT_00bb2612[(uint)iVar8 * 0x28u];
              } else {
                DAT_00bb7397 = DAT_00bb2432[(uint)iVar8 * 0x28u];
              }
            }
          }
        }
      }
    } else {
      if (DAT_00aa6120 == 0) goto LAB_004e80c8;
      if (A5AA == 4) {
        uVar14 = 0x305;
LAB_004e8017:
        pcVar5 = FUN_004ddc10(uVar14);
        uVar14 = 0xffffffff;
        do {
          pcVar15 = pcVar5;
          if (uVar14 == 0) break;
          uVar14 = uVar14 - 1;
          pcVar15 = pcVar5 + 1;
          cVar1 = *pcVar5;
          pcVar5 = pcVar15;
        } while (cVar1 != '\0');
        uVar14 = ~uVar14;
        pcVar5 = pcVar15 - uVar14;
        pcVar15 = DAT_00b69d20;
        for (uVar6 = uVar14 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(pcVar15, pcVar5, 4);
          pcVar5 = pcVar5 + 4;
          pcVar15 = pcVar15 + 4;
        }
        for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
          *pcVar15 = *pcVar5;
          pcVar5 = pcVar5 + 1;
          pcVar15 = pcVar15 + 1;
        }
      } else if (A5AA == 5) {
        uVar14 = 0x30a;
        goto LAB_004e8017;
      }
      FUN_004db4e0(10, DAT_00b69d20, 0x58, 0x14a, 0xffff);
      iVar8 = FUN_004dbfa0(DAT_00b69d20, 10);
      FUN_004d6460(local_14, iVar8 + 0x5a, 0x14a, 0xfd, 0x157);
      piVar9 = local_14;
      iVar10 = 1;
      iVar8 = 0;
      uVar14 = DAT_00b6a2c0;
      pcVar5 = FUN_004ddc10(DAT_00aa6124 + 0x306);
      FUN_004db7d0(10, pcVar5, piVar9, iVar8, iVar10, uVar14);
    }
    local_20 = 1;
    iVar8 = 0x158;
  }
LAB_004e80c8:
  if (DAT_00b69e30 == 2) {
LAB_004e82aa:
    uVar14 = 0xffff;
    iVar10 = iVar7;
    iVar12 = iVar8;
    pcVar5 = FUN_004ddc10(0x1a5);
    FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(0x1a5);
    iVar10 = FUN_004dbfa0(pcVar5, iVar10);
    FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    iVar10 = local_20;
    if (A5AA == 5) {
      if (bVar3) {
        pcVar15 = s_settingsound_0052ad80;
        iVar13 = 0x14;
        uVar11 = 0xffff;
        iVar12 = 10;
        pcVar5 = FUN_004ddc10(DAT_00bb7394 + 0x328);
        iVar12 = FUN_004da650(local_14, pcVar5, iVar12, uVar11, iVar13, pcVar15);
        if (iVar12 != 0) {
          bVar4 = 1;
          DAT_00bb7394 = DAT_00bb7394 + 1;
          if (3 < DAT_00bb7394) {
            DAT_00bb7394 = 0;
          }
        }
      } else {
        uVar14 = DAT_00bb7394 + 0x328;
LAB_004e8408:
        piVar9 = local_14;
        iVar13 = 1;
        iVar12 = 0;
        uVar6 = DAT_00b6a2c0;
        pcVar5 = FUN_004ddc10(uVar14);
        FUN_004db7d0(10, pcVar5, piVar9, iVar12, iVar13, uVar6);
      }
    } else {
      if (!bVar3) {
        uVar14 = DAT_00bb7394 + 0x1b1;
        goto LAB_004e8408;
      }
      pcVar15 = s_settingsound_0052ad80;
      iVar13 = 0x14;
      uVar11 = 0xffff;
      iVar12 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb7394 + 0x1b1);
      iVar12 = FUN_004da650(local_14, pcVar5, iVar12, uVar11, iVar13, pcVar15);
      if (iVar12 != 0) {
        bVar4 = 1;
        DAT_00bb7394 = DAT_00bb7394 + 1;
        if (2 < DAT_00bb7394) {
          DAT_00bb7394 = 0;
        }
      }
    }
  } else {
    if (bVar3) {
      uVar14 = 0xffff;
      iVar12 = 0x58;
      iVar10 = iVar8;
      pcVar5 = FUN_004ddc10(0x1aa);
      FUN_004db4e0(10, pcVar5, iVar12, iVar10, uVar14);
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(0x1aa);
      iVar10 = FUN_004dbfa0(pcVar5, iVar10);
      FUN_004d6460(local_14, iVar10 + 0x5a, iVar8, 0xfd, iVar8 + 0xd);
      pcVar15 = s_settingsound_0052ad80;
      iVar12 = 0x15;
      uVar11 = 0xffff;
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb7399 + 0x1b7);
      iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
      if (iVar10 != 0) {
        bVar4 = 1;
        DAT_00bb7399 = DAT_00bb7399 ^ 1;
      }
      iVar8 = iVar8 + 0xe;
      local_20 = local_20 + 1;
      if (local_20 == 6) {
        iVar7 = 0x106;
        iVar8 = 0x14a;
      }
    }
    if ((DAT_00b69e30 == 2) || ((A5AA != 3 && (A5AA != 4)))) goto LAB_004e82aa;
    uVar14 = 0xffff;
    iVar10 = iVar7;
    iVar12 = iVar8;
    pcVar5 = FUN_004ddc10(0x300);
    FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(0x300);
    iVar10 = FUN_004dbfa0(pcVar5, iVar10);
    FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    iVar10 = local_20;
    if (bVar3) {
      pcVar15 = s_settingsound_0052ad80;
      iVar13 = 0x14;
      uVar11 = 0xffff;
      iVar12 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb73a5 + 0x301);
      iVar12 = FUN_004da650(local_14, pcVar5, iVar12, uVar11, iVar13, pcVar15);
      if (iVar12 != 0) {
        bVar4 = 1;
        DAT_00bb73a5 = DAT_00bb73a5 + 1;
        if (3 < DAT_00bb73a5) {
          DAT_00bb73a5 = 0;
        }
      }
    } else {
      piVar9 = local_14;
      iVar13 = 1;
      iVar12 = 0;
      uVar14 = DAT_00b6a2c0;
      pcVar5 = FUN_004ddc10(DAT_00bb73a5 + 0x301);
      FUN_004db7d0(10, pcVar5, piVar9, iVar12, iVar13, uVar14);
    }
  }
  local_20 = iVar10 + 1;
  iVar8 = iVar8 + 0xe;
  if (local_20 == 6) {
    iVar7 = 0x106;
    iVar8 = 0x14a;
  }
  if (DAT_00b69e30 != 2) {
    uVar14 = 0xffff;
    iVar12 = iVar7;
    iVar13 = iVar8;
    pcVar5 = FUN_004ddc10(0x1af);
    FUN_004db4e0(10, pcVar5, iVar12, iVar13, uVar14);
    iVar12 = 10;
    pcVar5 = FUN_004ddc10(0x1af);
    iVar12 = FUN_004dbfa0(pcVar5, iVar12);
    FUN_004d6460(local_14, iVar12 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    if (DAT_00bb739e == 0) {
      pcVar5 = FUN_004ddc10(0x1bd);
      uVar14 = 0xffffffff;
      do {
        pcVar15 = pcVar5;
        if (uVar14 == 0) break;
        uVar14 = uVar14 - 1;
        pcVar15 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar15;
      } while (cVar1 != '\0');
      uVar14 = ~uVar14;
      pcVar5 = pcVar15 - uVar14;
      pcVar15 = DAT_00b69d20;
      for (uVar6 = uVar14 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(pcVar15, pcVar5, 4);
        pcVar5 = pcVar5 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
        *pcVar15 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar15 = pcVar15 + 1;
      }
    } else if (DAT_00bb739e == 0xff) {
      pcVar5 = FUN_004ddc10(0x1be);
      uVar14 = 0xffffffff;
      do {
        pcVar15 = pcVar5;
        if (uVar14 == 0) break;
        uVar14 = uVar14 - 1;
        pcVar15 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar15;
      } while (cVar1 != '\0');
      uVar14 = ~uVar14;
      pcVar5 = pcVar15 - uVar14;
      pcVar15 = DAT_00b69d20;
      for (uVar6 = uVar14 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(pcVar15, pcVar5, 4);
        pcVar5 = pcVar5 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
        *pcVar15 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar15 = pcVar15 + 1;
      }
    } else {
      pcVar5 = FUN_004ddc10(0x1c0);
      _sprintf(DAT_00b69d20, s__d__s_0052b4c8, (uint)DAT_00bb739e, pcVar5);
    }
    if (bVar3) {
      iVar12 = FUN_004da650(local_14, DAT_00b69d20, 10, 0xffff, 0x16, s_settingsound_0052ad80);
      if (iVar12 == 1) {
        bVar4 = 1;
        if (DAT_00bb739e == 0) {
          DAT_00bb739e = 0xff;
        } else if (DAT_00bb739e == 0xff) {
          DAT_00bb739e = 1;
        } else {
          DAT_00bb739e = DAT_00bb739e + 1;
          if (0x14 < DAT_00bb739e) {
            DAT_00bb739e = 0;
          }
        }
      }
      if (iVar12 == 2) {
        bVar4 = 1;
        if (DAT_00bb739e == 0xff) {
          DAT_00bb739e = 0;
        } else if (DAT_00bb739e == 0) {
          DAT_00bb739e = 0x14;
        } else {
          DAT_00bb739e = DAT_00bb739e - 1;
          if (DAT_00bb739e == 0) {
            DAT_00bb739e = 0xff;
          }
        }
      }
    } else {
      FUN_004db7d0(10, DAT_00b69d20, local_14, 0, 1, DAT_00b6a2c0);
    }
    iVar8 = iVar8 + 0xe;
    if (local_20 == 5) {
      iVar7 = 0x106;
      iVar8 = 0x14a;
    }
    uVar14 = 0xffff;
    iVar12 = iVar7;
    iVar13 = iVar8;
    pcVar5 = FUN_004ddc10(0x1b0);
    FUN_004db4e0(10, pcVar5, iVar12, iVar13, uVar14);
    iVar12 = 10;
    pcVar5 = FUN_004ddc10(0x1b0);
    iVar12 = FUN_004dbfa0(pcVar5, iVar12);
    FUN_004d6460(local_14, iVar12 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    if (DAT_00bb739f == 0) {
      pcVar5 = FUN_004ddc10(0x1bd);
      uVar14 = 0xffffffff;
      do {
        pcVar15 = pcVar5;
        if (uVar14 == 0) break;
        uVar14 = uVar14 - 1;
        pcVar15 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar15;
      } while (cVar1 != '\0');
      uVar14 = ~uVar14;
      pcVar5 = pcVar15 - uVar14;
      pcVar15 = DAT_00b69d20;
      for (uVar6 = uVar14 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(pcVar15, pcVar5, 4);
        pcVar5 = pcVar5 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
        *pcVar15 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar15 = pcVar15 + 1;
      }
    } else {
      pcVar5 = FUN_004ddc10(0x1c0);
      _sprintf(DAT_00b69d20, s__d__s_0052b4c8, (uint)DAT_00bb739f, pcVar5);
    }
    if (bVar3) {
      iVar12 = FUN_004da650(local_14, DAT_00b69d20, 10, 0xffff, 0x17, s_settingsound_0052ad80);
      if (iVar12 == 1) {
        bVar4 = 1;
        DAT_00bb739f = DAT_00bb739f + 1;
        if (10 < DAT_00bb739f) {
          DAT_00bb739f = 0;
        }
      }
      if (iVar12 == 2) {
        bVar4 = 1;
        if (DAT_00bb739f == 0) {
          DAT_00bb739f = 10;
        } else {
          DAT_00bb739f = DAT_00bb739f - 1;
        }
      }
    } else {
      FUN_004db7d0(10, DAT_00b69d20, local_14, 0, 1, DAT_00b6a2c0);
    }
    iVar8 = iVar8 + 0xe;
    local_20 = iVar10 + 3;
    if (local_20 == 6) {
      iVar7 = 0x106;
      iVar8 = 0x14a;
    }
  }
  if (A5AA == 4) {
    uVar14 = 0xffff;
    iVar10 = iVar7;
    iVar12 = iVar8;
    pcVar5 = FUN_004ddc10(0x1a9);
    FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(0x1a9);
    iVar10 = FUN_004dbfa0(pcVar5, iVar10);
    FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    bVar2 = 0;
    if (bVar3) {
      if (DAT_00b69e30 == 2) {
        if (DAT_00bb73a6 == 1) {
          iVar10 = RD4(DAT_00bb0106, (uint)SEL * 0xa0u);
          if (iVar10 == 1) goto LAB_004e885d;
        }
      } else if (DAT_00bb73a6 == 1) {
        iVar10 = RD4(DAT_00bb10a6, (uint)SEL * 0xa0u);
        if (iVar10 == 1) goto LAB_004e885d;
      }
      bVar2 = 1;
    }
LAB_004e885d:
    if (bVar2) {
      pcVar15 = s_settingsound_0052ad80;
      iVar12 = 0x1a;
      uVar11 = 0xffff;
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb7398 + 0x1b4);
      iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
      if (iVar10 != 0) {
        bVar4 = 1;
        DAT_00bb7398 = DAT_00bb7398 + 1;
        if (2 < DAT_00bb7398) {
          DAT_00bb7398 = 0;
        }
      }
    } else {
      piVar9 = local_14;
      iVar12 = 1;
      iVar10 = 0;
      uVar14 = DAT_00b6a2c0;
      pcVar5 = FUN_004ddc10(DAT_00bb7398 + 0x1b4);
      FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar14);
    }
    iVar8 = iVar8 + 0xe;
    local_20 = local_20 + 1;
    if (local_20 == 6) {
      iVar7 = 0x106;
      iVar8 = 0x14a;
    }
  }
  if (A5AA != 5) {
    uVar14 = 0xffff;
    if (A5AA == 4) {
      iVar10 = iVar7;
      iVar12 = iVar8;
      pcVar5 = FUN_004ddc10(0x2e0);
      FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
      uVar14 = 0x2e0;
    } else {
      iVar10 = iVar7;
      iVar12 = iVar8;
      pcVar5 = FUN_004ddc10(0x1a8);
      FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
      uVar14 = 0x1a8;
    }
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(uVar14);
    iVar10 = FUN_004dbfa0(pcVar5, iVar10);
    bVar2 = 0;
    FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    if (bVar3) {
      if (A5AA == 4) {
        if (DAT_00b69e30 == 2) {
          if (DAT_00bb73a6 == 1) {
            iVar10 = RD4(DAT_00bb0106, (uint)SEL * 0xa0u);
            if (iVar10 == 1) goto LAB_004e8adc;
          }
        } else if (DAT_00bb73a6 == 1) {
          iVar10 = RD4(DAT_00bb10a6, (uint)SEL * 0xa0u);
          if (iVar10 == 1) goto LAB_004e8adc;
        }
      } else if (A5AA == 5) {
        if (DAT_00b69e30 == 2) {
          if (DAT_00bb73a6 == 1) {
            iVar10 = RD4(DAT_00bb2046, (uint)SEL * 0x28u);
            if (iVar10 == 1) goto LAB_004e8adc;
          }
        } else {
          iVar10 = FUN_004cfee0();
          if (iVar10 == 0) {
            if (DAT_00bb73a6 == 1) {
              iVar10 = RD4(DAT_00bb260e, (uint)SEL * 0x28u);
              if (iVar10 == 1) goto LAB_004e8adc;
            }
          } else if (DAT_00bb73a6 == 1) {
            iVar10 = RD4(DAT_00bb242e, (uint)SEL * 0x28u);
            if (iVar10 == 1) goto LAB_004e8adc;
          }
        }
      }
      bVar2 = 1;
    }
LAB_004e8adc:
    if (bVar2) {
      pcVar5 = s_settingsound_0052ad80;
      iVar12 = 0x18;
      uVar11 = 0xffff;
      iVar10 = 10;
      if (A5AA == 4) {
        pcVar15 = FUN_004ddc10(DAT_00bb7397 + 0x333);
        iVar10 = FUN_004da650(local_14, pcVar15, iVar10, uVar11, iVar12, pcVar5);
        if (iVar10 != 0) {
          bVar4 = 1;
          DAT_00bb7397 = DAT_00bb7397 + 1;
          if (2 < DAT_00bb7397) {
            DAT_00bb7397 = 0;
          }
        }
      } else {
        pcVar15 = FUN_004ddc10(DAT_00bb7397 + 0xec);
        iVar10 = FUN_004da650(local_14, pcVar15, iVar10, uVar11, iVar12, pcVar5);
        if (iVar10 != 0) {
          bVar4 = 1;
          DAT_00bb7397 = DAT_00bb7397 ^ 1;
        }
      }
    } else {
      piVar9 = local_14;
      iVar12 = 1;
      iVar10 = 0;
      if (A5AA == 4) {
        uVar14 = DAT_00bb7397 + 0x333;
      } else {
        uVar14 = DAT_00bb7397 + 0xec;
      }
      uVar6 = DAT_00b6a2c0;
      pcVar5 = FUN_004ddc10(uVar14);
      FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar6);
    }
    iVar8 = iVar8 + 0xe;
    local_20 = local_20 + 1;
    if (local_20 == 6) {
      iVar7 = 0x106;
      iVar8 = 0x14a;
    }
  }
  if (DAT_00b69e30 != 2) {
    if ((A5AA == 1) || (A5AA == 2)) {
      uVar14 = 0xffff;
      iVar10 = iVar7;
      iVar12 = iVar8;
      pcVar5 = FUN_004ddc10(0x23c);
      FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(0x23c);
      iVar10 = FUN_004dbfa0(pcVar5, iVar10);
      FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
      if (bVar3) {
        pcVar15 = s_settingsound_0052ad80;
        iVar12 = 0x19;
        uVar11 = 0xffff;
        iVar10 = 10;
        pcVar5 = FUN_004ddc10(DAT_00bb73a4 + 0xec);
        iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
        if (iVar10 != 0) {
          bVar4 = 1;
          DAT_00bb73a4 = DAT_00bb73a4 ^ 1;
        }
      } else {
        piVar9 = local_14;
        iVar12 = 1;
        iVar10 = 0;
        uVar14 = DAT_00b6a2c0;
        pcVar5 = FUN_004ddc10(DAT_00bb73a4 + 0xec);
        FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar14);
      }
      iVar8 = iVar8 + 0xe;
      local_20 = local_20 + 1;
      if (local_20 == 6) {
        iVar7 = 0x106;
        iVar8 = 0x14a;
      }
    }
    if (DAT_00b69e30 == 2) goto LAB_004e8f19;
    if (A5AA != 5) {
      uVar14 = 0xffff;
      iVar10 = iVar7;
      iVar12 = iVar8;
      pcVar5 = FUN_004ddc10(0x1ac);
      FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(0x1ac);
      iVar10 = FUN_004dbfa0(pcVar5, iVar10);
      FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
      if (bVar3) {
        pcVar15 = s_settingsound_0052ad80;
        iVar12 = 0x1b;
        uVar11 = 0xffff;
        iVar10 = 10;
        pcVar5 = FUN_004ddc10(DAT_00bb739b + 0x218);
        iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
        if (iVar10 != 0) {
          bVar4 = 1;
          DAT_00bb739b = DAT_00bb739b + 1;
          if (2 < DAT_00bb739b) {
            DAT_00bb739b = 0;
          }
        }
      } else {
        piVar9 = local_14;
        iVar12 = 1;
        iVar10 = 0;
        uVar14 = DAT_00b6a2c0;
        pcVar5 = FUN_004ddc10(DAT_00bb739b + 0x218);
        FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar14);
      }
      if (((DAT_00bb739b == 2) && (A5AA != 1)) && (A5AA != 2)) {
        DAT_00bb739b = 0;
      }
      iVar8 = iVar8 + 0xe;
      local_20 = local_20 + 1;
      if (local_20 == 6) {
        iVar7 = 0x106;
        iVar8 = 0x14a;
      }
    }
  }
  if (DAT_00b69e30 != 2) {
    uVar14 = 0xffff;
    iVar10 = iVar7;
    iVar12 = iVar8;
    pcVar5 = FUN_004ddc10(0x1ad);
    FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(0x1ad);
    iVar10 = FUN_004dbfa0(pcVar5, iVar10);
    FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
    if (bVar3) {
      pcVar15 = s_settingsound_0052ad80;
      iVar12 = 0x1c;
      uVar11 = 0xffff;
      iVar10 = 10;
      pcVar5 = FUN_004ddc10(DAT_00bb739c + 0x1bb);
      iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
      if (iVar10 != 0) {
        bVar4 = 1;
        DAT_00bb739c = DAT_00bb739c ^ 1;
      }
    } else {
      piVar9 = local_14;
      iVar12 = 1;
      iVar10 = 0;
      uVar14 = DAT_00b6a2c0;
      pcVar5 = FUN_004ddc10(DAT_00bb739c + 0x1bb);
      FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar14);
    }
    iVar8 = iVar8 + 0xe;
    local_20 = local_20 + 1;
    if (local_20 == 6) {
      iVar7 = 0x106;
      iVar8 = 0x14a;
    }
  }
LAB_004e8f19:
  uVar14 = 0xffff;
  iVar10 = iVar7;
  iVar12 = iVar8;
  pcVar5 = FUN_004ddc10(0x1ae);
  FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
  iVar10 = 10;
  pcVar5 = FUN_004ddc10(0x1ae);
  iVar10 = FUN_004dbfa0(pcVar5, iVar10);
  FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
  if (bVar3) {
    pcVar15 = s_settingsound_0052ad80;
    iVar12 = 0x1d;
    uVar11 = 0xffff;
    iVar10 = 10;
    pcVar5 = FUN_004ddc10(DAT_00bb739d + 0x1bd);
    iVar10 = FUN_004da650(local_14, pcVar5, iVar10, uVar11, iVar12, pcVar15);
    if (iVar10 != 0) {
      bVar4 = 1;
      DAT_00bb739d = DAT_00bb739d + 1;
      if (2 < DAT_00bb739d) {
        DAT_00bb739d = 0;
      }
      if (((DAT_00b69e30 != 2) && ((A5AA == 3 || (A5AA == 4)))) &&
          (DAT_00bb739d == 2)) {
        DAT_00bb739d = 0;
      }
    }
  } else {
    piVar9 = local_14;
    iVar12 = 1;
    iVar10 = 0;
    uVar14 = DAT_00b6a2c0;
    pcVar5 = FUN_004ddc10(DAT_00bb739d + 0x1bd);
    FUN_004db7d0(10, pcVar5, piVar9, iVar10, iVar12, uVar14);
  }
  iVar8 = iVar8 + 0xe;
  if (local_20 == 5) {
    iVar7 = 0x106;
    iVar8 = 0x14a;
  }
  uVar14 = 0xffff;
  iVar10 = iVar7;
  iVar12 = iVar8;
  pcVar5 = FUN_004ddc10(0x1a6);
  FUN_004db4e0(10, pcVar5, iVar10, iVar12, uVar14);
  iVar10 = 10;
  pcVar5 = FUN_004ddc10(0x1a6);
  iVar10 = FUN_004dbfa0(pcVar5, iVar10);
  FUN_004d6460(local_14, iVar10 + iVar7 + 2, iVar8, iVar7 + 0xa5, iVar8 + 0xd);
  if (bVar3) {
    pcVar15 = s_settingsound_0052ad80;
    iVar7 = 0x1f;
    uVar11 = 0xffff;
    iVar8 = 10;
    pcVar5 = FUN_004ddc10(DAT_00bb7395 + 0xec);
    iVar8 = FUN_004da650(local_14, pcVar5, iVar8, uVar11, iVar7, pcVar15);
    if (iVar8 != 0) {
      bVar4 = 1;
      DAT_00bb7395 = DAT_00bb7395 ^ 1;
    }
  } else {
    piVar9 = local_14;
    iVar7 = 1;
    iVar8 = 0;
    uVar14 = DAT_00b6a2c0;
    pcVar5 = FUN_004ddc10(DAT_00bb7395 + 0xec);
    FUN_004db7d0(10, pcVar5, piVar9, iVar8, iVar7, uVar14);
  }
  if ((DAT_00b69e30 != 2) && (bVar4)) {
    DAT_00aa6af0 = 0x59;
    DAT_00aa6af4 = (uint)DAT_00bb7394;
    DAT_00aa6af8 = (uint)DAT_00bb7395;
    DAT_00aa6afc = (uint)DAT_00bb7396;
    DAT_00aa6b00 = (uint)DAT_00bb7397;
    DAT_00aa6b04 = (uint)DAT_00bb7398;
    DAT_00aa6b08 = (uint)DAT_00bb7399;
    DAT_00aa6b0c = (uint)DAT_00bb739a;
    DAT_00aa6b10 = (uint)DAT_00bb739b;
    DAT_00aa6b14 = (uint)DAT_00bb739c;
    DAT_00aa6b18 = (uint)DAT_00bb739d;
    DAT_00aa6b1c = (uint)DAT_00bb739e;
    DAT_00aa6b20 = (uint)DAT_00bb739f;
    DAT_00aa6b24 = _rand();
    DAT_00aa6b28 = (uint)DAT_00bb736f;
    DAT_00aa6b2c = (uint)DAT_00bb73a4;
    DAT_00aa6b30 = (uint)DAT_00bb7370;
    DAT_00aa6b34 = (uint)DAT_00bb73a5;
    DAT_00aa6b38 = (uint)DAT_00bb73a6;
    FUN_004cef70(0, &DAT_00aa6af0, 0x4c);
  }
  return 1;
}
