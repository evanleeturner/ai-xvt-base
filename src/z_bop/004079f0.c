/* SPDX-License-Identifier: MIT */
/* z_bop 004079f0 */
extern int DAT_0052f864;
extern unsigned int DAT_00a68750[0x400];
extern unsigned int DAT_00a697c8[];
extern unsigned char DAT_00a69750[];
extern unsigned char DAT_00a8f7c8[];
extern const char s_0_RAN_OUT_OF_MYCACHETEXTURES____0051a5d7[];
extern void __cdecl FUN_004acc30(const char *);

void * __cdecl FUN_004079f0(unsigned int param_1)
{
  int iVar1;
  unsigned int *puVar2;
  unsigned int uVar3;
  unsigned int *puVar4;

  if (DAT_0052f864 == -1) {
    puVar2 = (unsigned int *)&DAT_00a697c8;
    puVar4 = (unsigned int *)&DAT_00a68750;
    for (iVar1 = 0x400; iVar1 != 0; iVar1 = iVar1 - 1) {
      *puVar4 = 0;
      puVar4 = puVar4 + 1;
    }
    do {
      *puVar2 = 0;
      puVar2 = puVar2 + 0x26;
    } while (puVar2 < (unsigned int *)&DAT_00a8f7c8);
  }
  iVar1 = 0;
  DAT_0052f864 = param_1 & 0x3ff;
  uVar3 = DAT_0052f864;
  do {
    if (DAT_00a68750[uVar3] == param_1) break;
    uVar3 = uVar3 + 1;
    if (uVar3 == 0x400) {
      uVar3 = 0;
    }
    iVar1 = iVar1 + 1;
  } while (iVar1 < 0x400);
  if (iVar1 == 0x400) {
    iVar1 = 0;
    do {
      if (DAT_00a697c8[DAT_0052f864 * 0x26] == 0) break;
      DAT_0052f864 = DAT_0052f864 + 1;
      if (DAT_0052f864 == 0x400) {
        DAT_0052f864 = 0;
      }
      iVar1 = iVar1 + 1;
    } while (iVar1 < 0x400);
    uVar3 = DAT_0052f864;
    if (iVar1 == 0x400) {
      FUN_004acc30(s_0_RAN_OUT_OF_MYCACHETEXTURES____0051a5d7 + 1);
      return (void *)&DAT_00a69750 + DAT_0052f864 * 0x98;
    }
  }
  DAT_0052f864 = uVar3;
  DAT_00a68750[uVar3] = param_1;
  return (void *)&DAT_00a69750 + uVar3 * 0x98;
}
