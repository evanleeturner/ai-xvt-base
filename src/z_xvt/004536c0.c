/* SPDX-License-Identifier: MIT */
/* z_xvt 004536c0 */
extern undefined4 * __cdecl FUN_0048e480(int param_1, char * param_2);
extern void __cdecl FUN_004536c0(undefined4 * param_1, int param_2);
extern float _DAT_005f09a8;
extern float _DAT_005f09ac;
extern float _DAT_005f09b0;
extern float _DAT_005f09b4;
extern float _DAT_005f09c8;
extern float _DAT_005f09cc;

void __cdecl FUN_004536c0(undefined4 *param_1, int param_2)
{
  undefined4 *puVar1;
  float *pfVar2;
  int iVar3;
  int iVar4;

  if (param_1 != (undefined4 *)0x0) {
    while (param_1[1] == 7) {
      param_1 = FUN_0048e480(param_2, (char *)param_1[5]);
      if (param_1 == (undefined4 *)0x0) {
        return;
      }
    }
    if (param_1[1] == 3) {
      iVar3 = param_1[4];
      pfVar2 = (float *)param_1[5];
      if (0 < iVar3) {
        do {
          if (_DAT_005f09b0 < *pfVar2) {
            _DAT_005f09b0 = *pfVar2;
          }
          if (*pfVar2 < _DAT_005f09b4) {
            _DAT_005f09b4 = *pfVar2;
          }
          if (_DAT_005f09c8 < pfVar2[1]) {
            _DAT_005f09c8 = pfVar2[1];
          }
          if (pfVar2[1] < _DAT_005f09cc) {
            _DAT_005f09cc = pfVar2[1];
          }
          if (_DAT_005f09a8 < pfVar2[2]) {
            _DAT_005f09a8 = pfVar2[2];
          }
          if (pfVar2[2] < _DAT_005f09ac) {
            _DAT_005f09ac = pfVar2[2];
          }
          pfVar2 = pfVar2 + 3;
          iVar3 = iVar3 + -1;
        } while (iVar3 != 0);
      }
    }
    iVar4 = 0;
    iVar3 = 0;
    if (0 < (int)param_1[2]) {
      do {
        puVar1 = (undefined4 *)(param_1[3] + iVar4);
        iVar4 = iVar4 + 4;
        iVar3 = iVar3 + 1;
        FUN_004536c0((undefined4 *)*puVar1, param_2);
      } while (iVar3 < (int)param_1[2]);
    }
  }
  return;
}
