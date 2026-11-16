/* SPDX-License-Identifier: MIT */
/* z_xvt 00491150 */
extern int DAT_006a860c;
extern int DAT_0065865c;
extern int DAT_006a866c;

unsigned char FUN_00491010(char *param_1);

unsigned char __stdcall FUN_00491150(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  int *piVar3;
  
  bVar2 = 1;
  piVar3 = &DAT_006a860c;
  do {
    if (*piVar3 != -1) {
      bVar1 = FUN_00491010((char *)((int)&DAT_0065865c + *piVar3 * 0x145));
      bVar2 = bVar2 & bVar1;
    }
    piVar3 = piVar3 + 3;
  } while (piVar3 < &DAT_006a866c);
  return bVar2;
}
