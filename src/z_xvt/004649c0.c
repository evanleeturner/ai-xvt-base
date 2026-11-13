/* SPDX-License-Identifier: MIT */
/* z_xvt 004649c0 */
extern uint DAT_005f1d20[];
extern unsigned char DAT_00527d21[];
extern unsigned char DAT_00527d20[];
extern uint DAT_0052552c;
extern uint DAT_00525534;
extern uint DAT_0052550c;
extern unsigned char DAT_005f1118[];

void __cdecl FUN_004649c0(uint param_1, uint param_2, unsigned char param_3)
{
  uint uVar1;
  unsigned char *puVar2;
  short sVar3;
  uint uVar4;
  ushort uVar5;
  
  uVar4 = 0;
  sVar3 = 7;
  uVar5 = 0;
  do {
    uVar1 = *(uint *)(DAT_005f1d20 + ((int)(char)DAT_00527d21[uVar4] + (param_2 & 0xffff)) * 4) +
            (int)(char)DAT_00527d20[uVar4] + (param_1 & 0xffff);
    if ((DAT_0052552c != 0x13) && (DAT_00525534 != 0xa0000)) {
      uVar1 = uVar1 % DAT_0052550c;
    }
    uVar4 += 2;
    puVar2 = (unsigned char *)(uVar1 + DAT_00525534);
    sVar3 -= 1;
    DAT_005f1118[uVar5] = *puVar2;
    *puVar2 = param_3;
    uVar5 += 1;
  } while (sVar3 != 0);
  return;
}
