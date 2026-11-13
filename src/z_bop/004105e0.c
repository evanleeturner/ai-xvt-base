/* SPDX-License-Identifier: MIT */
/* z_bop 004105e0 */
extern int __cdecl FUN_0040e190(int param_1);
extern void __stdcall FUN_00410ff0(void);

extern short DAT_0051a81c;
extern char *PTR_DAT_0051a818;
extern unsigned short DAT_00a0a1d4;
extern unsigned short DAT_00a0a1d2;
extern unsigned int DAT_005233ec;
extern unsigned int DAT_005233f4;
extern unsigned int DAT_005233cc;
extern char DAT_0054f9c0[];

void __stdcall FUN_004105e0(void)
{
  short sVar1;
  unsigned short uVar2;
  int iVar3;
  unsigned int uVar4;
  char *pcVar5;
  char *puVar6;
  unsigned int uVar7;
  
  uVar7 = 0;
  uVar2 = 0;
  for (sVar1 = DAT_0051a81c; sVar1 != 0; sVar1 = sVar1 - 1) {
    pcVar5 = PTR_DAT_0051a818 + uVar7;
    iVar3 = FUN_0040e190((int)pcVar5[1] + (unsigned int)DAT_00a0a1d4);
    uVar4 = iVar3 + *pcVar5 + (unsigned int)DAT_00a0a1d2;
    if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
      uVar4 = uVar4 % DAT_005233cc;
      FUN_00410ff0();
      FUN_00410ff0();
    }
    uVar7 += 2;
    puVar6 = (char *)(uVar4 + DAT_005233f4);
    DAT_0054f9c0[uVar2] = *puVar6;
    *puVar6 = 0xce;
    uVar2 = uVar2 + 1;
  }
  return;
}
