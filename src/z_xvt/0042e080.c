/* SPDX-License-Identifier: MIT */
/* z_xvt 0042e080 */
/* Globals (types inferred from the decompile) */
extern int DAT_006db5a8;
extern unsigned char DAT_0071bce5;   /* packed: unaligned int at +idx*0x5bd */
extern int DAT_0071ecf4;
extern char *DAT_006a89a4;
extern unsigned int DAT_005373d0;
extern int DAT_005373d4;
extern int DAT_005373d8;
extern int DAT_00525560;
extern int DAT_005250d8;
extern int DAT_005397fc;
extern unsigned char *DAT_006a89b0;
extern unsigned char *DAT_006a89bc;
extern unsigned char DAT_00536fb8;
extern unsigned int DAT_00536f78;
extern unsigned char DAT_005375f8;
extern int DAT_005377f8;
extern int DAT_005377fc;
extern int DAT_00537800;
extern int DAT_00537804;

/* Prototypes */
int __cdecl FUN_0048f530(void);
void __cdecl FUN_004a7510(void);
void __cdecl FUN_00438a40(void);
int __cdecl FUN_00432c10(int param_1);
int * __cdecl FUN_00432cc0(int param_1);
void __cdecl FUN_00438c30(int param_1, unsigned char *param_2, int param_3);
void __cdecl FUN_004310c0(unsigned int *param_1, int param_2);
int __cdecl FUN_0049a1f0(int *param_1, char *param_2);
int __cdecl FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3);
int __cdecl FUN_004a53f0(void);
unsigned int __cdecl FUN_004a5470(void);
void __stdcall FUN_0042c710(void);
void __cdecl FUN_00438b70(void);
void __cdecl FUN_004a74f0(void);
int __cdecl FUN_0049a190(unsigned int *param_1, unsigned char *param_2, int param_3);
int __cdecl FUN_0049a180(int param_1);
int __cdecl FUN_0042e660(int param_1, int param_2);

