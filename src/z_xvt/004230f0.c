/* SPDX-License-Identifier: MIT */
/* z_xvt 004230f0 */
extern int DAT_00732a38;
extern int DAT_007091a0;
extern int DAT_006d4988;
extern int DAT_007394e4;
extern int DAT_006db71c;
extern int DAT_00732940;
extern int DAT_0073abd0;
extern int DAT_006db798;

void __stdcall FUN_004230f0(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int val_pi_minus_1;
  int val_pi_0;
  int val_pi_1;
  int val_73abd0;
  int val_7091a0_a7;
  int dst_val;
  
  iVar1 = 0;
  if (0 < DAT_00732a38) {
    iVar3 = 0;
    iVar2 = 0;
    do {
      iVar5 = DAT_007091a0 + iVar3;
      iVar2 = iVar2 + 0x23;
      iVar3 = iVar3 + 0xaf;
      iVar1 = iVar1 + 1;
      __builtin_memcpy((void *)(DAT_006d4988 + -4 + iVar2), &iVar5, 4);
    } while (iVar1 < DAT_00732a38);
  }
  iVar1 = 0;
  iVar2 = 0;
  if (0 < DAT_00732a38) {
    piVar4 = (int *)&DAT_007394e4;
    do {
      __builtin_memcpy(&val_pi_minus_1, (void *)(piVar4 - 1), 4);
      if (val_pi_minus_1 == -1) {
        __builtin_memcpy(&val_pi_0, (void *)piVar4, 4);
        if (val_pi_0 == -1) {
          __builtin_memcpy(&val_pi_1, (void *)(piVar4 + 1), 4);
          if (val_pi_1 != -1) {
            dst_val = DAT_006db71c + val_pi_1 * 0x4c;
            __builtin_memcpy((void *)(DAT_007091a0 + 0xab + iVar1), &dst_val, 4);
          }
        }
        else {
          dst_val = val_pi_0 * 0x436 + DAT_00732940;
          __builtin_memcpy((void *)(DAT_007091a0 + 0xa7 + iVar1), &dst_val, 4);
          __builtin_memcpy(&val_73abd0, (void *)(&DAT_0073abd0 + val_pi_0), 4);
          if (val_73abd0 != -1) {
            __builtin_memcpy(&val_7091a0_a7, (void *)(DAT_007091a0 + 0xa7 + iVar1), 4);
            dst_val = val_73abd0 * 0x23 + DAT_006d4988;
            __builtin_memcpy((void *)(val_7091a0_a7 + 0x432), &dst_val, 4);
          }
        }
      }
      else {
        dst_val = DAT_006db798 + val_pi_minus_1 * 8;
        __builtin_memcpy((void *)(DAT_007091a0 + 0xa3 + iVar1), &dst_val, 4);
      }
      piVar4 = piVar4 + 3;
      iVar1 = iVar1 + 0xaf;
      iVar2 = iVar2 + 1;
    } while (iVar2 < DAT_00732a38);
  }
  return;
}
