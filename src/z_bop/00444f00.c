/* SPDX-License-Identifier: MIT */
/* z_bop 00444f00 */
void __cdecl FUN_00444f90(uint param_1);
void __cdecl FUN_00443890(uint param_1, uint param_2, uint param_3);

extern uint DAT_00556928;
extern uint DAT_00556930;
extern uint DAT_00556949;
extern uint DAT_0055694d;
extern double DAT_00556951;
extern uint DAT_00556955;

void __cdecl FUN_00444f00(uint param_1)
{
  double dVar1;
  
  DAT_00556951 = 1.0;
  dVar1 = DAT_00556951;
  while (param_1 < DAT_00556930) {
    DAT_00556955 = (uint)((*(unsigned long long*)&dVar1) >> 32);
    *(uint*)&DAT_00556951 = *(uint*)&dVar1;
    DAT_0055694d = DAT_00556955;
    DAT_00556949 = *(uint*)&DAT_00556951;
    DAT_00556951 = *(double *)(DAT_00556928 + 0xe) - 1.0;
    DAT_00556930 = 0;
    FUN_00444f90(DAT_00556928);
    FUN_00443890(DAT_00556928, DAT_00556949, DAT_0055694d);
    dVar1 = DAT_00556951;
  }
  DAT_00556951 = dVar1;
  return;
}
