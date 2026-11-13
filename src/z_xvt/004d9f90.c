/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9f90 */
extern int DAT_0064f78c;
extern unsigned int DAT_0081b844;
extern unsigned int DAT_0081bc74;

void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);

void __cdecl FUN_004d9f90(int *param_1, char *param_2)
{
  unsigned int uVar1;
  
  if (DAT_0064f78c == 0) {
    FUN_004dd030(param_1, 2, 2);
    FUN_004da920(10, param_2, param_1, DAT_0081bc74);
    FUN_004dd030(param_1, -2, -2);
    uVar1 = DAT_0081b844;
  }
  else {
    FUN_004dd030(param_1, 2, 2);
    FUN_004da920(10, param_2, param_1, DAT_0081b844);
    FUN_004dd030(param_1, -2, -2);
    uVar1 = DAT_0081bc74;
  }
  FUN_004da920(10, param_2, param_1, uVar1);
  DAT_0064f78c = 0;
  return;
}
