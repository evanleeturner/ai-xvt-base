/* SPDX-License-Identifier: MIT */
/* z_bop 004a4cb0 */
extern unsigned short DAT_009a73b0;
extern int DAT_009a1fe8;
extern int DAT_009a73b6;
extern int DAT_00a08104;

void __cdecl FUN_004a4cb0(unsigned short param_1)
{
  short *psVar1;
  int iVar2;
  unsigned short uVar3;
  int ptr_val;
  
  psVar1 = (short *)((unsigned int)DAT_009a73b0 * 0x23 + 0x10 + DAT_009a1fe8);
  uVar3 = *psVar1 - *(short *)(DAT_009a73b6 + 0x32);
  if (0x7fff < uVar3) {
    uVar3 = -uVar3;
  }
  if (uVar3 < 0x301) {
    *psVar1 = *(short *)(DAT_009a73b6 + 0x32);
    __builtin_memcpy(&ptr_val, (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f + DAT_009a1fe8), sizeof(int));
    *(unsigned char *)(ptr_val + 0x92) = 1;
    iVar2 = ptr_val;
    *(unsigned char *)(iVar2 + 0x8b) = *(unsigned char *)(iVar2 + 0x92);
    *(unsigned char *)(DAT_00a08104 + 0xab) = 3;
    return;
  }
  *(unsigned char *)(DAT_00a08104 + 0xab) = 2;
  *(unsigned short *)(DAT_00a08104 + 0xac) = param_1;
  return;
}
