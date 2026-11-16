/* SPDX-License-Identifier: MIT */
/* z_bop 004d1940 */
extern unsigned char DAT_00acebc5;
extern unsigned char DAT_00acebd1;
extern unsigned char DAT_00acebd5;
extern unsigned char DAT_00acebd9;
extern unsigned char DAT_00acebdd;
extern unsigned char DAT_00acebe1;
extern unsigned char DAT_00acebe5;
extern unsigned char DAT_00acebe9;
extern unsigned char DAT_00acebed;
extern unsigned char DAT_00acebf1;
extern unsigned char DAT_00acebf5;
extern unsigned char DAT_00acf125;
extern unsigned char DAT_00b53579;
extern unsigned char DAT_00b53575;
extern unsigned char DAT_00b53571;
extern unsigned char DAT_00acf571;
extern unsigned char DAT_00b69a89;
extern unsigned char DAT_00b63f91;
extern unsigned char DAT_00acf14d;
extern unsigned char DAT_00acf151;
extern unsigned char DAT_00acf351;
extern unsigned char DAT_00acf355;
extern unsigned char DAT_00acf359;
extern unsigned char DAT_00acf35d;
extern unsigned char DAT_00acf55d;
extern unsigned char DAT_00acf561;
extern unsigned char DAT_00b5357d;
extern unsigned char DAT_00b63d7d;

undefined4 __cdecl
FUN_004d1940(undefined4 *param_1, undefined4 *param_2, undefined4 *param_3, undefined4 *param_4,
             undefined4 *param_5, undefined4 *param_6, int param_7, int *param_8, int *param_9,
             undefined4 *param_10, undefined4 *param_11, int *param_12, undefined4 *param_13,
             undefined4 *param_14, undefined4 *param_15, undefined4 *param_16, undefined4 *param_17,
             undefined4 *param_18, undefined4 *param_19, undefined4 *param_20, undefined4 *param_21,
             undefined4 *param_22)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  unsigned char *puVar4;
  undefined4 *puVar6;
  unsigned char *puVar7;
  unsigned char *pbVar8;
  undefined4 *puVar9;

  __builtin_memcpy(param_1, &DAT_00acebc5, 4);
  __builtin_memcpy(&param_2[0], &DAT_00acebd1, 4);
  __builtin_memcpy(&param_2[1], &DAT_00acebd5, 4);
  __builtin_memcpy(&param_2[2], &DAT_00acebd9, 4);
  __builtin_memcpy(&param_2[3], &DAT_00acebdd, 4);
  __builtin_memcpy(&param_3[0], &DAT_00acebe1, 4);
  __builtin_memcpy(&param_3[1], &DAT_00acebe5, 4);
  __builtin_memcpy(&param_3[2], &DAT_00acebe9, 4);
  __builtin_memcpy(&param_3[3], &DAT_00acebed, 4);
  __builtin_memcpy(param_4, &DAT_00acebf5, 4);
  __builtin_memcpy(param_5, &DAT_00acebf1, 4);
  puVar4 = &DAT_00acf125;
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    __builtin_memcpy(param_6, puVar4, 4);
    puVar4 = puVar4 + 4;
    param_6 = param_6 + 1;
  }
  __builtin_memcpy(param_9, &DAT_00b53579, 4);
  __builtin_memcpy(param_8, &DAT_00b53575, 4);
  __builtin_memcpy(param_10, &DAT_00b53571, 4);
  iVar3 = 0;
  __builtin_memcpy(&iVar1, &DAT_00b53575, 4);
  __builtin_memcpy(&iVar4, &DAT_00b53579, 4);
  if (0 < iVar4) {
    do {
      iVar5 = (int)((unsigned int)iVar1 + 1u);
      puVar4 = &DAT_00acf571 + (unsigned int)iVar1 * 0x210u;
      puVar6 = (undefined4 *)((unsigned int)iVar1 * 0x210u + (unsigned int)param_7);
      for (iVar2 = 0x84; iVar2 != 0; iVar2 = iVar2 + -1) {
        __builtin_memcpy(puVar6, puVar4, 4);
        puVar4 = puVar4 + 4;
        puVar6 = puVar6 + 1;
      }
      if (0x3ff < iVar5) {
        iVar5 = 0;
      }
      iVar3 = iVar3 + 1;
      iVar1 = iVar5;
      __builtin_memcpy(&iVar4, &DAT_00b53579, 4);
    } while (iVar3 < iVar4);
  }
  iVar1 = 0;
  __builtin_memcpy(param_12, &DAT_00b69a89, 4);
  __builtin_memcpy(&iVar4, &DAT_00b69a89, 4);
  if (0 < iVar4) {
    puVar4 = &DAT_00b63f91;
    do {
      pbVar8 = puVar4;
      puVar9 = param_11;
      for (iVar3 = 0x8e; iVar3 != 0; iVar3 = iVar3 + -1) {
        __builtin_memcpy(puVar9, pbVar8, 4);
        pbVar8 = pbVar8 + 4;
        puVar9 = puVar9 + 1;
      }
      param_11 = param_11 + 0x8e;
      iVar1 = iVar1 + 1;
      puVar4 = puVar4 + 0x238;
      __builtin_memcpy(&iVar4, &DAT_00b69a89, 4);
    } while (iVar1 < iVar4);
  }
  __builtin_memcpy(param_13, &DAT_00acf14d, 4);
  puVar4 = &DAT_00acf151;
  for (iVar1 = 0x80; iVar1 != 0; iVar1 = iVar1 + -1) {
    __builtin_memcpy(param_14, puVar4, 4);
    puVar4 = puVar4 + 4;
    param_14 = param_14 + 1;
  }
  __builtin_memcpy(param_15, &DAT_00acf351, 4);
  __builtin_memcpy(param_16, &DAT_00acf355, 4);
  __builtin_memcpy(param_17, &DAT_00acf359, 4);
  puVar4 = &DAT_00acf35d;
  for (iVar1 = 0x80; iVar1 != 0; iVar1 = iVar1 + -1) {
    __builtin_memcpy(param_18, puVar4, 4);
    puVar4 = puVar4 + 4;
    param_18 = param_18 + 1;
  }
  __builtin_memcpy(param_19, &DAT_00acf55d, 4);
  __builtin_memcpy(param_20, &DAT_00acf561, 4);
  puVar6 = param_21;
  for (iVar1 = 0x4200; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  puVar4 = &DAT_00b5357d;
  do {
    puVar7 = puVar4 + 0x210;
    puVar6 = param_21;
    for (iVar1 = 0x84; iVar1 != 0; iVar1 = iVar1 + -1) {
      __builtin_memcpy(puVar6, puVar4, 4);
      puVar4 = puVar4 + 4;
      puVar6 = puVar6 + 1;
    }
    param_21 = param_21 + 0x84;
    puVar4 = puVar7;
  } while (puVar7 < &DAT_00b63d7d);
  __builtin_memcpy(param_22, &DAT_00b63d7d, 4);
  return 1;
}
