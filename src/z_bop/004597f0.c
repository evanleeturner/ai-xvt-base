/* SPDX-License-Identifier: MIT */
/* z_bop 004597f0 */
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009d1308;
extern unsigned int DAT_009a1fe8;

unsigned int FUN_004597f0(void)
{
  unsigned int uVar3 = DAT_00a07ce8 & 0xffff;
  unsigned int uVar1 = uVar3;
  unsigned int sum = DAT_009d1308 + DAT_00a07ce8;
  
  while (1) {
    if ((int)uVar1 < (int)sum) {
      return 0xffff;
    }
    if (*(char *)(uVar3 * 0x23 + 3 + DAT_009a1fe8) == '\0') {
      break;
    }
    uVar3 = (unsigned int)((unsigned short)uVar3 + 1);
    uVar1 = uVar3;
  }
  return uVar3;
}
