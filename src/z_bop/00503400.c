/* SPDX-License-Identifier: MIT */
/* z_bop 00503400 */
void FUN_004d6460(int *rect, int a, int b, int c, int d);
char *FUN_004ddc10(int id);
void FUN_004db640(int font, char *text, int *rect, unsigned int color);
void FUN_004d64a0(int *rect, int a, int b);
int FUN_004dbfa0(char *text, int font);
void FUN_004b4cc0(char *name, int *out);
void FUN_004db4e0(int font, char *text, int x, int y, unsigned int color);
void FUN_004b5570(char *name, int x, int y);
int FUN_004d11e0(void);
void FUN_004d4cf0(int *p);
void FUN_004d4d20(int *p);
int __cdecl _sprintf(char *dst, const char *fmt, ...);

extern unsigned int DAT_0066da18[];
extern unsigned int DAT_0066da3c;
extern int DAT_0066da4c;
extern int DAT_0066d8d8[];
extern unsigned int DAT_0066d8a0[];
extern unsigned int DAT_0066d968[];
extern int DAT_0066d990[];
extern int DAT_0066d9b0;
extern short DAT_00aa5aac;
extern unsigned char DAT_00aa47ba[];
extern unsigned char DAT_00a91d18[];
extern char DAT_00a91d24[];
extern char DAT_00a91b90[];
extern char DAT_00b69d20[];
extern int DAT_00b69e30;
extern unsigned int DAT_00b6a2c0;
extern unsigned char DAT_00b6a270[];
extern unsigned char DAT_00b6a5a6;
extern unsigned char DAT_00b6baca;
extern unsigned char DAT_00b6bace;
extern unsigned char DAT_00b6bad6[];
extern unsigned char DAT_00b6bada[];
extern unsigned char DAT_00b6bade[];
extern unsigned char DAT_00b6bae2[];
extern unsigned char DAT_00b6c042[];
extern unsigned int DAT_00b6d598[];
extern unsigned char DAT_00b6d5a6[];
extern unsigned char DAT_00b6d5aa[];
extern unsigned char DAT_00b6d5ae[];
extern unsigned char DAT_00b6d84a[];
extern unsigned char DAT_00b6d866;
extern unsigned char DAT_00b6d86a;
extern unsigned char DAT_00b6d962;
extern unsigned char DAT_00b6da16[];
extern char DAT_0052b460[];
extern char DAT_0052b4d4[];
extern char s__c_s___c_s_0052adb0[];
extern char s__s__s____s_0052d504[];
extern char s_medlvl_d_0052b49c[];
extern char s__c_s___c_s__c_s_0052d530[];
extern char s__c_s__c_d_c__s__c_d_c__s_0052d514[];
extern char s__c_d___c_s_0052d4a0[];
extern char s__c_d___c_s__c_s_0052d490[];
extern char s__c_s__c_s_0052a6b0[];

#define RD32(dst, src) __builtin_memcpy(&(dst), (src), 4)
#define CUR_IDX (*(int *)((char *)DAT_0066da18 + iVar6))
#define RDIDX(dst, base) RD32(dst, (base) + (unsigned int)CUR_IDX * 20u)

