/* SPDX-License-Identifier: MIT */
/* z_bop 0044ce40 */
extern unsigned int DAT_009a806c;
extern unsigned int DAT_009a7bac;
extern unsigned int DAT_009a8c14;
extern int DAT_005233d8;
extern int DAT_009ecc34;
extern unsigned char DAT_009e967c;
extern unsigned char DAT_009e967e;
extern unsigned char DAT_009e9670;
extern unsigned char DAT_009e968a;
extern unsigned char DAT_009e9683;
extern unsigned char DAT_009e9a77;
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int _DAT_00a07cec;
extern unsigned short DAT_009a2000;
extern unsigned char DAT_009fd458;
extern unsigned char DAT_009d8c88;
extern short DAT_00a08ba0;
extern unsigned short _DAT_00a07be0;
extern unsigned short _DAT_00a07be2;
extern unsigned char DAT_009a20ae;
extern unsigned short DAT_00a083bc;
extern unsigned short DAT_00a08376;
extern unsigned short DAT_00a08378;
extern unsigned short DAT_00a083b8;
extern unsigned short DAT_00a08606;
extern unsigned short DAT_00a08608;
extern unsigned short DAT_00a08a00;
extern unsigned short DAT_00a08604;
extern unsigned short DAT_00521548;
extern unsigned short DAT_0052154c;
extern unsigned char DAT_00a0a1e0;
extern unsigned short DAT_00a08374;
extern unsigned char DAT_009ed232;
extern unsigned char DAT_005233c8;
extern unsigned short DAT_005569d8;
extern unsigned char DAT_005569ec;
extern unsigned char DAT_009d8b8c;
extern void (*DAT_009d1300)(void);
extern void (*DAT_009eca20)(unsigned int, unsigned int, unsigned int, unsigned int, int);
extern void (*DAT_009a6fe4)(int);
extern void *DAT_00a0a160;
extern void *DAT_00a0a164;
extern void *DAT_00a0a168;
extern void *DAT_00a0a0e4;
extern void *DAT_00a0a0e8;
extern unsigned char DAT_00524078;
extern unsigned char DAT_009a1ed0;
extern unsigned char DAT_009a1ed9;
extern unsigned short DAT_00a08108;
extern int DAT_005233ec;
extern unsigned short DAT_009d6930;
extern unsigned char *DAT_00a08c70;
extern unsigned char *DAT_00a0a130;
extern char s_CJNRFCJNRF_00523f80[];

unsigned int FUN_0040e0c0(int, int, int, int);
void FUN_004a9640(char);
void FUN_00450650(int, int);
void FUN_004a95c0(unsigned int);
void FUN_004a9520(int, int, int, int);
unsigned int FUN_00480530(void);
void FUN_004a9630(int);
void FUN_004a9590(unsigned int);
void FUN_004a9500(int, int);
void FUN_004a9960(const void *);
void FUN_004a97f0(const void *);
void FUN_004a9820(const void *);
void __cdecl FUN_004a9ac0(const void *);
void FUN_0044e0a0(unsigned int);
unsigned int FUN_00405820(unsigned int);
unsigned int FUN_00425c60(unsigned int, unsigned int);
void FUN_004277f0(unsigned short, int, int);
unsigned int FUN_004a9890(unsigned int);
void FUN_0043c290(unsigned short, int);
unsigned int FUN_0044e170(unsigned int);

#define M5BD ((int)((unsigned int)DAT_009ecc34 * 0x5bdu))
#define M10 ((int)((unsigned int)DAT_009ecc34 * 0x10u))
#define MULA ((int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac))

