/* SPDX-License-Identifier: MIT */
/* z_bop 00420110 */
extern int DAT_0052361c;
extern int DAT_009fd398;
extern int DAT_009a8e34;
extern unsigned short DAT_00521f24[];
extern int DAT_00527e90;

int __cdecl FUN_004ac720(int param_1);
void __stdcall FUN_004ac740(void);
void __cdecl FUN_0040bbf0(unsigned short param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_00421850(unsigned short param_1);
void __cdecl FUN_00421930(int param_1);
void __cdecl FUN_00421560(short param_1, short param_2, int param_3, int param_4);

void __cdecl FUN_00420110(int param_1, int param_2, int param_3, unsigned short param_4)
{
  int iVar1;
  
  DAT_0052361c = 1;
  DAT_009fd398 = 0x100000;
  DAT_009a8e34 = 0x7fffffff;
  iVar1 = FUN_004ac720(((unsigned int)(unsigned short)((unsigned int)param_1 >> 16) << 16) | (unsigned int)(unsigned short)DAT_00521f24[param_1 * 0xd]);
  FUN_004ac740();
  iVar1 = iVar1 + *(int *)(*(int *)(iVar1 + 0x10) + iVar1);
  if (DAT_00527e90 != 0) {
    FUN_0040bbf0(param_4, param_2, param_3, 0x100, iVar1);
    return;
  }
  FUN_00421850(param_4);
  FUN_00421930(iVar1);
  FUN_00421560((short)param_2, (short)param_3, 0x100, iVar1);
  return;
}
