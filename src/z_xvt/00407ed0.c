/* SPDX-License-Identifier: MIT */
/* z_xvt 00407ed0 */
extern short DAT_006db6c2;
extern short DAT_0070acec;
extern char *DAT_00730884;
extern short DAT_006db6c8;
extern short DAT_0071b716;
extern short DAT_00732a20;
extern short DAT_006da53c;
extern short DAT_0071b718;
extern short DAT_0071e518;
extern short DAT_006d9d96;
extern short DAT_006da53e;
extern short DAT_00703c94;
extern short DAT_00703ad4;
extern short DAT_0071e522;

int __stdcall FUN_00407ed0(void)
{
  short sVar1;
  short sVar2;
  unsigned short uVar3;

  sVar2 = 0;
  if (DAT_006db6c2 < 0) {
    do {
      sVar2 = sVar2 - DAT_0070acec;
      DAT_006db6c2 = DAT_006db6c2 + *(short *)(DAT_00730884 + 0x2c) + 1;
      DAT_006db6c8 = DAT_006db6c8 + *(short *)(DAT_00730884 + 0x2e) + 1;
    } while (DAT_006db6c2 < 0);
  }
  if (DAT_0070acec <= DAT_006db6c2) {
    do {
      sVar2 = sVar2 + DAT_0070acec;
      DAT_006db6c2 = DAT_006db6c2 - (*(short *)(DAT_00730884 + 0x2c) + 1);
      DAT_006db6c8 = DAT_006db6c8 - (*(short *)(DAT_00730884 + 0x2e) + 1);
    } while (DAT_0070acec <= DAT_006db6c2);
  }
  DAT_006db6c8 = DAT_006db6c8 - *(short *)(DAT_00730884 + 0x32 + DAT_006db6c2 * 4);
  sVar1 = DAT_006db6c8;
  DAT_0071b716 = DAT_006db6c8;
  if (DAT_00732a20 <= DAT_006db6c8) {
    DAT_006da53c = *(short *)(DAT_00730884 + 0x2e) + DAT_006db6c8;
    return 0;
  }
  DAT_0071b718 = 0;
  DAT_0071e518 = 0;
  if (DAT_006db6c8 < 0) {
    DAT_006d9d96 = -DAT_006db6c8;
    DAT_006db6c8 = DAT_006d9d96;
    if (*(short *)(DAT_00730884 + 0x2e) < DAT_006d9d96) {
      DAT_0071e518 = -1;
    }
    else {
      uVar3 = 0;
      DAT_0071e518 = DAT_006d9d96;
      if (-1 < *(short *)(DAT_00730884 + 0x2c)) {
        do {
          if ((unsigned int)*(unsigned short *)(DAT_00730884 + 0x32 + (unsigned int)uVar3 * 4) == (unsigned int)DAT_006d9d96) {
            DAT_0071e518 = *(short *)(DAT_00730884 + 0x30 + (unsigned int)uVar3 * 4);
            break;
          }
          uVar3 = uVar3 + 1;
        } while ((int)(unsigned short)uVar3 <= (int)*(short *)(DAT_00730884 + 0x2c));
      }
    }
  }
  DAT_006da53e = *(short *)(DAT_00730884 + 0x2c);
  DAT_00703c94 = DAT_00732a20;
  DAT_006da53c = *(short *)(DAT_00730884 + 0x2e) + sVar1;
  if (DAT_006da53c < 0) {
    DAT_00703ad4 = -1;
  }
  else {
    if ((DAT_006da53c < 0) || (DAT_00732a20 <= DAT_006da53c)) {
      uVar3 = 0;
      DAT_00703c94 = DAT_00732a20 - sVar1;
      sVar1 = *(short *)(DAT_00730884 + 0x2c);
      if (-1 < sVar1) {
        do {
          if ((unsigned int)*(unsigned short *)(DAT_00730884 + 0x32 + (unsigned int)uVar3 * 4) == (unsigned int)DAT_00703c94) break;
          uVar3 = uVar3 + 1;
        } while ((int)(unsigned short)uVar3 <= (int)sVar1);
      }
      sVar1 = *(short *)(DAT_00730884 + 0x30 + (unsigned int)uVar3 * 4);
    }
    else {
      sVar1 = *(short *)(DAT_00730884 + 0xe);
    }
    DAT_00703ad4 = sVar1 + -1;
  }
  DAT_0071e522 = DAT_006db6c2 + sVar2;
  return 1;
}
