/* SPDX-License-Identifier: MIT */
/* z_bop 004185b0 */
extern int DAT_00523428;
extern int DAT_009a8db0;
extern int DAT_009ed670;
extern char DAT_009e9680;
extern int DAT_009ecc34;
extern char DAT_009ec468;

void __cdecl FUN_004186e0(int param_1, int *param_2);

void __stdcall FUN_004185b0(void)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  char *pcStack_8;
  int *piStack_4;
  
  if (DAT_00523428 == 0) {
    iVar1 = 0;
    piVar4 = &DAT_009a8db0;
    piStack_4 = &DAT_009ed670;
    pcStack_8 = &DAT_009e9680;
    do {
      if ((*pcStack_8 != '\0') && (iVar1 != DAT_009ecc34)) {
        iVar3 = 0;
        piVar2 = piStack_4;
        if (0 < *piVar4) {
          do {
            if ((*piVar2 == 0) && (piVar2[1] == 2)) {
              FUN_004186e0(iVar1, piVar2);
            }
            else {
              piVar2 = (int *)((int)piVar2 + 0x12);
              iVar3 = iVar3 + 1;
            }
          } while (iVar3 < *piVar4);
        }
      }
      piVar4 = piVar4 + 1;
      piStack_4 = piStack_4 + 0x7e9;
      pcStack_8 = pcStack_8 + 0x5bd;
      iVar1 = iVar1 + 1;
    } while (pcStack_8 < &DAT_009ec468);
  }
  return;
}
