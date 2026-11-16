/* SPDX-License-Identifier: MIT */
/* z_bop 00500650 */
/* Globals at addresses that are not a multiple of 4 are declared as byte
   arrays and read with __builtin_memcpy. All other globals keep their plain
   types.

   iTmp0..iTmp3 are the only new locals. The code needs scratch ints to hold
   the memcpy'd values, and I did not find a way to avoid them. */

extern char DAT_00b6a606[];            /* int, misaligned */
extern char DAT_00b6a5aa[];            /* int, misaligned */
extern char DAT_00b6bace[];            /* int, misaligned */
extern char DAT_00b6baca[];            /* int, misaligned */
extern char DAT_00b69d20[];
extern char DAT_00b6bba6[];            /* int, misaligned */
extern char DAT_00b6bbb2[];            /* int[], misaligned */
extern char DAT_00b6a5a6[];            /* int, misaligned */
extern char DAT_00b6bbae[];            /* int *, misaligned */
extern char DAT_00bb00ee[];            /* int, misaligned */
extern char DAT_00bb00ea[];            /* int, misaligned */
extern char DAT_00bb00f2[];            /* int, misaligned */
extern unsigned int DAT_00b6a2c0;
extern unsigned int DAT_00b69ccc;
extern int DAT_00b69e30;
extern unsigned int DAT_0066d920[];
extern int DAT_0066d968[];
extern int DAT_0066d8d8[];
extern int DAT_0066da4c;
extern char DAT_00b6d84a[];            /* int[], misaligned */
extern char DAT_00b6d85a[];            /* int[], misaligned */
extern char DAT_00b6d85e[];            /* int[], misaligned */
extern char DAT_00b6d862[];            /* int[], misaligned */
extern char DAT_00aa47ba[];
extern short DAT_00aa5aac;
extern char DAT_00b6c042[];            /* int[], misaligned */
extern char DAT_00a91d24[];
extern char DAT_00a91d18[];
extern char DAT_00a91cc0[];
extern int DAT_0066d990[];
extern char DAT_00b6d5a6[];            /* int[], misaligned */
extern unsigned int DAT_00b6d598[];
extern char DAT_00b6d5de[];            /* int[], misaligned */
extern char DAT_00b6d5ae[];            /* int[], misaligned */
extern char DAT_00b6d5aa[];            /* int[], misaligned */
extern char DAT_00b6d5b2[];            /* int[], misaligned */
extern char DAT_00b6d5b6[];            /* int[], misaligned */
extern char DAT_00b6d5ba[];            /* int[], misaligned */
extern char DAT_00b6d5c6[];            /* int[], misaligned */
extern unsigned char DAT_00b6a270[];
extern int DAT_0066d9c0[];
extern char DAT_00b6bcc2[];            /* int[], misaligned */
extern char DAT_00b6bce2[];            /* int[], misaligned */
extern char DAT_00b6bd02[];            /* int[], misaligned */
extern char DAT_00b6bdc2[];            /* int[], misaligned */
extern char DAT_00b6be82[];            /* int[], misaligned */
extern char DAT_00b6bea2[];            /* int[], misaligned */
extern char DAT_00b6bec2[];            /* int[], misaligned */
extern char DAT_00b6bf82[];            /* int[], misaligned */

extern char DAT_0052d4cc[];
extern char DAT_0052b460[];
extern char s__c_s__c_s_0052a6b0[];
extern char s__c_s__c_d__s__d__s_0052d4b8[];
extern char s__c_s__c_s_d_0052d4ac[];
extern char s__c_d___c_s_0052d4a0[];
extern char s__d___d__0052b464[];
extern char s__c_d___c_s__c_s_0052d490[];
extern char s___s__s__0052d488[];
extern char s__c_d___c__s__s__0052d478[];
extern char s__d__d__0052b450[];

