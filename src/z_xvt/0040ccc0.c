/* SPDX-License-Identifier: MIT */
/* z_xvt 0040ccc0 */
extern unsigned short DAT_00523b58[];
extern void (*DAT_006d9984)(short);
extern unsigned char DAT_0071e524;
extern unsigned char DAT_006da9fa;
extern void FUN_0048ed70(unsigned int);

void __cdecl FUN_0040ccc0(unsigned short param_1, unsigned int param_2, unsigned int param_3)
{
  int bVar1;
  unsigned char uVar2;
  unsigned char uVar3;
  unsigned short uVar4;
  short sVar5;
  
  uVar3 = DAT_0071e524;
  uVar2 = DAT_006da9fa;
  if (param_1 != 0xffff) {
    bVar1 = 0;
    for (; param_2 != 0; param_2 = param_2 - 1) {
      uVar4 = param_1 / DAT_00523b58[param_2];
      param_1 = param_1 - DAT_00523b58[param_2] * uVar4;
      if (((bVar1) || (param_2 <= param_3)) || (uVar4 != 0)) {
        bVar1 = 1;
        if (9 < uVar4) {
          uVar4 = 9;
        }
        sVar5 = uVar4 + 0x30;
      }
      else {
        sVar5 = 0x20;
        bVar1 = 0;
      }
      (*DAT_006d9984)(sVar5);
    }
    return;
  }
  DAT_0071e524 = 0;
  FUN_0048ed70(0x40);
  for (; param_2 != 0; param_2 = param_2 - 1) {
    (*DAT_006d9984)(0x30);
  }
  DAT_0071e524 = uVar3;
  DAT_006da9fa = uVar2;
  return;
}
