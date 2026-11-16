/* SPDX-License-Identifier: MIT */
/* z_xvt 0045d2a0 */
#define U(x) ((unsigned int)(x))

extern int DAT_006a8858;
extern unsigned int *DAT_006a8866; /* unaligned: address 0x6a8866 */
extern int DAT_006a884a;           /* unaligned: address 0x6a884a */
extern int DAT_006a8874;
extern unsigned int DAT_006d4984;  /* width unresolved; Ghidra shows (uint) cast */
extern int DAT_005f10cc;
extern int DAT_005f10dc;
extern int DAT_005f10b8;
extern int DAT_005f10d8;
extern int DAT_005f10e0;
extern int DAT_005f10c8;
extern int DAT_0052ae1c;
extern int DAT_005279f4;
extern int DAT_005f0fac;
extern int DAT_005f0fb0;
extern int DAT_005f102c;
extern int DAT_005f1030;
extern int DAT_005f1034;

void __cdecl FUN_0045ed70(int param_1, int param_2, int param_3);
void __cdecl FUN_0045d7d0(int param_1, int param_2, int param_3);
void __cdecl FUN_0045dd00(int param_1, int param_2, int param_3);
void __cdecl FUN_0045e290(int param_1, int param_2, int param_3);
void __cdecl FUN_0045e7d0(int param_1, int param_2, int param_3);
int __cdecl FUN_0045f410(int param_1, int param_2);
int *__cdecl FUN_0045c050(int param_1, int param_2, int param_3, byte *param_4);
/* x87 input (ST0) is not visible in the decompile */
long long __cdecl __ftol(void);

