/* SPDX-License-Identifier: MIT */
/* z_xvt 004a10d0 */
/* Prototypes */
int  __cdecl FUN_004a1980(uint param_1);
void __cdecl FUN_004a01a0(uint param_1, uint param_2);
uint __cdecl FUN_00478420(uint param_1, uint param_2);

/* Globals (byte arrays are accessed unaligned via memcpy) */
extern unsigned char *DAT_006d4988;
extern uint DAT_00709310[];
extern ushort DAT_00709374[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_0071b738[];
extern unsigned char DAT_0071b740[];
extern unsigned char DAT_0071b79d[];
extern unsigned char DAT_0071b7a1[];
extern unsigned char DAT_0071b9c1[];
extern unsigned char DAT_0071b9c3[];
extern unsigned char DAT_0071b9c5[];
extern unsigned char DAT_0071b9c7[];
extern unsigned char DAT_0071e520[];

uint __cdecl FUN_004a10d0(uint param_1, int param_2)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  unsigned char *puVar7;
  unsigned char *puVar9;
  uint uVar10;
  unsigned char *puVar11;
  unsigned char *iVar12;
  uint uVar13;
  uint uVar14;
  unsigned char *puVar15;
  int local_8;
  uint local_4;
  unsigned char *p;
  unsigned char *pInner;
  unsigned char *piVar8;
  uint idx;
  uint tmpu;
  int tmpi;
  ushort tmps;
  ushort tmps2;
  short tmpsh;

  local_4 = 0xffffffff;
  iVar5 = FUN_004a1980(param_1);
  uVar6 = (uint)(iVar5 / 2);
  p = DAT_006d4988 + param_1 * 0x23;

  idx = DAT_0070ad48[(uint)p[0x16] * 0x566];
  DAT_00709310[idx] = DAT_00709310[idx] - uVar6;
  idx = DAT_0070ad48[(uint)p[0x16] * 0x566];
  DAT_00709374[idx] = (ushort)(DAT_00709374[idx] + 1);

  __builtin_memcpy(&iVar5, p + 0x1b, 4);
  if (iVar5 == -1) {
    __builtin_memcpy(&iVar5, DAT_0070b252 + (uint)p[0x16] * 0x566, 4);
    if (iVar5 == -1) {
      return uVar6;
    }
    if (DAT_0071b740[(uint)iVar5 * 0x5bd] == 0) {
      return uVar6;
    }
  }
  __builtin_memcpy(&pInner, p + 0x1f, 4);
  __builtin_memcpy(&iVar12, pInner + 0xa7, 4);
  iVar4 = (int)((uint)iVar5 * 0x5bd);
  __builtin_memcpy(&tmpu, DAT_0071b79d + iVar4, 4);
  tmpu = tmpu - uVar6;
  __builtin_memcpy(DAT_0071b79d + iVar4, &tmpu, 4);

  if (param_2 != 0) {
    __builtin_memcpy(&tmps, iVar12 + 0xc3, 2);
    __builtin_memcpy(&tmps2, iVar12 + 199, 2);
    if (tmps < tmps2) {
      uVar6 = (uint)tmps2 - (uint)tmps;
    }
    else {
      uVar6 = 0;
    }
    __builtin_memcpy(&tmps, iVar12 + 0x20b, 2);
    if (0 < (short)tmps) {
      uVar6 = uVar6 + tmps;
    }
    __builtin_memcpy(&tmps, iVar12 + 0x20d, 2);
    if (0 < (short)tmps) {
      uVar6 = uVar6 + tmps;
    }
    uVar10 = 0;
    piVar8 = iVar12 + 0x1a1;
    iVar5 = 8;
    do {
      __builtin_memcpy(&iVar1, piVar8, 4);
      piVar8 = piVar8 + 4;
      uVar10 = uVar10 + (uint)iVar1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    piVar8 = iVar12 + 0x1c1;
    iVar5 = 6;
    do {
      __builtin_memcpy(&iVar1, piVar8, 4);
      piVar8 = piVar8 + 4;
      uVar10 = uVar10 + (uint)iVar1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    if (uVar6 >> 1 < uVar10) {
      FUN_004a01a0(param_1, param_1);
    }
  }
  {
    uint a, b;
    __builtin_memcpy(&a, iVar12 + 0xd1, 4);
    __builtin_memcpy(&b, iVar12 + 0xcd, 4);
    uVar6 = FUN_00478420(a, b);
  }
  if (0x7fff < (ushort)uVar6) {
    puVar9 = iVar12 + 0x1a1;
    __builtin_memcpy(&tmpsh, DAT_0071b9c1 + iVar4, 2);
    tmpsh = (short)(tmpsh + 1);
    __builtin_memcpy(DAT_0071b9c1 + iVar4, &tmpsh, 2);
    uVar6 = 0;
    iVar5 = 8;
    local_8 = 0;
    puVar15 = puVar9;
    do {
      __builtin_memcpy(&uVar10, puVar15, 4);
      puVar15 = puVar15 + 4;
      uVar6 = uVar6 + uVar10;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    uVar10 = 0;
    puVar15 = iVar12 + 0x1c1;
    iVar5 = 6;
    puVar7 = puVar15;
    do {
      __builtin_memcpy(&uVar14, puVar7, 4);
      puVar7 = puVar7 + 4;
      uVar10 = uVar10 + uVar14;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    __builtin_memcpy(&uVar13, iVar12 + 0xd5, 4);
    __builtin_memcpy(&uVar2, iVar12 + 0xd9, 4);
    __builtin_memcpy(&uVar3, iVar12 + 0xdd, 4);
    uVar14 = uVar2 | uVar3 | uVar13 | uVar10 | uVar6;
    if (uVar14 == 0) {
      local_8 = 0xfa;
    }
    else if ((((uVar13 < uVar3) || (uVar13 < uVar2)) || (uVar13 < uVar6)) || (uVar13 < uVar10)) {
      if (((uVar2 < uVar3) || (uVar2 < uVar13)) || ((uVar2 < uVar6 || (uVar2 < uVar10)))) {
        if (((uVar3 < uVar2) || (uVar3 < uVar13)) || ((uVar3 < uVar6 || (uVar3 < uVar10)))) {
          if ((((uVar6 < uVar2) || (uVar6 < uVar13)) || (uVar6 < uVar3)) || (uVar6 < uVar10)) {
            uVar6 = 0;
            iVar5 = 6;
            do {
              __builtin_memcpy(&tmpu, puVar15, 4);
              if (uVar6 < tmpu) {
                __builtin_memcpy(&uVar6, puVar15, 4);
              }
              puVar15 = puVar15 + 4;
              iVar5 = iVar5 + -1;
            } while (iVar5 != 0);
          }
          else {
            uVar6 = 0;
            uVar13 = 0xffffffff;
            uVar10 = 0;
            puVar11 = DAT_0071b738;
            do {
              __builtin_memcpy(&tmpu, puVar9, 4);
              if (uVar6 < tmpu) {
                __builtin_memcpy(&tmps, puVar11, 2);
                uVar13 = (uint)tmps;
                uVar6 = tmpu;
                uVar14 = uVar10;
              }
              puVar11 = puVar11 + 0x5bd;
              puVar9 = puVar9 + 4;
              uVar10 = uVar10 + 1;
            } while ((uint)puVar11 < (uint)DAT_0071e520);
            if (uVar13 != 0xffffffff) {
              local_4 = uVar14;
            }
          }
        }
        else {
          local_8 = 0x19;
          __builtin_memcpy(&tmpsh, DAT_0071b9c7 + iVar4, 2);
          tmpsh = (short)(tmpsh + 1);
          __builtin_memcpy(DAT_0071b9c7 + iVar4, &tmpsh, 2);
        }
      }
      else {
        local_8 = 0x32;
        __builtin_memcpy(&tmpsh, DAT_0071b9c5 + iVar4, 2);
        tmpsh = (short)(tmpsh + 1);
        __builtin_memcpy(DAT_0071b9c5 + iVar4, &tmpsh, 2);
      }
    }
    else {
      __builtin_memcpy(&tmpsh, DAT_0071b9c3 + iVar4, 2);
      tmpsh = (short)(tmpsh + 1);
      __builtin_memcpy(DAT_0071b9c3 + iVar4, &tmpsh, 2);
      __builtin_memcpy(&tmps, DAT_0071b738 + iVar4, 2);
      local_8 = (int)((uint)tmps * 5 + 5);
    }
    __builtin_memcpy(&tmpi, DAT_0071b7a1 + iVar4, 4);
    tmpi = (int)((uint)tmpi - (uint)local_8);
    __builtin_memcpy(DAT_0071b7a1 + iVar4, &tmpi, 4);
  }
  return local_4;
}
