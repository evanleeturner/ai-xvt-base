/* SPDX-License-Identifier: MIT */
/* z_xvt 00421a80 */
extern unsigned int DAT_00732a38;
extern int DAT_00703c88;
extern int DAT_006d4988;
extern unsigned int DAT_007394b0;
extern unsigned int DAT_007394b4;
extern unsigned int DAT_007394b8;
extern unsigned short DAT_007394bc;
extern unsigned short DAT_007394be;
extern unsigned short DAT_007394c0;
extern short DAT_006db6c0;
extern short DAT_006d4a6c;

unsigned int FUN_00421a80(unsigned short param_1, short param_2, unsigned char param_3)
{
  unsigned short uVar5;
  unsigned int uVar2;
  int iVar3;
  unsigned short uVar4;
  unsigned int tmp_int;
  unsigned short tmp_short;
  unsigned char tmp_char;

  uVar2 = DAT_00732a38 & 0xFFFF;
  uVar5 = (unsigned short)DAT_00732a38;
  while ((int)uVar2 < (int)(DAT_00703c88 + DAT_00732a38)) {
    iVar3 = DAT_006d4988;
    if (*(char *)((unsigned int)uVar5 * 0x23 + 3 + DAT_006d4988) == 0) {
      goto LAB_00421ad3;
    }
    uVar5 = uVar5 + 1;
    uVar2 = (unsigned int)uVar5;
  }
  iVar3 = 0;
  uVar5 = 0xFFFF;
LAB_00421ad3:
  uVar4 = (unsigned short)((unsigned int)iVar3 >> 16);
  if (uVar5 != 0xFFFF) {
    iVar3 = (unsigned int)uVar5 * 0x23;
    __builtin_memcpy((void*)(DAT_006d4988 + 4 + iVar3), &DAT_007394b0, 4);
    __builtin_memcpy((void*)(DAT_006d4988 + 8 + iVar3), &DAT_007394b4, 4);
    __builtin_memcpy((void*)(DAT_006d4988 + 0xC + iVar3), &DAT_007394b8, 4);
    __builtin_memcpy((void*)(DAT_006d4988 + 0x10 + iVar3), &DAT_007394bc, 2);
    __builtin_memcpy((void*)(DAT_006d4988 + 0x12 + iVar3), &DAT_007394be, 2);
    __builtin_memcpy((void*)(DAT_006d4988 + 0x14 + iVar3), &DAT_007394c0, 2);

    __builtin_memcpy(&tmp_int, (void*)(DAT_006d4988 + 4 + iVar3), 4);
    tmp_int = tmp_int << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 4 + iVar3), &tmp_int, 4);

    __builtin_memcpy(&tmp_int, (void*)(DAT_006d4988 + 8 + iVar3), 4);
    tmp_int = tmp_int << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 8 + iVar3), &tmp_int, 4);

    __builtin_memcpy(&tmp_int, (void*)(DAT_006d4988 + 0xC + iVar3), 4);
    tmp_int = tmp_int << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 0xC + iVar3), &tmp_int, 4);

    __builtin_memcpy(&tmp_short, (void*)(DAT_006d4988 + 0x10 + iVar3), 2);
    tmp_short = tmp_short << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x10 + iVar3), &tmp_short, 2);

    __builtin_memcpy(&tmp_short, (void*)(DAT_006d4988 + 0x12 + iVar3), 2);
    tmp_short = tmp_short << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x12 + iVar3), &tmp_short, 2);

    __builtin_memcpy(&tmp_short, (void*)(DAT_006d4988 + 0x14 + iVar3), 2);
    tmp_short = tmp_short << 8;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x14 + iVar3), &tmp_short, 2);

    __builtin_memcpy((void*)(DAT_006d4988 + iVar3), &DAT_006db6c0, 2);
    DAT_006db6c0 = DAT_006db6c0 + 1;

    tmp_char = (unsigned char)param_1;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x16 + iVar3), &tmp_char, 1);

    tmp_char = (unsigned char)param_2;
    __builtin_memcpy((void*)(DAT_006d4988 + 2 + iVar3), &tmp_char, 1);

    tmp_char = param_3;
    __builtin_memcpy((void*)(DAT_006d4988 + 3 + iVar3), &tmp_char, 1);

    tmp_short = 0x3FF;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x17 + iVar3), &tmp_short, 2);

    tmp_char = 0;
    __builtin_memcpy((void*)(DAT_006d4988 + 0x19 + iVar3), &tmp_char, 1);

    if (param_2 == 8) {
      __builtin_memcpy(&tmp_short, (void*)((char*)&DAT_006d4a6c + (unsigned int)param_1 * 0x126), 2);
      tmp_char = ((unsigned char)tmp_short & 7) * 0x1D;
      __builtin_memcpy((void*)(DAT_006d4988 + 0x1A + iVar3), &tmp_char, 1);
    }
    else {
      tmp_char = 0;
      __builtin_memcpy((void*)(DAT_006d4988 + 0x1A + iVar3), &tmp_char, 1);
    }

    uVar4 = (unsigned short)((unsigned int)param_1 * 0x93 >> 16);

    __builtin_memcpy(&tmp_short, (void*)((char*)&DAT_006d4a6c + (unsigned int)param_1 * 0x126), 2);
    tmp_short = tmp_short + 1;
    __builtin_memcpy((void*)((char*)&DAT_006d4a6c + (unsigned int)param_1 * 0x126), &tmp_short, 2);
  }
  return (uVar4 << 16) | uVar5;
}
