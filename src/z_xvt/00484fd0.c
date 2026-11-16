/* SPDX-License-Identifier: MIT */
/* z_xvt 00484fd0 */
extern unsigned char DAT_006a884a[4];
extern int DAT_006a8858;
extern unsigned char DAT_006a8866[4];
extern int DAT_006a886c;
extern int DAT_006a8874;
extern unsigned char DAT_006a887e[4];
extern unsigned short DAT_006d4984;
extern unsigned int DAT_0060267c;
extern float *DAT_00602688;
extern float *DAT_00602698;

unsigned int __cdecl FUN_004859c0(int param_1, unsigned int *param_2, float *param_3, float *param_4);
unsigned int __cdecl FUN_00485c90(unsigned int *param_1, float *param_2, float *param_3);
void __cdecl FUN_004853c0(int param_1);

void __cdecl FUN_00484fd0(int param_1)
{
  int iVar1;
  unsigned int *puVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  unsigned int *puVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int local_18;
  int *local_10;
  int local_4;
  int g884a;
  int g8866;
  int g887e;

  iVar7 = DAT_006a886c;
  iVar1 = (int)((unsigned int)DAT_006a8858 +
                (unsigned int)*(int *)(param_1 + 0xbc) * 0x18u);
  iVar8 = 0;
  __builtin_memcpy(&g884a, DAT_006a884a, 4);
  local_10 = (int *)(int)((unsigned int)*(int *)(param_1 + 0xb8) * 0x70u +
                          (unsigned int)g884a);
  *(int *)(param_1 + 0xc0) = DAT_006a886c;
  iVar6 = 0;
  *(unsigned int *)(param_1 + 0xcc) = 0;
  __builtin_memcpy(&g8866, DAT_006a8866, 4);
  puVar2 = (unsigned int *)(int)((unsigned int)g8866 + (unsigned int)iVar7 * 0x1cu);
  __builtin_memcpy(&g887e, DAT_006a887e, 4);
  if (0 < *(int *)(param_1 + 0x90)) {
    do {
      iVar8 = (int)((unsigned int)iVar8 + 4u);
      *(unsigned int *)(int)((unsigned int)g887e - 4u + (unsigned int)iVar8) = 0xffffffffu;
      iVar6 = (int)((unsigned int)iVar6 + 1u);
    } while (iVar6 < *(int *)(param_1 + 0x90));
  }
  local_4 = 0;
  puVar9 = puVar2;
  if (0 < *(int *)(param_1 + 0xc4)) {
    do {
      iVar7 = (int)((unsigned int)*local_10 * 0x40u + (unsigned int)*(int *)(param_1 + 0xa0));
      if (local_10[3] == -1) {
        local_10[3] = (int)(unsigned int)DAT_006d4984;
        DAT_00602688 = (float *)0;
        DAT_00602698 = (float *)0;
        local_18 = (int)(4u - (unsigned int)(*(int *)(int)((unsigned int)iVar7 + 0x1cu) == -1));
        iVar6 = 0;
        iVar8 = (int)((unsigned int)iVar7 + 0x10u + (unsigned int)local_18 * 4u);
        iVar10 = 0;
        do {
          iVar3 = *(int *)(int)((unsigned int)iVar8 - 4u);
          DAT_0060267c = 0;
          local_18 = (int)((unsigned int)local_18 - 1u);
          __builtin_memcpy(&g887e, DAT_006a887e, 4);
          iVar4 = *(int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u);
          iVar11 = iVar10;
          if (iVar4 == -1) {
            uVar5 = FUN_004859c0(param_1, puVar9,
                                 (float *)(int)((unsigned int)iVar1 +
                                                (unsigned int)*(int *)(int)((unsigned int)DAT_006a8874 +
                                                                            (unsigned int)*(int *)(int)((unsigned int)iVar8 - 0x14u) * 4u) * 0x18u),
                                 (float *)(int)((unsigned int)iVar1 +
                                                (unsigned int)*(int *)(int)((unsigned int)DAT_006a8874 +
                                                                            (unsigned int)*(int *)(int)((unsigned int)iVar7 + (unsigned int)iVar6 * 4u) * 4u) * 0x18u));
            if ((int)uVar5 < 0) {
              if (DAT_0060267c == 0) {
                __builtin_memcpy(&g887e, DAT_006a887e, 4);
                *(unsigned int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u) = 0xfffffffeu;
              }
            }
            else {
              local_10[(int)((unsigned int)iVar10 + 0x14u)] = (int)puVar9;
              puVar9[6] = DAT_0060267c;
              __builtin_memcpy(&g887e, DAT_006a887e, 4);
              *(unsigned int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u) = *(unsigned int *)(param_1 + 0xcc);
              *(int *)(param_1 + 0xcc) = (int)((unsigned int)*(int *)(param_1 + 0xcc) + 1u);
              puVar9 = puVar9 + 7;
              iVar11 = (int)((unsigned int)iVar10 + 1u);
            }
          }
          else if (iVar4 != -2) {
            iVar11 = (int)((unsigned int)iVar10 + 1u);
            local_10[(int)((unsigned int)iVar10 + 0x14u)] =
                (int)(puVar2 + (int)((unsigned int)iVar4 * 7u));
            if (puVar2[(int)(((unsigned int)*(int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u)) * 7u + 6u)] != 0) {
              DAT_00602698 = DAT_00602688;
              DAT_00602688 = (float *)puVar2[(int)(((unsigned int)*(int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u)) * 7u + 6u)];
            }
          }
          iVar6 = local_18;
          iVar8 = (int)((unsigned int)iVar8 - 4u);
          iVar10 = iVar11;
        } while (0 < local_18);
        iVar12 = iVar11;
        if (DAT_00602698 != (float *)0) {
          uVar5 = FUN_004859c0(param_1, puVar9, DAT_00602688, DAT_00602698);
          if (-1 < (int)uVar5) {
            iVar12 = (int)((unsigned int)iVar11 + 1u);
            local_10[(int)((unsigned int)iVar11 + 0x14u)] = (int)puVar9;
            puVar9 = puVar9 + 7;
            *(int *)(param_1 + 0xcc) = (int)((unsigned int)*(int *)(param_1 + 0xcc) + 1u);
          }
        }
      }
      else {
        local_10[3] = (int)(unsigned int)DAT_006d4984;
        local_18 = (int)(4u - (unsigned int)(*(int *)(int)((unsigned int)iVar7 + 0x1cu) == -1));
        iVar6 = 0;
        iVar8 = (int)((unsigned int)iVar7 + 0x10u + (unsigned int)local_18 * 4u);
        iVar10 = 0;
        do {
          iVar3 = *(int *)(int)((unsigned int)iVar8 - 4u);
          local_18 = (int)((unsigned int)local_18 - 1u);
          __builtin_memcpy(&g887e, DAT_006a887e, 4);
          iVar4 = *(int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u);
          iVar12 = iVar10;
          if (iVar4 == -1) {
            uVar5 = FUN_00485c90(puVar9,
                                 (float *)(int)((unsigned int)iVar1 +
                                                (unsigned int)*(int *)(int)((unsigned int)DAT_006a8874 +
                                                                            (unsigned int)*(int *)(int)((unsigned int)iVar8 - 0x14u) * 4u) * 0x18u),
                                 (float *)(int)((unsigned int)iVar1 +
                                                (unsigned int)*(int *)(int)((unsigned int)DAT_006a8874 +
                                                                            (unsigned int)*(int *)(int)((unsigned int)iVar7 + (unsigned int)iVar6 * 4u) * 4u) * 0x18u));
            if ((int)uVar5 < 0) {
              __builtin_memcpy(&g887e, DAT_006a887e, 4);
              *(unsigned int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u) = 0xfffffffeu;
            }
            else {
              local_10[(int)((unsigned int)iVar10 + 0x14u)] = (int)puVar9;
              puVar9 = puVar9 + 7;
              __builtin_memcpy(&g887e, DAT_006a887e, 4);
              *(unsigned int *)(int)((unsigned int)g887e + (unsigned int)iVar3 * 4u) = *(unsigned int *)(param_1 + 0xcc);
              *(int *)(param_1 + 0xcc) = (int)((unsigned int)*(int *)(param_1 + 0xcc) + 1u);
              iVar12 = (int)((unsigned int)iVar10 + 1u);
            }
          }
          else if (iVar4 != -2) {
            iVar12 = (int)((unsigned int)iVar10 + 1u);
            local_10[(int)((unsigned int)iVar10 + 0x14u)] =
                (int)(puVar2 + (int)((unsigned int)iVar4 * 7u));
          }
          iVar6 = local_18;
          iVar8 = (int)((unsigned int)iVar8 - 4u);
          iVar10 = iVar12;
        } while (0 < local_18);
      }
      local_10[0x19] = iVar12;
      if (iVar12 == 0) {
        local_10[0x11] = 0;
        local_10[0x10] = 0;
      }
      else {
        FUN_004853c0((int)local_10);
      }
      local_4 = (int)((unsigned int)local_4 + 1u);
      local_10 = local_10 + 0x1c;
    } while (local_4 < *(int *)(param_1 + 0xc4));
  }
  DAT_006a886c = (int)((unsigned int)DAT_006a886c + (unsigned int)*(int *)(param_1 + 0xcc));
  return;
}
