/* SPDX-License-Identifier: MIT */
/* z_bop 0040e0c0 */
extern int DAT_005233d8;
extern int DAT_005233f4;
extern int *DAT_005505e4;
extern int *DAT_005505e0;
extern int DAT_009d8c08;
extern int DAT_009cc460[];
extern int DAT_0054f9e0[];
extern int _DAT_0054f9b0;

extern int __stdcall FUN_004ac7a0(void);

void __cdecl FUN_0040e0c0(int param_1, int param_2, unsigned int param_3, int param_4)
{
  int iVar1;
  unsigned int i;

  iVar1 = DAT_005233d8;
  if (param_1 == 0) {
    DAT_005233f4 = FUN_004ac7a0();
    DAT_005505e4 = &DAT_009d8c08;
    DAT_005505e0 = DAT_009cc460;
    return;
  }
  DAT_005233f4 = param_1;
  if (param_4 != -1) {
    i = 0;
    if (param_3 != 0) {
      do {
        DAT_0054f9e0[i] = param_2 * iVar1 * i;
        i = i + 1;
      } while (i < param_3);
    }
    _DAT_0054f9b0 = param_4;
    DAT_005505e4 = &_DAT_0054f9b0;
    DAT_005505e0 = DAT_0054f9e0;
    return;
  }
  i = 0;
  if (param_3 != 0) {
    do {
      DAT_009cc460[i] = DAT_009d8c08 * i;
      i = i + 1;
    } while (i < param_3);
  }
  DAT_005505e4 = &DAT_009d8c08;
  DAT_005505e0 = DAT_009cc460;
  return;
}
