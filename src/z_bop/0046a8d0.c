/* SPDX-License-Identifier: MIT */
/* z_bop 0046a8d0 */
extern unsigned short DAT_009ecc46;
extern unsigned short DAT_009ecc48;
extern unsigned short DAT_009ecc50;
extern unsigned int DAT_009a8e14;
extern unsigned int DAT_009e9648;
extern unsigned int DAT_00a00520;
extern short DAT_00a080fe;
extern short DAT_009a8070;
extern unsigned int DAT_009a8c00;
extern short DAT_00a080f0;

void __cdecl FUN_0046aa70(unsigned int param_1, unsigned int param_2);

void __cdecl FUN_0046a8d0(unsigned int param_1, unsigned int param_2, unsigned int param_3)
{
  short sVar1;
  int bVar2;
  
  bVar2 = (int)param_1 < 0;
  if (bVar2) {
    param_1 = -param_1;
  }
  DAT_009ecc46 = (unsigned short)bVar2;
  bVar2 = (int)param_2 < 0;
  if (bVar2) {
    param_2 = -param_2;
  }
  DAT_009ecc48 = (unsigned short)bVar2;
  bVar2 = (int)param_3 < 0;
  if (bVar2) {
    param_3 = -param_3;
  }
  DAT_009ecc50 = (unsigned short)bVar2;
  DAT_009a8e14 = param_1;
  DAT_009e9648 = param_2;
  DAT_00a00520 = param_3;
  FUN_0046aa70(param_1, param_2);
  sVar1 = DAT_00a080fe;
  if (DAT_009ecc48 != 0) {
    sVar1 = -DAT_00a080fe;
  }
  if (DAT_009ecc46 != 0) {
    sVar1 = -0x8000 - sVar1;
  }
  DAT_009a8070 = 0x4000 - sVar1;
  FUN_0046aa70(DAT_009a8c00, DAT_00a00520);
  sVar1 = DAT_00a080fe;
  if (DAT_009ecc50 != 0) {
    sVar1 = -DAT_00a080fe;
  }
  DAT_00a080f0 = 0x4000 - sVar1;
  return;
}
