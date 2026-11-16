/* SPDX-License-Identifier: MIT */
/* z_xvt 004537e0 */
extern float _DAT_005f09a8;
extern float _DAT_005f09ac;
extern float _DAT_005f09b0;
extern float _DAT_005f09b4;
extern float _DAT_005f09c8;
extern float _DAT_005f09cc;

void __cdecl FUN_004536c0(int param_1, int param_2);

float __cdecl FUN_004537e0(int param_1, int param_2)
{
  int iVar2;
  int iVar3;
  float local_8;
  int count;
  int array_base;

  iVar2 = 0;
  _DAT_005f09b0 = 0.0;
  _DAT_005f09b4 = 0.0;
  _DAT_005f09c8 = 0.0;
  _DAT_005f09cc = 0.0;
  _DAT_005f09a8 = 0.0;
  _DAT_005f09ac = 0.0;
  __builtin_memcpy(&count, (void *)(param_1 + 6), 4);
  __builtin_memcpy(&array_base, (void *)(param_1 + 10), 4);
  if (count > 0) {
    iVar3 = 0;
    do {
      int *puVar1 = (int *)(array_base + iVar3);
      iVar3 += 4;
      iVar2 += 1;
      FUN_004536c0(*puVar1, param_1);
    } while (iVar2 < count);
  }
  _DAT_005f09b0 = _DAT_005f09b0 - _DAT_005f09b4;
  _DAT_005f09c8 = _DAT_005f09c8 - _DAT_005f09cc;
  _DAT_005f09a8 = _DAT_005f09a8 - _DAT_005f09ac;
  if (param_2 != 0) {
    if (param_2 == 1) {
      local_8 = _DAT_005f09b0;
    }
    if (param_2 == 2) {
      local_8 = _DAT_005f09c8;
    }
    if (param_2 == 3) {
      local_8 = _DAT_005f09a8;
    }
    return local_8;
  }
  if ((_DAT_005f09c8 < _DAT_005f09b0) && (_DAT_005f09a8 < _DAT_005f09b0)) {
    return _DAT_005f09b0;
  }
  if ((_DAT_005f09b0 < _DAT_005f09c8) && (_DAT_005f09a8 < _DAT_005f09c8)) {
    return _DAT_005f09c8;
  }
  return _DAT_005f09a8;
}
