/* SPDX-License-Identifier: MIT */
/* z_xvt 004f4720 */
extern int DAT_00757870[];
extern int DAT_007579d0[];
extern unsigned char DAT_0081bfb6[4]; /* 4-byte read at unaligned address 0x0081bfb6 */
extern int DAT_0081b850;
extern short DAT_00650406;
extern int DAT_00650438;
extern int DAT_0065043c;
extern char DAT_0085e7c5;

extern char s_jewelsound_00531990[];
extern char s_map4u_00533ecc[];
extern char s_map4d_00533ed4[];
extern char s_map2d_00533ec4[];
extern char s_map2u_00533ebc[];
extern char s_map1u_00533eac[];
extern char s_map1d_00533eb4[];
extern char s_clearu_005337ec[];
extern char s_cleard_005337f4[];
extern char s_assignu_005337dc[];
extern char s_assignd_005337e4[];

int FUN_004cdbc0(void);
void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004ddc70(int *param_1, int *param_2);
int FUN_004dd7d0(int *param_1, int param_2, int param_3);
unsigned char FUN_004db530(void);
unsigned char FUN_004db550(void);
unsigned char FUN_004db570(void);
unsigned char FUN_004db590(void);
void FUN_004dd030(int *param_1, int param_2, int param_3);
void FUN_004d9ea0(int *param_1);
char *FUN_004dcfa0(int param_1);
int FUN_004d9a80(int *param_1, char *param_2, char *param_3, char *param_4, int param_5,
                 int param_6, int param_7, char *param_8);
void FUN_004f2c10(void);
void FUN_004d9d20(int *param_1, char *param_2, char *param_3, int param_4);
void FUN_004de190(char *param_1, int param_2, int param_3, int param_4);
void FUN_004f2d10(void);
void FUN_004f5ac0(void);
void FUN_004f5990(void);

unsigned int FUN_004f4720(unsigned int param_1, int param_2)
{
  int iVar2;
  char *pcVar3;
  int *piVar4;
  int bVar5;
  int bVar6;
  unsigned int uVar7;
  int iVar8;
  char *pcVar9;
  int local_34[4];
  int local_24;
  int local_20[8];
  int idx;

  __builtin_memcpy(&idx, DAT_0081bfb6, 4);

  iVar2 = FUN_004cdbc0();
  bVar5 = iVar2 != DAT_00757870[(int)((unsigned int)idx * 8u)];
  bVar6 = DAT_0081b850 == 2;
  local_20[1] = 1;
  local_20[2] = 1;
  local_20[3] = 0;
  local_20[4] = 1;
  local_20[5] = 0;
  if ((!bVar6 && bVar5) || (DAT_007579d0[idx] < 2)) {
    local_20[6] = 0;
    local_20[7] = 0;
  }
  else {
    local_20[6] = 1;
    local_20[7] = 1;
  }
  local_20[(short)(DAT_00650406 ^ 1) + 1] = 2;
  FUN_004dcfd0(local_34, 0x16, 0xc6, 0x2a, 0xde);
  FUN_004ddc70(local_20, &local_24);
  iVar2 = FUN_004dd7d0(local_34, local_20[0], local_24);
  if ((iVar2 != 0) &&
      ((FUN_004db530() != 0 || FUN_004db550() != 0) ||
       FUN_004db570() != 0 || FUN_004db590() != 0)) {
    local_20[4] = 2;
  }
  if ((bVar6 || !bVar5) && (local_20[6] != 0)) {
    FUN_004dcfd0(local_34, 0x16, 0x132, 0x2a, 0x14a);
    piVar4 = local_20 + 6;
    do {
      iVar2 = FUN_004dd7d0(local_34, local_20[0], local_24);
      if ((iVar2 != 0) &&
          ((FUN_004db530() != 0 || FUN_004db550() != 0) ||
           (FUN_004db570() != 0 || FUN_004db590() != 0))) {
        *piVar4 = 2;
      }
      piVar4 = piVar4 + 1;
      FUN_004dd030(local_34, 0, 0x1c);
    } while (piVar4 < local_20 + 8);
  }
  FUN_004d9ea0(local_20 + 1);
  FUN_004dcfd0(local_34, 0x16, 0xc6, 0x2a, 0xde);
  pcVar9 = s_jewelsound_00531990;
  iVar8 = 0xe;
  uVar7 = 0;
  iVar2 = 0xc;
  pcVar3 = FUN_004dcfa0(0xc5);
  iVar2 = FUN_004d9a80(local_34, s_map4u_00533ecc, s_map4d_00533ed4, pcVar3, iVar2, uVar7, iVar8, pcVar9);
  if (iVar2 != 0) {
    DAT_00650438 = 0;
    DAT_0065043c = 0;
    FUN_004f2c10();
  }
  FUN_004dd030(local_34, 0, -0x38);
  if (DAT_00650406 == 0) {
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0xc3);
    FUN_004d9d20(local_34, s_map2d_00533ec4, pcVar3, iVar2);
  }
  else {
    pcVar9 = s_jewelsound_00531990;
    iVar8 = 0xc;
    uVar7 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0xc3);
    iVar2 = FUN_004d9a80(local_34, s_map2u_00533ebc, s_map2u_00533ebc, pcVar3, iVar2, uVar7, iVar8, pcVar9);
    if (iVar2 != 0) {
      DAT_00650406 = 0;
    }
  }
  FUN_004dd030(local_34, 0, -0x1c);
  if (DAT_00650406 == 0) {
    pcVar9 = s_jewelsound_00531990;
    iVar8 = 0xb;
    uVar7 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x223);
    iVar2 = FUN_004d9a80(local_34, s_map1u_00533eac, s_map1u_00533eac, pcVar3, iVar2, uVar7, iVar8, pcVar9);
    if (iVar2 != 0) {
      DAT_00650406 = 1;
    }
  }
  else {
    iVar2 = FUN_004dd7d0(local_34, local_20[0], local_24);
    if ((iVar2 != 0) &&
        (FUN_004db570() != 0 || FUN_004db590() != 0)) {
      if (DAT_0085e7c5 != '\0') {
        FUN_004de190(s_jewelsound_00531990, 1, 0, 0xff);
      }
      FUN_004f2d10();
    }
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0xc4);
    FUN_004d9d20(local_34, s_map1d_00533eb4, pcVar3, iVar2);
  }
  if ((bVar6 || !bVar5) && (local_20[6] != 0)) {
    FUN_004dcfd0(local_34, 0x16, 0x14e, 0x2a, 0x166);
    pcVar9 = s_jewelsound_00531990;
    iVar8 = 0xf;
    uVar7 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0xca);
    iVar2 = FUN_004d9a80(local_34, s_clearu_005337ec, s_cleard_005337f4, pcVar3, iVar2, uVar7, iVar8, pcVar9);
    if (iVar2 != 0) {
      FUN_004f5ac0();
    }
    FUN_004dd030(local_34, 0, -0x1c);
    pcVar9 = s_jewelsound_00531990;
    iVar8 = 0x11;
    uVar7 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0xc9);
    iVar2 = FUN_004d9a80(local_34, s_assignu_005337dc, s_assignd_005337e4, pcVar3, iVar2, uVar7, iVar8, pcVar9);
    if (iVar2 != 0) {
      FUN_004f5990();
    }
  }
  return 0;
}
