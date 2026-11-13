/* SPDX-License-Identifier: MIT */
/* z_bop 00405790 */
extern char DAT_0051c573[];
extern char DAT_00a08104[];

void __cdecl FUN_00405790(undefined4 param_1, uint param_2, short param_3)
{
  int iVar1;
  int iVar2;
  int tmp;

  __builtin_memcpy(&iVar2, DAT_0051c573 + (uint)*(byte *)(DAT_00a08104 + 4) * 0x101, 4);
  iVar1 = (param_2 & 0xffff) * 4;
  
  __builtin_memcpy(&tmp, DAT_00a08104 + 0x211 + (param_2 & 0xffff) * 4, 4);
  tmp += (int)param_3;
  __builtin_memcpy(DAT_00a08104 + 0x211 + iVar1, &tmp, 4);
  
  __builtin_memcpy(&tmp, DAT_00a08104 + 0x211 + iVar1, 4);
  if (tmp < 0) {
    tmp = 0;
    __builtin_memcpy(DAT_00a08104 + 0x211 + iVar1, &tmp, 4);
  }
  
  __builtin_memcpy(&tmp, DAT_00a08104 + 0x211 + iVar1, 4);
  if (iVar2 * 2 < tmp) {
    tmp = iVar2 * 2;
    __builtin_memcpy(DAT_00a08104 + 0x211 + iVar1, &tmp, 4);
  }
  return;
}
