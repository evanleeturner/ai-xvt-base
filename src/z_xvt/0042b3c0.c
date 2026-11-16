/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b3c0 */
/* Assumptions (not confirmed by the supplied evidence):
 * - The declared types of the DAT_ globals below are inferred from the
 *   decompiler's casts and usage. Widths of DAT_006db6c0, DAT_0070ace0,
 *   DAT_00703abc, DAT_006ccad0, DAT_006cca90 and DAT_006ccaa8 are guesses.
 * - Struct-array reads at strides 0x23, 0xaf, 0x436, 0x126 and 0x566 can be
 *   unaligned, so they use __builtin_memcpy.
 */

extern uint DAT_006da530;
extern ushort DAT_006ccad0;
extern int DAT_006d4988;
extern uint *DAT_006db71c;
extern uint DAT_00732a38;
extern uint DAT_0070a000;
extern uint *DAT_007091a0;
extern uint DAT_00703c88;
extern uint DAT_0071eae8;
extern ushort DAT_006cca90;
extern uint *DAT_00732940;
extern uint DAT_0072f4e0;
extern ushort DAT_006ccaa8;
extern uint *DAT_006db798;
extern uint DAT_0071f2d8[2];
extern uint DAT_0070ac30[2];
extern int DAT_006d49a0;
extern uint DAT_006d4a60[];
extern uint DAT_007092c0[];
extern ushort DAT_006db6c0;
extern uint DAT_0071e6a0[];
extern int DAT_00732fe4;
extern uint DAT_0070acf0[];
extern uint DAT_007308b0[];
extern uint DAT_006daa00[];
extern uint DAT_006db57c;
extern uint DAT_0072f444;
extern ushort DAT_0070ace0;
extern uint DAT_00732e9c;
extern uint DAT_006da4f0;
extern uint DAT_00732a3c;
extern uint DAT_006d4980;
extern uint DAT_00703acc;
extern uint DAT_0073088c;
extern uint DAT_0071b704;
extern uint DAT_00732904;
extern uint DAT_006da4f4;
extern uint DAT_00703ac8;
extern uint DAT_00703c90;
extern uint DAT_0071e53c;
extern uint DAT_006db7a4;
extern uint DAT_00703ac0;
extern uint DAT_00703ca0[];
extern uint DAT_006da9e8;
extern uint DAT_006db7c0[];
extern uint DAT_006d9d44;
extern byte DAT_00703abc;
extern uint DAT_00732e88;
extern char DAT_0071b740[];
extern char DAT_0071e528[];

uint __cdecl FUN_0042bbf0(uint *param_1, uint param_2);

#define ROL1(x) (((x) << 1) | ((x) >> 31))