extern unsigned char FUN_004dca90(int param_1);
extern void FUN_004b5570(char *param_1, int param_2, int param_3);
extern void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char *FUN_004ddc10(int param_1);
extern int _sprintf(char *buf, const char *fmt, ...);
extern void FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
extern void FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
extern void FUN_004d4cf0(unsigned int *param_1);
extern void FUN_004d4d20(void *param_1);
extern int FUN_004d11e0(void);

unsigned int __cdecl FUN_00500650(int param_1)
{
  unsigned char bVar1;
  unsigned short uVar2;
  char *pcVar3;
  int *piVar4;
  char *pcVar5;
  char *pcVar6;
  int iVar7;
  unsigned int *puVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int iVar12;
  int iVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned int uVar16;
  unsigned int *puVar17;
  int local_38;
  int local_34;
  int *local_30[3];
  int local_24;
  int local_20[4];
  unsigned int local_10[4];
  int iTmp0;
  int iTmp1;
  int iTmp2;
  int iTmp3;

  bVar1 = FUN_004dca90(0x12);
  if (((bVar1 != 0) &&
      (bVar1 = FUN_004dca90(0x10), bVar1 != 0)) &&
     (bVar1 = FUN_004dca90(0x11), bVar1 != 0)) {
    FUN_004b5570(DAT_0052d4cc, 0xb8, 0x16a);
  }
  FUN_004d6460(local_20, 0x54, 0x5a, 0x1b2, 0x6a);
  __builtin_memcpy(&iTmp0, DAT_00b6a606, 4);
  if (iTmp0 == 0) {
    piVar4 = local_20;
    uVar16 = 0xffff;
    pcVar3 = FUN_004ddc10(0x137);
  }
  else {
    __builtin_memcpy(&iTmp0, DAT_00b6a5aa, 4);
    if (iTmp0 == 1) {
      __builtin_memcpy(&iTmp1, DAT_00b6bace, 4);
      __builtin_memcpy(&iTmp2, DAT_00b6baca, 4);
      if ((int)((unsigned int)iTmp1 - (unsigned int)iTmp2) == 1) {
        pcVar3 = FUN_004ddc10(0xce);
        uVar14 = 4;
        pcVar5 = FUN_004ddc10(0x2af);
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 1, pcVar5, uVar14, pcVar3);
      }
      else {
        pcVar3 = FUN_004ddc10(0x154);
        __builtin_memcpy(&iVar10, DAT_00b6bace, 4);
        pcVar5 = FUN_004ddc10(0x14f);
        __builtin_memcpy(&iVar12, DAT_00b6baca, 4);
        iVar12 = (int)((unsigned int)iVar12 + 1u);
        uVar14 = 4;
        pcVar6 = FUN_004ddc10(0x2af);
        _sprintf(DAT_00b69d20, s__c_s__c_d__s__d__s_0052d4b8, 1, pcVar6, uVar14, iVar12, pcVar5,
                 iVar10, pcVar3);
      }
    }
    else if (iTmp0 == 3) {
      local_30[0] = (int *)0;
      local_30[1] = (int *)0;
      local_30[2] = (int *)0;
      __builtin_memcpy(&iTmp1, DAT_00b6bba6, 4);
      if (-1 < iTmp1) {
        piVar4 = (int *)DAT_00b6bbb2;
        iVar10 = (int)((unsigned int)iTmp1 + 1u);
        do {
          __builtin_memcpy(&iVar12, (char *)piVar4, 4);
          piVar4 = piVar4 + 1;
          local_30[iVar12] = (int *)((unsigned int)local_30[iVar12] + 1u);
          iVar10 = (int)((unsigned int)iVar10 - 1u);
        } while (iVar10 != 0);
      }
      __builtin_memcpy(&iTmp1, DAT_00b6a5a6, 4);
      if (iTmp1 == 0) {
        iVar10 = 0;
      }
      else if (iTmp1 == 1) {
        iVar10 = 1;
      }
      else {
        iVar10 = 0;
      }
      __builtin_memcpy(&piVar11, DAT_00b6bbae, 4);
      if ((piVar11 == local_30[0]) || (piVar11 == local_30[1])) {
        pcVar3 = FUN_004ddc10(0xce);
        uVar14 = 4;
        pcVar5 = FUN_004ddc10(0x2b1);
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 1, pcVar5, uVar14, pcVar3);
      }
      else {
        pcVar3 = FUN_004ddc10(0x2b2);
        __builtin_memcpy(&piVar11, DAT_00b6bbae, 4);
        pcVar5 = FUN_004ddc10(0x14f);
        piVar4 = local_30[iVar10];
        uVar14 = 4;
        pcVar6 = FUN_004ddc10(0x2b1);
        _sprintf(DAT_00b69d20, s__c_s__c_d__s__d__s_0052d4b8, 1, pcVar6, uVar14, piVar4, pcVar5,
                 piVar11, pcVar3);
      }
    }
    else {
      __builtin_memcpy(&iTmp1, DAT_00bb00ee, 4);
      __builtin_memcpy(&iTmp2, DAT_00bb00ea, 4);
      __builtin_memcpy(&iTmp3, DAT_00bb00f2, 4);
      if ((iTmp1 == iTmp2) && (iTmp3 == 1)) {
        pcVar3 = FUN_004ddc10(0xce);
        uVar14 = 4;
        pcVar5 = FUN_004ddc10(0x311);
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 1, pcVar5, uVar14, pcVar3);
      }
      else {
        iVar10 = (int)((unsigned int)iTmp2 + 1u);
        pcVar3 = FUN_004ddc10(0x158);
        uVar14 = 4;
        pcVar5 = FUN_004ddc10(0x311);
        _sprintf(DAT_00b69d20, s__c_s__c_s_d_0052d4ac, 1, pcVar5, uVar14, pcVar3, iVar10);
      }
    }
    piVar4 = local_20;
    uVar16 = 0xffff;
    pcVar3 = DAT_00b69d20;
  }
  FUN_004db640(0xf, pcVar3, piVar4, uVar16);
  iVar12 = 0x72;
  iVar10 = 0xe4;
  uVar16 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x14a);
  FUN_004db4e0(0xc, pcVar3, iVar10, iVar12, uVar16);
  iVar12 = 0x72;
  iVar10 = 0x12a;
  uVar16 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x150);
  FUN_004db4e0(0xc, pcVar3, iVar10, iVar12, uVar16);
  iVar13 = 0x72;
  iVar12 = 0x16b;
  iVar10 = 0x86;
  uVar16 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x21b);
  FUN_004db4e0(0xc, pcVar3, iVar12, iVar13, uVar16);
  puVar8 = DAT_0066d920;
  local_34 = 0;
  local_24 = 0;
  __builtin_memcpy(&local_38, DAT_00b6d84a + DAT_0066d920[0] * 28u, 4);
  do {
    iVar12 = 0x58;
    uVar16 = *puVar8;
    if (uVar16 == 0xffffffff) break;
    if (DAT_0066d968[uVar16] != 0) {
      __builtin_memcpy(&iTmp0, DAT_00b6d84a + uVar16 * 28u, 4);
      if (local_38 != iTmp0) {
        local_34 = local_24;
        __builtin_memcpy(&local_38, DAT_00b6d84a + uVar16 * 28u, 4);
      }
      uVar2 = 2;
      if (2 < local_34) {
        uVar2 = 1;
      }
      if (DAT_0066da4c == 0) {
        __builtin_memcpy(&iTmp0, DAT_00b6a5aa, 4);
        if ((iTmp0 == 1) || (iTmp0 == 2)) {
          _sprintf(DAT_00b69d20, s__c_d___c_s_0052d4a0, uVar2,
                   (int)((unsigned int)local_34 + 1u), 1,
                   DAT_00aa47ba + uVar16 * 0x1e5u);
          FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d84a + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
          FUN_004db4e0(0xc, DAT_00b69d20, 0xe4, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d85a + *puVar8 * 28u, 4);
          __builtin_memcpy(&iTmp1, DAT_00b6d85e + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, s__d___d__0052b464, iTmp0, iTmp1);
          FUN_004db4e0(0xc, DAT_00b69d20, 0x12a, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d862 + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
          iVar12 = 0x67;
          FUN_004db4e0(0xc, DAT_00b69d20, 0x16b, iVar10, 0xffff);
          iVar10 = (int)((unsigned int)iVar10 + 0xfu);
        }
        else if (DAT_0066d8d8[uVar16] == 0) {
          _sprintf(DAT_00b69d20, s__c_d___c_s_0052d4a0, uVar2,
                   (int)((unsigned int)local_34 + 1u), 1,
                   DAT_00aa47ba + uVar16 * 0x1e5u);
          iVar12 = 0x67;
          FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar10, 0xffff);
          iVar10 = (int)((unsigned int)iVar10 + 0xfu);
        }
        else {
          _sprintf(DAT_00b69d20, s__c_d___c_s_0052d4a0, uVar2,
                   (int)((unsigned int)local_34 + 1u), 1,
                   DAT_00aa47ba + uVar16 * 0x1e5u);
          FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d84a + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
          FUN_004db4e0(0xc, DAT_00b69d20, 0xe4, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d85a + *puVar8 * 28u, 4);
          __builtin_memcpy(&iTmp1, DAT_00b6d85e + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, s__d___d__0052b464, iTmp0, iTmp1);
          FUN_004db4e0(0xc, DAT_00b69d20, 0x12a, iVar10, 0xffff);
          __builtin_memcpy(&iTmp0, DAT_00b6d862 + *puVar8 * 28u, 4);
          _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
          iVar12 = 0x67;
          FUN_004db4e0(0xc, DAT_00b69d20, 0x16b, iVar10, 0xffff);
          iVar10 = (int)((unsigned int)iVar10 + 0xfu);
        }
      }
      else if ((DAT_0066d8d8[uVar16] != 0) && (iVar13 = 0, 0 < DAT_00aa5aac)) {
        local_30[0] = (int *)DAT_00b6c042;
        pcVar3 = DAT_00a91d24;
        iVar9 = iVar10;
        do {
          iVar10 = iVar9;
          if ((*pcVar3 != '\0') && ((unsigned int)(unsigned char)pcVar3[-0xc] == *puVar8)) {
            pcVar5 = pcVar3 + -100;
            uVar14 = 1;
            __builtin_memcpy(&iTmp0, (char *)local_30[0], 4);
            pcVar6 = FUN_004ddc10((int)((unsigned int)iTmp0 + 0x9au));
            _sprintf(DAT_00b69d20, s__c_d___c_s__c_s_0052d490, uVar2,
                     (int)((unsigned int)local_34 + 1u), 6, pcVar6,
                     uVar14, pcVar5);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar12, iVar9, 0xffff);
            __builtin_memcpy(&iTmp0, DAT_00b6d84a + *puVar8 * 28u, 4);
            _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
            FUN_004db4e0(0xc, DAT_00b69d20, 0xe4, iVar9, 0xffff);
            __builtin_memcpy(&iTmp0, DAT_00b6d85a + *puVar8 * 28u, 4);
            __builtin_memcpy(&iTmp1, DAT_00b6d85e + *puVar8 * 28u, 4);
            _sprintf(DAT_00b69d20, s__d___d__0052b464, iTmp0, iTmp1);
            iVar12 = 0x16b;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x12a, iVar9, 0xffff);
            __builtin_memcpy(&iTmp0, DAT_00b6d862 + *puVar8 * 28u, 4);
            _sprintf(DAT_00b69d20, DAT_0052b460, iTmp0);
            iVar10 = (int)((unsigned int)iVar9 + 0xfu);
            FUN_004db4e0(0xc, DAT_00b69d20, 0x16b, iVar9, 0xffff);
          }
          local_30[0] = local_30[0] + 1;
          pcVar3 = pcVar3 + 0x562;
          iVar13 = (int)((unsigned int)iVar13 + 1u);
          iVar9 = iVar10;
        } while (iVar13 < DAT_00aa5aac);
      }
      if (DAT_0066d8d8[*puVar8] == 0) {
        piVar4 = DAT_0066d990;
        iVar13 = iVar10;
        do {
          iVar9 = *piVar4;
          iVar10 = iVar13;
          if (iVar9 == -1) break;
          __builtin_memcpy(&iTmp0, DAT_00b6d5a6 + (unsigned int)iVar9 * 0x58u, 4);
          if ((unsigned int)(unsigned char)DAT_00a91d18[(unsigned int)iTmp0 * 0x562u] == *puVar8) {
            FUN_004d6460(local_20, iVar12, iVar13, 0xe1, (int)((unsigned int)iVar13 + 0xeu));
            FUN_004d4cf0(local_10);
            FUN_004d4d20(local_20);
            if (DAT_0066da4c == 0) {
              puVar17 = DAT_00b6d598 + (unsigned int)iVar9 * 0x16u;
              __builtin_memcpy(&iTmp0, DAT_00b6d5de + (unsigned int)iVar9 * 0x58u, 4);
              if (iTmp0 == 0) {
                uVar14 = 1;
                __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar9 * 0x58u, 4);
                pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
                _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar14, puVar17);
              }
              else {
                __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar9 * 0x58u, 4);
                pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
                _sprintf(DAT_00b69d20, s___s__s__0052d488, pcVar3);
              }
            }
            else {
              puVar17 = DAT_00b6d598 + (unsigned int)iVar9 * 0x16u;
              __builtin_memcpy(&iTmp0, DAT_00b6d5de + (unsigned int)iVar9 * 0x58u, 4);
              if (iTmp0 == 0) {
                uVar14 = 1;
                __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar9 * 0x58u, 4);
                pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
                _sprintf(DAT_00b69d20, s__c_d___c_s__c_s_0052d490, uVar2,
                         (int)((unsigned int)local_34 + 1u), 6,
                         pcVar3, uVar14, puVar17);
              }
              else {
                __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar9 * 0x58u, 4);
                pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
                _sprintf(DAT_00b69d20, s__c_d___c__s__s__0052d478, uVar2,
                         (int)((unsigned int)local_34 + 1u), 1,
                         pcVar3);
              }
            }
            uVar16 = DAT_00b69ccc;
            __builtin_memcpy(&iTmp0, DAT_00b6d5de + (unsigned int)iVar9 * 0x58u, 4);
            if (iTmp0 == 0) {
              iVar10 = FUN_004d11e0();
              __builtin_memcpy(&iTmp1, DAT_00b6d5aa + (unsigned int)iVar9 * 0x58u, 4);
              if (iVar10 == iTmp1) {
                uVar16 = *(unsigned int *)(DAT_00b6a270 +
                                           (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2u);
              }
              else {
                uVar16 = DAT_00b6a2c0;
                if (DAT_00b69e30 == 2) {
                  uVar16 = *(unsigned int *)(DAT_00b6a270 +
                                             (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2u);
                }
              }
            }
            FUN_004db4e0(0xc, DAT_00b69d20, iVar12, iVar13, uVar16);
            FUN_004d4d20(local_10);
            __builtin_memcpy(&iTmp0, DAT_00b6a5aa, 4);
            if (((iTmp0 == 1) || (iTmp0 == 2)) && (DAT_0066da4c != 0)) {
              __builtin_memcpy(&uVar14, DAT_00b6d84a + *puVar8 * 28u, 4);
            }
            else {
              __builtin_memcpy(&uVar14, DAT_00b6d5b2 + (unsigned int)iVar9 * 0x58u, 4);
            }
            _sprintf(DAT_00b69d20, DAT_0052b460, uVar14);
            FUN_004db4e0(0xc, DAT_00b69d20, 0xe4, iVar13, 0xffff);
            if (DAT_0066da4c == 0) {
              __builtin_memcpy(&iTmp0, DAT_00b6d5b6 + (unsigned int)iVar9 * 0x58u, 4);
              __builtin_memcpy(&iTmp1, DAT_00b6d5ba + (unsigned int)iVar9 * 0x58u, 4);
              _sprintf(DAT_00b69d20, s__d___d__0052b464, iTmp0, iTmp1);
              FUN_004db4e0(0xc, DAT_00b69d20, 0x12a, iVar13, 0xffff);
              __builtin_memcpy(&uVar14, DAT_00b6d5c6 + (unsigned int)iVar9 * 0x58u, 4);
            }
            else {
              __builtin_memcpy(&iTmp0, DAT_00b6d85a + *puVar8 * 28u, 4);
              __builtin_memcpy(&iTmp1, DAT_00b6d85e + *puVar8 * 28u, 4);
              _sprintf(DAT_00b69d20, s__d___d__0052b464, iTmp0, iTmp1);
              FUN_004db4e0(0xc, DAT_00b69d20, 0x12a, iVar13, 0xffff);
              __builtin_memcpy(&uVar14, DAT_00b6d862 + *puVar8 * 28u, 4);
            }
            _sprintf(DAT_00b69d20, DAT_0052b460, uVar14);
            iVar10 = (int)((unsigned int)iVar13 + 0xfu);
            FUN_004db4e0(0xc, DAT_00b69d20, 0x16b, iVar13, 0xffff);
            iVar12 = (int)((-(unsigned int)(DAT_0066da4c == 0) & 0xfu) + 0x58u);
          }
          piVar4 = piVar4 + 1;
          iVar13 = iVar10;
        } while (piVar4 < &DAT_0066d990[8]);
      }
      __builtin_memcpy(&iTmp0, DAT_00b6a5aa, 4);
      if (((iTmp0 == 1) || (iTmp0 == 2)) &&
         ((DAT_0066da4c == 0 && (iVar13 = 0, 0 < DAT_00aa5aac)))) {
        local_30[0] = (int *)DAT_00b6c042;
        pcVar3 = DAT_00a91d24;
        iVar9 = iVar10;
        do {
          iVar10 = iVar9;
          if ((*pcVar3 != '\0') && ((unsigned int)(unsigned char)pcVar3[-0xc] == *puVar8)) {
            iVar7 = 0;
            do {
              __builtin_memcpy(&iTmp0, DAT_00b6d5aa + (unsigned int)iVar7 * 0x58u, 4);
              __builtin_memcpy(&iTmp1, DAT_00b6d5aa + (unsigned int)iVar7 * 0x58u - 4u, 4);
              if ((iTmp0 != 0) && (iTmp1 == iVar13)) break;
              iVar7 = (int)((unsigned int)iVar7 + 1u);
            } while (iVar7 < 8);
            if (iVar7 == 8) {
              pcVar5 = pcVar3 + -100;
              uVar14 = 1;
              __builtin_memcpy(&iTmp0, (char *)local_30[0], 4);
              pcVar6 = FUN_004ddc10((int)((unsigned int)iTmp0 + 0x9au));
              _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar6, uVar14, pcVar5);
              iVar10 = (int)((unsigned int)iVar9 + 0xfu);
              FUN_004db4e0(0xc, DAT_00b69d20, iVar12, iVar9, 0xffff);
            }
          }
          local_30[0] = local_30[0] + 1;
          pcVar3 = pcVar3 + 0x562;
          iVar13 = (int)((unsigned int)iVar13 + 1u);
          iVar9 = iVar10;
        } while (iVar13 < DAT_00aa5aac);
      }
    }
    puVar8 = puVar8 + 1;
    local_24 = (int)((unsigned int)local_24 + 1u);
  } while (puVar8 < &DAT_0066d920[10]);
  iVar9 = (int)((unsigned int)iVar10 + 0xfu);
  iVar10 = (int)((unsigned int)iVar10 + 0x1eu);
  iVar13 = iVar9;
  uVar16 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x21e);
  FUN_004db4e0(0xc, pcVar3, iVar12, iVar13, uVar16);
  local_30[0] = DAT_0066d9c0;
  iVar12 = iVar10;
  do {
    iVar13 = *local_30[0];
    if (iVar13 == -1) break;
    if (iVar13 < 8) {
      iVar7 = FUN_004d11e0();
      __builtin_memcpy(&iTmp0, DAT_00b6d5aa + (unsigned int)iVar13 * 0x58u, 4);
      __builtin_memcpy(&iTmp1, DAT_00b6bcc2 + (unsigned int)iVar13 * 4u, 4);
      __builtin_memcpy(&iTmp2, DAT_00b6bce2 + (unsigned int)iVar13 * 4u, 4);
      if ((iVar7 != iTmp0) &&
         ((iTmp1 != 0 || (iTmp2 != 0)))) {
        FUN_004d6460(local_20, 0x58, iVar12, 0xce, (int)((unsigned int)iVar12 + 0xeu));
        FUN_004d4cf0(local_10);
        FUN_004d4d20(local_20);
        __builtin_memcpy(&iTmp0, DAT_00b6d5de + (unsigned int)iVar13 * 0x58u, 4);
        if (iTmp0 == 0) {
          puVar17 = DAT_00b6d598 + (unsigned int)iVar13 * 0x16u;
          uVar14 = 4;
          __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar13 * 0x58u, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
          _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar14, puVar17);
          uVar16 = 0xffff;
        }
        else {
          puVar17 = DAT_00b6d598 + (unsigned int)iVar13 * 0x16u;
          __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar13 * 0x58u, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
          _sprintf(DAT_00b69d20, s___s__s__0052d488, pcVar3, puVar17);
          uVar16 = DAT_00b69ccc;
        }
        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar12, uVar16);
        FUN_004d4d20(local_10);
        __builtin_memcpy(&uVar14, DAT_00b6bce2 + (unsigned int)iVar13 * 4u, 4);
        __builtin_memcpy(&uVar15, DAT_00b6bcc2 + (unsigned int)iVar13 * 4u, 4);
LAB_0050148e:
        _sprintf(DAT_00b69d20, s__d__d__0052b450, uVar15, uVar14);
        FUN_004db4e0(0xc, DAT_00b69d20, 0xd0, iVar12, 0xffff);
        iVar12 = (int)((unsigned int)iVar12 + 0xfu);
      }
    }
    else {
      iVar7 = (int)((unsigned int)iVar13 - 8u);
      iVar13 = (int)((unsigned int)iVar7 * 0x52u +
                     ((unsigned int)iVar13 * 9u - 0x48u) * 0x90u);
      __builtin_memcpy(&iTmp0, DAT_00b6bd02 + (unsigned int)iVar7 * 4u, 4);
      __builtin_memcpy(&iTmp1, DAT_00b6bdc2 + (unsigned int)iVar7 * 4u, 4);
      if ((DAT_0066d8d8[(unsigned char)DAT_00a91d18[iVar13]] != 0) &&
         ((iTmp0 != 0 || (iTmp1 != 0)))) {
        iVar13 = (int)((unsigned int)DAT_00a91cc0 + (unsigned int)iVar13);
        uVar14 = 4;
        __builtin_memcpy(&iTmp2, DAT_00b6c042 + (unsigned int)iVar7 * 4u, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp2 + 0x9au));
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar14, iVar13);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar12, 0xffff);
        __builtin_memcpy(&uVar14, DAT_00b6bdc2 + (unsigned int)iVar7 * 4u, 4);
        __builtin_memcpy(&uVar15, DAT_00b6bd02 + (unsigned int)iVar7 * 4u, 4);
        goto LAB_0050148e;
      }
    }
    local_30[0] = local_30[0] + 1;
  } while (local_30[0] < &DAT_0066d9c0[8]);
  iVar12 = 0x103;
  piVar4 = DAT_0066d9c0;
  uVar16 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x21d);
  FUN_004db4e0(0xc, pcVar3, iVar12, iVar9, uVar16);
  do {
    iVar12 = *piVar4;
    if (iVar12 == -1) {
      return 1;
    }
    if (iVar12 < 8) {
      iVar13 = FUN_004d11e0();
      __builtin_memcpy(&iTmp0, DAT_00b6d5aa + (unsigned int)iVar12 * 0x58u, 4);
      __builtin_memcpy(&iTmp1, DAT_00b6be82 + (unsigned int)iVar12 * 4u, 4);
      __builtin_memcpy(&iTmp2, DAT_00b6bea2 + (unsigned int)iVar12 * 4u, 4);
      if ((iVar13 != iTmp0) &&
         ((iTmp1 != 0 || (iTmp2 != 0)))) {
        FUN_004d6460(local_20, 0x103, iVar10, 0x179, (int)((unsigned int)iVar10 + 0xeu));
        FUN_004d4cf0(local_10);
        FUN_004d4d20(local_20);
        puVar17 = DAT_00b6d598 + (unsigned int)iVar12 * 0x16u;
        __builtin_memcpy(&iTmp0, DAT_00b6d5de + (unsigned int)iVar12 * 0x58u, 4);
        if (iTmp0 == 0) {
          uVar14 = 4;
          __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar12 * 0x58u, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
          _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar14, puVar17);
          uVar16 = 0xffff;
        }
        else {
          __builtin_memcpy(&iTmp1, DAT_00b6d5ae + (unsigned int)iVar12 * 0x58u, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp1 + 0x9au));
          _sprintf(DAT_00b69d20, s___s__s__0052d488, pcVar3);
          uVar16 = DAT_00b69ccc;
        }
        FUN_004db4e0(0xc, DAT_00b69d20, 0x103, iVar10, uVar16);
        FUN_004d4d20(local_10);
        __builtin_memcpy(&uVar14, DAT_00b6bea2 + (unsigned int)iVar12 * 4u, 4);
        __builtin_memcpy(&uVar15, DAT_00b6be82 + (unsigned int)iVar12 * 4u, 4);
LAB_005016bb:
        _sprintf(DAT_00b69d20, s__d__d__0052b450, uVar15, uVar14);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x17b, iVar10, 0xffff);
        iVar10 = (int)((unsigned int)iVar10 + 0xfu);
      }
    }
    else {
      iVar13 = (int)((unsigned int)iVar12 - 8u);
      iVar12 = (int)((unsigned int)iVar13 * 0x52u +
                     ((unsigned int)iVar12 * 9u - 0x48u) * 0x90u);
      __builtin_memcpy(&iTmp0, DAT_00b6bec2 + (unsigned int)iVar13 * 4u, 4);
      __builtin_memcpy(&iTmp1, DAT_00b6bf82 + (unsigned int)iVar13 * 4u, 4);
      if ((DAT_0066d8d8[(unsigned char)DAT_00a91d18[iVar12]] != 0) &&
         ((iTmp0 != 0 || (iTmp1 != 0)))) {
        iVar12 = (int)((unsigned int)DAT_00a91cc0 + (unsigned int)iVar12);
        uVar14 = 4;
        __builtin_memcpy(&iTmp2, DAT_00b6c042 + (unsigned int)iVar13 * 4u, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp2 + 0x9au));
        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar14, iVar12);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x103, iVar10, 0xffff);
        __builtin_memcpy(&uVar14, DAT_00b6bf82 + (unsigned int)iVar13 * 4u, 4);
        __builtin_memcpy(&uVar15, DAT_00b6bec2 + (unsigned int)iVar13 * 4u, 4);
        goto LAB_005016bb;
      }
    }
    piVar4 = piVar4 + 1;
    if (&DAT_0066d9c0[8] <= piVar4) {
      return 1;
    }
  } while (1);
}
