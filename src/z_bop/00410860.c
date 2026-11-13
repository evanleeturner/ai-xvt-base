/* SPDX-License-Identifier: MIT */
/* z_bop 00410860 */
extern char DAT_0051a820[];
extern char DAT_0051a821[];
extern int DAT_005233cc;
extern int DAT_005233ec;
extern int DAT_005233f4;
extern char DAT_0054f9d8[];

int __cdecl FUN_0040e190(int param_1);
void __stdcall FUN_00410ff0(void);

void __cdecl FUN_00410860(uint param_1, uint param_2)
{
  int iVar1;
  uint uVar2;
  ushort uVar3;
  uint uVar4;
  short local_6;
  
  local_6 = 7;
  uVar4 = 0;
  uVar3 = 0;
  do {
    iVar1 = FUN_0040e190((int)(char)DAT_0051a821[uVar4] + (param_2 & 0xffff));
    uVar2 = iVar1 + (char)DAT_0051a820[uVar4] + (param_1 & 0xffff);
    if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
      uVar2 = uVar2 % DAT_005233cc;
      FUN_00410ff0();
    }
    uVar4 += 2;
    local_6--;
    *(unsigned char *)(uVar2 + DAT_005233f4) = DAT_0054f9d8[uVar3];
    uVar3++;
  } while (local_6 != 0);
  return;
}
