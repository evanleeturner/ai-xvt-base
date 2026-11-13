/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8e80 */
extern int DAT_00532adc;
extern int DAT_0064f688;
void * __cdecl memcpy(void *, const void *, unsigned int);

int __cdecl FUN_004d8e80(int param_1)
{
  int *piVar1;
  unsigned int uVar2;
  
  uVar2 = 0;
  if (DAT_00532adc != 0) {
    piVar1 = &DAT_0064f688;
    do {
      if (*piVar1 == param_1) {
        memcpy((void *)piVar1, (const void *)(piVar1 + 1), (unsigned int)((DAT_00532adc - uVar2) * 4 - 4));
        DAT_00532adc = DAT_00532adc - 1;
        return 1;
      }
      piVar1 = piVar1 + 1;
      uVar2 = uVar2 + 1;
    } while (uVar2 < (unsigned int)DAT_00532adc);
  }
  return 0;
}
