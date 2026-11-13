/* SPDX-License-Identifier: MIT */
/* z_bop 00404670 */
extern void FUN_00402cb0(unsigned short);
extern unsigned int FUN_00404450(unsigned short);

extern unsigned char DAT_00524120[];
extern unsigned char DAT_009d8cd2[];
extern void *DAT_009a1fe8;
extern short DAT_009a73c2;

unsigned int FUN_00404670(int param_1, unsigned int param_2, unsigned short param_3)
{
  unsigned int uVar1;
  short sVar2;
  unsigned int uVar3;

  if (param_1 == -1) {
    return 0;
  }
  sVar2 = 0;
  uVar3 = 0;
  FUN_00402cb0((unsigned short)param_1);
  do {
    if (DAT_00524120[DAT_009d8cd2[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + param_1 * 0x23) * 0x566 + uVar3]] == param_2) {
      DAT_009a73c2 = sVar2;
      uVar1 = FUN_00404450(param_3);
      if ((short)uVar1 != 0) {
        return 1;
      }
    }
    uVar3 = uVar3 + 0x52;
    sVar2 = sVar2 + 1;
  } while (uVar3 < 0xf6);
  return 0;
}
