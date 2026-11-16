/* SPDX-License-Identifier: MIT */
/* z_bop 0040b2c0 */
extern undefined4 DAT_0051a520;
extern undefined2 DAT_009a8062;
extern undefined4 DAT_0054f994;
extern undefined4 DAT_0054f9a0;
extern undefined4 DAT_0053f970;
extern undefined4 DAT_0054f988;

void __cdecl FUN_004012c0(undefined4 param_1);
void __stdcall FUN_00482be0(void);
void __stdcall FUN_00408140(void);
void __stdcall FUN_004b1c10(void);
void __stdcall FUN_004b1c90(void);
void __cdecl FUN_004b1d10(undefined4 param_1, undefined4 param_2);
void __stdcall FUN_004b1d70(void);
void __cdecl FUN_004b1de0(undefined4 param_1, undefined4 param_2);
void __stdcall FUN_004b2020(void);
void __stdcall FUN_004b1c50(void);
void __stdcall FUN_00408110(void);

void __stdcall FUN_0040b2c0(void)
{
  if (DAT_0051a520 == 0) {
    FUN_004012c0(0);
  }
  else {
    FUN_004012c0(1);
    FUN_00482be0();
  }
  DAT_009a8062 = 0;
  if ((DAT_0054f994 != 0) && (DAT_0054f9a0 != 0)) {
    FUN_00408140();
    FUN_004b1c10();
    FUN_004b1c90();
    FUN_004b1d10(DAT_0053f970, DAT_0054f994);
    FUN_004b1d70();
    FUN_004b1de0(DAT_0054f988, DAT_0054f9a0);
    FUN_004b2020();
    FUN_004b1c50();
    FUN_00408110();
    return;
  }
  return;
}
