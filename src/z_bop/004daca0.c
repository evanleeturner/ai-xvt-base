/* SPDX-License-Identifier: MIT */
/* z_bop 004daca0 */
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
extern unsigned int DAT_00665574;
extern unsigned int DAT_00b6a264;
extern unsigned int DAT_00b69e24;

void __cdecl FUN_004daca0(int *param_1, char *param_2)
{
  unsigned int uVar1;
  
  if (DAT_00665574 == 0) {
    FUN_004d64a0(param_1, 2, 2);
    FUN_004db640(10, param_2, param_1, DAT_00b6a264);
    FUN_004d64a0(param_1, -2, -2);
    uVar1 = DAT_00b69e24;
  }
  else {
    FUN_004d64a0(param_1, 2, 2);
    FUN_004db640(10, param_2, param_1, DAT_00b69e24);
    FUN_004d64a0(param_1, -2, -2);
    uVar1 = DAT_00b6a264;
  }
  FUN_004db640(10, param_2, param_1, uVar1);
  DAT_00665574 = 0;
  return;
}
