/* SPDX-License-Identifier: MIT */
/* z_bop 004651f0 */
extern int DAT_00557364;
extern int DAT_00523420;
extern int DAT_00557368;
extern int DAT_009a8c2c;
extern int _DAT_00557360;
extern int DAT_0055736c;
extern int DAT_005242e0;
extern int DAT_00556f00;
extern int DAT_00a07cc8;
extern int _DAT_009ed23c;

extern void FUN_0046d350(int param_1, unsigned int *param_2, int param_3);
extern int FUN_004ac760(void);
extern int FUN_004aa7f0(void);
extern unsigned int FUN_004aa870(void);
extern void FUN_00463d00(void);
extern void FUN_0046d300(unsigned int *param_1, int param_2);
extern int FUN_0046f660(int param_1);
extern void FUN_00463c50(int param_1);

void __cdecl FUN_004651f0(int param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  int iVar5;

  DAT_00557364 = DAT_00523420;
  DAT_00557368 = param_2;
  iVar5 = 10;
  iVar4 = 0;
  _DAT_00557360 = 0x3e;
  DAT_0055736c = DAT_009a8c2c;
  FUN_0046d350(param_1, (unsigned int *)&_DAT_00557360, 0x10);
  FUN_004ac760();
  do {
    iVar1 = DAT_009a8c2c;
    DAT_005242e0 = 1;
    do {
      if (0xeb < (unsigned int)(DAT_009a8c2c - iVar1)) break;
      iVar2 = FUN_004aa7f0();
      if ((iVar2 != 0) && (uVar3 = FUN_004aa870(), uVar3 == 0x1b)) {
        iVar5 = 1;
        DAT_009a8c2c = DAT_009a8c2c + 0xec;
        break;
      }
      FUN_00463d00();
      iVar2 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + iVar2;
      if (DAT_00556f00 != 0) {
        DAT_00556f00 = 0;
        DAT_009a8c2c = iVar1;
      }
    } while (DAT_005242e0 != 0);
    iVar4 = (iVar4 - iVar1) + DAT_009a8c2c;
    if (0xeb < iVar4) {
      iVar4 = 0;
      _DAT_00557360 = 0x1d;
      FUN_0046d300((unsigned int *)&_DAT_00557360, 4);
    }
    if ((DAT_005242e0 == 0) || (iVar5 = iVar5 - 1, iVar5 == 0)) {
      if (DAT_005242e0 == 1) {
        iVar4 = FUN_0046f660(param_1);
        FUN_00463c50(iVar4);
        FUN_004ac760();
        DAT_005242e0 = 0;
        DAT_009a8c2c = DAT_00a07cc8 + _DAT_009ed23c;
      }
      else {
        FUN_004ac760();
        DAT_009a8c2c = DAT_00a07cc8 + _DAT_009ed23c;
      }
      _DAT_00557360 = 0x17;
      DAT_00557364 = 0;
      FUN_0046d300((unsigned int *)&_DAT_00557360, 8);
      return;
    }
  } while (1);
}