unsigned int FUN_0044ce40(unsigned short param_1)
{
  unsigned short *puVar2;
  unsigned char bVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  unsigned short uVar7;
  unsigned short uVar8;
  unsigned short uVar9;
  short sVar10;
  unsigned char *pbVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned int uVar16;
  short sVar17;
  unsigned short uVar18;
  int iVar19;
  int iVar20;
  short sVar21;
  unsigned short uCnt;
  unsigned int uVar26;
  int bVar25;
  unsigned short uStack_ee;
  unsigned short uStack_ec;
  unsigned short local_e8;
  short local_e6;
  short sStack_e4;
  short sStack_e2;
  unsigned short uStack_e0;
  unsigned short local_d4;
  unsigned int uStack_cc;
  unsigned short *local_c8;
  unsigned int local_c0[24];
  unsigned int local_60[24];
  unsigned short *ps60 = (unsigned short *)local_60;
  unsigned short *ps_c0 = (unsigned short *)local_c0;
  unsigned short *puVar11;
  char *p;
  int iTmp;
  int iTmp2;
  unsigned short sTmp;
  unsigned int uTmp;
  unsigned short extraout_var = 0;
  unsigned short uVar24 = 0;

  FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14, MULA);
  FUN_004a9640('\0');
  uCnt = 1;
  iVar6 = M5BD;
  __builtin_memcpy(&uVar8, &DAT_009e967c + iVar6, 2);
  __builtin_memcpy(&sTmp, &DAT_009e967e + iVar6, 2);
  ps60[0] = sTmp;
  ps_c0[0] = sTmp;
  uVar7 = 5 - param_1;
  __builtin_memcpy(&iVar5, &DAT_009e9670 + iVar6, 4);
  if (iVar5 != -1) {
    __builtin_memcpy(&iTmp, (char *)(int)((unsigned int)iVar5 * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8), 4);
    __builtin_memcpy(&iTmp2, (char *)(iTmp + 0xa9), 4);
    if (*(short *)(iTmp2 + 0x2b4) == 0 && (&DAT_00a08ba0)[uVar7] != -2) {
      _DAT_00a07be0 = 0x60;
      _DAT_00a07be2 = 0x57;
      (&DAT_00a08ba0)[uVar7] = -2;
      FUN_00450650(0x56, DAT_009ecc34);
      goto LAB_0044e070;
    }
  }
  if (param_1 == 1) {
    uStack_e0 = 0;
    if (_DAT_00a07cec < DAT_009a1fe0) {
      p = (char *)(int)((unsigned int)_DAT_00a07cec * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8);
      iVar19 = _DAT_00a07cec;
      do {
        __builtin_memcpy(&iVar5, p, 4);
        if (*(p - 28) != '\0' && iVar5 != 0) {
          __builtin_memcpy(&iTmp, (char *)(iVar5 + 0xa9), 4);
          if (iTmp != 0) {
            bVar3 = *(unsigned char *)(iVar5 + 0x89);
            if ((unsigned int)bVar3 != (unsigned int)uVar8 &&
                (&DAT_009fd458)[(unsigned int)bVar3 + (unsigned int)uVar8 * 0x1e5u] == '\0') {
              cVar4 = *(char *)(iTmp + 7);
              if (cVar4 != '\x03' && cVar4 != '\x04') {
                ps60[uStack_e0] = (unsigned short)((unsigned short)bVar3 << 8 | (unsigned short)iVar19);
                uStack_e0 = uStack_e0 + 1;
              }
            }
          }
        }
        p = p + 0x23;
        iVar19 = iVar19 + 1;
      } while (iVar19 < DAT_009a1fe0);
    }
  }
  else {
    uVar15 = 0;
    if (0 < DAT_009a2000) {
      pbVar12 = &DAT_009d8c88;
      do {
        if ((unsigned int)*pbVar12 == (unsigned int)uVar8) {
          bVar25 = 0;
        }
        else {
          bVar25 = (&DAT_009fd458)[(unsigned int)*pbVar12 + (unsigned int)uVar8 * 0x1e5u] == '\0';
        }
        if (!bVar25 && (unsigned int)ps60[0] != uVar15) {
          ps_c0[uCnt] = (unsigned short)uVar15;
          uCnt = uCnt + 1;
        }
        pbVar12 = pbVar12 + 0x566;
        uVar15 = uVar15 + 1;
      } while ((int)uVar15 < (int)DAT_009a2000);
    }
    uStack_e0 = 0;
    if (uCnt != 0) {
      puVar11 = ps_c0;
      uVar15 = (unsigned int)uCnt;
      do {
        if (_DAT_00a07cec < DAT_009a1fe0) {
          p = (char *)(int)((unsigned int)_DAT_00a07cec * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8);
          uVar26 = *puVar11;
          iVar19 = _DAT_00a07cec;
          do {
            if ((unsigned short)*(unsigned char *)(p - 9) == (unsigned short)uVar26 && *(p - 28) != '\0') {
              __builtin_memcpy(&iVar5, p, 4);
              if (iVar5 != 0) {
                __builtin_memcpy(&iTmp, (char *)(iVar5 + 0xa9), 4);
                if (iTmp != 0) {
                  cVar4 = *(char *)(iTmp + 7);
                  if (cVar4 != '\x03' && cVar4 != '\x04') {
                    ps60[uStack_e0] = (unsigned short)((unsigned short)*(unsigned char *)(iVar5 + 0x89) << 8 | (unsigned short)iVar19);
                    uStack_e0 = uStack_e0 + 1;
                  }
                }
              }
            }
            p = p + 0x23;
            iVar19 = iVar19 + 1;
          } while (iVar19 < DAT_009a1fe0);
        }
        puVar11 = puVar11 + 1;
        uVar15 = uVar15 - 1;
      } while (uVar15 != 0);
    }
  }
  uVar15 = (unsigned int)uStack_e0;
  iVar19 = 0;
  do {
    iVar5 = iVar19 + 1;
    for (iVar20 = iVar5; iVar20 < (int)uVar15; iVar20 = iVar20 + 1) {
      uVar8 = ps60[iVar20];
      uVar18 = ps60[iVar19];
      if ((unsigned int)(uVar8 & 0xff00) < (unsigned int)(uVar18 & 0xffffff00u)) {
        ps60[iVar19] = uVar8;
        ps60[iVar20] = uVar18;
        iVar20 = iVar5;
      }
    }
    iVar19 = iVar5;
  } while (iVar5 < (int)uVar15);
  if (uStack_e0 == 0) {
    short *psVar1 = &DAT_00a08ba0 + uVar7;
    if (*psVar1 != -2) {
      if (*psVar1 == 0) {
        *psVar1 = 0;
      }
      else {
        *psVar1 = -2;
      }
      goto LAB_0044e070;
    }
  }
  uVar8 = DAT_009a20ae + 2;
  if (param_1 == 1 && (&DAT_009e9683)[iVar6] != '\0') {
    sStack_e2 = DAT_00a083bc + 2;
    sVar21 = DAT_00a08376 + 2;
    local_e6 = DAT_00a08378 + -2 + DAT_00a083bc;
    sStack_e4 = (DAT_00a083b8 - 2) + DAT_00a08376;
  }
  else {
    sStack_e2 = DAT_00a08606 + 2;
    sVar21 = DAT_00a08608 + 2;
    local_e6 = DAT_00a08a00 + -2 + DAT_00a08606;
    if ((&DAT_00a08ba0)[uVar7] != -2 && (&DAT_009e9683)[iVar6] == '\0') {
      sStack_e4 = (uStack_e0 + 1) * uVar8 + DAT_00a08608;
      if ((int)sStack_e4 <= (int)((unsigned int)DAT_00a08604 + (unsigned int)DAT_00a08608 + -2) &&
          (int)(unsigned int)DAT_00a08608 < (int)sStack_e4)
        goto LAB_0044d311;
    }
    sStack_e4 = (DAT_00a08604 - 2) + DAT_00a08608;
  }
