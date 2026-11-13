/* SPDX-License-Identifier: MIT */
/* z_xvt 00428980 */
void __cdecl FUN_00428980(unsigned char *param_1);

extern int DAT_00536ef4;
extern int DAT_00536f25;

void __cdecl FUN_00428980(unsigned char *param_1)
{
  int iVar1;
  unsigned char *ptr_child;
  int ptr_base;
  unsigned char byte_val;
  int int_val;
  int int_src;
  double dbl_val;
  double dbl_src;

  if (param_1[2] != 0) {
    iVar1 = 0;
    do {
      if ((1 << (iVar1 & 0x1f)) & (unsigned int)param_1[2]) {
        __builtin_memcpy(&ptr_child, param_1 + iVar1 * 4 + 0x32, sizeof(ptr_child));
        FUN_00428980(ptr_child);
      }
      iVar1 = iVar1 + 1;
    } while (iVar1 < 8);
  }
  if (param_1[1] == DAT_00536ef4) {
    __builtin_memcpy(&ptr_base, param_1 + 0x2e, sizeof(ptr_base));
    
    __builtin_memcpy(&byte_val, (unsigned char *)(ptr_base + 2), sizeof(byte_val));
    byte_val = byte_val & ~(1 << (*param_1 & 0x1f));
    __builtin_memcpy((unsigned char *)(ptr_base + 2), &byte_val, sizeof(byte_val));
    
    __builtin_memcpy(&int_val, (unsigned char *)(ptr_base + 10), sizeof(int_val));
    __builtin_memcpy(&int_src, param_1 + 10, sizeof(int_src));
    int_val = int_val + int_src;
    __builtin_memcpy((unsigned char *)(ptr_base + 10), &int_val, sizeof(int_val));
    
    __builtin_memcpy(&dbl_val, (unsigned char *)(ptr_base + 0x16), sizeof(dbl_val));
    __builtin_memcpy(&dbl_src, param_1 + 0x16, sizeof(dbl_src));
    dbl_val = dbl_src + dbl_val;
    __builtin_memcpy((unsigned char *)(ptr_base + 0x16), &dbl_val, sizeof(dbl_val));
    
    __builtin_memcpy(&dbl_val, (unsigned char *)(ptr_base + 0x1e), sizeof(dbl_val));
    __builtin_memcpy(&dbl_src, param_1 + 0x1e, sizeof(dbl_src));
    dbl_val = dbl_src + dbl_val;
    __builtin_memcpy((unsigned char *)(ptr_base + 0x1e), &dbl_val, sizeof(dbl_val));
    
    __builtin_memcpy(&dbl_val, (unsigned char *)(ptr_base + 0x26), sizeof(dbl_val));
    __builtin_memcpy(&dbl_src, param_1 + 0x26, sizeof(dbl_src));
    dbl_val = dbl_src + dbl_val;
    __builtin_memcpy((unsigned char *)(ptr_base + 0x26), &dbl_val, sizeof(dbl_val));
    
    DAT_00536f25 = DAT_00536f25 - 1;
  }
  return;
}
