/* SPDX-License-Identifier: MIT */
/* z_xvt 004428e0 */
extern int DAT_006d4988;
extern char DAT_0052686c;
extern char DAT_0052b157;
extern char DAT_0052b159;
extern char DAT_0052b15b;
extern char DAT_0052b15d;

unsigned int __cdecl FUN_004428e0(int param_1)
{
  int iVar1;
  int iVar2;
  int tmp_int;
  unsigned char tmp_byte1;
  unsigned char tmp_byte2;
  unsigned char tmp_byte3;
  unsigned short tmp_ushort1;
  unsigned short tmp_ushort2;
  short tmp_short1;
  short tmp_short2;
  short tmp_short3;
  int sum;
  int div_result;

  iVar2 = param_1 * 0x23 + DAT_006d4988;
  __builtin_memcpy(&tmp_int, (void *)(iVar2 + 0x1f), 4);
  if (tmp_int == 0) {
    __builtin_memcpy(&tmp_byte1, (void *)(iVar2 + 3), 1);
    __builtin_memcpy(&tmp_ushort1, (void *)(&DAT_0052686c + (unsigned int)tmp_byte1 * 0x16), 2);
    return (unsigned int)tmp_ushort1;
  }
  __builtin_memcpy(&iVar1, (void *)(tmp_int + 0xa7), 4);
  if (iVar1 != 0) {
    __builtin_memcpy(&tmp_byte2, (void *)(iVar1 + 4), 1);
    iVar2 = (unsigned int)tmp_byte2 * 0xef;
    __builtin_memcpy(&tmp_short1, (void *)(&DAT_0052b159 + iVar2), 2);
    __builtin_memcpy(&tmp_short2, (void *)(&DAT_0052b15b + iVar2), 2);
    __builtin_memcpy(&tmp_short3, (void *)(&DAT_0052b15d + iVar2), 2);
    sum = (int)tmp_short1 + (int)tmp_short2 + (int)tmp_short3;
    div_result = sum / 3;
    __builtin_memcpy(&tmp_byte3, (void *)(&DAT_0052b157 + iVar2), 1);
    return (unsigned int)(div_result << (tmp_byte3 & 0x1f));
  }
  __builtin_memcpy(&tmp_byte1, (void *)(iVar2 + 3), 1);
  __builtin_memcpy(&tmp_ushort2, (void *)(&DAT_0052686c + (unsigned int)tmp_byte1 * 0x16), 2);
  return (unsigned int)tmp_ushort2;
}
