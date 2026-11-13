/* SPDX-License-Identifier: MIT */
/* z_bop 00430540 */
extern unsigned char DAT_00521050[];
extern unsigned char DAT_00521038[];
extern unsigned char DAT_00521068[];
extern unsigned char DAT_00a0aba0[];
extern unsigned short FUN_004305d0(unsigned short param_1);

unsigned int __cdecl FUN_00430540(int param_1, int param_2)
{
  unsigned char bVar1;
  unsigned char bVar2;
  unsigned char bVar3;
  unsigned short uVar4;
  unsigned int uVar5;
  unsigned int uVar6;

  bVar1 = DAT_00521050[param_1];
  bVar2 = DAT_00521038[param_1];
  uVar4 = FUN_004305d0((unsigned short)bVar1);
  uVar5 = (unsigned int)uVar4;
  bVar3 = DAT_00521068[param_1];
  if ((bVar3 != 0) &&
      (uVar6 = (unsigned int)bVar1,
       bVar3 <= DAT_00a0aba0[(unsigned int)bVar2 + uVar5 + param_2 * 0x61])) {
    while (uVar6 != 0) {
      uVar5 = uVar5 + 1;
      if ((int)(unsigned int)bVar1 <= (int)uVar5) {
        uVar5 = 0;
      }
      uVar6 = uVar6 - 1;
      if (DAT_00a0aba0[(unsigned int)bVar2 + uVar5 + param_2 * 0x61] < bVar3) {
        return uVar5;
      }
    }
    uVar5 = 0xffffffff;
  }
  return uVar5;
}