unsigned int __cdecl FUN_00503400(int param_1)
{
  unsigned int *puVar1;
  unsigned int *puVar11;
  char *pcVar2;
  char *pcVar4;
  char *pcVar5;
  char *pcVar7;
  unsigned char *puVar8;
  unsigned char *pbVar21;
  unsigned char *pbVar22;
  unsigned char *pbVar23;
  int iVar6;
  int iVar9;
  int iVar10;
  int iVar12;
  int iVar13;
  int iVar16;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned int uVar17;
  unsigned int uVar18;
  unsigned int uVar19;
  unsigned int *puVar20;
  int *local_8c;
  char *local_88;
  int local_84[4];
  int local_74;
  int local_70;
  int local_6c;
  int local_68;
  int local_64[4];
  int local_54[4];
  int local_44[4];
  char local_34[52];
  int t0;
  int t1;
  int t2;
  unsigned int ut;

  FUN_004d6460(local_84, 0x54, 0x5a, 0x1b2, 0x6a);
  uVar19 = 0xffff;
  pcVar2 = FUN_004ddc10(0x13b);
  FUN_004db640(0xf, pcVar2, local_84, uVar19);
  FUN_004d6460(local_84, 0x54, 0x6e, 0x1b2, 0x73);
  FUN_004db640(0xc, DAT_00a91b90, local_84, DAT_00b6a2c0);
  FUN_004d64a0(local_84, 0, 0x14);
  RD32(t0, &DAT_00b6bace);
  RD32(t1, &DAT_00b6baca);
  if ((int)((unsigned int)t0 - (unsigned int)t1) == 1) {
    if (DAT_0066da4c != 0) {
      puVar11 = &DAT_0066da18[0];
      while (uVar19 = *puVar11, uVar19 != 0xffffffffu) {
        if (DAT_0066d8d8[uVar19] == 0) {
          iVar12 = 0;
          pbVar21 = DAT_00b6d5aa;
          do {
            RD32(t0, pbVar21);
            if (t0 != 0) {
              RD32(t1, pbVar21 - 4);
              if ((unsigned int)DAT_00a91d18[(unsigned int)t1 * 0x562u] == uVar19) {
                puVar20 = DAT_00b6d598 + (unsigned int)iVar12 * 0x16u;
                uVar14 = 1;
                RD32(t2, DAT_00b6d5ae + DAT_0066da18[0] * 0x58u);
                pcVar2 = FUN_004ddc10((int)((unsigned int)t2 + 0x9a));
                uVar17 = 6;
                pcVar5 = FUN_004ddc10(0x2ae);
                _sprintf(DAT_00b69d20, s__c_s___c_s__c_s_0052d530, 2, pcVar5, uVar17, pcVar2,
                         uVar14, puVar20);
                break;
              }
            }
            pbVar21 = pbVar21 + 0x58;
            iVar12 = (int)((unsigned int)iVar12 + 1u);
          } while (pbVar21 < &DAT_00b6d86a);
          if (iVar12 == 8) {
            puVar8 = DAT_00aa47ba + *puVar11 * 0x1e5u;
            uVar17 = 1;
            pcVar2 = FUN_004ddc10(0x2ae);
            _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 2, pcVar2, uVar17, puVar8);
            FUN_004db640(0xf, DAT_00b69d20, local_84, 0xffff);
            FUN_004d64a0(local_84, 0, 0x11);
          }
          else {
            uVar19 = DAT_00b6a2c0;
            RD32(ut, &DAT_00b6a5a6);
            if (*puVar11 == ut) {
              uVar19 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeu) * 2u));
            }
            FUN_004db640(0xf, DAT_00b69d20, local_84, uVar19);
            FUN_004d64a0(local_84, 0, 0x11);
            RD32(ut, &DAT_00b6a5a6);
            RD32(t0, &DAT_00b6d962);
            RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
            if ((*puVar11 == ut) && (t1 != 0)) {
              RD32(t0, &DAT_00b6d962);
              RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
              pcVar2 = FUN_004ddc10((int)((unsigned int)t1 + 0x17d));
              pcVar5 = FUN_004ddc10(0x17b);
              pcVar7 = FUN_004ddc10(0x179);
              _sprintf(DAT_00b69d20, s__s__s____s_0052d504, pcVar7, pcVar5, pcVar2);
              iVar12 = FUN_004dbfa0(DAT_00b69d20, 0xc);
              RD32(t0, &DAT_00b6d962);
              RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
              _sprintf(local_34, s_medlvl_d_0052b49c, t1);
              FUN_004b4cc0(local_34, local_64);
              iVar6 = (int)(((unsigned int)((int)((unsigned int)local_84[2] - (unsigned int)local_84[0]) >> 1)
                             - (unsigned int)((int)(((unsigned int)local_64[2] - (unsigned int)local_64[0]) + (unsigned int)iVar12 + 10u + 0xbu) >> 1))
                            + (unsigned int)local_84[0]);
              FUN_004db4e0(0xc, DAT_00b69d20, iVar6, local_84[1], 0xffff);
              FUN_004b5570(local_34, (int)((unsigned int)iVar12 + 10u + (unsigned int)iVar6 + 10u), local_84[1]);
              FUN_004d64a0(local_84, 0, 0xf);
            }
          }
        }
        else {
          iVar12 = 0;
          if (0 < DAT_00aa5aac) {
            pbVar21 = DAT_00b6c042;
            pcVar2 = DAT_00a91d24;
            do {
              if ((*pcVar2 != '\0') && ((unsigned int)(unsigned char)pcVar2[-0xc] == *puVar11)) {
                pcVar5 = pcVar2 + -100;
                uVar14 = 1;
                RD32(t0, pbVar21);
                pcVar7 = FUN_004ddc10((int)((unsigned int)t0 + 0x9a));
                uVar17 = 6;
                pcVar4 = FUN_004ddc10(0x2ae);
                _sprintf(DAT_00b69d20, s__c_s___c_s__c_s_0052d530, 2, pcVar4, uVar17, pcVar7,
                         uVar14, pcVar5);
                FUN_004db640(0xf, DAT_00b69d20, local_84, 0xffff);
                FUN_004d64a0(local_84, 0, 0x11);
              }
              pbVar21 = pbVar21 + 4;
              pcVar2 = pcVar2 + 0x562;
              iVar12 = (int)((unsigned int)iVar12 + 1u);
            } while (iVar12 < DAT_00aa5aac);
          }
        }
        puVar1 = puVar11 + 1;
        {
          int a, b;
          RD32(a, DAT_00b6bad6 + *puVar1 * 20u);
          RD32(b, DAT_00b6bad6 + *puVar11 * 20u);
          if (a < b) break;
        }
        puVar11 = puVar1;
        if (&DAT_0066da3c <= puVar1) break;
      }
    }
    else {
      puVar11 = &DAT_0066da18[0];
      while (*puVar11 != 0xffffffffu) {
        puVar8 = DAT_00aa47ba + *puVar11 * 0x1e5u;
        uVar17 = 1;
        pcVar2 = FUN_004ddc10(0x2ae);
        _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 2, pcVar2, uVar17, puVar8);
        FUN_004db640(0xf, DAT_00b69d20, local_84, 0xffff);
        FUN_004d64a0(local_84, 0, 0x11);
        RD32(ut, &DAT_00b6a5a6);
        RD32(t0, &DAT_00b6d962);
        RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
        if ((*puVar11 == ut) && (t1 != 0)) {
          RD32(t0, &DAT_00b6d962);
          RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
          pcVar2 = FUN_004ddc10((int)((unsigned int)t1 + 0x17d));
          pcVar5 = FUN_004ddc10(0x17b);
          pcVar7 = FUN_004ddc10(0x179);
          _sprintf(DAT_00b69d20, s__s__s____s_0052d504, pcVar7, pcVar5, pcVar2);
          iVar12 = FUN_004dbfa0(DAT_00b69d20, 0xc);
          RD32(t0, &DAT_00b6d962);
          RD32(t1, DAT_00b6da16 + (unsigned int)t0 * 0x109e0u);
          _sprintf(local_34, s_medlvl_d_0052b49c, t1);
          FUN_004b4cc0(local_34, local_64);
          iVar6 = (int)(((unsigned int)((int)((unsigned int)local_84[2] - (unsigned int)local_84[0]) >> 1)
                         - (unsigned int)((int)(((unsigned int)local_64[2] - (unsigned int)local_64[0]) + (unsigned int)iVar12 + 10u + 0xbu) >> 1))
                        + (unsigned int)local_84[0]);
          FUN_004db4e0(0xc, DAT_00b69d20, iVar6, local_84[1], 0xffff);
          FUN_004b5570(local_34, (int)((unsigned int)iVar12 + 10u + (unsigned int)iVar6 + 10u), local_84[1]);
          FUN_004d64a0(local_84, 0, 0xf);
        }
        puVar1 = puVar11 + 1;
        {
          int a, b;
          RD32(a, DAT_00b6bad6 + *puVar1 * 20u);
          RD32(b, DAT_00b6bad6 + *puVar11 * 20u);
          if (a < b) break;
        }
        puVar11 = puVar1;
        if ((unsigned int *)0x66da3b < puVar1) break;
      }
    }
  }
  FUN_004d64a0(local_84, 0, 8);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x152);
  FUN_004db640(0xc, pcVar2, local_84, uVar19);
  FUN_004d64a0(local_84, 0, 0xf);
  pcVar2 = FUN_004ddc10(0x154);
  uVar18 = 4;
  uVar15 = 1;
  RD32(iVar12, &DAT_00b6bace);
  pcVar5 = FUN_004ddc10(0x14f);
  uVar14 = 4;
  RD32(t0, &DAT_00b6baca);
  iVar6 = (int)((unsigned int)t0 + 1u);
  uVar17 = 1;
  pcVar7 = FUN_004ddc10(0x153);
  _sprintf(DAT_00b69d20, s__c_s__c_d_c__s__c_d_c__s_0052d514, 4, pcVar7, uVar17, iVar6, uVar14,
           pcVar5, uVar15, iVar12, uVar18, pcVar2);
  FUN_004db640(0xc, DAT_00b69d20, local_84, 0xffff);
  iVar6 = (int)((unsigned int)local_84[1] + 0xfu);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x148);
  FUN_004db4e0(0xc, pcVar2, 0x58, iVar6, uVar19);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(699);
  FUN_004db4e0(0xc, pcVar2, 0xee, iVar6, uVar19);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x13e);
  FUN_004db4e0(0xc, pcVar2, 0x134, iVar6, uVar19);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x13f);
  FUN_004db4e0(0xc, pcVar2, 0x15c, iVar6, uVar19);
  iVar12 = (int)((unsigned int)local_84[1] + 0x1eu);
  uVar19 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x140);
  FUN_004db4e0(0xc, pcVar2, 0x184, iVar6, uVar19);
  local_74 = 0;
  local_68 = 0;
  RD32(local_6c, DAT_00b6d84a + DAT_0066da18[0] * 0x1cu);
  iVar6 = 0;
  do {
    iVar13 = 0x58;
    iVar16 = *(int *)((char *)DAT_0066da18 + iVar6);
    if (iVar16 == -1) {
      return 1;
    }
    if (*(int *)((char *)DAT_0066d8a0 + iVar6) != 0) {
      RD32(t0, DAT_00b6bad6 + (unsigned int)iVar16 * 20u);
      if (t0 != local_6c) {
        local_74 = local_68;
        RD32(local_6c, DAT_00b6bad6 + (unsigned int)iVar16 * 20u);
      }
      local_70 = 2;
      if (2 < local_74) {
        local_70 = 1;
      }
      if (DAT_0066da4c == 0) {
        _sprintf(DAT_00b69d20, s__c_d___c_s_0052d4a0, local_70, (int)((unsigned int)local_74 + 1u), 1,
                 DAT_00aa47ba + (unsigned int)iVar16 * 0x1e5u);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar12, 0xffff);
        RDIDX(t0, DAT_00b6bad6);
        _sprintf(DAT_00b69d20, DAT_0052b460, t0);
        FUN_004db4e0(0xc, DAT_00b69d20, 0xee, iVar12, 0xffff);
        RDIDX(t0, DAT_00b6bada);
        if (t0 == 0) {
          _sprintf(DAT_00b69d20, DAT_0052b4d4);
        }
        else {
          _sprintf(DAT_00b69d20, DAT_0052b460, t0);
        }
        FUN_004db4e0(0xc, DAT_00b69d20, 0x134, iVar12, 0xffff);
        RDIDX(t0, DAT_00b6bade);
        if (t0 == 0) {
          _sprintf(DAT_00b69d20, DAT_0052b4d4);
        }
        else {
          _sprintf(DAT_00b69d20, DAT_0052b460, t0);
        }
        FUN_004db4e0(0xc, DAT_00b69d20, 0x15c, iVar12, 0xffff);
        RDIDX(t0, DAT_00b6bae2);
        if (t0 == 0) {
          _sprintf(DAT_00b69d20, DAT_0052b4d4);
        }
        else {
          _sprintf(DAT_00b69d20, DAT_0052b460, t0);
        }
        iVar13 = 0x67;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x184, iVar12, 0xffff);
        iVar12 = (int)((unsigned int)iVar12 + 0xfu);
      }
      else if ((DAT_0066d8d8[iVar16] != 0) && (local_88 = (char *)0, 0 < DAT_00aa5aac)) {
        pbVar22 = DAT_00b6c042;
        pcVar2 = DAT_00a91d24;
        iVar16 = iVar12;
        do {
          iVar12 = iVar16;
          if ((*pcVar2 != '\0') &&
              ((unsigned int)(unsigned char)pcVar2[-0xc] == *(unsigned int *)((char *)DAT_0066da18 + iVar6))) {
            pcVar5 = pcVar2 + -100;
            uVar17 = 1;
            RD32(t0, pbVar22);
            pcVar7 = FUN_004ddc10((int)((unsigned int)t0 + 0x9a));
            _sprintf(DAT_00b69d20, s__c_d___c_s__c_s_0052d490, local_70, (int)((unsigned int)local_74 + 1u), 6,
                     pcVar7, uVar17, pcVar5);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar13, iVar16, 0xffff);
            RDIDX(t0, DAT_00b6bad6);
            _sprintf(DAT_00b69d20, DAT_0052b460, t0);
            FUN_004db4e0(0xc, DAT_00b69d20, (int)((unsigned int)iVar13 + 0x96u), iVar16, 0xffff);
            RDIDX(t0, DAT_00b6bada);
            if (t0 == 0) {
              _sprintf(DAT_00b69d20, DAT_0052b4d4);
            }
            else {
              _sprintf(DAT_00b69d20, DAT_0052b460, t0);
            }
            iVar12 = (int)((unsigned int)iVar13 + 0x104u);
            FUN_004db4e0(0xc, DAT_00b69d20, (int)((unsigned int)iVar13 + 0xdcu), iVar16, 0xffff);
            RDIDX(t0, DAT_00b6bade);
            if (t0 == 0) {
              _sprintf(DAT_00b69d20, DAT_0052b4d4);
            }
            else {
              _sprintf(DAT_00b69d20, DAT_0052b460, t0);
            }
            iVar13 = (int)((unsigned int)iVar13 + 300u);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar12, iVar16, 0xffff);
            RDIDX(t0, DAT_00b6bae2);
            if (t0 == 0) {
              _sprintf(DAT_00b69d20, DAT_0052b4d4);
            }
            else {
              _sprintf(DAT_00b69d20, DAT_0052b460, t0);
            }
            iVar12 = (int)((unsigned int)iVar16 + 0xfu);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar13, iVar16, 0xffff);
          }
          pbVar22 = pbVar22 + 4;
          pcVar2 = pcVar2 + 0x562;
          local_88 = (char *)((unsigned int)local_88 + 1u);
          iVar16 = iVar12;
        } while ((int)local_88 < (int)DAT_00aa5aac);
      }
      if (DAT_0066d8d8[CUR_IDX] == 0) {
        local_8c = &DAT_0066d990[0];
        iVar16 = iVar12;
        do {
          iVar10 = *local_8c;
          iVar12 = iVar16;
          if (iVar10 == -1) break;
          RD32(t0, DAT_00b6d5a6 + (unsigned int)iVar10 * 0x58u);
          if ((unsigned int)DAT_00a91d18[(unsigned int)t0 * 0x562u] ==
              *(unsigned int *)((char *)DAT_0066da18 + iVar6)) {
            puVar20 = DAT_00b6d598 + (unsigned int)iVar10 * 0x16u;
            uVar17 = 1;
            if (DAT_0066da4c == 0) {
              RD32(t1, DAT_00b6d5ae + (unsigned int)iVar10 * 0x58u);
              pcVar2 = FUN_004ddc10((int)((unsigned int)t1 + 0x9a));
              _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar2);
              iVar12 = FUN_004d11e0();
              RD32(t2, DAT_00b6d5aa + (unsigned int)iVar10 * 0x58u);
              if ((iVar12 == t2) || (uVar19 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                uVar19 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeu) * 2u));
              }
            }
            else {
              RD32(t1, DAT_00b6d5ae + (unsigned int)iVar10 * 0x58u);
              pcVar2 = FUN_004ddc10((int)((unsigned int)t1 + 0x9a));
              _sprintf(DAT_00b69d20, s__c_d___c_s__c_s_0052d490, local_70, (int)((unsigned int)local_74 + 1u), 6,
                       pcVar2, uVar17, puVar20);
              FUN_004d6460(local_54, iVar13, iVar16, (int)((unsigned int)iVar13 + 0x94u),
                           (int)((unsigned int)iVar16 + 0xfu));
              FUN_004d4cf0(local_44);
              FUN_004d4d20(local_54);
              iVar12 = FUN_004d11e0();
              RD32(t2, DAT_00b6d5aa + (unsigned int)iVar10 * 0x58u);
              if ((iVar12 == t2) || (uVar19 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                uVar19 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeu) * 2u));
              }
              FUN_004db4e0(0xc, DAT_00b69d20, iVar13, iVar16, uVar19);
              FUN_004d4d20(local_44);
              RDIDX(t0, DAT_00b6bad6);
              _sprintf(DAT_00b69d20, DAT_0052b460, t0);
              FUN_004db4e0(0xc, DAT_00b69d20, (int)((unsigned int)iVar13 + 0x96u), iVar16, 0xffff);
              RDIDX(t0, DAT_00b6bada);
              if (t0 == 0) {
                _sprintf(DAT_00b69d20, DAT_0052b4d4);
              }
              else {
                _sprintf(DAT_00b69d20, DAT_0052b460, t0);
              }
              iVar12 = (int)((unsigned int)iVar13 + 0x104u);
              FUN_004db4e0(0xc, DAT_00b69d20, (int)((unsigned int)iVar13 + 0xdcu), iVar16, 0xffff);
              RDIDX(t0, DAT_00b6bade);
              if (t0 == 0) {
                _sprintf(DAT_00b69d20, DAT_0052b4d4);
              }
              else {
                _sprintf(DAT_00b69d20, DAT_0052b460, t0);
              }
              iVar13 = (int)((unsigned int)iVar13 + 300u);
              FUN_004db4e0(0xc, DAT_00b69d20, iVar12, iVar16, 0xffff);
              RDIDX(t0, DAT_00b6bae2);
              if (t0 == 0) {
                _sprintf(DAT_00b69d20, DAT_0052b4d4);
              }
              else {
                _sprintf(DAT_00b69d20, DAT_0052b460, t0);
              }
              uVar19 = 0xffff;
            }
            iVar12 = (int)((unsigned int)iVar16 + 0xfu);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar13, iVar16, uVar19);
            iVar13 = (int)((((unsigned int)0 - (unsigned int)(DAT_0066da4c == 0)) & 0xfu) + 0x58u);
          }
          local_8c = local_8c + 1;
          iVar16 = iVar12;
        } while (local_8c < &DAT_0066d9b0);
      }
      if (((*(int *)((char *)DAT_0066d968 + iVar6) != 0) && (DAT_0066da4c == 0)) &&
          (iVar16 = 0, 0 < DAT_00aa5aac)) {
        pbVar22 = DAT_00b6c042;
        local_88 = DAT_00a91d24;
        iVar10 = iVar12;
        do {
          iVar12 = iVar10;
          if ((*local_88 != '\0') &&
              ((unsigned int)(unsigned char)local_88[-0xc] == *(unsigned int *)((char *)DAT_0066da18 + iVar6))) {
            iVar9 = 0;
            pbVar23 = DAT_00b6d5a6;
            do {
              RD32(t0, pbVar23);
              if (t0 == iVar16) {
                RD32(t1, pbVar23 + 4);
                if (t1 != 0) break;
              }
              pbVar23 = pbVar23 + 0x58;
              iVar9 = (int)((unsigned int)iVar9 + 1u);
            } while (pbVar23 < &DAT_00b6d866);
            if (iVar9 == 8) {
              pcVar2 = local_88 + -100;
              uVar17 = 1;
              RD32(t0, pbVar22);
              pcVar5 = FUN_004ddc10((int)((unsigned int)t0 + 0x9a));
              _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar5, uVar17, pcVar2);
              iVar12 = (int)((unsigned int)iVar10 + 0xfu);
              FUN_004db4e0(0xc, DAT_00b69d20, iVar13, iVar10, 0xffff);
            }
          }
          pbVar22 = pbVar22 + 4;
          local_88 = local_88 + 0x562;
          iVar16 = (int)((unsigned int)iVar16 + 1u);
          iVar10 = iVar12;
        } while (iVar16 < DAT_00aa5aac);
      }
    }
    iVar6 = (int)((unsigned int)iVar6 + 4u);
    local_68 = (int)((unsigned int)local_68 + 1u);
    if (0x1f < iVar6) {
      return 1;
    }
  } while (1);
}
