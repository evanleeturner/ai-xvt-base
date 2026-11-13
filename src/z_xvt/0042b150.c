/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b150 */
int __cdecl FUN_0042b150(int param_1);

int __cdecl FUN_0042b150(int param_1)
{
  short *psVar1;
  int iVar2;
  int tmp_ptr;
  int tmp_1f;
  int tmp_ab;
  int tmp_a7;
  int tmp_nested;
  
  if (param_1 == 0) {
    return 0;
  }
  __builtin_memcpy(&tmp_ptr, (char *)param_1 + 0x432, 4);
  psVar1 = (short *)tmp_ptr;
  if (psVar1 == (short *)0x0) {
    return param_1 + 0x2a;
  }
  if (*(short *)(param_1 + 900) != *psVar1) {
    tmp_ptr = 0;
    __builtin_memcpy((char *)param_1 + 0x432, &tmp_ptr, 4);
    return param_1 + 0x2a;
  }
  __builtin_memcpy(&tmp_1f, (char *)psVar1 + 0x1f, 4);
  iVar2 = tmp_1f;
  if (iVar2 == 0) {
    return param_1 + 0x2a;
  }
  __builtin_memcpy(&tmp_ab, (char *)iVar2 + 0xab, 4);
  if (tmp_ab != 0) {
    return tmp_ab + 4;
  }
  __builtin_memcpy(&tmp_a7, (char *)iVar2 + 0xa7, 4);
  if (tmp_a7 != 0) {
    if (*(char *)(tmp_a7 + 6) == '\0') {
      *(char *)(param_1 + 6) = 1;
      __builtin_memcpy(&tmp_nested, (char *)tmp_1f + 0xa7, 4);
      iVar2 = FUN_0042b150(tmp_nested);
      *(char *)(param_1 + 6) = 0;
      return iVar2;
    }
    return param_1 + 0x2a;
  }
  return param_1 + 0x2a;
}
