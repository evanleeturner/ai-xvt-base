/* SPDX-License-Identifier: MIT */
/* z_bop 00435850 */
extern unsigned int __cdecl FUN_00435a10(unsigned int, unsigned int, unsigned int);
extern unsigned char DAT_009d8e25[];
extern unsigned char DAT_009d8e26[];
extern unsigned char DAT_009d8e27[];
extern unsigned char DAT_009d8e33[];
extern unsigned char DAT_009d8e28[];
extern unsigned char DAT_009d8e29[];
extern unsigned char DAT_009d6968[];
extern unsigned int DAT_009ed228;
extern unsigned int DAT_009ed22c;
extern unsigned int DAT_009ed22d;

unsigned int __cdecl FUN_00435850(unsigned int param_1, unsigned int param_2, int param_3)
{
  unsigned char bVar1;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int local_4;
  
  iVar5 = 8;
  uVar2 = (param_2 & 0xffff) * 0x2b3;
  iVar4 = (param_2 & 0xffff) * 0x566;
  do {
    uVar2 = (uVar2 >> 8) | DAT_009d8e25[iVar4];
    if (DAT_009d8e25[iVar4] == 0x02) {
      uVar2 = DAT_009d8e26[iVar4];
      if (uVar2 == (param_1 & 0xffff)) {
        bVar1 = DAT_009d8e27[iVar4];
        uVar2 = bVar1;
        if ((bVar1 == 0x12) || ((bVar1 == 0x13) && (param_3 == 1))) {
          uVar2 = FUN_00435a10(DAT_009ed228, DAT_009ed22c, DAT_009ed22d);
          if (((unsigned int)DAT_009d8e33[iVar4] * 5 == 0) ||
             (uVar2 <= (unsigned int)DAT_009d8e33[iVar4] * 5)) {
            local_4 = (char)DAT_009d8e28[iVar4];
          }
          uVar2 = 0;
          iVar3 = 0;
          local_4 = local_4 * 0xfa;
          do {
            if (DAT_009d8e29[iVar3 + iVar4] != 0) {
              DAT_009d6968[iVar3] = DAT_009d6968[iVar3] + local_4;
            }
            iVar3 = iVar3 + 1;
          } while (iVar3 < 10);
        }
      }
    }
    iVar4 = iVar4 + 0x4e;
    iVar5 = iVar5 - 1;
  } while (iVar5 != 0);
  return uVar2;
}
