/* SPDX-License-Identifier: MIT */
/* z_bop 0044b960 */
undefined4 __cdecl FUN_0044bad0(int *param_1, undefined4 *param_2, int param_3);

int * __cdecl FUN_0044b960(undefined4 *param_1)
{
  int iVar1;
  uint unaff_ESI;
  int *piVar2;
  undefined1 local_4[4];

  if (param_1 == (undefined4 *)0x0) {
    return (int *)0x0;
  }
  piVar2 = (int *)param_1[param_1[3] + 4];
  if (piVar2 != (int *)0x0) {
    iVar1 = (**(int (**)(int *, undefined1 *))(*piVar2 + 0x24))(piVar2, local_4);
    unaff_ESI = iVar1;
    if (iVar1 < 0) {
      unaff_ESI = 0;
    }
    if ((unaff_ESI & 1) != 0) {
      if ((int)param_1[2] < 2) {
        piVar2 = (int *)0x0;
      }
      else {
        iVar1 = param_1[3];
        param_1[3] = iVar1 + 1;
        if ((int)param_1[2] <= iVar1 + 1) {
          param_1[3] = 0;
        }
        piVar2 = (int *)param_1[param_1[3] + 4];
        iVar1 = (**(int (**)(int *, undefined1 *))(*piVar2 + 0x24))(piVar2, local_4);
        if ((-1 < iVar1) && ((unaff_ESI & 1) != 0)) {
          (**(void (**)(int *))(*piVar2 + 0x48))(piVar2);
          (**(void (**)(int *, int))(*piVar2 + 0x34))(piVar2, 0);
        }
      }
    }
    if ((piVar2 != (int *)0x0) && ((unaff_ESI & 2) != 0)) {
      iVar1 = (**(int (**)(int *))(*piVar2 + 0x50))(piVar2);
      if ((-1 < iVar1) &&
         (iVar1 = FUN_0044bad0(piVar2, (undefined4 *)*param_1, param_1[1]), iVar1 != 0)) {
        return piVar2;
      }
      piVar2 = (int *)0x0;
    }
  }
  return piVar2;
}
