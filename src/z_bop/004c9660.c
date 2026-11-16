/* SPDX-License-Identifier: MIT */
/* z_bop 004c9660 */
extern int DAT_00b6a2b0;
extern int DAT_00b6d962;      /* unaligned (0x00b6d962), accessed via memcpy */
extern int DAT_00664f2c;
extern unsigned int DAT_00b6a5a6;  /* unaligned */
extern unsigned int DAT_00b6a5aa;  /* unaligned */
extern unsigned int DAT_00b6a5ae;  /* unaligned, 6 dwords */
extern unsigned int DAT_00b6a606;  /* unaligned */
extern unsigned int DAT_00b6a60a;  /* unaligned */
extern unsigned int DAT_00b6d96a;  /* unaligned */
extern unsigned int DAT_00b6d96e;  /* unaligned */
extern unsigned int DAT_00b6d972;  /* unaligned, 6 dwords */
extern unsigned int DAT_00b6d9aa;  /* unaligned */
extern unsigned int DAT_00b6d9ae;  /* unaligned */
extern unsigned int DAT_00b7e34a;  /* unaligned */
extern unsigned int DAT_00b7e34e;  /* unaligned */
extern unsigned int DAT_00b7e352;  /* unaligned, 6 dwords */

extern char s_jewelsound_0052ad74[];
extern char s_reg1u_0052b530[];
extern char s_reg1d_0052b538[];
extern char s_reg2u_0052b540[];
extern char s_reg2d_0052b548[];
extern char s_reg3u_0052b550[];
extern char s_reg3d_0052b558[];
extern char s_reg4u_0052b560[];
extern char s_reg4d_0052b568[];
extern char s_reg5u_0052b570[];
extern char s_reg5d_0052b578[];
extern char s_reg8u_0052b580[];
extern char s_reg8d_0052b588[];
extern char s_reg6d_0052b590[];
extern char s_reg6u_0052b598[];
extern char s_reg7d_0052b5a0[];
extern char s_reg7u_0052b5a8[];

void FUN_004daba0(int *param_1);
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
char *FUN_004ddc10(int param_1);
int FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4, int param_5,
                 unsigned int param_6, int param_7, char *param_8);
void FUN_004daa20(int *param_1, char *param_2, char *param_3, int param_4);
void FUN_004caf60(void);