unsigned int __cdecl FUN_0042e080(int param_1, unsigned char *param_2, int param_3)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  unsigned char *pbVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  int iVar11;
  unsigned int *puVar12;
  char *pcVar13;
  int *piVar14;
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
  int iFlag;

  local_104 = 1;
  iVar2 = FUN_0048f530();
  DAT_006db5a8 = (int)((unsigned int)DAT_006db5a8 + (unsigned int)iVar2);
  __builtin_memcpy(&iFlag, (unsigned char *)&DAT_0071bce5 + (unsigned int)((unsigned int)DAT_0071ecf4 * 0x5bdU), 4);
  if (iFlag != 0) {
    FUN_004a7510();
  }
  local_118 = 0;
  FUN_00438a40();
  uVar7 = 0xffffffff;
  pcVar13 = DAT_006a89a4;
  do {
    pcVar16 = pcVar13;
    if (uVar7 == 0) break;
    uVar7 = uVar7 - 1;
    pcVar16 = pcVar13 + 1;
    cVar1 = *pcVar13;
    pcVar13 = pcVar16;
  } while (cVar1 != '\0');
  uVar7 = ~uVar7;
  puVar12 = (unsigned int *)(pcVar16 + -uVar7);
  puVar15 = local_100;
  for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
    __builtin_memcpy(puVar15, puVar12, 4);
    puVar12 = puVar12 + 1;
    puVar15 = puVar15 + 1;
  }
  for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
    *(char *)puVar15 = *(char *)puVar12;
    puVar12 = (unsigned int *)((int)puVar12 + 1);
    puVar15 = (unsigned int *)((int)puVar15 + 1);
  }
  iVar2 = FUN_00432c10(param_1);
  piVar3 = FUN_00432cc0(iVar2);
  if (piVar3 != (int *)0) {
    uVar7 = 0xffffffff;
    do {
      piVar14 = piVar3;
      if (uVar7 == 0) break;
      uVar7 = uVar7 - 1;
      piVar14 = (int *)((int)piVar3 + 1);
      iVar2 = *(char *)piVar3;
      piVar3 = piVar14;
    } while ((char)iVar2 != '\0');
    uVar7 = ~uVar7;
    iVar2 = -1;
    puVar12 = local_100;
    do {
      puVar15 = puVar12;
      if (iVar2 == 0) break;
      iVar2 = iVar2 + -1;
      puVar15 = (unsigned int *)((int)puVar12 + 1);
      uVar8 = *(unsigned char *)puVar12;
      puVar12 = puVar15;
    } while ((char)uVar8 != '\0');
    pcVar13 = (char *)((int)piVar14 - uVar7);
    pcVar16 = (char *)((int)puVar15 + -1);
    for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
      __builtin_memcpy(pcVar16, pcVar13, 4);
      pcVar13 = pcVar13 + 4;
      pcVar16 = pcVar16 + 4;
    }
    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
      *pcVar16 = *pcVar13;
      pcVar13 = pcVar13 + 1;
      pcVar16 = pcVar16 + 1;
    }
  }
  FUN_00438c30(1, (unsigned char *)local_100, 0x30);
  DAT_005373d0 = 0x17;
  DAT_005373d4 = param_1;
  FUN_004310c0(&DAT_005373d0, 8);
  DAT_005373d0 = 0x3d;
  DAT_005373d4 = DAT_00525560;
  iVar2 = FUN_0049a1f0(&DAT_005373d8, (char *)param_2);
  FUN_00431110(param_1, &DAT_005373d0, (unsigned int)(iVar2 + 8));
  local_124 = 10;
  DAT_005250d8 = 1;
  uVar7 = local_100[0];
  do {
    iVar2 = 0;
    DAT_005397fc = 0;
    do {
      if (0xeb < iVar2) break;
      iVar4 = FUN_004a53f0();
      if ((iVar4 != 0) && (uVar8 = FUN_004a5470(), uVar8 == 0x1b)) {
        local_124 = 1;
        iVar2 = 0xec;
        break;
      }
      iVar4 = DAT_006db5a8;
      FUN_0042c710();
      iVar5 = FUN_0048f530();
      iVar2 = (int)((((unsigned int)iVar2 - (unsigned int)iVar4) + (unsigned int)DAT_006db5a8) + (unsigned int)iVar5);
      DAT_006db5a8 = iVar4;
    } while (DAT_005397fc == 0);
    local_118 = (int)((unsigned int)local_118 + (unsigned int)iVar2);
    if (0xeb < local_118) {
      local_118 = 0;
      DAT_005373d0 = 0x1d;
      FUN_00431110(param_1, &DAT_005373d0, 4);
      FUN_004310c0(&DAT_005373d0, 4);
      uVar7 = (unsigned int)(uVar7 == 0);
      pbVar6 = DAT_006a89b0;
      if (uVar7 != 0) {
        pbVar6 = DAT_006a89bc;
      }
      FUN_00438c30(3, pbVar6, 0x30);
    }
    if ((DAT_005397fc != 0) || (local_124 = local_124 + -1, local_124 == 0)) {
      if (DAT_005397fc == 0) {
        DAT_005373d4 = FUN_00432c10(param_1);
        DAT_005373d0 = 0x15;
        FUN_004310c0(&DAT_005373d0, 8);
        FUN_00438b70();
        FUN_0048f530();
        __builtin_memcpy(&iFlag, (unsigned char *)&DAT_0071bce5 + (unsigned int)((unsigned int)DAT_0071ecf4 * 0x5bdU), 4);
        if (iFlag != 0) {
          FUN_004a74f0();
        }
        DAT_005250d8 = 0;
        return 0;
      }
      iVar2 = 0;
      iVar4 = FUN_0049a190((unsigned int *)&DAT_00536fb8, param_2, param_3);
      puVar18 = &DAT_00536f78;
      for (iVar5 = 0x10; iVar5 != 0; iVar5 = iVar5 + -1) {
        *puVar18 = 0;
        puVar18 = puVar18 + 1;
      }
      iStack_120 = 0;
      iVar5 = FUN_0049a180(param_3);
      DAT_005377f8 = 0x3f;
      uVar8 = 0x1ec;
      DAT_005377fc = DAT_00525560;
      piVar3 = &DAT_00537804;
      DAT_00537800 = 0;
      iStack_108 = 0;
      if (0 < iVar4) {
        iStack_110 = 0;
        do {
          if (*(int *)((unsigned char *)&DAT_00536fb8 + iStack_110) ==
              *(int *)((unsigned char *)&DAT_005375f8 + iStack_110)) {
            iStack_120 = (int)((unsigned int)iStack_120 + (unsigned int)iVar5);
          }
          else {
            iStack_11c = iVar5;
            if ((param_3 < (int)((unsigned int)iVar5 + (unsigned int)iStack_120)) &&
                (iStack_11c = (int)((unsigned int)param_3 - (unsigned int)iStack_120), iStack_11c < 0)) {
              iStack_11c = 0;
            }
            while (iStack_11c != 0) {
              uStack_114 = (unsigned int)iStack_11c + 8U;
              if ((int)uVar8 < (int)((unsigned int)iStack_11c + 8U)) {
                uStack_114 = uVar8;
              }
              *piVar3 = iStack_120;
              uVar9 = uStack_114 - 8;
              piVar3[1] = (int)uVar9;
              pbVar6 = param_2 + iStack_120;
              pbVar17 = (unsigned char *)(piVar3 + 2);
              for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                __builtin_memcpy(pbVar17, pbVar6, 4);
                pbVar6 = pbVar6 + 4;
                pbVar17 = pbVar17 + 4;
              }
              for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                *pbVar17 = *pbVar6;
                pbVar6 = pbVar6 + 1;
                pbVar17 = pbVar17 + 1;
              }
              iStack_11c = (int)((unsigned int)iStack_11c - (unsigned int)piVar3[1]);
              iStack_120 = (int)((unsigned int)iStack_120 + (unsigned int)piVar3[1]);
              uVar8 = uVar8 - uStack_114;
              piVar3 = (int *)((int)piVar3 + uStack_114);
              if (uVar8 < 0x20) {
                *piVar3 = -1;
                iVar11 = iVar2 + 1;
                FUN_00431110(param_1, (unsigned int *)(&DAT_005377f8 + iVar2 * 0x80), 0x1fc - uVar8);
                if (iVar11 == 0x10) {
                  iVar2 = FUN_0042e660(param_1, 0x10);
                  if (iVar2 == 0) {
                    FUN_00438b70();
                    FUN_0048f530();
                    __builtin_memcpy(&iFlag, (unsigned char *)&DAT_0071bce5 + (unsigned int)((unsigned int)DAT_0071ecf4 * 0x5bdU), 4);
                    if (iFlag != 0) {
                      FUN_004a74f0();
                    }
                    DAT_005250d8 = 0;
                    return 0;
                  }
                  DAT_005373d0 = 0x1d;
                  FUN_004310c0(&DAT_005373d0, 4);
                  uVar7 = (unsigned int)(uVar7 == 0);
                  pbVar6 = DAT_006a89b0;
                  if (uVar7 != 0) {
                    pbVar6 = DAT_006a89bc;
                  }
                  iVar11 = 0;
                  FUN_00438c30(3, pbVar6, 0x30);
                  puVar18 = &DAT_00536f78;
                  for (iVar2 = 0x10; iVar2 != 0; iVar2 = iVar2 + -1) {
                    *puVar18 = 0;
                    puVar18 = puVar18 + 1;
                  }
                }
                uVar8 = 0x1ec;
                (&DAT_005377f8)[iVar11 * 0x80] = 0x3f;
                piVar3 = &DAT_00537804 + iVar11 * 0x80;
                (&DAT_005377fc)[iVar11 * 0x80] = DAT_00525560;
                (&DAT_00537800)[iVar11 * 0x80] = iVar11;
                iVar2 = iVar11;
              }
            }
          }
          iStack_110 = iStack_110 + 4;
          iStack_108 = iStack_108 + 1;
        } while (iStack_108 < iVar4);
      }
      if (uVar8 < 0x1f0) {
        *piVar3 = -1;
        FUN_00431110(param_1, (unsigned int *)(&DAT_005377f8 + iVar2 * 0x80), 0x1fc - uVar8);
        iVar2 = FUN_0042e660(param_1, iVar2 + 1);
        if (iVar2 == 0) {
          local_104 = 0;
        }
      }
      FUN_00438b70();
      FUN_0048f530();
      __builtin_memcpy(&iFlag, (unsigned char *)&DAT_0071bce5 + (unsigned int)((unsigned int)DAT_0071ecf4 * 0x5bdU), 4);
      if (iFlag != 0) {
        FUN_004a74f0();
      }
      DAT_005250d8 = 0;
      return local_104;
    }
  } while (1);
}
