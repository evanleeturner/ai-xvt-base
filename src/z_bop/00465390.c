/* SPDX-License-Identifier: MIT */
/* z_bop 00465390 */
extern int DAT_009a8c2c;
extern char *DAT_00a60a04;
extern unsigned char *DAT_00a60a10;
extern unsigned char *DAT_00a60a1c;
extern unsigned int DAT_00557360;
extern int DAT_00557364;
extern int DAT_00523420;
extern char DAT_00557368[];
extern int DAT_005242e0;
extern int DAT_0055978c;
extern unsigned char DAT_00556f48[];
extern unsigned char DAT_00557588[];
extern unsigned int DAT_00556f08[];
extern int DAT_00557788[];
extern int DAT_0055778c;
extern int DAT_00557790;
extern int DAT_00557794;

int FUN_004ac760(void);
void FUN_00448da0(void);
int FUN_0046f660(int param_1);
char *FUN_0046f720(unsigned char *param_1);
void FUN_00448f90(int param_1, unsigned char *param_2, int param_3);
void FUN_0046d300(unsigned int *param_1, int param_2);
int FUN_00417960(char *param_1, char *param_2);
void FUN_0046d350(int param_1, unsigned int *param_2, int param_3);
int FUN_004aa7f0(void);
unsigned int FUN_004aa870(void);
void FUN_00463d00(void);
void FUN_00463c50(int param_1);
void FUN_00448ed0(void);
int FUN_00417900(unsigned int *param_1, unsigned char *param_2, int param_3);
int FUN_004178f0(int param_1);
int FUN_004658c0(int param_1, int param_2);