unsigned int FUN_004c9660(void)
{
  char *pcVar1;
  unsigned char *puVar2;
  unsigned char *puVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  char *pcVar7;
  int local_30[4];
  int local_20[8];
  int tmpIdx;

  local_20[0] = 1;
  local_20[1] = 1;
  local_20[2] = 1;
  local_20[3] = 1;
  local_20[4] = 1;
  local_20[5] = 1;
  local_20[6] = 1;
  local_20[7] = 1;
  if (DAT_00b6a2b0 == 5) {
    local_20[7] = 2;
  }
  else {
    local_20[DAT_00b6a2b0] = 2;
  }
  __builtin_memcpy(&tmpIdx, &DAT_00b6d962, 4);
  local_20[tmpIdx + 5] = 2;
  FUN_004daba0(local_20);
  FUN_004d6460(local_30, 0x16, 0x14e, 0x2a, 0x166);
  __builtin_memcpy(&tmpIdx, &DAT_00b6d962, 4);
  if (tmpIdx == 1) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x283);
    FUN_004daa20(local_30, s_reg7d_0052b5a0, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0x11;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x283);
    iVar4 = FUN_004da780(local_30, s_reg7u_0052b5a8, s_reg7u_0052b5a8, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      tmpIdx = 1;
      __builtin_memcpy(&DAT_00b6d962, &tmpIdx, 4);
      __builtin_memcpy(&DAT_00b6a5a6, &DAT_00b7e34a, 4);
      __builtin_memcpy(&DAT_00b6a5aa, &DAT_00b7e34e, 4);
      puVar2 = (unsigned char *)&DAT_00b7e352;
      puVar3 = (unsigned char *)&DAT_00b6a5ae;
      for (iVar4 = 6; iVar4 != 0; iVar4 = iVar4 + -1) {
        __builtin_memcpy(puVar3, puVar2, 4);
        puVar2 = puVar2 + 4;
        puVar3 = puVar3 + 4;
      }
      __builtin_memcpy(&DAT_00b6a606, (unsigned char *)&DAT_00b6d9aa + (unsigned int)tmpIdx * 0x109e0u, 4);
      __builtin_memcpy(&DAT_00b6a60a, (unsigned char *)&DAT_00b6d9ae + (unsigned int)tmpIdx * 0x109e0u, 4);
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  __builtin_memcpy(&tmpIdx, &DAT_00b6d962, 4);
  if (tmpIdx == 0) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x282);
    FUN_004daa20(local_30, s_reg6d_0052b590, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0x10;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x282);
    iVar4 = FUN_004da780(local_30, s_reg6u_0052b598, s_reg6u_0052b598, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      tmpIdx = 0;
      __builtin_memcpy(&DAT_00b6d962, &tmpIdx, 4);
      __builtin_memcpy(&DAT_00b6a5a6, &DAT_00b6d96a, 4);
      __builtin_memcpy(&DAT_00b6a5aa, &DAT_00b6d96e, 4);
      puVar2 = (unsigned char *)&DAT_00b6d972;
      puVar3 = (unsigned char *)&DAT_00b6a5ae;
      for (iVar4 = 6; iVar4 != 0; iVar4 = iVar4 + -1) {
        __builtin_memcpy(puVar3, puVar2, 4);
        puVar2 = puVar2 + 4;
        puVar3 = puVar3 + 4;
      }
      __builtin_memcpy(&DAT_00b6a606, (unsigned char *)&DAT_00b6d9aa + (unsigned int)tmpIdx * 0x109e0u, 4);
      __builtin_memcpy(&DAT_00b6a60a, (unsigned char *)&DAT_00b6d9ae + (unsigned int)tmpIdx * 0x109e0u, 4);
      FUN_004caf60();
    }
  }
  FUN_004d6460(local_30, 0x16, 0xfe, 0x2a, 0x116);
  if (DAT_00b6a2b0 == 5) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x31c);
    FUN_004daa20(local_30, s_reg8d_0052b588, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0x12;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x31c);
    iVar4 = FUN_004da780(local_30, s_reg8u_0052b580, s_reg8u_0052b580, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      DAT_00b6a2b0 = 5;
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  if (DAT_00b6a2b0 == 4) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x33a);
    FUN_004daa20(local_30, s_reg5d_0052b578, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0xf;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(0x33a);
    iVar4 = FUN_004da780(local_30, s_reg5u_0052b570, s_reg5u_0052b570, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      DAT_00b6a2b0 = 4;
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  if (DAT_00b6a2b0 == 3) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(10);
    FUN_004daa20(local_30, s_reg4d_0052b568, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0xe;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(10);
    iVar4 = FUN_004da780(local_30, s_reg4u_0052b560, s_reg4u_0052b560, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      DAT_00b6a2b0 = 3;
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  if (DAT_00b6a2b0 == 2) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(8);
    FUN_004daa20(local_30, s_reg3d_0052b558, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0xd;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(8);
    iVar4 = FUN_004da780(local_30, s_reg3u_0052b550, s_reg3u_0052b550, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      DAT_00b6a2b0 = 2;
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  if (DAT_00b6a2b0 == 1) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(9);
    FUN_004daa20(local_30, s_reg2d_0052b548, pcVar1, iVar4);
  }
  else {
    pcVar7 = s_jewelsound_0052ad74;
    iVar6 = 0xc;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(9);
    iVar4 = FUN_004da780(local_30, s_reg2u_0052b540, s_reg2u_0052b540, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 != 0) {
      DAT_00664f2c = 1;
      DAT_00b6a2b0 = 1;
      FUN_004caf60();
    }
  }
  FUN_004d64a0(local_30, 0, -0x1c);
  if (DAT_00b6a2b0 == 0) {
    iVar4 = 0xc;
    pcVar1 = FUN_004ddc10(7);
    FUN_004daa20(local_30, s_reg1d_0052b538, pcVar1, iVar4);
    return 1;
  }
  pcVar7 = s_jewelsound_0052ad74;
  iVar6 = 0xb;
  uVar5 = 0;
  iVar4 = 0xc;
  pcVar1 = FUN_004ddc10(7);
  iVar4 = FUN_004da780(local_30, s_reg1u_0052b530, s_reg1u_0052b530, pcVar1, iVar4, uVar5, iVar6, pcVar7);
  if (iVar4 != 0) {
    DAT_00664f2c = 1;
    DAT_00b6a2b0 = 0;
    FUN_004caf60();
  }
  return 1;
}
