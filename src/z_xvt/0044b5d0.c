/* SPDX-License-Identifier: MIT */
/* z_xvt 0044b5d0 */
extern short DAT_006d4984;
extern short DAT_006d4a42;
extern int DAT_00709ffc;
extern short DAT_00732e44;
extern char DAT_0052562f[];

int __cdecl FUN_00413f20(int, int);
void __cdecl FUN_0049c6b0(unsigned int, int, int, unsigned short);

void __cdecl FUN_0044b5d0(int param_1, int param_2, int param_3, unsigned short param_4, int param_5)
{
  short sVar1;
  int iVar3;
  unsigned int arg1;

  if (param_1 < 0) {
    if (param_3 < -param_1) {
      return;
    }
    sVar1 = -(short)FUN_00413f20(-param_1, param_3);
  }
  else {
    if (param_3 < param_1) {
      return;
    }
    sVar1 = (short)FUN_00413f20(param_1, param_3);
  }

  if (param_2 < 0) {
    if (param_3 < -param_2) {
      return;
    }
    iVar3 = -FUN_00413f20(-param_2, param_3);
  }
  else {
    if (param_3 < param_2) {
      return;
    }
    iVar3 = FUN_00413f20(param_2, param_3);
  }

  iVar3 = iVar3 + (unsigned int)DAT_006d4a42 + DAT_00709ffc;

  arg1 = (unsigned int)(short)((unsigned int)iVar3 >> 16);
  arg1 = (arg1 << 16) | param_4;
  arg1 = arg1 >> 8;
  arg1 = arg1 & 0xFFFFFF;
  arg1 = (arg1 << 8) | (unsigned char)DAT_0052562f[param_5];
  arg1 = arg1 & 0xFFFF00FF;

  FUN_0049c6b0(arg1, sVar1 + DAT_00732e44, DAT_006d4984 - (short)iVar3, param_4);
}