LAB_0044d311:
  if ((&DAT_009e9683)[iVar6] == '\0' && DAT_00521548 == 0xffff) {
    DAT_00521548 = uVar7;
  }
  sVar17 = (&DAT_00a08ba0)[uVar7];
  if (*(short *)(&DAT_00a0a1e0 + ((unsigned int)DAT_00a08374 + (unsigned int)(unsigned short)(param_1 + 0x82)) * 2) == sVar17) {
    sVar17 = uVar8 + sVar21;
    local_d4 = (unsigned short)DAT_009ed232;
    FUN_004a9630(1);
    FUN_004a9590(0x46);
    FUN_004a9640('\0');
    FUN_004a9520(sStack_e2, sVar21, local_e6, sStack_e4);
    if ((&DAT_009e9683)[M5BD] == '\0') {
      if ((&DAT_009e9a77)[M5BD] == '\0') {
        uVar26 = 0x2c;
      }
      else {
        uVar26 = (unsigned int)DAT_005233c8;
      }
    }
    else {
      uVar26 = 0x34;
    }
    FUN_004a95c0(uVar26);
    if (uStack_e0 == DAT_005569d8) {
      bVar25 = 0;
    }
    else {
      if ((&DAT_009e9a77)[M5BD] == '\0') {
        FUN_004a95c0((unsigned int)DAT_005233c8);
      }
      if (param_1 == 1 && (&DAT_009e9683)[M5BD] != '\0') {
        sVar10 = (DAT_00a083b8 - 2) + DAT_00a08376;
      }
      else {
        sVar10 = (DAT_00a08604 - 2) + DAT_00a08608;
      }
      FUN_004a9520(sStack_e2 + -2, sVar17, local_e6 + 2, sVar10 + 2);
      (*DAT_009d1300)();
      if ((&DAT_009e9a77)[M5BD] == '\0') {
        FUN_004a95c0(0x2c);
        sVar10 = uStack_e0 * uVar8 + sVar17;
        if ((int)((unsigned int)DAT_00a08604 + (unsigned int)DAT_00a08608 + -2) < (int)sVar10) {
          sVar10 = sStack_e4;
        }
        FUN_004a9520(sStack_e2 + -2, sVar17, local_e6 + 2, sVar10 + 2);
        (*DAT_009d1300)();
      }
      bVar25 = 1;
      DAT_005569d8 = uStack_e0;
    }
  }
  else {
    if ((&DAT_009e9683)[iVar6] == '\0') {
      if (sVar17 == -2 || (&DAT_009e9a77)[iVar6] != '\0') {
        uVar26 = (unsigned int)DAT_005233c8;
      }
      else {
        uVar26 = 0x2c;
      }
    }
    else {
      uVar26 = 0x34;
    }
    FUN_004a95c0(uVar26);
    sVar17 = sStack_e4;
    if ((&DAT_009e9683)[M5BD] != '\0') {
      uVar18 = DAT_00a08608;
      uVar9 = DAT_00a08604;
      if (param_1 == 1) {
        uVar18 = DAT_00a08376;
        uVar9 = DAT_00a083b8;
      }
      sVar17 = (uVar9 - 2) + uVar18;
    }
    FUN_004a9520(sStack_e2 + -2, sVar21 + -2, local_e6 + 2, sVar17 + 2);
    (*DAT_009d1300)();
    FUN_004a9520(sStack_e2 + -2, sVar21 + -2, local_e6 + 2, sStack_e4 + 2);
    if ((&DAT_00a08ba0)[uVar7] == -2) {
      if (DAT_00521548 == uVar7) {
        DAT_00521548 = DAT_0052154c;
        uVar15 = FUN_00480530();
        DAT_0052154c = (unsigned short)uVar15;
      }
      FUN_0040e0c0(0, 0x140, 200, 0);
      return ((unsigned int)extraout_var << 16) | (unsigned int)ps_c0[0];
    }
    FUN_004a9630(1);
    FUN_004a9590(0x46);
    FUN_004a9640('\0');
    FUN_004a9520(sStack_e2, sVar21, local_e6, sStack_e4);
    if ((&DAT_009e9683)[M5BD] == '\0') {
      if ((&DAT_009e9a77)[M5BD] == '\0') {
        uVar26 = 0x2c;
      }
      else {
        uVar26 = (unsigned int)DAT_005233c8;
      }
    }
    else {
      uVar26 = 0x34;
    }
    FUN_004a95c0(uVar26);
    bVar3 = DAT_009ed232;
    uVar18 = (unsigned short)DAT_009ed232;
    FUN_004a9500(sStack_e2, sVar21);
    FUN_004a9960(DAT_00a0a160);
    local_d4 = (unsigned short)bVar3;
    FUN_004a97f0(DAT_00a0a0e4);
    FUN_004a9820(&DAT_00524078);
    FUN_004a9820(DAT_00a0a0e8);
    FUN_004a9500(sStack_e2 + local_d4 * 0xb, sVar21);
    FUN_004a9960(&DAT_009a1ed0);
    sVar17 = DAT_00a08108 + local_d4;
    if (param_1 == 1) {
      FUN_004a9500(sVar17, sVar21);
      FUN_004a9960(DAT_00a0a164);
      if (DAT_005233ec != 0x13) {
        FUN_004a9500(sVar17 + local_d4 * 0x11, sVar21);
        FUN_004a9960(DAT_00a0a168);
      }
    }
    else if ((&DAT_009e9683)[M5BD] == '\0' || DAT_005233ec != 0x13) {
      FUN_004a9500(sVar17, sVar21);
      FUN_004a9ac0(DAT_00a0a164);
    }
    bVar25 = 1;
    *(unsigned short *)(&DAT_005569ec + (unsigned int)param_1 * 2) = 0;
    *(unsigned short *)(&DAT_009d8b8c + M10) = 0x1d8;
    FUN_004a9520(sStack_e2, uVar8 + sVar21, local_e6, sStack_e4);
    DAT_005569d8 = uStack_e0;
    local_d4 = uVar18;
  }
  local_e8 = uVar8 + sVar21;
  if (DAT_00521548 == uVar7) {
    FUN_004a95c0(0x46);
    uVar14 = ((unsigned int)uVar24 << 16) | (unsigned short)(sVar21 + -2);
    ps_c0[0] = (unsigned short)(sStack_e4 + 2);
    FUN_004a9520(sStack_e2 - 2U, sVar21 + -2, local_e6 + 2U, sStack_e4 + 2);
    uVar26 = (unsigned short)(sStack_e2 - 2U);
    uVar13 = (unsigned short)(local_e6 + 2U);
    uVar16 = local_c0[0];
LAB_0044d9ba:
    (*DAT_009eca20)(uVar26, uVar14, uVar13, uVar16, 1);
  }
  else {
    if (DAT_0052154c == uVar7) {
      if ((&DAT_009e9683)[M5BD] == '\0') {
        if ((&DAT_009e9a77)[M5BD] == '\0') {
          uVar26 = 0x4e;
        }
        else {
          uVar26 = (unsigned int)DAT_005233c8;
        }
      }
      else {
        uVar26 = 0x34;
      }
      FUN_004a95c0(uVar26);
      uVar16 = (unsigned short)(sStack_e4 + 2U);
      uVar14 = ((unsigned int)uVar24 << 16) | (unsigned short)(sVar21 + -2);
      ps_c0[0] = (unsigned short)(local_e6 + 2);
      FUN_004a9520(sStack_e2 - 2U, sVar21 + -2, local_e6 + 2, sStack_e4 + 2U);
      uVar26 = (unsigned short)(sStack_e2 - 2U);
      uVar13 = local_c0[0];
      goto LAB_0044d9ba;
    }
    if ((&DAT_009e9a77)[M5BD] == '\0') {
      FUN_004a95c0(0x4e);
      uVar16 = (unsigned short)(sStack_e4 + 2U);
      uVar13 = (unsigned short)(local_e6 + 2U);
      uVar14 = ((unsigned int)uVar24 << 16) | (unsigned short)(sVar21 + -2);
      ps_c0[0] = (unsigned short)(sStack_e2 + -2);
      FUN_004a9520(sStack_e2 + -2, sVar21 + -2, local_e6 + 2U, sStack_e4 + 2U);
      uVar26 = local_c0[0];
      goto LAB_0044d9ba;
    }
  }
  if ((&DAT_009e9683)[M5BD] == '\0') {
    if ((&DAT_009e9a77)[M5BD] == '\0') {
      uVar26 = 0x2c;
    }
    else {
      uVar26 = (unsigned int)DAT_005233c8;
    }
  }
  else {
    uVar26 = 0x34;
  }
  FUN_004a95c0(uVar26);
  if (DAT_00521548 == uVar7) {
    if (DAT_009d6930 == 0xa6) {
      if (*(short *)(&DAT_005569ec + (unsigned int)param_1 * 2) != 0) {
        *(short *)(&DAT_005569ec + (unsigned int)param_1 * 2) =
            *(short *)(&DAT_005569ec + (unsigned int)param_1 * 2) + -1;
        bVar25 = 1;
      }
    }
    else if (DAT_009d6930 == 0xa7 && 1 < uStack_e0) {
      if ((int)(unsigned int)*(unsigned short *)(&DAT_005569ec + (unsigned int)param_1 * 2) <=
          (int)((unsigned int)((int)((unsigned int)local_e8 - (unsigned int)(int)sStack_e4) / (int)(unsigned int)uVar8) + uVar15)) {
        *(unsigned short *)(&DAT_005569ec + (unsigned int)param_1 * 2) =
            *(unsigned short *)(&DAT_005569ec + (unsigned int)param_1 * 2) + 1;
        bVar25 = 1;
      }
    }
  }
  if (*(short *)(&DAT_009d8b8c + M10) < 1) {
    bVar25 = 1;
    *(short *)(&DAT_009d8b8c + M10) = 0x1d8;
  }
  puVar2 = (unsigned short *)(&DAT_005569ec + (unsigned int)param_1 * 2);
  uVar7 = *puVar2;
  sVar21 = (short)((int)((unsigned int)(int)sStack_e4 - (unsigned int)local_e8) / (int)(unsigned int)uVar8);
  uStack_ec = uVar7 + sVar21;
  if (uStack_e0 < uStack_ec) {
    if (uVar7 != 0) {
      *puVar2 = uVar7 - 1;
    }
    uStack_ec = *puVar2 + sVar21;
  }
  uStack_ee = 0xffff;
  if (bVar25) {
    FUN_004a9520(sStack_e2, local_e8, local_e6, sStack_e4);
    (*DAT_009d1300)();
    uStack_cc = 0;
    if (uStack_e0 != 0) {
      local_c8 = ps60;
      do {
        FUN_004a9590(0x43);
        if ((int)(unsigned int)*puVar2 <= (int)uStack_cc && (int)uStack_cc <= (int)(unsigned int)uStack_ec) {
          uVar7 = *local_c8;
          uVar18 = uVar7 >> 8;
          pbVar12 = (unsigned char *)(s_CJNRFCJNRF_00523f80 + uVar18);
          FUN_004a9590((unsigned int)*pbVar12);
          if (uStack_ee != uVar18 && *puVar2 == uStack_cc) {
            if ((&DAT_009e9683)[M5BD] != '\0') {
              FUN_0040e0c0(0, 0x140, 200, 0);
              FUN_004a95c0(0x2c);
              FUN_004a95c0(0x34);
              FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14, MULA);
            }
            FUN_004a9520(sStack_e2, local_e8, local_e6, sStack_e4);
            uStack_ee = uVar18;
          }
          FUN_0044e0a0((((unsigned int)local_e8 << 16) | (unsigned int)uVar7) & 0xffff00ffu);
          FUN_004a9500(sStack_e2, local_e8);
          DAT_009a1ed9 = 0;
          FUN_004a9960(&DAT_009a1ed0);
          sVar21 = sStack_e2 + local_d4 * 0xb;
          local_c0[0] = (uVar7 & 0xff) * 0x23u;
          __builtin_memcpy(&iVar6, (char *)(int)((unsigned int)DAT_009a1fe8 + 0x1fu + local_c0[0]), 4);
          if (iVar6 != 0) {
            __builtin_memcpy(&iVar6, (char *)(iVar6 + 0xa9), 4);
            if (iVar6 == 0) {
LAB_0044ddd7:
              if (*(char *)(iVar6 + 7) == '\x03' || *(char *)(iVar6 + 7) == '\x04')
                goto LAB_0044de32;
              __builtin_memcpy(&uVar26, (char *)(iVar6 + 0xcb), 4);
              __builtin_memcpy(&uTmp, (char *)(iVar6 + 0xc3), 4);
              if (uVar26 < uTmp) {
                uStack_e0 = 1;
                goto LAB_0044de3a;
              }
              uVar26 = FUN_00425c60(uVar26 - uTmp, uVar26);
              uVar26 = (uVar26 & 0xffff) / 0x28f;
              if (uVar26 == 0) {
                uStack_e0 = 100;
                goto LAB_0044de3a;
              }
LAB_0044de42:
              uVar14 = FUN_004a9890(uVar26);
              FUN_004a9500(DAT_00a08108 + (3 - (short)uVar14) * local_d4, local_e8);
              if (uVar26 < 0x15) {
                uVar26 = 0x4a;
              }
              else if (uVar26 < 0x33) {
                uVar26 = 0x4e;
              }
              else {
                uVar26 = 0x46;
              }
              FUN_004a9590(uVar26);
              FUN_004a9960(&DAT_009a1ed0);
            }
            else {
              if (*(char *)(iVar6 + 7) != '\x03') {
                if (*(char *)(iVar6 + 7) != '\x04') {
                  __builtin_memcpy(&iTmp, (char *)(iVar6 + 0x215), 4);
                  __builtin_memcpy(&iTmp2, (char *)(iVar6 + 0x211), 4);
                  uVar26 = (unsigned int)((int)((unsigned int)iTmp + (unsigned int)iTmp2) / 2);
                  uVar13 = FUN_00405820((((unsigned int)local_e8 << 16) | (unsigned int)uVar7) & 0xffff00ffu);
                  if (uVar26 == 0 || uVar13 == 0) {
                    uVar26 = 0;
                  }
                  else {
                    uVar26 = FUN_00425c60(uVar26, uVar13);
                    uVar26 = ((uVar26 & 0xffff) / 0x28f) * 2;
                  }
                  if (uVar26 < 0x15) {
                    uVar13 = 0x4a;
                  }
                  else if (uVar26 < 0x33) {
                    uVar13 = 0x4e;
                  }
                  else {
                    uVar13 = 0x46;
                  }
                  FUN_004a9590(uVar13);
                  FUN_004a9500(sVar21 + local_d4, local_e8);
                  FUN_004277f0((unsigned short)uVar26, 3, 1);
                  FUN_004a9590((unsigned int)*pbVar12);
                  (*DAT_009a6fe4)(0x2f);
                }
                goto LAB_0044ddd7;
              }
LAB_0044de32:
              uStack_e0 = 0;
LAB_0044de3a:
              uVar26 = (unsigned int)uStack_e0;
              if (uVar26 != 0) goto LAB_0044de42;
            }
            FUN_004a9590((unsigned int)*pbVar12);
            sVar21 = local_d4 * 2 + DAT_00a08108;
            if ((&DAT_009e9683)[M5BD] == '\0' || param_1 != 0 || DAT_005233ec != 0x13) {
              FUN_004a9500(sVar21, local_e8);
              __builtin_memcpy(&iVar19, (char *)(int)((unsigned int)DAT_009a1fe8 + 0x1bu + local_c0[0]), 4);
              if (iVar19 == -1) {
                uVar18 = *(unsigned short *)(iVar6 + 0x42);
                if (uVar18 == 0xff || uVar18 == 0xffff || 0x7fff < uVar18)
                  goto LAB_0044df8d;
LAB_0044df83:
                FUN_0043c290(uVar18, 7);
              }
              else {
                __builtin_memcpy(&uVar18, &DAT_009e968a + (int)((unsigned int)iVar19 * 0x5bdu), 2);
                if (uVar18 != 0xffff && (int)(unsigned int)uVar18 < DAT_009a1fe0) {
                  __builtin_memcpy(&iTmp, (char *)(int)((unsigned int)uVar18 * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8), 4);
                  if (iTmp != 0) goto LAB_0044df83;
                }
LAB_0044df8d:
                FUN_004a97f0((char *)DAT_00a08c70);
              }
              if (param_1 == 0 && DAT_005233ec != 0x13 &&
                  (&DAT_009e9683)[M5BD] == '\0') {
                FUN_004a9ac0(&DAT_009a1ed0);
              }
              else {
                FUN_004a9960(&DAT_009a1ed0);
              }
            }
          }
          if (param_1 != 0 && DAT_005233ec != 0x13) {
            FUN_004a9500(sVar21 + local_d4 * 0xf, local_e8);
            uVar26 = FUN_0044e170((((unsigned int)local_e8 << 16) | (unsigned int)uVar7) & 0xffff00ffu);
            pbVar12 = DAT_00a08c70;
            if ((uVar26 & 0xffff) != 0) {
              pbVar12 = (unsigned char *)(&DAT_00a0a130)[uVar26 & 0xffff];
            }
            FUN_004a9960(pbVar12);
          }
          local_e8 = local_e8 + uVar8;
        }
        local_c8 = local_c8 + 1;
        uStack_cc = uStack_cc + 1;
      } while ((int)uStack_cc < (int)uVar15);
    }
  }
LAB_0044e070:
  uVar15 = FUN_0040e0c0(0, 0x140, 200, 0);
  return uVar15;
}
