/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a0d0 */
extern int _DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;

void __cdecl FUN_0042a0d0(unsigned int param_1, unsigned char param_2, unsigned char param_3)
{
  int iVar1;
  int uVar2;
  int iVar3;
  
  if (_DAT_00732a3c < DAT_006d4980) {
    iVar3 = _DAT_00732a3c * 0x23;
    uVar2 = _DAT_00732a3c;
    do {
      char c;
      __builtin_memcpy(&c, (void *)(DAT_006d4988 + 3 + iVar3), 1);
      unsigned char b;
      __builtin_memcpy(&b, (void *)(DAT_006d4988 + iVar3 + 0x16), 1);
      if ((c != '\0') && (b != param_1)) {
        int ptr_val;
        __builtin_memcpy(&ptr_val, (void *)(DAT_006d4988 + iVar3 + 0x1f), 4);
        int inner_ptr;
        __builtin_memcpy(&inner_ptr, (void *)(ptr_val + 0xa7), 4);
        iVar1 = inner_ptr;
        *(unsigned char *)(iVar1 + 0xae) = param_2;
        *(unsigned char *)(iVar1 + 0xaf) = param_3;
      }
      iVar3 = iVar3 + 0x23;
      uVar2 = uVar2 + 1;
    } while (uVar2 < DAT_006d4980);
  }
  return;
}
