/* SPDX-License-Identifier: MIT */
/* z_bop 004d8690 */
extern int DAT_0066543c;
extern int DAT_00b6a2a8;
extern int DAT_0052bf54;
extern char DAT_0052bfdc;
extern char DAT_0052bfd4;
extern unsigned int DAT_00b69ccc;
extern int DAT_00b69cd0;
extern char DAT_00bb73a7;
extern char s_jewelsound_0052ad74[];
extern char DAT_00aa6a80[];
extern unsigned int DAT_00aa6af0;
extern char DAT_00aa6af4[];
extern char DAT_00b69d20[];
extern char DAT_00b6a2e0[];
extern char DAT_0051bf3c[];
extern int DAT_00aa5df0[8];
extern int DAT_00aa5cb0[];
extern char DAT_00b6a5a6[4]; /* packed int at unaligned address */
extern int DAT_00bb2810;

void __cdecl FUN_004dcb30(void);
void __cdecl FUN_004de090(int *a, int *b);
void __cdecl FUN_004b5570(void *a, int b, int c);
void __cdecl FUN_004d6460(int *r, int a, int b, int c, int d);
char *__cdecl FUN_004ddc10(int a);
void __cdecl FUN_004db640(int a, char *b, int *c, unsigned int d);
int __cdecl FUN_004d6b90(int *r, int a, int b);
unsigned char __cdecl FUN_004dc220(void);
unsigned char __cdecl FUN_004dc240(void);
void __cdecl FUN_004de5a0(char *a, int b, int c, int d);
int __cdecl FUN_004da380(int *r, char *b, int c, int d, int e, char *f);
int __cdecl FUN_004d11e0(void);
int __cdecl FUN_004d12b0(int a);
void __cdecl FUN_004cef70(int a, unsigned int *b, int c);
int __cdecl FUN_004dbbd0(int a, int b, int *c, int d, int e, int f);
int __cdecl FUN_004d9d70(int *r, int a, int b, int c, int d, int e, int f);

