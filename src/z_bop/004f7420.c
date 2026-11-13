/* SPDX-License-Identifier: MIT */
/* z_bop 004f7420 */
extern short DAT_0066925a;
extern short FUN_004f7380(void);
extern short FUN_004f7590(short param_2);

int FUN_004f7420(short param_1, short param_2)
{
  short uVar1;
  
  if (DAT_0066925a - param_1 == 1) {
    return 0;
  }
  if (DAT_0066925a <= param_1) {
    goto LAB_004f7454;
  }
  uVar1 = FUN_004f7380();
  while (DAT_0066925a <= param_1) {
LAB_004f7454:
    uVar1 = FUN_004f7590(param_2);
  }
  return (uVar1 | 0x10000);
}
