/* SPDX-License-Identifier: MIT */
/* z_xvt 00409d90 */
extern char DAT_006db590;
extern char DAT_0085e79d;
extern char DAT_0085e795;
extern char DAT_0085e796;
extern char DAT_0085e794;
extern char DAT_0085e797;

extern unsigned int DAT_00523ae4;
extern unsigned short DAT_00523ae8;
extern unsigned int DAT_00523aec[3];
extern unsigned int DAT_00523af8[3];

extern char DAT_00523b4c;
extern char DAT_00523b1c;
extern char s_RTO2_LST_00523b40[];
extern char s_ITO2_LST_00523b28[];
extern char s_ITO1_LST_00523b34[];
extern char s_ICMD_LST_00523b10[];
extern char s_PCMD_LST_00523b04[];
extern char s_DEMO_TIE_00525588[];

/* Tables with odd strides are packed: accessed by byte address + memcpy. */
extern unsigned char DAT_0071b73a[];
extern int DAT_0071ecf4;
extern short DAT_006d49a0;
extern char DAT_0070ad54[];
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_0070ad43[];

unsigned int FUN_004784d0(void);
void FUN_00409bf0(char *param_1, unsigned short param_2);

void __stdcall FUN_00409d90(void)
{
  char cVar1;
  short sVar2;
  unsigned short uVar3;
  unsigned int uVar4;
  int iVar5;
  unsigned char *piVar6;
  int iVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int *puVar10;
  unsigned int uVar11;
  unsigned short uVar12;
  char *pcVar13;
  char *pcVar14;
  char *pcVar15;
  unsigned int *puVar16;
  _Bool bVar17;
  unsigned int auStack_c4;
  /* uStack_bc overlays uStack_c0 + 4 (the 2 bytes copied from DAT_00523ae8) */
  char uStack_c0[0x40];
  /* acStack_84 overlays uStack_80 - 4 */
  char uStack_80[0x40];
  /* cStack_3d is uStack_40[3], uStack_3c is uStack_40[4..7], uStack_38 is uStack_40[8] */
  char uStack_40[12];

  if ((DAT_006db590 != '\0') && (DAT_0085e79d != '\0')) {
    if (DAT_0085e795 != '\0') {
      __builtin_memcpy(uStack_c0, &DAT_00523ae4, 4);
      __builtin_memcpy(uStack_c0 + 4, &DAT_00523ae8, 2);
      __builtin_memcpy(&sVar2, &DAT_0071b73a[(unsigned int)DAT_0071ecf4 * 0x5bdu], 2);
      if (sVar2 == 0) {
        uVar4 = FUN_004784d0();
        if ((uVar4 & 1) == 0) {
          pcVar13 = s_RTO2_LST_00523b40;
        }
        else {
          pcVar13 = &DAT_00523b4c;
        }
      }
      else {
        uVar4 = FUN_004784d0();
        if ((uVar4 & 1) == 0) {
          pcVar13 = s_ITO2_LST_00523b28;
        }
        else {
          pcVar13 = s_ITO1_LST_00523b34;
        }
      }
      uVar4 = 0xffffffff;
      do {
        pcVar15 = pcVar13;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar15;
      } while (cVar1 != '\0');
      uVar4 = ~uVar4;
      iVar8 = -1;
      pcVar13 = uStack_c0;
      do {
        pcVar14 = pcVar13;
        if (iVar8 == 0) break;
        iVar8 = iVar8 + -1;
        pcVar14 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar14;
      } while (cVar1 != '\0');
      pcVar13 = pcVar15 - uVar4;
      pcVar15 = pcVar14 - 1;
      for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar15, pcVar13, 4);
        pcVar13 = pcVar13 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar13;
        pcVar13 = pcVar13 + 1;
        pcVar15 = pcVar15 + 1;
      }
      FUN_00409bf0(uStack_c0, 0x2b5);
    }
    if (DAT_0085e796 != '\0') {
      __builtin_memcpy(uStack_c0, &DAT_00523ae4, 4);
      __builtin_memcpy(uStack_c0 + 4, &DAT_00523ae8, 2);
      __builtin_memcpy(&sVar2, &DAT_0071b73a[(unsigned int)DAT_0071ecf4 * 0x5bdu], 2);
      if (sVar2 == 0) {
        pcVar13 = &DAT_00523b1c;
      }
      else if (sVar2 == 1) {
        pcVar13 = s_ICMD_LST_00523b10;
      }
      else {
        pcVar13 = s_PCMD_LST_00523b04;
      }
      uVar4 = 0xffffffff;
      do {
        pcVar15 = pcVar13;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar15;
      } while (cVar1 != '\0');
      uVar4 = ~uVar4;
      iVar8 = -1;
      pcVar13 = uStack_c0;
      do {
        pcVar14 = pcVar13;
        if (iVar8 == 0) break;
        iVar8 = iVar8 + -1;
        pcVar14 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar14;
      } while (cVar1 != '\0');
      pcVar13 = pcVar15 - uVar4;
      pcVar15 = pcVar14 - 1;
      for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar15, pcVar13, 4);
        pcVar13 = pcVar13 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar13;
        pcVar13 = pcVar13 + 1;
        pcVar15 = pcVar15 + 1;
      }
      FUN_00409bf0(uStack_c0, 0x321);
    }
    if (DAT_0085e794 != '\0') {
      iVar8 = 0;
      if (0 < DAT_006d49a0) {
        iVar5 = (int)DAT_006d49a0;
        pcVar13 = DAT_0070ad54;
        do {
          if (*pcVar13 != '\0') {
            iVar8 = iVar8 + 1;
          }
          pcVar13 = pcVar13 + 0x566;
          iVar5 = iVar5 + -1;
        } while (iVar5 != 0);
      }
      iVar5 = 0;
      if (0 < DAT_006d49a0) {
        piVar6 = DAT_0070b252;
        do {
          __builtin_memcpy(&iVar7, piVar6, 4);
          if (iVar7 == DAT_0071ecf4) break;
          piVar6 = piVar6 + 0x566;
          iVar5 = iVar5 + 1;
        } while (iVar5 < DAT_006d49a0);
      }
      if ((iVar5 < DAT_006d49a0) &&
          ((1 < DAT_0070ad43[(unsigned int)iVar5 * 0x566u]) || (1 < iVar8))) {
        auStack_c4 = (unsigned int)DAT_0070ad43[(unsigned int)iVar5 * 0x566u];
        uVar3 = (unsigned short)FUN_004784d0();
        uVar4 = 0;
        if (uVar3 / 6 != 0) {
          uVar4 = (unsigned int)uVar3 % (unsigned int)(uVar3 / 6);
        }
        bVar17 = auStack_c4 != 0;
        auStack_c4 = auStack_c4 - 1;
        if (bVar17) {
          uVar3 = 0x6f;
          do {
            __builtin_memcpy(uStack_c0, &DAT_00523ae4, 4);
            __builtin_memcpy(uStack_c0 + 4, &DAT_00523ae8, 2);
            __builtin_memcpy(&sVar2, &DAT_0071b73a[(unsigned int)DAT_0071ecf4 * 0x5bdu], 2);
            if (sVar2 == 1) {
              puVar10 = DAT_00523af8;
            }
            else {
              puVar10 = DAT_00523aec;
            }
            __builtin_memcpy(uStack_40 + 4, &puVar10[1], 4);
            uStack_40[8] = *(char *)(puVar10 + 2);
            uVar9 = 0xffffffff;
            __builtin_memcpy(uStack_40, puVar10, 3);
            uStack_40[3] = (char)((char)uVar4 + '1');
            pcVar13 = uStack_40;
            do {
              pcVar15 = pcVar13;
              if (uVar9 == 0) break;
              uVar9 = uVar9 - 1;
              pcVar15 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar15;
            } while (cVar1 != '\0');
            uVar9 = ~uVar9;
            iVar8 = -1;
            pcVar13 = uStack_c0;
            do {
              pcVar14 = pcVar13;
              if (iVar8 == 0) break;
              iVar8 = iVar8 + -1;
              pcVar14 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar14;
            } while (cVar1 != '\0');
            uVar12 = (unsigned short)((short)uVar4 + 1);
            pcVar13 = pcVar15 - uVar9;
            pcVar15 = pcVar14 - 1;
            for (uVar4 = uVar9 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
              __builtin_memcpy(pcVar15, pcVar13, 4);
              pcVar13 = pcVar13 + 4;
              pcVar15 = pcVar15 + 4;
            }
            for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
              *pcVar15 = *pcVar13;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            }
            FUN_00409bf0(uStack_c0, uVar3);
            if (5 < uVar12) {
              uVar12 = 0;
            }
            uVar4 = (unsigned int)uVar12;
            bVar17 = auStack_c4 != 0;
            auStack_c4 = auStack_c4 + -1;
            uVar3 = (unsigned short)(uVar3 + 0x61);
          } while (bVar17);
        }
      }
    }
    if (DAT_0085e797 != '\0') {
      __builtin_memcpy(uStack_c0, &DAT_00523ae4, 4);
      __builtin_memcpy(uStack_c0 + 4, &DAT_00523ae8, 2);
      uVar4 = 0xffffffff;
      pcVar13 = s_DEMO_TIE_00525588;
      do {
        pcVar15 = pcVar13;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar15;
      } while (cVar1 != '\0');
      uVar4 = ~uVar4;
      pcVar13 = pcVar15 - uVar4;
      pcVar15 = uStack_80;
      for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar15, pcVar13, 4);
        pcVar13 = pcVar13 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar13;
        pcVar13 = pcVar13 + 1;
        pcVar15 = pcVar15 + 1;
      }
      uVar4 = 0xffffffff;
      pcVar13 = uStack_80;
      do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar13 + 1;
      } while (cVar1 != '\0');
      uVar4 = ~uVar4;
      iVar5 = 0;
      iVar8 = (int)(uVar4 - 1);
      uStack_80[uVar4 - 4] = 'l';
      uStack_80[uVar4 - 3] = 's';
      uStack_80[uVar4 - 2] = 't';
      while (uStack_80[0] != '\\') {
        if (iVar8 <= iVar5) goto LAB_0040a14a;
        iVar7 = iVar5 + 1;
        iVar5 = iVar5 + 1;
        uStack_80[0] = uStack_80[iVar7];
      }
      if (iVar5 < iVar8) {
        iVar5 = iVar5 + 1;
      }
      else {
LAB_0040a14a:
        iVar5 = 0;
      }
      uVar4 = 0xffffffff;
      pcVar13 = uStack_c0;
      do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar13 + 1;
      } while (cVar1 != '\0');
      iVar7 = (int)(~uVar4 - 1);
      if (iVar5 < iVar8) {
        uVar11 = (unsigned int)(iVar8 - iVar5);
        puVar10 = (unsigned int *)(uStack_80 + iVar5);
        puVar16 = (unsigned int *)(uStack_c0 + (~uVar4 - 1));
        for (uVar9 = uVar11 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          __builtin_memcpy(puVar16, puVar10, 4);
          puVar10 = puVar10 + 1;
          puVar16 = puVar16 + 1;
        }
        for (uVar4 = uVar11 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
          *(char *)puVar16 = *(char *)puVar10;
          puVar10 = (unsigned int *)((char *)puVar10 + 1);
          puVar16 = (unsigned int *)((char *)puVar16 + 1);
        }
        iVar7 = (int)((unsigned int)iVar7 + uVar11);
      }
      uStack_c0[iVar7] = 0;
      FUN_00409bf0(uStack_c0, 0x5c);
    }
  }
  return;
}
