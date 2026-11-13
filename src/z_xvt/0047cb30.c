/* SPDX-License-Identifier: MIT */
/* z_xvt 0047cb30 */
extern unsigned char DAT_00525508;
extern unsigned int DAT_00525518;
extern unsigned int DAT_006db594;
extern unsigned int DAT_006da538;
extern unsigned char *DAT_006da9ec;
extern void (*DAT_00703c80)(void);

void __cdecl FUN_00461f20(int param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4);
void __cdecl FUN_0048eda0(unsigned int param_1);
void __cdecl FUN_0048ed00(int param_1, int param_2, short param_3, short param_4);

void __stdcall FUN_0047cb30(void)
{
  unsigned char bVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned char *pbVar5;
  unsigned short uVar1;

  bVar2 = DAT_00525508;
  if (DAT_00525518 == 1) {
    uVar1 = (unsigned short)(DAT_00525508 | (DAT_00525508 << 8));
    uVar4 = DAT_006db594 * DAT_006da538;
    pbVar5 = DAT_006da9ec;
    for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(unsigned int *)pbVar5 = (unsigned int)(uVar1 | (uVar1 << 16));
      pbVar5 = pbVar5 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pbVar5 = bVar2;
      pbVar5 = pbVar5 + 1;
    }
    return;
  }
  FUN_00461f20((int)DAT_006da9ec, DAT_006da538, DAT_006db594, DAT_00525518 * DAT_006da538);
  FUN_0048eda0((unsigned int)DAT_00525508);
  FUN_0048ed00(0, 0, (short)DAT_006da538, (short)DAT_006db594);
  (*DAT_00703c80)();
  FUN_00461f20(0, 0x140, 0xf0, 0);
  return;
}