unsigned int __cdecl FUN_00465390(int param_1, unsigned char *param_2, int param_3)
{
  char cVar1;
  int iVar2;
  char *pcVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  unsigned char *pbVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  int iVar13;
  unsigned int *puVar14;
  unsigned int *puVar15;
  char *pcVar16;
  unsigned char *pbVar17;
  unsigned int *puVar18;
  int local_124;
  int iStack_120;
  int iStack_11c;
  int local_118;
  unsigned int uStack_114;
  int iStack_110;
  int iStack_108;
  unsigned int local_104;
  unsigned int local_100[64];

  local_104 = 1;
  iVar2 = FUN_004ac760();
  local_118 = 0;
  DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar2);
  FUN_00448da0();
  uVar9 = 0xffffffff;
  pcVar4 = DAT_00a60a04;
  do {
    pcVar16 = pcVar4;
    if (uVar9 == 0) break;
    uVar9 = uVar9 - 1;
    pcVar16 = pcVar4 + 1;
    cVar1 = *pcVar4;
    pcVar4 = pcVar16;
  } while (cVar1 != '\0');
  uVar9 = ~uVar9;
  puVar14 = (unsigned int *)(pcVar16 + -uVar9);
  puVar15 = local_100;
  for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
    *puVar15 = *puVar14;
    puVar14 = puVar14 + 1;
    puVar15 = puVar15 + 1;
  }
  for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
    *(char *)puVar15 = (char)*puVar14;
    puVar14 = (unsigned int *)((int)puVar14 + 1);
    puVar15 = (unsigned int *)((int)puVar15 + 1);
  }
  pcVar4 = FUN_0046f720((unsigned char *)FUN_0046f660(param_1));
  if (pcVar4 != (char *)0x0) {
    char *pcScan;
    char *pcScanNext;
    uVar9 = 0xffffffff;
    do {
      pcVar16 = pcVar4;
      if (uVar9 == 0) break;
      uVar9 = uVar9 - 1;
      pcVar16 = pcVar4 + 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar16;
    } while (cVar1 != '\0');
    uVar9 = ~uVar9;
    iVar2 = -1;
    pcScan = (char *)local_100;
    do {
      pcScanNext = pcScan;
      if (iVar2 == 0) break;
      iVar2 = (int)((unsigned int)iVar2 - 1U);
      pcScanNext = pcScan + 1;
      cVar1 = *pcScan;
      pcScan = pcScanNext;
    } while (cVar1 != '\0');
    pcVar4 = pcVar16 + -uVar9;
    pcVar16 = pcScanNext - 1;
    for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
      *(unsigned int *)pcVar16 = *(unsigned int *)pcVar4;
      pcVar4 = pcVar4 + 4;
      pcVar16 = pcVar16 + 4;
    }
    for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
      *pcVar16 = *pcVar4;
      pcVar4 = pcVar4 + 1;
      pcVar16 = pcVar16 + 1;
    }
  }
  FUN_00448f90(1, (unsigned char *)local_100, 0x30);
  DAT_00557360 = 0x17;
  DAT_00557364 = param_1;
  FUN_0046d300(&DAT_00557360, 8);
  DAT_00557360 = 0x3d;
  DAT_00557364 = DAT_00523420;
  iVar2 = FUN_00417960(DAT_00557368, (char *)param_2);
  FUN_0046d350(param_1, &DAT_00557360, (int)((unsigned int)iVar2 + 8U));
  local_124 = 10;
  DAT_005242e0 = 1;
  uVar9 = local_100[0];
  do {
    iVar2 = 0;
    DAT_0055978c = 0;
    do {
      if (0xeb < iVar2) break;
      iVar5 = FUN_004aa7f0();
      if ((iVar5 != 0) && (uVar10 = FUN_004aa870(), uVar10 == 0x1b)) {
        local_124 = 1;
        iVar2 = 0xec;
        break;
      }
      iVar5 = DAT_009a8c2c;
      FUN_00463d00();
      iVar6 = FUN_004ac760();
      iVar2 = (int)((((unsigned int)iVar2 - (unsigned int)iVar5) + (unsigned int)DAT_009a8c2c) + (unsigned int)iVar6);
      DAT_009a8c2c = iVar5;
    } while (DAT_0055978c == 0);
    local_118 = (int)((unsigned int)local_118 + (unsigned int)iVar2);
    if (0xeb < local_118) {
      local_118 = 0;
      DAT_00557360 = 0x1d;
      FUN_0046d350(param_1, &DAT_00557360, 4);
      FUN_0046d300(&DAT_00557360, 4);
      uVar9 = (unsigned int)(uVar9 == 0);
      pbVar8 = DAT_00a60a10;
      if (uVar9 != 0) {
        pbVar8 = DAT_00a60a1c;
      }
      FUN_00448f90(3, pbVar8, 0x30);
    }
    if ((DAT_0055978c != 0) ||
        (local_124 = (int)((unsigned int)local_124 - 1U), local_124 == 0)) {
      if (DAT_0055978c == 0) {
        iVar2 = FUN_0046f660(param_1);
        FUN_00463c50(iVar2);
        FUN_00448ed0();
        FUN_004ac760();
        DAT_005242e0 = 0;
        return 0;
      }
      iVar2 = 0;
      iVar5 = FUN_00417900((unsigned int *)DAT_00556f48, param_2, param_3);
      puVar18 = DAT_00556f08;
      for (iVar6 = 0x10; iVar6 != 0; iVar6 = (int)((unsigned int)iVar6 - 1U)) {
        *puVar18 = 0;
        puVar18 = puVar18 + 1;
      }
      iStack_120 = 0;
      iVar6 = FUN_004178f0(param_3);
      DAT_00557788[0] = 0x3f;
      uVar10 = 0x1ec;
      DAT_0055778c = DAT_00523420;
      piVar7 = &DAT_00557794;
      DAT_00557790 = 0;
      iStack_108 = 0;
      if (0 < iVar5) {
        iStack_110 = 0;
        do {
          if (*(int *)(DAT_00556f48 + iStack_110) == *(int *)(DAT_00557588 + iStack_110)) {
            iStack_120 = (int)((unsigned int)iStack_120 + (unsigned int)iVar6);
          }
          else {
            iStack_11c = iVar6;
            if ((param_3 < (int)((unsigned int)iVar6 + (unsigned int)iStack_120)) &&
               (iStack_11c = (int)((unsigned int)param_3 - (unsigned int)iStack_120), iStack_11c < 0)) {
              iStack_11c = 0;
            }
            while (iStack_11c != 0) {
              uStack_114 = (unsigned int)iStack_11c + 8U;
              if ((int)uVar10 < (int)((unsigned int)iStack_11c + 8U)) {
                uStack_114 = uVar10;
              }
              *piVar7 = iStack_120;
              uVar11 = uStack_114 - 8;
              piVar7[1] = (int)uVar11;
              pbVar8 = param_2 + iStack_120;
              pbVar17 = (unsigned char *)(piVar7 + 2);
              for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                *(int *)pbVar17 = *(int *)pbVar8;
                pbVar8 = pbVar8 + 4;
                pbVar17 = pbVar17 + 4;
              }
              for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
                *pbVar17 = *pbVar8;
                pbVar8 = pbVar8 + 1;
                pbVar17 = pbVar17 + 1;
              }
              iStack_11c = (int)((unsigned int)iStack_11c - (unsigned int)piVar7[1]);
              iStack_120 = (int)((unsigned int)iStack_120 + (unsigned int)piVar7[1]);
              uVar10 = uVar10 - uStack_114;
              piVar7 = (int *)((int)piVar7 + uStack_114);
              if (uVar10 < 0x20) {
                *piVar7 = -1;
                iVar13 = (int)((unsigned int)iVar2 + 1U);
                FUN_0046d350(param_1,
                             (unsigned int *)(DAT_00557788 + (int)((unsigned int)iVar2 * 0x80U)),
                             (int)(0x1fcU - uVar10));
                if (iVar13 == 0x10) {
                  iVar2 = FUN_004658c0(param_1, 0x10);
                  if (iVar2 == 0) {
                    FUN_00448ed0();
                    FUN_004ac760();
                    DAT_005242e0 = 0;
                    return 0;
                  }
                  DAT_00557360 = 0x1d;
                  FUN_0046d300(&DAT_00557360, 4);
                  uVar9 = (unsigned int)(uVar9 == 0);
                  pbVar8 = DAT_00a60a10;
                  if (uVar9 != 0) {
                    pbVar8 = DAT_00a60a1c;
                  }
                  iVar13 = 0;
                  FUN_00448f90(3, pbVar8, 0x30);
                  puVar18 = DAT_00556f08;
                  for (iVar2 = 0x10; iVar2 != 0; iVar2 = (int)((unsigned int)iVar2 - 1U)) {
                    *puVar18 = 0;
                    puVar18 = puVar18 + 1;
                  }
                }
                uVar10 = 0x1ec;
                DAT_00557788[(int)((unsigned int)iVar13 * 0x80U)] = 0x3f;
                piVar7 = &DAT_00557794 + (int)((unsigned int)iVar13 * 0x80U);
                (&DAT_0055778c)[(int)((unsigned int)iVar13 * 0x80U)] = DAT_00523420;
                (&DAT_00557790)[(int)((unsigned int)iVar13 * 0x80U)] = iVar13;
                iVar2 = iVar13;
              }
            }
          }
          iStack_110 = (int)((unsigned int)iStack_110 + 4U);
          iStack_108 = (int)((unsigned int)iStack_108 + 1U);
        } while (iStack_108 < iVar5);
      }
      if (uVar10 < 0x1f0) {
        *piVar7 = -1;
        FUN_0046d350(param_1,
                     (unsigned int *)(DAT_00557788 + (int)((unsigned int)iVar2 * 0x80U)),
                     (int)(0x1fcU - uVar10));
        iVar2 = FUN_004658c0(param_1, (int)((unsigned int)iVar2 + 1U));
        if (iVar2 == 0) {
          local_104 = 0;
        }
      }
      FUN_00448ed0();
      FUN_004ac760();
      DAT_005242e0 = 0;
      return local_104;
    }
  } while (1);
}
