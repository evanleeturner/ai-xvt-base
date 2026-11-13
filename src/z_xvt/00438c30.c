/* SPDX-License-Identifier: MIT */
/* z_xvt 00438c30 */
extern int DAT_00525520;
extern int DAT_0052551c;
extern unsigned char DAT_006d4a4e;
extern int DAT_00525528;
extern int DAT_00525524;
extern int DAT_005257e0;
extern unsigned char DAT_0071e524;
extern int DAT_0052ae54;
extern void (__cdecl *DAT_00703c80)(void);

void FUN_0048ee20(char param_1);
void FUN_0048ed70(unsigned int param_1);
void FUN_0048eda0(unsigned int param_1);
void FUN_0048edd0(unsigned int param_1);
void FUN_004a7040(void);
void FUN_004a6a80(void);
void FUN_0048ed00(short param_1, short param_2, short param_3, short param_4);
void FUN_0048ece0(short param_1, short param_2);
void FUN_0048f270(unsigned char *param_2);
void FUN_004a6f90(void);

void __cdecl FUN_00438c30(int param_1, unsigned char *param_2, unsigned char param_3)
{
  unsigned int uVar1;
  unsigned int uVar2;
  short sVar3;
  short sVar4;
  short local_8;
  short local_4;

  FUN_0048ee20('\0');
  uVar2 = (unsigned int)(DAT_00525520 - DAT_0052551c) >> 1;
  sVar3 = (short)((unsigned int)(DAT_0052551c + DAT_00525520) >> 1) - (short)(uVar2 / 2);
  uVar1 = (unsigned int)DAT_006d4a4e;
  sVar4 = (short)((unsigned int)(DAT_00525528 + DAT_00525524) >> 1) - (short)((uVar1 * 5) / 2);
  if (DAT_005257e0 != 0) {
    FUN_0048ed70(0x2f);
    FUN_0048eda0(param_3 + 2);
    DAT_0071e524 = 1;
    FUN_0048edd0(0x2c);
    FUN_004a7040();
    DAT_0052ae54 = 0;
    FUN_004a6a80();
    local_8 = (short)(uVar1 * 5);
    local_4 = (short)uVar2;
    if (param_1 == 1) {
      FUN_0048ed00(sVar3 + -1, sVar4 + -1, local_4 + sVar3 + 1, local_8 + sVar4 + 1);
      (*DAT_00703c80)();
      param_1 = 0;
    }
    FUN_0048eda0((unsigned int)param_3);
    FUN_0048ed00(sVar3, (unsigned short)DAT_006d4a4e * (short)param_1 + sVar4, local_4 + sVar3, local_8 + sVar4);
    (*DAT_00703c80)();
    if (param_1 == 0) {
      param_1 = 1;
    }
    FUN_0048ece0((unsigned short)DAT_006d4a4e + sVar3, (short)param_1 * (unsigned short)DAT_006d4a4e + sVar4);
    FUN_0048f270(param_2);
    FUN_004a6f90();
    DAT_0052ae54 = 1;
    FUN_004a7040();
  }
  return;
}
