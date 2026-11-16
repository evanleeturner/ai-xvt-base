/* SPDX-License-Identifier: MIT */
/* z_bop 00436d00 */
extern unsigned char *DAT_009a1fe8;
extern uint DAT_009a8e2c;
extern uint DAT_009a8e30;
extern uint DAT_009a8e34;
extern short DAT_009a6fe0;
extern short DAT_00a0813c;
extern int DAT_009ecc34;
extern int DAT_009eca28;
extern unsigned char DAT_009e968a;
extern unsigned char DAT_009e9a5b;
extern unsigned char DAT_009e9a5f;
extern unsigned char DAT_009e9a63;
extern unsigned char DAT_009e9a67;
extern unsigned char DAT_009d8c87;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009fd39c;
extern int DAT_009fd3a0;
extern unsigned char DAT_00a08100;
extern int DAT_00a081ec;
extern uint DAT_009a7bac;
extern unsigned char DAT_009a1ed0;
extern unsigned char DAT_009a20ae;
extern uint DAT_009a8c00;
extern unsigned char DAT_00521540;
extern void (*DAT_009cc454)(int, int, int, int, uint);
extern void (*DAT_009a6fe4)(int);

int FUN_00427570(uint, uint);
int FUN_00427590(uint, uint);
void FUN_00458e20(uint, uint);
uint FUN_00427420(int, int, int);
uint FUN_00427490(int, int, int);
uint FUN_00427500(int, int, int);
void FUN_00427390(uint, uint, uint);
void FUN_004a95c0(uint);
void FUN_004a9640(char);
void FUN_00427bd0(short, short, unsigned char *);
void FUN_00427d30(short, int, unsigned char *);
int FUN_00483030(uint);
void FUN_0043c290(ushort, int);
unsigned long long FUN_004278c0(int, int, unsigned char *);
void FUN_004a9500(short, short);
void FUN_004a9960(unsigned char *);
void FUN_004034e0(uint, uint);
void FUN_004277f0(ushort, int, int);

