/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9d20 */
extern int DAT_00532af4;
extern int DAT_00532af8;
extern int DAT_0064f680;
extern char *DAT_0064f684;

void __cdecl FUN_004d0e60(char *param_1, int param_2, int param_3);
int __cdecl FUN_004df760(char param_1, char param_2, char param_3);
void __cdecl FUN_004ddc70(int *param_1, int *param_2);
void __cdecl FUN_004d9f90(int *param_1, char *param_2);
int __cdecl FUN_004dd7d0(int *param_1, int param_2, int param_3);
int __cdecl FUN_004db2f0(char *param_1, int param_2);
void __cdecl FUN_004ddd10(int *param_1, int *param_2);
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004dd3e0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004dd570(int *param_1, int param_2, int param_3, short param_4);
void __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, int param_4);

void __cdecl FUN_004d9d20(int *param_1, char *param_2, char *param_3, int param_4)
{
  int iVar1;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10[4];

  FUN_004d0e60(param_2, 0, 0);
  if (DAT_00532af4 == 0) {
    DAT_00532af4 = 1;
    DAT_00532af8 = FUN_004df760(0x60, 0x60, 0x60);
  }
  FUN_004ddc70(&local_1c, &local_20);
  if (DAT_0064f680 != 0) {
    FUN_004d9f90(param_1, DAT_0064f684);
  }
  if ((param_3 != (char *)0x0) && (iVar1 = FUN_004dd7d0(param_1, local_1c, local_20), iVar1 != 0)) {
    iVar1 = FUN_004db2f0(param_3, param_4);
    FUN_004ddd10(&local_18, &local_14);
    local_1c = local_1c + local_18;
    local_20 = local_20 + local_14;
    if (0x27f < local_1c + iVar1 + 5) {
      local_1c = 0x27a - iVar1;
    }
    if (0x1df < local_20 + param_4 + 5) {
      local_20 = 0x1da - param_4;
    }
    FUN_004dcfd0(local_10, local_1c, local_20, local_1c + iVar1 + 5, local_20 + param_4 + 3);
    FUN_004dd3e0(local_10, 0, 0, 0xffff, 1);
    FUN_004dd570(local_10, 0, 0, (short)DAT_00532af8);
    FUN_004da920(param_4, param_3, local_10, 0);
  }
  return;
}
