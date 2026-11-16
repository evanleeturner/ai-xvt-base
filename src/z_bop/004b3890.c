/* SPDX-License-Identifier: MIT */
/* z_bop 004b3890 */
extern int DAT_00a90bf0;
extern int DAT_00a90bf4;
extern int DAT_00a90bf8;
extern int DAT_00a90bfc;
extern float DAT_00a90b60;
extern float DAT_00a90b64;
extern float _DAT_00a90b80;
extern float _DAT_00a90b84;
extern float _DAT_00a90ba0;
extern int _DAT_00a90b34;
extern int _DAT_00a90b30;
extern int _DAT_00a90b38;
extern float _DAT_00a90ba4;
extern float _DAT_00a90bc0;
extern float _DAT_00a90bc4;
extern int _DAT_00a90b40;
extern int _DAT_00a90b3c;
extern int _DAT_00a90b44;
extern int _DAT_00a90b48;
extern int _DAT_00a90b54;
extern int _DAT_00a90b4c;
extern int _DAT_00a90b50;

void __cdecl FUN_004b3890(int *param_1)
{
  int iVar1;
  float *puVar2;
  
  DAT_00a90bf0 = *param_1;
  DAT_00a90bf4 = param_1[1];
  DAT_00a90bf8 = param_1[2];
  DAT_00a90bfc = param_1[3];
  puVar2 = &DAT_00a90b60;
  for (iVar1 = 0x20; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  DAT_00a90b60 = (float)DAT_00a90bf0;
  DAT_00a90b64 = (float)DAT_00a90bf4;
  _DAT_00a90b80 = (float)(DAT_00a90bf8 + DAT_00a90bf0);
  _DAT_00a90b84 = (float)DAT_00a90bf4;
  _DAT_00a90ba0 = (float)(DAT_00a90bf8 + DAT_00a90bf0);
  _DAT_00a90b34 = 1;
  _DAT_00a90b30 = 0;
  _DAT_00a90b38 = 2;
  _DAT_00a90ba4 = (float)(DAT_00a90bf4 + DAT_00a90bfc);
  _DAT_00a90bc0 = (float)DAT_00a90bf0;
  _DAT_00a90bc4 = (float)(DAT_00a90bf4 + DAT_00a90bfc);
  _DAT_00a90b40 = 0;
  _DAT_00a90b3c = 0x8200;
  _DAT_00a90b44 = 0;
  _DAT_00a90b48 = 2;
  _DAT_00a90b54 = 0;
  _DAT_00a90b4c = 3;
  _DAT_00a90b50 = 0x8200;
  return;
}
