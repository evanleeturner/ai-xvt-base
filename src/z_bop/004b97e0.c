/* SPDX-License-Identifier: MIT */
/* z_bop 004b97e0 */
typedef struct _iobuf FILE;

extern unsigned char DAT_00bb72b0[], DAT_00bb72b2[], DAT_00bb72b4[], DAT_00bb72b6[];
extern unsigned char DAT_00bb72b8[], DAT_00bb72ba[], DAT_00bb72bc[], DAT_00bb72be[];
extern unsigned char DAT_00bb72c0[], DAT_00bb72c2[], DAT_00bb72c4[], DAT_00bb72c6[];
extern unsigned char DAT_00bb72c8[], DAT_00bb72ca[], DAT_00bb72cc[], DAT_00bb72ce[];
extern unsigned char DAT_00bb72b1, DAT_00bb72b3, DAT_00bb72b5, DAT_00bb72b7;
extern unsigned char DAT_00bb72b9, DAT_00bb72bb, DAT_00bb72bd, DAT_00bb72bf;
extern unsigned char DAT_00bb72c1, DAT_00bb72c3, DAT_00bb72c5, DAT_00bb72c7;
extern unsigned char DAT_00bb72c9, DAT_00bb72cb, DAT_00bb72cd, DAT_00bb72cf;
extern unsigned char DAT_00bb72d0;
extern unsigned char DAT_00bb72d1[], DAT_00bb7311[];
extern unsigned char DAT_00bb7351[], DAT_00bb7355[], DAT_00bb7359[], DAT_00bb735d;
extern unsigned char DAT_00bb735f[], DAT_00bb7363[], DAT_00bb7367[], DAT_00bb736b[];
extern unsigned char DAT_00bb736f, DAT_00bb7370, DAT_00bb7371, DAT_00bb7372, DAT_00bb7373;
extern unsigned char DAT_00bb7374, DAT_00bb7375, DAT_00bb7376, DAT_00bb7377, DAT_00bb7378;
extern unsigned char DAT_00bb7379, DAT_00bb737a, DAT_00bb737b, DAT_00bb737c, DAT_00bb737d;
extern unsigned char DAT_00bb737e, DAT_00bb737f;
extern unsigned char DAT_00bb7380[];
extern unsigned char DAT_00bb7394, DAT_00bb7395, DAT_00bb7396, DAT_00bb7397, DAT_00bb7398;
extern unsigned char DAT_00bb7399, DAT_00bb739a, DAT_00bb739b, DAT_00bb739c, DAT_00bb739d;
extern unsigned char DAT_00bb739e, DAT_00bb739f;
extern int DAT_00bb73a0;
extern unsigned char DAT_00bb73a4, DAT_00bb73a5, DAT_00bb73a6, DAT_00bb73a7, DAT_00bb73a8;
extern unsigned char DAT_00bb73a9[], DAT_00bb73ef[], DAT_00bb7435[], DAT_00bb747b[];
extern unsigned char DAT_00bb5897[];
extern char DAT_00b69d20[];
extern char DAT_0051c4bc;
extern char s_config2_cfg_0052a74c[];
extern char s_config_cfg_0052a740[];
extern char *PTR_s_lastpilot_0052a4c8[];

unsigned char FUN_004d5e60(void);
int FUN_004d6300(int param_1);
char *FUN_004ddc10(int param_1);
FILE *FUN_004cc4a0(char *param_1, char *param_2);
void FUN_004ccf50(void);
void FUN_004ccf70(void);
void FUN_004cc590(FILE *param_1);
char *_fgets(char *param_1, int param_2, FILE *param_3);
int _atoi(const char *param_1);