uint __stdcall FUN_0042b3c0(void)
{
  int iVar1;
  int iVar2;
  uint *puVar3;
  uint uVar4;
  uint *puVar5;
  char *pcVar6;
  int iVar7;
  uint uVar8;
  uint local_38;
  uint local_34;
  uint local_30;
  uint local_2c;
  uint local_28;
  uint local_24;
  uint local_20;
  uint local_1c;
  uint local_18;
  uint local_14;
  uint local_10;
  uint local_c;
  uint local_8;
  uint local_4;
  char *pb;
  char *pq;
  uint tmp;
  ushort tmp16;
  byte tmp8;

  local_38 = 0;
  if (0 < (int)DAT_006da530) {
    pcVar6 = (char *)((uint)DAT_006ccad0 * 0x23 + 3 + DAT_006d4988);
    puVar3 = DAT_006db71c;
    uVar8 = DAT_006da530;
    do {
      if (*pcVar6 != '\0') {
        iVar1 = 0x13;
        local_4 = 0;
        puVar5 = puVar3;
        do {
          local_4 = ROL1(local_4 ^ *puVar5);
          puVar5 = puVar5 + 1;
          iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
        local_38 = ROL1(local_38 ^ local_4);
      }
      puVar3 = puVar3 + 0x13;
      pcVar6 = pcVar6 + 0x23;
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  iVar1 = (int)(DAT_00732a38 - DAT_0070a000);
  if (0 < iVar1) {
    pcVar6 = (char *)(DAT_006d4988 + 3);
    pb = (char *)DAT_007091a0;
    iVar7 = iVar1;
    do {
      if (*pcVar6 != '\0') {
        iVar2 = 0x22;
        local_18 = 0;
        puVar5 = (uint *)pb;
        do {
          __builtin_memcpy(&tmp, puVar5, 4);
          local_18 = ROL1(local_18 ^ tmp);
          puVar5 = puVar5 + 1;
          iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
        __builtin_memcpy(&tmp8, puVar5, 1);
        local_18 = local_18 ^ tmp8;
        local_38 = local_38 ^ ROL1(local_18);
        local_38 = ROL1(local_38);
      }
      pb = pb + 0xaf;
      pcVar6 = pcVar6 + 0x23;
      iVar7 = iVar7 + -1;
    } while (iVar7 != 0);
    if (0 < iVar1) {
      pcVar6 = (char *)(DAT_006d4988 + 3);
      do {
        if (*pcVar6 != '\0') {
          iVar7 = 7;
          local_30 = 0;
          puVar3 = (uint *)(pcVar6 + -3);
          do {
            __builtin_memcpy(&tmp, puVar3, 4);
            local_30 = ROL1(local_30 ^ tmp);
            iVar7 = iVar7 + -1;
            puVar3 = puVar3 + 1;
          } while (iVar7 != 0);
          __builtin_memcpy(&tmp16, puVar3, 2);
          local_30 = local_30 ^ tmp16;
          __builtin_memcpy(&tmp8, (char *)puVar3 + 2, 1);
          uVar8 = ROL1(local_30) ^ (uint)tmp8;
          local_38 = local_38 ^ ROL1(uVar8);
          local_38 = ROL1(local_38);
        }
        pcVar6 = pcVar6 + 0x23;
        iVar1 = iVar1 + -1;
      } while (iVar1 != 0);
    }
  }
  if ((int)DAT_00732a38 < (int)(DAT_00703c88 + DAT_00732a38)) {
    pcVar6 = (char *)(DAT_00732a38 * 0x23 + 3 + DAT_006d4988);
    uVar8 = DAT_00703c88;
    do {
      if (*pcVar6 != '\0') {
        iVar1 = 7;
        local_34 = 0;
        puVar3 = (uint *)(pcVar6 + -3);
        do {
          __builtin_memcpy(&tmp, puVar3, 4);
          local_34 = ROL1(local_34 ^ tmp);
          iVar1 = iVar1 + -1;
          puVar3 = puVar3 + 1;
        } while (iVar1 != 0);
        __builtin_memcpy(&tmp16, puVar3, 2);
        local_34 = local_34 ^ tmp16;
        __builtin_memcpy(&tmp8, (char *)puVar3 + 2, 1);
        uVar4 = ROL1(local_34) ^ (uint)tmp8;
        local_38 = local_38 ^ ROL1(uVar4);
        local_38 = ROL1(local_38);
      }
      pcVar6 = pcVar6 + 0x23;
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  if (0 < (int)DAT_0071eae8) {
    pcVar6 = (char *)((uint)DAT_006cca90 * 0x23 + 3 + DAT_006d4988);
    pb = (char *)DAT_00732940;
    uVar8 = DAT_0071eae8;
    do {
      if (*pcVar6 != '\0') {
        iVar1 = 0xf9;
        local_1c = 0;
        puVar5 = (uint *)pb;
        do {
          __builtin_memcpy(&tmp, puVar5, 4);
          local_1c = ROL1(local_1c ^ tmp);
          puVar5 = puVar5 + 1;
          iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
        __builtin_memcpy(&tmp16, puVar5, 2);
        local_1c = local_1c ^ tmp16;
        local_38 = local_38 ^ ROL1(local_1c);
        local_38 = ROL1(local_38);
      }
      pb = pb + 0x436;
      pcVar6 = pcVar6 + 0x23;
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  if (0 < (int)DAT_0072f4e0) {
    pcVar6 = (char *)((uint)DAT_006ccaa8 * 0x23 + 3 + DAT_006d4988);
    puVar3 = DAT_006db798;
    uVar8 = DAT_0072f4e0;
    do {
      if (*pcVar6 != '\0') {
        iVar1 = 2;
        local_8 = 0;
        puVar5 = puVar3;
        do {
          local_8 = ROL1(local_8 ^ *puVar5);
          puVar5 = puVar5 + 1;
          iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
        local_38 = ROL1(local_38 ^ local_8);
      }
      puVar3 = puVar3 + 2;
      pcVar6 = pcVar6 + 0x23;
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  puVar3 = DAT_0071f2d8;
  iVar1 = 2;
  local_c = 0;
  do {
    local_c = ROL1(local_c ^ *puVar3);
    puVar3 = puVar3 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  puVar3 = DAT_0070ac30;
  iVar1 = 2;
  local_10 = 0;
  do {
    local_10 = ROL1(local_10 ^ *puVar3);
    puVar3 = puVar3 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  local_10 = ROL1(local_38 ^ local_c) ^ local_10;
  local_38 = ROL1(local_10);
  if (0 < DAT_006d49a0) {
    iVar1 = DAT_006d49a0;
    pq = (char *)DAT_006d4a60;
    do {
      iVar7 = 0x49;
      local_20 = 0;
      puVar5 = (uint *)pq;
      do {
        __builtin_memcpy(&tmp, puVar5, 4);
        local_20 = ROL1(local_20 ^ tmp);
        puVar5 = puVar5 + 1;
        iVar7 = iVar7 + -1;
      } while (iVar7 != 0);
      __builtin_memcpy(&tmp16, puVar5, 2);
      local_20 = local_20 ^ tmp16;
      local_38 = local_38 ^ ROL1(local_20);
      local_38 = ROL1(local_38);
      pq = pq + 0x126;
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  puVar3 = DAT_007092c0;
  iVar1 = 0x34c;
  local_14 = 0;
  do {
    local_14 = ROL1(local_14 ^ *puVar3);
    puVar3 = puVar3 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  uVar8 = ROL1(local_38 ^ local_14) ^ (uint)DAT_006db6c0;
  puVar3 = DAT_0071e6a0;
  iVar1 = 5;
  local_24 = 0;
  do {
    local_24 = ROL1(local_24 ^ *puVar3);
    puVar3 = puVar3 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  local_24 = local_24 ^ *(ushort *)puVar3;
  uVar8 = ROL1(uVar8) ^ ROL1(local_24);
  uVar8 = ROL1(uVar8) ^ (uint)DAT_00732fe4;
  puVar3 = (uint *)&DAT_006d49a0;
  iVar1 = 0x28;
  local_28 = 0;
  do {
    local_28 = ROL1(local_28 ^ *puVar3);
    puVar3 = puVar3 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  local_28 = local_28 ^ *(ushort *)puVar3;
  uVar8 = ROL1(uVar8) ^ ROL1(local_28);
  local_38 = ROL1(uVar8);
  if (0 < DAT_006d49a0) {
    iVar1 = DAT_006d49a0;
    pq = (char *)DAT_0070acf0;
    do {
      iVar7 = 0x158;
      local_2c = 0;
      puVar5 = (uint *)pq;
      do {
        __builtin_memcpy(&tmp, puVar5, 4);
        local_2c = ROL1(local_2c ^ tmp);
        puVar5 = puVar5 + 1;
        iVar7 = iVar7 + -1;
      } while (iVar7 != 0);
      __builtin_memcpy(&tmp16, puVar5, 2);
      local_2c = local_2c ^ tmp16;
      local_38 = local_38 ^ ROL1(local_2c);
      local_38 = ROL1(local_38);
      pq = pq + 0x566;
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  uVar8 = FUN_0042bbf0(DAT_007308b0, 0x1c80);
  local_38 = ROL1(local_38 ^ uVar8);
  puVar3 = DAT_006daa00;
  do {
    uVar8 = FUN_0042bbf0(puVar3, 0x126);
    local_38 = ROL1(local_38 ^ uVar8);
    puVar3 = (uint *)((int)puVar3 + 0x126);
  } while (puVar3 < (uint *)&DAT_006db57c);
  uVar8 = ROL1(local_38 ^ DAT_0072f444) ^ (uint)DAT_0070ace0;
  uVar8 = ROL1(uVar8) ^ DAT_0071eae8;
  uVar8 = ROL1(uVar8) ^ DAT_006da530;
  uVar8 = ROL1(uVar8) ^ DAT_0072f4e0;
  uVar8 = ROL1(uVar8) ^ DAT_00732e9c;
  uVar8 = ROL1(uVar8) ^ DAT_006da4f0;
  uVar8 = ROL1(uVar8) ^ DAT_0070a000;
  uVar8 = ROL1(uVar8) ^ DAT_00732a3c;
  uVar8 = ROL1(uVar8) ^ DAT_006d4980;
  uVar8 = ROL1(uVar8) ^ DAT_00703acc;
  uVar8 = ROL1(uVar8) ^ DAT_0073088c;
  uVar8 = ROL1(uVar8) ^ DAT_0071b704;
  uVar8 = ROL1(uVar8) ^ DAT_00732904;
  uVar8 = ROL1(uVar8) ^ DAT_006da4f4;
  uVar8 = ROL1(uVar8) ^ DAT_00703ac8;
  uVar8 = ROL1(uVar8) ^ DAT_00703c90;
  uVar8 = ROL1(uVar8) ^ DAT_0071e53c;
  uVar8 = ROL1(uVar8) ^ DAT_006db7a4;
  uVar8 = ROL1(uVar8) ^ DAT_00703ac0;
  uVar8 = ROL1(uVar8) ^ DAT_00732a38;
  uVar4 = ROL1(uVar8) ^ DAT_00703c88;
  uVar8 = FUN_0042bbf0(DAT_00703ca0, 0x5500);
  uVar8 = ROL1(uVar4) ^ uVar8;
  uVar4 = ROL1(uVar8) ^ DAT_006da9e8;
  uVar8 = FUN_0042bbf0(DAT_006db7c0, 0x1ffff);
  uVar8 = ROL1(uVar4) ^ uVar8;
  uVar8 = ROL1(uVar8) ^ DAT_006d9d44;
  uVar8 = ROL1(uVar8) ^ (uint)DAT_00703abc;
  uVar8 = ROL1(uVar8) ^ DAT_00732e88;
  local_38 = ROL1(uVar8);
  pcVar6 = DAT_0071b740;
  do {
    if (*pcVar6 != '\0') {
      uVar8 = FUN_0042bbf0((uint *)(pcVar6 + -0x10), 0x5bd);
      local_38 = ROL1(local_38 ^ uVar8);
    }
    pcVar6 = pcVar6 + 0x5bd;
  } while (pcVar6 < DAT_0071e528);
  return local_38;
}
