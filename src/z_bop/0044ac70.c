/* SPDX-License-Identifier: MIT */
/* z_bop 0044ac70 */
extern int DAT_005233ec;
extern int DAT_005233f4;
extern int DAT_005233cc;
extern short DAT_00a00588;
extern short DAT_00a00530;

extern int __cdecl FUN_0040e190(int param_1);
extern void __stdcall FUN_00410ff0(void);

void __cdecl FUN_0044ac70(unsigned short *param_1, short param_2)
{
  unsigned short uVar1;
  int iVar2;
  unsigned int uVar3;
  short *psVar4;
  
  for (; param_2 != 0; param_2 = param_2 + -1) {
    uVar1 = *param_1;
    iVar2 = FUN_0040e190((unsigned int)param_1[1]);
    uVar3 = iVar2 + (unsigned int)uVar1 * 2;
    if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
      uVar3 = uVar3 % DAT_005233cc;
      FUN_00410ff0();
      FUN_00410ff0();
    }
    psVar4 = (short *)(uVar3 + (DAT_005233f4 & 0xfffffffe));
    if ((int)*psVar4 == (int)DAT_00a00588) {
      *psVar4 = *(short *)(&DAT_00a00530 + (unsigned int)(unsigned char)param_1[2] * 2);
    }
    else {
      param_1[2] = 0;
    }
    param_1 = param_1 + 3;
  }
  return;
}