void FUN_004b97e0(void)
{
  char **ppuVar1;
  char cVar2;
  unsigned char bVar3;
  unsigned char uVar4;
  int extraout_var;
  char *pcVar5;
  char **ppuVar6;
  int iVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned char *pbVar10;
  int iVar11;
  unsigned char *pbVar12;
  int *piVar13;
  int iVar14;
  char *pcVar15;
  unsigned int *puVar16;
  unsigned char *pbDst;
  _Bool bVar17;
  _Bool bVar18;
  int iVar6;
  FILE *local_58;
  int local_50[20];

  piVar13 = local_50;
  for (iVar7 = 0x14; iVar7 != 0; iVar7 = iVar7 + -1) {
    *piVar13 = 0;
    piVar13 = piVar13 + 1;
  }
  puVar16 = (unsigned int *)DAT_00bb72b0;
  for (iVar7 = 0x84; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar16 = 0;
    puVar16 = puVar16 + 1;
  }
  *(unsigned char *)puVar16 = 0;
  iVar7 = 0;
  do {
    DAT_00bb72b0[iVar7] = 1;
    iVar14 = iVar7 + 1;
    DAT_00bb72b2[iVar7] = 2;
    DAT_00bb72b4[iVar7] = 1;
    DAT_00bb72b6[iVar7] = 1;
    DAT_00bb72b8[iVar7] = 1;
    DAT_00bb72ba[iVar7] = 1;
    DAT_00bb72bc[iVar7] = 1;
    DAT_00bb72be[iVar7] = 2;
    DAT_00bb72c0[iVar7] = 10;
    DAT_00bb72c2[iVar7] = 10;
    DAT_00bb72c4[iVar7] = 2;
    DAT_00bb72c6[iVar7] = 2;
    DAT_00bb72c8[iVar7] = 0;
    DAT_00bb72ca[iVar7] = 2;
    uVar4 = FUN_004d5e60();
    DAT_00bb72cc[iVar7] = uVar4;
    DAT_00bb72ce[iVar7] = 1;
    iVar7 = iVar14;
  } while (iVar14 < 2);
  iVar7 = 0;
  DAT_00bb72d0 = 0;
  DAT_00bb7371 = 1;
  DAT_00bb7372 = 1;
  DAT_00bb7373 = 1;
  DAT_00bb73a7 = 1;
  DAT_00bb7374 = 2;
  DAT_00bb7375 = 2;
  DAT_00bb7376 = 1;
  DAT_00bb7377 = 1;
  DAT_00bb7378 = 1;
  DAT_00bb737f = 1;
  DAT_00bb7379 = 9;
  DAT_00bb737a = 9;
  DAT_00bb737b = 9;
  DAT_00bb737c = 9;
  DAT_00bb737d = 9;
  DAT_00bb7394 = 1;
  DAT_00bb7395 = 1;
  DAT_00bb7396 = 1;
  DAT_00bb7397 = 0;
  DAT_00bb7398 = 1;
  DAT_00bb7399 = 0;
  DAT_00bb739a = 0;
  DAT_00bb739b = 1;
  DAT_00bb739c = 1;
  DAT_00bb739d = 1;
  DAT_00bb739f = 1;
  DAT_00bb73a0 = 0;
  DAT_00bb736f = 0;
  DAT_00bb73a4 = 0;
  DAT_00bb73a8 = 1;
  DAT_00bb73a5 = 0;
  DAT_00bb73a6 = 1;
  DAT_00bb737e = 5;
  DAT_00bb739e = 0xff;
  DAT_00bb7370 = 8;
  /* FUN_004d6300 result: EAX, signed; Ghidra splits it into uVar4 (low byte)
     and extraout_var (upper bytes). iVar6 is the full value. */
  iVar6 = FUN_004d6300(0);
  uVar4 = (unsigned char)iVar6;
  extraout_var = iVar6 >> 8;
  if (iVar6 != 0 && -1 < extraout_var) {
    do {
      if (0xf < iVar7) break;
      switch (iVar7) {
      case 0: DAT_00bb7380[iVar7] = 0x9c; break;
      case 1: DAT_00bb7380[iVar7] = 0x9d; break;
      case 2: DAT_00bb7380[iVar7] = 0x72; break;
      case 3: DAT_00bb7380[iVar7] = 0x2e; break;
      case 4: DAT_00bb7380[iVar7] = 0x65; break;
      case 5: DAT_00bb7380[iVar7] = 0x69; break;
      case 6: DAT_00bb7380[iVar7] = 0x5b; break;
      case 7: DAT_00bb7380[iVar7] = 8; break;
      case 8: DAT_00bb7380[iVar7] = 0xd; break;
      case 9: DAT_00bb7380[iVar7] = 0x5d;
      }
      iVar7 = iVar7 + 1;
    } while (iVar7 < iVar6);
  }
  pcVar5 = FUN_004ddc10(0x316);
  uVar8 = 0xffffffff;
  do {
    pcVar15 = pcVar5;
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    pcVar15 = pcVar5 + 1;
    cVar2 = *pcVar5;
    pcVar5 = pcVar15;
  } while (cVar2 != '\0');
  uVar8 = ~uVar8;
  pcVar5 = pcVar15 + -uVar8;
  pcVar15 = (char *)DAT_00bb73a9;
  for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
    __builtin_memcpy(pcVar15, pcVar5, 4);
    pcVar5 = pcVar5 + 4;
    pcVar15 = pcVar15 + 4;
  }
  for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
    *pcVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar15 = pcVar15 + 1;
  }
  pcVar5 = FUN_004ddc10(0x317);
  uVar8 = 0xffffffff;
  do {
    pcVar15 = pcVar5;
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    pcVar15 = pcVar5 + 1;
    cVar2 = *pcVar5;
    pcVar5 = pcVar15;
  } while (cVar2 != '\0');
  uVar8 = ~uVar8;
  pcVar5 = pcVar15 + -uVar8;
  pcVar15 = (char *)DAT_00bb73ef;
  for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
    __builtin_memcpy(pcVar15, pcVar5, 4);
    pcVar5 = pcVar5 + 4;
    pcVar15 = pcVar15 + 4;
  }
  for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
    *pcVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar15 = pcVar15 + 1;
  }
  pcVar5 = FUN_004ddc10(0x318);
  uVar8 = 0xffffffff;
  do {
    pcVar15 = pcVar5;
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    pcVar15 = pcVar5 + 1;
    cVar2 = *pcVar5;
    pcVar5 = pcVar15;
  } while (cVar2 != '\0');
  uVar8 = ~uVar8;
  pcVar5 = pcVar15 + -uVar8;
  pcVar15 = (char *)DAT_00bb7435;
  for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
    __builtin_memcpy(pcVar15, pcVar5, 4);
    pcVar5 = pcVar5 + 4;
    pcVar15 = pcVar15 + 4;
  }
  for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
    *pcVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar15 = pcVar15 + 1;
  }
  pcVar5 = FUN_004ddc10(0x319);
  uVar8 = 0xffffffff;
  do {
    pcVar15 = pcVar5;
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    pcVar15 = pcVar5 + 1;
    cVar2 = *pcVar5;
    pcVar5 = pcVar15;
  } while (cVar2 != '\0');
  uVar8 = ~uVar8;
  pcVar5 = pcVar15 + -uVar8;
  pcVar15 = (char *)DAT_00bb747b;
  for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
    __builtin_memcpy(pcVar15, pcVar5, 4);
    pcVar5 = pcVar5 + 4;
    pcVar15 = pcVar15 + 4;
  }
  for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
    *pcVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar15 = pcVar15 + 1;
  }
  local_58 = FUN_004cc4a0(s_config2_cfg_0052a74c, &DAT_0051c4bc);
  bVar18 = local_58 == (FILE *)0;
  if (bVar18) {
    FUN_004ccf50();
    local_58 = FUN_004cc4a0(s_config_cfg_0052a740, &DAT_0051c4bc);
    FUN_004ccf70();
  }
  if (local_58 != (FILE *)0) {
    while (1) {
      pcVar5 = _fgets(DAT_00b69d20, 0x100, local_58);
      if (pcVar5 == (char *)0) break;
      uVar8 = 0xffffffff;
      pcVar5 = DAT_00b69d20;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar2 = *pcVar5;
        pcVar5 = pcVar5 + 1;
      } while (cVar2 != '\0');
      if (DAT_00b69d20[~uVar8 - 2] == '\n') {
        uVar8 = 0xffffffff;
        pcVar5 = DAT_00b69d20;
        do {
          if (uVar8 == 0) break;
          uVar8 = uVar8 - 1;
          cVar2 = *pcVar5;
          pcVar5 = pcVar5 + 1;
        } while (cVar2 != '\0');
        DAT_00b69d20[~uVar8 - 2] = 0;
      }
      pcVar5 = (char *)0;
      iVar7 = 0;
      uVar8 = 0xffffffff;
      pcVar15 = DAT_00b69d20;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar2 = *pcVar15;
        pcVar15 = pcVar15 + 1;
      } while (cVar2 != '\0');
      if (~uVar8 != 1 && -1 < (int)(~uVar8 - 1)) {
        do {
          if (DAT_00b69d20[iVar7] == ' ') {
            DAT_00b69d20[iVar7] = 0;
            pcVar5 = DAT_00b69d20 + iVar7 + 1;
            break;
          }
          iVar7 = iVar7 + 1;
          uVar8 = 0xffffffff;
          pcVar15 = DAT_00b69d20;
          do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar2 = *pcVar15;
            pcVar15 = pcVar15 + 1;
          } while (cVar2 != '\0');
        } while (iVar7 < (int)(~uVar8 - 1));
      }
      if (pcVar5 != (char *)0) {
        iVar14 = 0;
        iVar7 = -1;
        if (*PTR_s_lastpilot_0052a4c8[0] != '\0') {
          ppuVar6 = PTR_s_lastpilot_0052a4c8;
          do {
            pbVar10 = (unsigned char *)*ppuVar6;
            pbVar12 = (unsigned char *)DAT_00b69d20;
            do {
              bVar3 = *pbVar10;
              bVar17 = bVar3 < *pbVar12;
              if (bVar3 != *pbVar12) {
LAB_004b9bdb:
                iVar11 = (1 - (unsigned int)bVar17) - (unsigned int)(bVar17 != 0);
                goto LAB_004b9be0;
              }
              if (bVar3 == 0) break;
              bVar3 = pbVar10[1];
              bVar17 = bVar3 < pbVar12[1];
              if (bVar3 != pbVar12[1]) goto LAB_004b9bdb;
              pbVar10 = pbVar10 + 2;
              pbVar12 = pbVar12 + 2;
            } while (bVar3 != 0);
            iVar11 = 0;
LAB_004b9be0:
            iVar7 = iVar14;
            if (iVar11 == 0) break;
            ppuVar1 = ppuVar6 + 1;
            ppuVar6 = ppuVar6 + 1;
            iVar14 = iVar14 + 1;
            iVar7 = -1;
          } while (**ppuVar1 != '\0');
        }
        switch (iVar7) {
        case 0:
          __builtin_memcpy(DAT_00bb7351, pcVar5, 4);
          __builtin_memcpy(DAT_00bb7355, pcVar5 + 4, 4);
          __builtin_memcpy(DAT_00bb7359, pcVar5 + 8, 4);
          DAT_00bb735d = pcVar5[0xc];
          break;
        case 1: iVar7 = _atoi(pcVar5); DAT_00bb72b0[0] = (unsigned char)iVar7; break;
        case 2: iVar7 = _atoi(pcVar5); DAT_00bb72b2[0] = (unsigned char)iVar7; break;
        case 3: iVar7 = _atoi(pcVar5); DAT_00bb72b4[0] = (unsigned char)iVar7; break;
        case 4: iVar7 = _atoi(pcVar5); DAT_00bb72b6[0] = (unsigned char)iVar7; break;
        case 5: iVar7 = _atoi(pcVar5); DAT_00bb72b8[0] = (unsigned char)iVar7; break;
        case 6: iVar7 = _atoi(pcVar5); DAT_00bb72ba[0] = (unsigned char)iVar7; break;
        case 7: iVar7 = _atoi(pcVar5); DAT_00bb72bc[0] = (unsigned char)iVar7; break;
        case 8: iVar7 = _atoi(pcVar5); DAT_00bb72be[0] = (unsigned char)iVar7; break;
        case 9: iVar7 = _atoi(pcVar5); DAT_00bb72c0[0] = (unsigned char)iVar7; break;
        case 10: iVar7 = _atoi(pcVar5); DAT_00bb72c2[0] = (unsigned char)iVar7; break;
        case 0xb: iVar7 = _atoi(pcVar5); DAT_00bb72c4[0] = (unsigned char)iVar7; break;
        case 0xc: iVar7 = _atoi(pcVar5); DAT_00bb72c6[0] = (unsigned char)iVar7; break;
        case 0xd: iVar7 = _atoi(pcVar5); DAT_00bb72c8[0] = (unsigned char)iVar7; break;
        case 0xe: iVar7 = _atoi(pcVar5); DAT_00bb72ca[0] = (unsigned char)iVar7; break;
        case 0xf: iVar7 = _atoi(pcVar5); DAT_00bb72b1 = (unsigned char)iVar7; break;
        case 0x10: iVar7 = _atoi(pcVar5); DAT_00bb72b3 = (unsigned char)iVar7; break;
        case 0x11: iVar7 = _atoi(pcVar5); DAT_00bb72b5 = (unsigned char)iVar7; break;
        case 0x12: iVar7 = _atoi(pcVar5); DAT_00bb72b7 = (unsigned char)iVar7; break;
        case 0x13: iVar7 = _atoi(pcVar5); DAT_00bb72b9 = (unsigned char)iVar7; break;
        case 0x14: iVar7 = _atoi(pcVar5); DAT_00bb72bb = (unsigned char)iVar7; break;
        case 0x15: iVar7 = _atoi(pcVar5); DAT_00bb72bd = (unsigned char)iVar7; break;
        case 0x16: iVar7 = _atoi(pcVar5); DAT_00bb72bf = (unsigned char)iVar7; break;
        case 0x17: iVar7 = _atoi(pcVar5); DAT_00bb72c1 = (unsigned char)iVar7; break;
        case 0x18: iVar7 = _atoi(pcVar5); DAT_00bb72c3 = (unsigned char)iVar7; break;
        case 0x19: iVar7 = _atoi(pcVar5); DAT_00bb72c5 = (unsigned char)iVar7; break;
        case 0x1a: iVar7 = _atoi(pcVar5); DAT_00bb72c7 = (unsigned char)iVar7; break;
        case 0x1b: iVar7 = _atoi(pcVar5); DAT_00bb72c9 = (unsigned char)iVar7; break;
        case 0x1c: iVar7 = _atoi(pcVar5); DAT_00bb72cb = (unsigned char)iVar7; break;
        case 0x1d: iVar7 = _atoi(pcVar5); DAT_00bb72d0 = (unsigned char)iVar7; break;
        case 0x1e:
          pbDst = DAT_00bb72d1;
          for (iVar7 = 0x10; iVar7 != 0; iVar7 = iVar7 + -1) {
            __builtin_memcpy(pbDst, pcVar5, 4);
            pcVar5 = pcVar5 + 4;
            pbDst = pbDst + 4;
          }
          break;
        case 0x1f:
          pbDst = DAT_00bb7311;
          for (iVar7 = 0x10; iVar7 != 0; iVar7 = iVar7 + -1) {
            __builtin_memcpy(pbDst, pcVar5, 4);
            pcVar5 = pcVar5 + 4;
            pbDst = pbDst + 4;
          }
          break;
        case 0x20: iVar7 = _atoi(pcVar5); DAT_00bb7371 = (unsigned char)iVar7; break;
        case 0x21: iVar7 = _atoi(pcVar5); DAT_00bb7372 = (unsigned char)iVar7; break;
        case 0x22: iVar7 = _atoi(pcVar5); DAT_00bb7373 = (unsigned char)iVar7; break;
        case 0x23: iVar7 = _atoi(pcVar5); DAT_00bb73a7 = (unsigned char)iVar7; break;
        case 0x24: iVar7 = _atoi(pcVar5); DAT_00bb7374 = (unsigned char)iVar7; break;
        case 0x25: iVar7 = _atoi(pcVar5); DAT_00bb7375 = (unsigned char)iVar7; break;
        case 0x26: iVar7 = _atoi(pcVar5); DAT_00bb7376 = (unsigned char)iVar7; break;
        case 0x27: iVar7 = _atoi(pcVar5); DAT_00bb7377 = (unsigned char)iVar7; break;
        case 0x28: iVar7 = _atoi(pcVar5); DAT_00bb7378 = (unsigned char)iVar7; break;
        case 0x29: iVar7 = _atoi(pcVar5); DAT_00bb7379 = (unsigned char)iVar7; break;
        case 0x2a: iVar7 = _atoi(pcVar5); DAT_00bb737a = (unsigned char)iVar7; break;
        case 0x2b: iVar7 = _atoi(pcVar5); DAT_00bb737b = (unsigned char)iVar7; break;
        case 0x2c: iVar7 = _atoi(pcVar5); DAT_00bb737c = (unsigned char)iVar7; break;
        case 0x2d: iVar7 = _atoi(pcVar5); DAT_00bb737d = (unsigned char)iVar7; break;
        case 0x2e: iVar7 = _atoi(pcVar5); DAT_00bb737e = (unsigned char)iVar7; break;
        case 0x2f: case 0x30: case 0x31: case 0x32: case 0x33:
        case 0x34: case 0x35: case 0x36: case 0x37: case 0x38:
        case 0x39: case 0x3a: case 0x3b: case 0x3c: case 0x3d:
        case 0x3e: case 0x3f: case 0x40: case 0x41: case 0x42:
          iVar14 = _atoi(pcVar5);
          DAT_00bb7351[iVar7] = (unsigned char)iVar14;
          if (bVar18) {
            /* Ghidra: auStackY_10c[iVar7]; at -0x10c, index 0x2f lands on local_50[0] */
            local_50[iVar7 - 0x2f] = 1;
          }
          break;
        case 0x4f: iVar7 = _atoi(pcVar5); DAT_00bb7394 = (unsigned char)iVar7; break;
        case 0x50: iVar7 = _atoi(pcVar5); DAT_00bb7395 = (unsigned char)iVar7; break;
        case 0x51: iVar7 = _atoi(pcVar5); DAT_00bb7396 = (unsigned char)iVar7; break;
        case 0x52: iVar7 = _atoi(pcVar5); DAT_00bb7397 = (unsigned char)iVar7; break;
        case 0x53: iVar7 = _atoi(pcVar5); DAT_00bb7398 = (unsigned char)iVar7; break;
        case 0x54: iVar7 = _atoi(pcVar5); DAT_00bb7399 = (unsigned char)iVar7; break;
        case 0x55: iVar7 = _atoi(pcVar5); DAT_00bb739a = (unsigned char)iVar7; break;
        case 0x56: iVar7 = _atoi(pcVar5); DAT_00bb739b = (unsigned char)iVar7; break;
        case 0x57: iVar7 = _atoi(pcVar5); DAT_00bb739c = (unsigned char)iVar7; break;
        case 0x58: iVar7 = _atoi(pcVar5); DAT_00bb739d = (unsigned char)iVar7; break;
        case 0x59: iVar7 = _atoi(pcVar5); DAT_00bb739e = (unsigned char)iVar7; break;
        case 0x5a: iVar7 = _atoi(pcVar5); DAT_00bb739f = (unsigned char)iVar7; break;
        case 0x5b:
          DAT_00bb73a0 = _atoi(pcVar5);
          break;
        case 0x5c:
          __builtin_memcpy(DAT_00bb735f, pcVar5, 4);
          __builtin_memcpy(DAT_00bb7363, pcVar5 + 4, 4);
          __builtin_memcpy(DAT_00bb7367, pcVar5 + 8, 4);
          __builtin_memcpy(DAT_00bb736b, pcVar5 + 0xc, 4);
          break;
        case 0x5d: iVar7 = _atoi(pcVar5); DAT_00bb736f = (unsigned char)iVar7; break;
        case 0x5e: iVar7 = _atoi(pcVar5); DAT_00bb73a4 = (unsigned char)iVar7; break;
        case 0x5f: iVar7 = _atoi(pcVar5); DAT_00bb73a8 = (unsigned char)iVar7; break;
        case 0x60: iVar7 = _atoi(pcVar5); DAT_00bb737f = (unsigned char)iVar7; break;
        case 0x61: iVar7 = _atoi(pcVar5); DAT_00bb7370 = (unsigned char)iVar7; break;
        case 0x62: iVar7 = _atoi(pcVar5); DAT_00bb73a5 = (unsigned char)iVar7; break;
        case 99: case 100: case 0x65: case 0x66:
          pbDst = DAT_00bb5897 + iVar7 * 0x46;
          for (iVar14 = 0x11; iVar14 != 0; iVar14 = iVar14 + -1) {
            __builtin_memcpy(pbDst, pcVar5, 4);
            pcVar5 = pcVar5 + 4;
            pbDst = pbDst + 4;
          }
          __builtin_memcpy(pbDst, pcVar5, 2);
          break;
        case 0x67: iVar7 = _atoi(pcVar5); DAT_00bb72cc[0] = (unsigned char)iVar7; break;
        case 0x68: iVar7 = _atoi(pcVar5); DAT_00bb72ce[0] = (unsigned char)iVar7; break;
        case 0x69: iVar7 = _atoi(pcVar5); DAT_00bb72cd = (unsigned char)iVar7; break;
        case 0x6a: iVar7 = _atoi(pcVar5); DAT_00bb72cf = (unsigned char)iVar7;
        }
      }
    }
    FUN_004cc590(local_58);
    if (bVar18) {
      iVar7 = 0;
      do {
        if (((local_50[iVar7] != 0) && (bVar3 = DAT_00bb7380[iVar7], 0x7b < bVar3)) &&
            (bVar3 < 0xe6)) {
          DAT_00bb7380[iVar7] = bVar3 + 4;
        }
        iVar7 = iVar7 + 1;
      } while (iVar7 < 0x14);
    }
  }
  return;
}
