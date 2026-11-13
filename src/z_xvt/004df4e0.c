/* SPDX-License-Identifier: MIT */
/* z_xvt 004df4e0 */
extern int DAT_00759657;
extern unsigned char DAT_00759ad3;
extern char DAT_007596d3[];
extern unsigned char DAT_00759acf;
extern unsigned char DAT_00759ad0;
extern unsigned char DAT_00759ad1;
extern int _DAT_00759ad4;
extern int _DAT_00759ad8;
extern int _DAT_00759adc;
extern int _DAT_00759ae0;
extern int _DAT_00759ae4;
extern int _DAT_00759ae8;

int FUN_004df760(int, int, int);

int __cdecl FUN_004df4e0(int *param_1, int param_2)
{
  int iVar1;
  char *puVar2;
  int local_val;
  
  if (DAT_00759657 != 8) {
    return 0;
  }
  DAT_00759ad3 = 1;
  puVar2 = DAT_007596d3;
  for (iVar1 = 0x100; iVar1 != 0; iVar1--) {
    __builtin_memcpy(&local_val, param_1, 4);
    __builtin_memcpy(puVar2, &local_val, 4);
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 4;
  }
  if (param_2 != 0) {
    DAT_007596d3[0] = 0;
    DAT_007596d3[1] = 0;
    DAT_007596d3[2] = 0;
    DAT_00759acf = 0xff;
    DAT_00759ad0 = 0xff;
    DAT_00759ad1 = 0xff;
  }
  _DAT_00759ad4 = 0xffff;
  _DAT_00759ad8 = FUN_004df760(0, 0xff, 0);
  _DAT_00759adc = FUN_004df760(0xff, 0, 0);
  _DAT_00759ae0 = FUN_004df760(0xff, 0xff, 0);
  _DAT_00759ae4 = FUN_004df760(0x32, 0x32, 0xff);
  _DAT_00759ae8 = FUN_004df760(0x80, 0x80, 0xff);
  return 1;
}