void FUN_00436d00(uint param_1)
{
  char cVar2;
  unsigned char uVar3;
  ushort uVar4;
  unsigned char *pEnt;
  unsigned char *pSub;
  unsigned char *pcVar5;
  long long lVar6;
  long long lVar7;
  long long lVar8;
  int iVar9;
  int bVar10;
  uint uVar11;
  short sVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  uint uVar19;
  uint uVar20;
  uint uVar21;
  uint uVar22;
  uint extraout_ECX = 0; /* decompiler extraout register; value unknown in original */
  uint extraout_EDX = 0; /* decompiler extraout register; value unknown in original */
  uint uVar23;
  unsigned long long uVar24;
  uint uVar25;
  short local_18;
  short local_8;
  ushort us;
  short s16;
  short s16b;
  int iv;

  uVar21 = DAT_009a8e34;
  uVar11 = DAT_009a8e30;
  uVar23 = DAT_009a8e2c;
  iVar9 = (int)(param_1 * 0x23);
  cVar2 = *(char *)(DAT_009a1fe8 + 2 + iVar9);
  if (((cVar2 != '\r') && (cVar2 != '\v')) && (0 < (int)DAT_009a8e34)) {
    iVar13 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
    iVar14 = FUN_00427590(uVar11, uVar21);
    iVar13 = (int)((uint)iVar13 + (uint)(int)DAT_009a6fe0);
    iVar14 = (int)((uint)iVar14 + (uint)(int)DAT_00a0813c);
    __builtin_memcpy(&us, &DAT_009e968a + DAT_009ecc34 * 0x5bd, 2);
    if ((uint)us == param_1) {
      uVar22 = 0xffff;
      if ((int)param_1 < DAT_009eca28) {
        __builtin_memcpy(&pEnt, DAT_009a1fe8 + 0x1f + iVar9, 4);
        __builtin_memcpy(&pSub, pEnt + 0xa9, 4);
        __builtin_memcpy(&us, pSub + 0x42, 2);
        uVar22 = (uint)us;
      }
      else {
        __builtin_memcpy(&pEnt, DAT_009a1fe8 + 0x1f + iVar9, 4);
        if (pEnt != 0) {
          __builtin_memcpy(&pSub, pEnt + 0xa9, 4);
          if (pSub != 0) {
            __builtin_memcpy(&us, pSub + 4, 2);
            uVar22 = (uint)us;
          }
        }
      }
      if (uVar22 != 0xffff) {
        FUN_00458e20(uVar22, (uint)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar9));
        iVar17 = DAT_009ecc34 * 0x5bd;
        __builtin_memcpy(&iv, &DAT_009e9a5b + iVar17, 4);
        DAT_00a07cdc = (int)((uint)DAT_00a07cdc - (uint)iv);
        __builtin_memcpy(&iv, &DAT_009e9a5f + iVar17, 4);
        DAT_00a07cd8 = (int)((uint)DAT_00a07cd8 - (uint)iv);
        __builtin_memcpy(&iv, &DAT_009e9a63 + iVar17, 4);
        DAT_00a07cd4 = (int)((uint)DAT_00a07cd4 - (uint)iv);
        DAT_009a8e2c = FUN_00427420(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
        DAT_009a8e30 = FUN_00427490(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
        DAT_009a8e34 = FUN_00427500(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
        if ((int)DAT_009a8e34 < 1) {
          FUN_00427390(uVar23, uVar11, uVar21);
        }
        uVar25 = 0x36;
        iVar17 = iVar13;
        iVar18 = iVar14;
        iVar15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
        iVar15 = (int)((uint)iVar15 + (uint)(int)DAT_00a0813c);
        iVar16 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
        (*DAT_009cc454)((int)((uint)iVar16 + (uint)(int)DAT_009a6fe0), iVar15, iVar17, iVar18, uVar25);
      }
    }
    __builtin_memcpy(&iVar17, DAT_009a1fe8 + iVar9 + 0x1f, 4);
    if (iVar17 == 0) {
      uVar3 = (&DAT_009d8c87)[(uint)*(unsigned char *)(DAT_009a1fe8 + iVar9 + 0x16) * 0x566];
    }
    else {
      uVar3 = *(unsigned char *)(iVar17 + 0x88);
    }
    switch (uVar3) {
    case 0:
      uVar22 = 0x3f;
      break;
    case 1:
    case 4:
      uVar22 = 0x37;
      break;
    case 2:
      uVar22 = 0x33;
      break;
    default:
      uVar22 = 0x3b;
    }
    FUN_004a95c0(uVar22);
    FUN_004a9640('\0');
    bVar10 = 0;
    if ((int)param_1 < DAT_009eca28) {
      bVar10 = 1;
    }
    else {
      __builtin_memcpy(&pEnt, DAT_009a1fe8 + 0x1f + iVar9, 4);
      if (pEnt == 0) {
        bVar10 = 1;
      }
      else {
        __builtin_memcpy(&iv, pEnt + 0xa9, 4);
        if (iv != 0) {
          bVar10 = 1;
        }
      }
    }
    if (bVar10) {
      __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
      DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
      __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
      DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
      __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
      DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
      if ((int)DAT_009a8e34 < 1) {
        FUN_00427390(uVar23, uVar11, uVar21);
      }
      iVar17 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
      iVar18 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
      iVar17 = (int)((uint)iVar17 + (uint)(int)DAT_009a6fe0);
      iVar18 = (int)((uint)iVar18 + (uint)(int)DAT_00a0813c);
      (*DAT_009cc454)(iVar17, iVar18, iVar13, iVar14,
                      ((uint)iVar17 & 0xffffff00u) | (uint)DAT_00a08100);
      pcVar5 = DAT_009a1fe8 + iVar9;
      __builtin_memcpy(&pEnt, pcVar5 + 0x1f, 4);
      if ((pEnt != 0) && (*pEnt == 0)) {
        if (pEnt[0x92] != 0) {
          __builtin_memcpy(&s16, pcVar5 + 0x12, 2);
          __builtin_memcpy(&s16b, pcVar5 + 0x10, 2);
          FUN_00427bd0(s16, s16b, pcVar5);
          __builtin_memcpy(&s16, DAT_009a1fe8 + iVar9 + 0x14, 2);
          FUN_00427d30(s16, 0, DAT_009a1fe8 + iVar9);
        }
        uVar22 = DAT_009a8e34;
        uVar11 = DAT_009a8e30;
        uVar23 = DAT_009a8e2c;
        __builtin_memcpy(&pEnt, DAT_009a1fe8 + 0x1f + iVar9, 4);
        __builtin_memcpy(&s16, pEnt + 0x8c, 2);
        lVar6 = (long long)(int)s16 * 0x100;
        __builtin_memcpy(&s16b, pEnt + 0x8e, 2);
        lVar7 = (long long)(int)s16b * 0x100;
        __builtin_memcpy(&s16, pEnt + 0x8c, 2);
        uVar19 = (uint)(int)s16;
        __builtin_memcpy(&us, pEnt + 0x79, 2);
        if (us < 0x400) {
          lVar8 = (long long)(int)((uint)us << 5) * (long long)(int)uVar19;
          uVar19 = (uint)lVar8 >> 0xf | ((uint)((unsigned long long)lVar8 >> 0x20)) << 0x11;
          __builtin_memcpy(&us, pEnt + 0x79, 2);
          __builtin_memcpy(&s16b, pEnt + 0x8e, 2);
          lVar8 = (long long)(int)((uint)us << 5) * (long long)(int)s16b;
          uVar20 = (uint)lVar8 >> 0xf | ((uint)((unsigned long long)lVar8 >> 0x20)) << 0x11;
        }
        else {
          __builtin_memcpy(&s16b, pEnt + 0x8e, 2);
          uVar20 = (uint)(int)s16b;
        }
        DAT_009fd39c = (int)((uint)DAT_009fd39c +
                             ((uint)lVar6 >> 0xf | ((uint)((unsigned long long)lVar6 >> 0x20)) << 0x11) +
                             uVar19);
        DAT_009fd3a0 = (int)((uint)DAT_009fd3a0 +
                             ((uint)lVar7 >> 0xf | ((uint)((unsigned long long)lVar7 >> 0x20)) << 0x11) +
                             uVar20);
        __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
        DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
        __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
        DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
        __builtin_memcpy(&iv, &DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
        DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, (int)(0xffff0000u - (uint)iv));
        if ((int)DAT_009a8e34 < 1) {
          FUN_00427390(uVar23, uVar11, uVar22);
        }
        uVar23 = (uint)DAT_00a08100;
        iVar15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
        iVar15 = (int)((uint)iVar15 + (uint)(int)DAT_00a0813c);
        iVar16 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
        (*DAT_009cc454)((int)((uint)iVar16 + (uint)(int)DAT_009a6fe0), iVar15, iVar17, iVar18, uVar23);
      }
      FUN_004a95c0(0x40);
      local_8 = (short)iVar13;
      local_18 = (short)iVar14;
      iVar13 = FUN_00483030(param_1);
      uVar21 = (uint)((uint)DAT_00a081ec * (uint)iVar13) / uVar21;
      if (uVar21 < DAT_009a7bac / 0x50) {
        uVar21 = DAT_009a7bac / 0x50;
      }
      if (DAT_009a7bac >> 1 < uVar21) {
        uVar21 = DAT_009a7bac >> 1;
      }
      iVar13 = (int)(uVar21 + 4);
      __builtin_memcpy(&iv, DAT_009a1fe8 + iVar9 + 0x1f, 4);
      if ((iv == 0) && (*(char *)(DAT_009a1fe8 + iVar9 + 2) == '\b')) {
        DAT_009a1ed0 = 0;
      }
      else {
        FUN_0043c290((ushort)param_1, 2);
      }
      if (DAT_009a1ed0 != 0) {
        iVar14 = (int)((((uint)(int)local_18 - (uint)DAT_009a20ae) - (uint)(iVar13 / 2)) + 0xffffffffu);
        uVar24 = FUN_004278c0(iVar14, iVar13 >> 0x1f, &DAT_009a1ed0);
        FUN_004a9500((short)((int)local_8 - (int)((ushort)uVar24 >> 1)), (short)iVar14);
        FUN_004a9960(&DAT_009a1ed0);
      }
      cVar2 = *(char *)(DAT_009a1fe8 + 2 + iVar9);
      if ((cVar2 != '\x06') && (cVar2 != '\a')) {
        FUN_004a95c0(0x40);
        __builtin_memcpy(&uVar4, &DAT_009e9a67 + DAT_009ecc34 * 0x5bd, 2);
        if (uVar4 != 0xffff) {
          FUN_004034e0((uint)uVar4, param_1);
          sVar12 = (short)(local_18 + (short)(iVar13 >> 1) + 1);
          uVar24 = FUN_004278c0((int)extraout_ECX, (int)extraout_EDX, &DAT_00521540);
          FUN_004a9500((short)(local_8 - (short)uVar24), sVar12);
          DAT_009a8c00 = DAT_009a8c00 * 0xa1;
          uVar23 = DAT_009a8c00 >> 0x10;
          if (9999 < uVar23) {
            uVar23 = 9999;
          }
          FUN_004277f0((ushort)(uVar23 / 100), 2, 1);
          (*DAT_009a6fe4)(0x2e);
          FUN_004277f0((ushort)((short)((long long)(unsigned long long)uVar23 / -100LL) * 100 +
                                (short)uVar23),
                       2, 2);
        }
      }
    }
  }
  return;
}
