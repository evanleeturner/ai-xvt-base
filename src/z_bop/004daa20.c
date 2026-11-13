/* SPDX-License-Identifier: MIT */
/* z_bop 004daa20 */
extern int DAT_0052c01c;
extern int DAT_0052c020;
extern int DAT_00665468;
extern char *DAT_0066546c;

void __cdecl FUN_004b5570(char *param_1, int param_2, int param_3);
int __cdecl FUN_004d6e30(int param_1, int param_2, int param_3);
void __cdecl FUN_004de090(int *param_1, int *param_2);
void __cdecl FUN_004daca0(int *param_1, char *param_2);
int __cdecl FUN_004d6b90(int *param_1, int param_2, int param_3);
int __cdecl FUN_004dbfa0(char *param_1, int param_2);
void __cdecl FUN_004de130(int *param_1, int *param_2);
void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004d67e0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004d6950(int *param_1, int param_2, int param_3, short param_4);
void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, int param_4);

void __cdecl FUN_004daa20(int *param_1, char *param_2, char *param_3, int param_4)
{
  int iVar1;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10[4];

  FUN_004b5570(param_2, 0, 0);
  if (DAT_0052c01c == 0) {
    DAT_0052c01c = 1;
    DAT_0052c020 = FUN_004d6e30(0x60, 0x60, 0x60);
  }
  FUN_004de090(&local_1c, &local_20);
  if (DAT_00665468 != 0) {
    FUN_004daca0(param_1, DAT_0066546c);
  }
  if ((param_3 != (char *)0x0) && (iVar1 = FUN_004d6b90(param_1, local_1c, local_20), iVar1 != 0)) {
    iVar1 = FUN_004dbfa0(param_3, param_4);
    FUN_004de130(&local_18, &local_14);
    local_1c = local_1c + local_18;
    local_20 = local_20 + local_14;
    if (0x27f < local_1c + iVar1 + 5) {
      local_1c = 0x27a - iVar1;
    }
    if (0x1df < local_20 + param_4 + 5U) {
      local_20 = 0x1da - param_4;
    }
    FUN_004d6460(local_10, local_1c, local_20, local_1c + iVar1 + 5, local_20 + param_4 + 3);
    FUN_004d67e0(local_10, 0, 0, 0xffff, 1);
    FUN_004d6950(local_10, 0, 0, (short)DAT_0052c020);
    FUN_004db640(param_4, param_3, local_10, 0);
  }
  return;
}
