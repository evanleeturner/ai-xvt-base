/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5c5c */
extern void *DAT_0052f1c4;
extern int DAT_0052f204;
extern void *DAT_0064ee60;

void __cdecl FUN_0045bbe0(void);
void __cdecl FID_conflict__memcpy(void *dst, const void *src, unsigned int len);

unsigned int __cdecl FUN_004b5c5c(void *param_1, int param_2)
{
  unsigned int uVar1;
  
  if (*(unsigned int *)(DAT_0052f1c4 + 0x18) < (unsigned int)(DAT_0052f204 + param_2)) {
    FUN_0045bbe0();
    uVar1 = 0;
  }
  else {
    if (param_1 != DAT_0064ee60) {
      FID_conflict__memcpy(DAT_0064ee60, param_1, param_2 << 5);
    }
    DAT_0064ee60 = (void *)((int)DAT_0064ee60 + param_2 * 0x20);
    DAT_0052f204 = DAT_0052f204 + param_2;
    uVar1 = 1;
  }
  return uVar1;
}
