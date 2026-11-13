/* SPDX-License-Identifier: MIT */
/* z_xvt 004079f0 */
extern unsigned short DAT_00703c60;
extern short DAT_006ffbe6;
extern int DAT_00730884;
extern short DAT_0071e522;

void __stdcall FUN_004079f0(void)
{
  int iVar1;
  unsigned short uVar2;
  short sVar3;
  short sVar4;
  
  uVar2 = DAT_00703c60;
  sVar3 = 1;
  if (DAT_006ffbe6 != 0) {
    DAT_006ffbe6 = 0;
    return;
  }
  DAT_00703c60 = DAT_00703c60 + *(short *)(DAT_00730884 + 0x1c);
  if (*(short *)(DAT_00730884 + 0x1e) == 0) {
    if (uVar2 <= DAT_00703c60) {
      return;
    }
  }
  else if (DAT_00703c60 < uVar2) {
    sVar3 = 2;
  }
  sVar4 = DAT_0071e522;
  if (DAT_0071e522 < 0) {
    do {
      sVar4 = sVar4 + *(short *)(DAT_00730884 + 0xe);
    } while (sVar4 < 0);
  }
  for (; (int)(unsigned int)*(unsigned short *)(DAT_00730884 + 0xe) <= (int)sVar4;
      sVar4 = sVar4 - *(unsigned short *)(DAT_00730884 + 0xe)) {
  }
  if (*(short *)(DAT_00730884 + 0x18) == 0) {
    iVar1 = DAT_00730884 + sVar4 * 4;
    if (*(short *)(DAT_00730884 + 0x14) == 1) {
      DAT_0071e522 = DAT_0071e522 + sVar3;
      if (*(short *)(iVar1 + 0x36) != *(short *)(iVar1 + 0x32)) {
        DAT_006ffbe6 = 1;
        return;
      }
    }
    else {
      DAT_0071e522 = DAT_0071e522 - sVar3;
      if (*(short *)(iVar1 + 0x2e) != *(short *)(iVar1 + 0x32)) {
        DAT_006ffbe6 = 1;
        return;
      }
    }
  }
  else {
    iVar1 = DAT_00730884 + sVar4 * 4;
    if (*(short *)(DAT_00730884 + 0x14) == 1) {
      DAT_0071e522 = DAT_0071e522 - sVar3;
      if (*(short *)(iVar1 + 0x2c) != *(short *)(iVar1 + 0x30)) {
        DAT_006ffbe6 = 1;
      }
    }
    else {
      DAT_0071e522 = DAT_0071e522 + sVar3;
      if (*(short *)(iVar1 + 0x34) != *(short *)(iVar1 + 0x30)) {
        DAT_006ffbe6 = 1;
        return;
      }
    }
  }
  return;
}