void __cdecl FUN_004d8690(int param_1)
{
  char cVar1;
  unsigned char uVar2;
  char *pcVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  unsigned int *puVar7;
  char *pcVar8;
  char *pcVar9;
  unsigned int *puVar10;
  int *piVar11;
  unsigned int uVar12;
  int iVar13;
  int local_18[4];
  int local_8;
  int local_4;

  if (param_1 == 0) {
    FUN_004dcb30();
    DAT_0066543c = 0;
  }
  if (DAT_00b6a2a8 == 0) {
    DAT_0052bf54 = 0;
  }
  else {
    FUN_004de090(&local_4, &local_8);
    if (DAT_0052bf54 == 0) {
      FUN_004b5570(&DAT_0052bfdc, 0, 0);
      FUN_004d6460(local_18, 0x22c, 0x5f, 0x24c, 0x69);
      piVar11 = local_18;
      uVar12 = DAT_00b69ccc;
      pcVar3 = FUN_004ddc10(0xd9);
      FUN_004db640(10, pcVar3, piVar11, uVar12);
      iVar4 = FUN_004d6b90(local_18, local_4, local_8);
      if ((iVar4 != 0) &&
          ((uVar2 = FUN_004dc220(), uVar2 != 0) ||
           (uVar2 = FUN_004dc240(), uVar2 != 0))) {
        if (DAT_00bb73a7 != '\0') {
          FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff);
        }
        DAT_0052bf54 = 1;
      }
      FUN_004b5570(&DAT_0052bfd4, 0, 0);
      FUN_004d6460(local_18, 0x208, 0x5f, 0x228, 0x69);
      piVar11 = local_18;
      uVar12 = 0xffff;
      pcVar3 = FUN_004ddc10(0x192);
      FUN_004db640(10, pcVar3, piVar11, uVar12);
    }
    else {
      FUN_004b5570(&DAT_0052bfd4, 0, 0);
      FUN_004d6460(local_18, 0x208, 0x5f, 0x228, 0x69);
      piVar11 = local_18;
      uVar12 = DAT_00b69ccc;
      pcVar3 = FUN_004ddc10(0x192);
      FUN_004db640(10, pcVar3, piVar11, uVar12);
      iVar4 = FUN_004d6b90(local_18, local_4, local_8);
      if ((iVar4 != 0) &&
          ((uVar2 = FUN_004dc220(), uVar2 != 0) ||
           (uVar2 = FUN_004dc240(), uVar2 != 0))) {
        if (DAT_00bb73a7 != '\0') {
          FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff);
        }
        DAT_0052bf54 = 0;
      }
      FUN_004b5570(&DAT_0052bfdc, 0, 0);
      FUN_004d6460(local_18, 0x22c, 0x5f, 0x24c, 0x69);
      piVar11 = local_18;
      uVar12 = 0xffff;
      pcVar3 = FUN_004ddc10(0xd9);
      FUN_004db640(10, pcVar3, piVar11, uVar12);
    }
  }
  FUN_004d6460(local_18, 0x1cd, 0x196, 0x253, 0x1a8);
  iVar4 = FUN_004da380(local_18, DAT_00aa6a80, 100, 0, 0xc, (char *)0);
  if ((iVar4 == 0) || (DAT_00aa6a80[0] == '\0')) goto LAB_004d8b1d;
  DAT_00aa6af0 = 0x4d;
  iVar4 = FUN_004d11e0();
  iVar5 = FUN_004d12b0(iVar4);
  iVar4 = DAT_0052bf54;
  if (iVar5 == 0) {
    DAT_00b69d20[0] = 4;
  }
  else if (DAT_0052bf54 == 1) {
    DAT_00b69d20[0] = 2;
  }
  else {
    DAT_00b69d20[0] = 3;
  }
  uVar12 = 0xffffffff;
  DAT_00b69d20[1] = 0;
  pcVar3 = DAT_00b6a2e0;
  do {
    pcVar9 = pcVar3;
    if (uVar12 == 0) break;
    uVar12 = uVar12 - 1;
    pcVar9 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar9;
  } while (cVar1 != '\0');
  uVar12 = ~uVar12;
  iVar5 = -1;
  pcVar3 = DAT_00b69d20;
  do {
    pcVar8 = pcVar3;
    if (iVar5 == 0) break;
    iVar5 = (int)((unsigned int)iVar5 - 1u);
    pcVar8 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar8;
  } while (cVar1 != '\0');
  pcVar3 = pcVar9 - uVar12;
  pcVar9 = pcVar8 - 1;
  for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *(unsigned int *)pcVar9 = *(unsigned int *)pcVar3;
    pcVar3 = pcVar3 + 4;
    pcVar9 = pcVar9 + 4;
  }
  for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
    *pcVar9 = *pcVar3;
    pcVar3 = pcVar3 + 1;
    pcVar9 = pcVar9 + 1;
  }
  uVar12 = 0xffffffff;
  pcVar3 = DAT_0051bf3c;
  do {
    pcVar9 = pcVar3;
    if (uVar12 == 0) break;
    uVar12 = uVar12 - 1;
    pcVar9 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar9;
  } while (cVar1 != '\0');
  uVar12 = ~uVar12;
  iVar5 = -1;
  pcVar3 = DAT_00b69d20;
  do {
    pcVar8 = pcVar3;
    if (iVar5 == 0) break;
    iVar5 = (int)((unsigned int)iVar5 - 1u);
    pcVar8 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar8;
  } while (cVar1 != '\0');
  pcVar3 = pcVar9 - uVar12;
  pcVar9 = pcVar8 - 1;
  for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *(unsigned int *)pcVar9 = *(unsigned int *)pcVar3;
    pcVar3 = pcVar3 + 4;
    pcVar9 = pcVar9 + 4;
  }
  for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
    *pcVar9 = *pcVar3;
    pcVar3 = pcVar3 + 1;
    pcVar9 = pcVar9 + 1;
  }
  uVar12 = 0xffffffff;
  pcVar3 = DAT_00b69d20;
  do {
    if (uVar12 == 0) break;
    uVar12 = uVar12 - 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  DAT_00b69d20[~uVar12 - 1u] = 1;
  DAT_00b69d20[~uVar12] = 0;
  uVar12 = 0xffffffff;
  pcVar3 = DAT_00aa6a80;
  do {
    pcVar9 = pcVar3;
    if (uVar12 == 0) break;
    uVar12 = uVar12 - 1;
    pcVar9 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar9;
  } while (cVar1 != '\0');
  uVar12 = ~uVar12;
  iVar5 = -1;
  pcVar3 = DAT_00b69d20;
  do {
    pcVar8 = pcVar3;
    if (iVar5 == 0) break;
    iVar5 = (int)((unsigned int)iVar5 - 1u);
    pcVar8 = pcVar3 + 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar8;
  } while (cVar1 != '\0');
  pcVar3 = pcVar9 - uVar12;
  pcVar9 = pcVar8 - 1;
  for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *(unsigned int *)pcVar9 = *(unsigned int *)pcVar3;
    pcVar3 = pcVar3 + 4;
    pcVar9 = pcVar9 + 4;
  }
  for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
    *pcVar9 = *pcVar3;
    pcVar3 = pcVar3 + 1;
    pcVar9 = pcVar9 + 1;
  }
  uVar12 = 0xffffffff;
  pcVar3 = DAT_00b69d20;
  do {
    if (uVar12 == 0) break;
    uVar12 = uVar12 - 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  puVar7 = (unsigned int *)DAT_00b69d20;
  puVar10 = (unsigned int *)DAT_00aa6af4;
  for (uVar6 = (~uVar12 - 1u) >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *puVar10 = *puVar7;
    puVar7 = puVar7 + 1;
    puVar10 = puVar10 + 1;
  }
  for (uVar12 = (~uVar12 - 1u) & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
    *(char *)puVar10 = *(char *)puVar7;
    puVar7 = (unsigned int *)((int)puVar7 + 1);
    puVar10 = (unsigned int *)((int)puVar10 + 1);
  }
  if (iVar4 == 0) {
    uVar12 = 0xffffffff;
    pcVar3 = DAT_00b69d20;
    do {
      if (uVar12 == 0) break;
      uVar12 = uVar12 - 1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
LAB_004d8b07:
    FUN_004cef70(0, &DAT_00aa6af0, ~uVar12 + 3u);
  }
  else {
    iVar4 = 0;
    piVar11 = DAT_00aa5df0;
    do {
      iVar5 = FUN_004d11e0();
      if (iVar5 == *piVar11) break;
      piVar11 = piVar11 + 1;
      iVar4 = (int)((unsigned int)iVar4 + 1u);
    } while (piVar11 < DAT_00aa5df0 + 8);
    if (iVar4 == 8) {
      uVar12 = 0xffffffff;
      pcVar3 = DAT_00b69d20;
      do {
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
      } while (cVar1 != '\0');
      goto LAB_004d8b07;
    }
    iVar4 = 0;
    do {
      __builtin_memcpy(&iVar13, DAT_00b6a5a6, 4);
      if (DAT_00aa5cb0[(int)((unsigned int)iVar4 + (unsigned int)iVar13 * 8u)] != 0) {
        uVar12 = 0xffffffff;
        pcVar3 = DAT_00b69d20;
        do {
          if (uVar12 == 0) break;
          uVar12 = uVar12 - 1;
          cVar1 = *pcVar3;
          pcVar3 = pcVar3 + 1;
        } while (cVar1 != '\0');
        __builtin_memcpy(&iVar13, DAT_00b6a5a6, 4);
        FUN_004cef70(DAT_00aa5cb0[(int)((unsigned int)iVar4 + (unsigned int)iVar13 * 8u)],
                     &DAT_00aa6af0, ~uVar12 + 3u);
      }
      iVar4 = (int)((unsigned int)iVar4 + 1u);
    } while (iVar4 < 8);
  }
  puVar7 = (unsigned int *)DAT_00aa6a80;
  for (iVar4 = 0x19; iVar4 != 0; iVar4 = (int)((unsigned int)iVar4 - 1u)) {
    *puVar7 = 0;
    puVar7 = puVar7 + 1;
  }
LAB_004d8b1d:
  FUN_004d6460(local_18, 0x1cd, 0x75, 0x253, 0x197);
  iVar5 = FUN_004dbbd0(0xc, DAT_00b69cd0, local_18, 0xffff, 2, 0x400);
  iVar4 = (int)((unsigned int)iVar5 - 0x14u);
  if (iVar5 < 0x15) {
    iVar4 = 0;
  }
  if (0 < iVar4) {
    iVar4 = (int)((unsigned int)iVar4 + 1u);
    if (DAT_0066543c == iVar4) {
      DAT_0066543c = (int)((unsigned int)DAT_0066543c - 1u);
    }
    FUN_004d6460(local_18, 0x254, 0x75, 0x25d, 0x197);
    iVar5 = FUN_004d9d70(local_18,
                         (int)((unsigned int)iVar4 - (unsigned int)DAT_0066543c - 1u),
                         iVar4, 0, 5, DAT_00bb2810, 7);
    DAT_0066543c = (int)((unsigned int)iVar4 - (unsigned int)iVar5 - 1u);
  }
  FUN_004d6460(local_18, 0x1cd, 0x75, 0x253, 0x18e);
  FUN_004dbbd0(0xc, DAT_00b69cd0, local_18, 0xffff, 2,
               (int)((unsigned int)iVar4 - (unsigned int)DAT_0066543c - 1u));
  return;
}
