/* SPDX-License-Identifier: MIT */
/* z_bop 00403470 */
extern int _DAT_00a07cec;
extern unsigned int DAT_009a1fe0;
extern char *DAT_009a1fe8;

void __cdecl FUN_00403470(unsigned int param_1, unsigned char param_2, unsigned char param_3)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  int local_ptr;
  int local_ptr2;
  
  if (_DAT_00a07cec < (int)DAT_009a1fe0) {
    iVar3 = _DAT_00a07cec * 0x23;
    uVar2 = _DAT_00a07cec;
    do {
      if ((*(char *)(DAT_009a1fe8 + 3 + iVar3) != '\0') &&
         (*(unsigned char *)(DAT_009a1fe8 + iVar3 + 0x16) == param_1)) {
        __builtin_memcpy(&local_ptr, DAT_009a1fe8 + iVar3 + 0x1f, 4);
        __builtin_memcpy(&local_ptr2, (char *)local_ptr + 0xa9, 4);
        iVar1 = local_ptr2;
        *(unsigned char *)(iVar1 + 0xae) = param_2;
        *(unsigned char *)(iVar1 + 0xaf) = param_3;
      }
      iVar3 = iVar3 + 0x23;
      uVar2 = uVar2 + 1;
    } while (uVar2 < DAT_009a1fe0);
  }
  return;
}
