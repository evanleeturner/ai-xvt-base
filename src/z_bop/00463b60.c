/* SPDX-License-Identifier: MIT */
/* z_bop 00463b60 */
extern int _DAT_00557360;
extern int DAT_00557364;
extern int DAT_009a8c2c;
extern int DAT_0052340c;
extern int DAT_00556ed0;
extern int DAT_00557368;
extern int _DAT_009ed23c;

int __stdcall FUN_0046f700(void);
void __cdecl FUN_0046d350(int param_1, unsigned int *param_2, unsigned int param_3);

void __stdcall FUN_00463b60(void)
{
  int iVar1;
  unsigned int *puVar2;
  unsigned int uVar3;
  
  _DAT_00557360 = 0x1e;
  DAT_00557364 = DAT_009a8c2c + DAT_0052340c;
  uVar3 = 0xc;
  puVar2 = (unsigned int *)&_DAT_00557360;
  DAT_00556ed0 = DAT_009a8c2c + DAT_0052340c;
  DAT_00557368 = _DAT_009ed23c;
  iVar1 = FUN_0046f700();
  FUN_0046d350(iVar1, puVar2, uVar3);
  return;
}
