/* SPDX-License-Identifier: MIT */
/* z_bop 0046f720 */
extern int DAT_009994fc;
extern int DAT_00999538;
extern int DAT_00999570;
extern int DAT_00999560;

void * __cdecl FUN_0046f660(int param_1);

void * __cdecl FUN_0046f720(void *param_1)
{
  void *puVar1;
  int iVar2;
  int *piVar3;
  
  if (DAT_009994fc == 1) {
    return &DAT_00999538;
  }
  iVar2 = 0;
  puVar1 = (void *)DAT_009994fc;
  if (DAT_009994fc > 0) {
    piVar3 = &DAT_00999570;
    while (1) {
      puVar1 = FUN_0046f660(*piVar3);
      if (puVar1 == param_1) break;
      piVar3 = piVar3 + 10;
      iVar2 = iVar2 + 1;
      if (DAT_009994fc <= iVar2) {
        return puVar1;
      }
    }
    puVar1 = (void *)&DAT_00999560 + iVar2 * 0x28;
  }
  return puVar1;
}
