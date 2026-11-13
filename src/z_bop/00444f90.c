/* SPDX-License-Identifier: MIT */
/* z_bop 00444f90 */
typedef unsigned char byte;
typedef unsigned int uint;

extern int DAT_00556930;
extern double DAT_00556949;
extern uint DAT_00556951;
extern uint DAT_00556955;

void FUN_00444eb0(byte *param_1);
void FUN_00444f90(byte *param_1);

void FUN_00444f90(byte *param_1)
{
  uint uVar1;
  double d_0e;
  int i_0a;
  uint u_12;
  uint u_0e_low;
  double g_556949;
  uint g_556951;
  uint g_556955;
  union {
    uint parts[2];
    double val;
  } u_thresh;
  byte *ptr_arg;

  __builtin_memcpy(&d_0e, param_1 + 0xe, 8);
  __builtin_memcpy(&i_0a, param_1 + 10, 4);
  __builtin_memcpy(&u_12, param_1 + 0x12, 4);
  __builtin_memcpy(&u_0e_low, param_1 + 0xe, 4);

  __builtin_memcpy(&g_556949, &DAT_00556949, 8);
  __builtin_memcpy(&g_556951, &DAT_00556951, 4);
  __builtin_memcpy(&g_556955, &DAT_00556955, 4);

  u_thresh.parts[0] = g_556951;
  u_thresh.parts[1] = g_556955;

  if (param_1[2] != 0) {
    uVar1 = 0;
    do {
      if (((1 << ((byte)uVar1 & 0x1f)) & (uint)param_1[2]) != 0) {
        __builtin_memcpy(&ptr_arg, param_1 + uVar1 * 4 + 0x32, 4);
        FUN_00444f90(ptr_arg);
      }
      uVar1 = uVar1 + 1;
    } while (uVar1 < 8);
  }
  if (d_0e <= g_556949) {
    FUN_00444eb0(param_1);
    return;
  }
  if (i_0a != 0) {
    DAT_00556930 = DAT_00556930 + 1;
  }
  if (d_0e < u_thresh.val) {
    __builtin_memcpy(&DAT_00556955, &u_12, 4);
    __builtin_memcpy(&DAT_00556951, &u_0e_low, 4);
  }
  return;
}