void __cdecl FUN_0045d2a0(int param_1)
{
  int iVar1;
  float *pfVar2;
  unsigned int *puVar3;
  unsigned int *puVar4;
  int iVar5;
  unsigned int uVar6;
  byte *pbVar7;
  int iVar8;
  int *piVar9;
  int iVar10;
  int iVar11;
  int *piVar12;
  int iVar13;
  unsigned int *puVar14;
  int iVar15;
  long long lVar16;
  int local_18;
  int *local_14;
  int local_c;
  byte *local_8;
  int *local_4;
  int iTmp;

  __builtin_memcpy(&puVar3, &DAT_006a8866, sizeof(puVar3));
  iVar8 = 0;
  local_8 = (byte *)0;
  iVar1 = (int)(U(DAT_006a8858) + U(*(int *)(param_1 + 0xbc)) * 0x18u);
  DAT_005f10cc = (int)(U(*(int *)(param_1 + 200)) + U(*(int *)(param_1 + 0xbc)));
  __builtin_memcpy(&iTmp, &DAT_006a884a, sizeof(iTmp));
  local_14 = (int *)(int)(U(*(int *)(param_1 + 0xb8)) * 0x70u + U(iTmp));
  __builtin_memcpy(&puVar4, &DAT_006a8866, sizeof(puVar4));
  DAT_005f10dc = DAT_005f10cc;
  if (0 < DAT_005f10cc) {
    do {
      *puVar4 = 0xffffffff;
      iVar8 = (int)(U(iVar8) + 1u);
      puVar4 = puVar4 + 1;
    } while (iVar8 < DAT_005f10cc);
  }
  local_c = 0;
  if (0 < *(int *)(param_1 + 0xc4)) {
    do {
      iVar8 = (int)(U(*local_14) * 0x40u + U(*(int *)(param_1 + 0xa0)));
      DAT_005f10b8 = (int)(4u - U((int)(*(int *)(iVar8 + 0x1c) == -1)));
      if (0 < DAT_005f10b8) {
        piVar9 = (int *)(int)(U(iVar8) + 0x20u);
        piVar12 = &DAT_005f1030;
        local_18 = DAT_005f10b8;
        do {
          iVar8 = *(int *)(int)(U(DAT_006a8874) + U(piVar9[-8]) * 4u);
          *piVar12 = iVar8;
          iVar5 = DAT_005f10dc;
          pfVar2 = (float *)(int)(U(*(int *)(param_1 + 0x80)) + U(*piVar9) * 8u);
          puVar4 = (unsigned int *)(int)(U(iVar1) + U(iVar8) * 0x18u);
          if ((*(float *)(int)(U(iVar1) + 0x10u + U(iVar8) * 0x18u) != *pfVar2) ||
              (*(float *)&puVar4[5] != pfVar2[1])) {
            iVar8 = (int)(U(DAT_005f10dc) * 0x18u);
            DAT_005f10dc = (int)(U(DAT_005f10dc) + 1u);
            *(unsigned int *)(int)(U(iVar1) + U(iVar8)) = *puVar4;
            iVar8 = (int)(U(iVar1) + U(iVar5) * 0x18u);
            *(unsigned int *)(int)(U(iVar8) + 4u) = puVar4[1];
            *(unsigned int *)(int)(U(iVar8) + 0xcu) = puVar4[3];
            *(unsigned int *)(int)(U(iVar8) + 8u) = puVar4[2];
            *(unsigned int *)(int)(U(iVar8) + 0x10u) =
                *(unsigned int *)(int)(U(*(int *)(param_1 + 0x80)) + U(*piVar9) * 8u);
            *(unsigned int *)(int)(U(iVar8) + 0x14u) =
                *(unsigned int *)(int)(U(*(int *)(param_1 + 0x80)) + 4u + U(*piVar9) * 8u);
            *piVar12 = iVar5;
          }
          piVar9 = piVar9 + 1;
          piVar12 = piVar12 + 1;
          local_18 = (int)(U(local_18) - 1u);
        } while (local_18 != 0);
      }
      if (local_14[3] == -1) {
        if (0 < DAT_005f10b8) {
          puVar4 = (unsigned int *)&DAT_005f1030;
          puVar14 = (unsigned int *)&DAT_005f0fb0;
          for (iVar8 = DAT_005f10b8; iVar8 != 0; iVar8 = (int)(U(iVar8) - 1u)) {
            *puVar14 = *puVar4;
            puVar4 = puVar4 + 1;
            puVar14 = puVar14 + 1;
          }
        }
        DAT_005f10d8 = DAT_005f10b8;
        iVar8 = 0;
        DAT_005f10b8 = 0;
        if (0 < DAT_005f10d8) {
          piVar9 = &DAT_005f0fb0;
          iVar5 = ((int *)&DAT_005f0fac)[DAT_005f10d8];
          do {
            iVar15 = *piVar9;
            piVar9 = piVar9 + 1;
            iVar8 = (int)(U(iVar8) + 1u);
            FUN_0045ed70(iVar5, iVar15, iVar1);
            iVar5 = iVar15;
          } while (iVar8 < DAT_005f10d8);
        }
      }
      iVar8 = 0;
      local_14[3] = (int)DAT_006d4984;
      DAT_005f10d8 = 0;
      if (0 < DAT_005f10b8) {
        piVar9 = &DAT_005f1030;
        iVar5 = ((int *)&DAT_005f102c)[DAT_005f10b8];
        do {
          iVar15 = *piVar9;
          piVar9 = piVar9 + 1;
          iVar8 = (int)(U(iVar8) + 1u);
          FUN_0045d7d0(iVar5, iVar15, iVar1);
          iVar5 = iVar15;
        } while (iVar8 < DAT_005f10b8);
      }
      iVar8 = 0;
      DAT_005f10b8 = 0;
      if (0 < DAT_005f10d8) {
        piVar9 = &DAT_005f0fb0;
        iVar5 = ((int *)&DAT_005f0fac)[DAT_005f10d8];
        do {
          iVar15 = *piVar9;
          piVar9 = piVar9 + 1;
          iVar8 = (int)(U(iVar8) + 1u);
          FUN_0045dd00(iVar5, iVar15, iVar1);
          iVar5 = iVar15;
        } while (iVar8 < DAT_005f10d8);
      }
      iVar8 = 0;
      DAT_005f10d8 = 0;
      if (0 < DAT_005f10b8) {
        piVar9 = &DAT_005f1030;
        iVar5 = ((int *)&DAT_005f102c)[DAT_005f10b8];
        do {
          iVar15 = *piVar9;
          piVar9 = piVar9 + 1;
          iVar8 = (int)(U(iVar8) + 1u);
          FUN_0045e290(iVar5, iVar15, iVar1);
          iVar5 = iVar15;
        } while (iVar8 < DAT_005f10b8);
      }
      iVar8 = 0;
      DAT_005f10b8 = 0;
      if (0 < DAT_005f10d8) {
        piVar9 = &DAT_005f0fb0;
        iVar5 = ((int *)&DAT_005f0fac)[DAT_005f10d8];
        do {
          iVar15 = *piVar9;
          piVar9 = piVar9 + 1;
          iVar8 = (int)(U(iVar8) + 1u);
          FUN_0045e7d0(iVar5, iVar15, iVar1);
          iVar5 = iVar15;
        } while (iVar8 < DAT_005f10d8);
      }
      iVar8 = 0;
      if (0 < DAT_005f10b8) {
        piVar9 = &DAT_005f1030;
        do {
          iVar5 = *piVar9;
          if (iVar5 < DAT_005f10cc) {
            if (puVar3[iVar5] == 0xffffffff) {
              uVar6 = (unsigned int)FUN_0045f410(iVar5, iVar1);
              puVar3[iVar5] = uVar6;
            }
            *piVar9 = (int)puVar3[*piVar9];
          }
          else {
            iVar5 = FUN_0045f410(iVar5, iVar1);
            *piVar9 = iVar5;
          }
          piVar9 = piVar9 + 1;
          iVar8 = (int)(U(iVar8) + 1u);
        } while (iVar8 < DAT_005f10b8);
      }
      if (2 < DAT_005f10b8) {
        iVar11 = 0;
        iVar8 = *(int *)(param_1 + 0xac);
        iVar5 = *(int *)(param_1 + 0xa8);
        iVar15 = *(int *)(iVar5 + 0x14);
        iVar13 = *(int *)(iVar5 + 0x10);
        if ((int)(U(iVar15) * U(iVar13) - U(*(int *)(iVar5 + 8))) == 0) {
          lVar16 = __ftol();
          iVar5 = (int)lVar16;
          for (; ((0x100 < iVar5 && (iVar13 != 8)) && (iVar15 != 8)); iVar15 = iVar15 >> 1) {
            iVar5 = iVar5 >> 2;
            iVar10 = (int)(U(iVar15) * U(iVar13));
            iVar13 = iVar13 >> 1;
            iVar11 = (int)(U(iVar11) + U(iVar10));
          }
        }
        pbVar7 = (byte *)(int)(U(iVar8) + U(iVar11));
        if (local_8 != pbVar7) {
          local_4 = FUN_0045c050(iVar13, iVar15, (int)(U(*(int *)(param_1 + 0xb4)) + 0x1000u), pbVar7);
          local_8 = pbVar7;
        }
      }
      iVar8 = 2;
      if (2 < DAT_005f10b8) {
        puVar4 = (unsigned int *)&DAT_005f1034;
        do {
          *(int *)(int)(U(DAT_005f10e0) * 0x14u + U(DAT_005f10c8)) = DAT_005f1030;
          *(unsigned int *)(int)(U(DAT_005f10e0) * 0x14u + 4u + U(DAT_005f10c8)) = *puVar4;
          *(unsigned int *)(int)(U(DAT_005f10e0) * 0x14u + 8u + U(DAT_005f10c8)) = puVar4[1];
          *(int **)(int)(U(DAT_005f10e0) * 0x14u + 0xcu + U(DAT_005f10c8)) = local_4;
          *(int *)(int)(U(DAT_005f10e0) * 0x14u + 0x10u + U(DAT_005f10c8)) = 0x3023;
          if (DAT_0052ae1c != 0) {
            piVar9 = (int *)(int)(U(DAT_005f10e0) * 0x14u + 0x10u + U(DAT_005f10c8));
            *piVar9 = (int)(U(*piVar9) + 0x300u);
          }
          if (DAT_005279f4 != 0) {
            piVar9 = (int *)(int)(U(DAT_005f10e0) * 0x14u + 0x10u + U(DAT_005f10c8));
            *piVar9 = (int)(U(*piVar9) + 0x400u);
            DAT_005279f4 = 0;
          }
          iVar8 = (int)(U(iVar8) + 1u);
          DAT_005f10e0 = (int)(U(DAT_005f10e0) + 1u);
          puVar4 = puVar4 + 1;
        } while (iVar8 < DAT_005f10b8);
      }
      local_c = (int)(U(local_c) + 1u);
      local_14 = local_14 + 0x1c;
    } while (local_c < *(int *)(param_1 + 0xc4));
  }
  return;
}
