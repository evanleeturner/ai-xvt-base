/* SPDX-License-Identifier: MIT */
/* z_bop 00407e40 */
void * __cdecl FUN_004079f0(unsigned int param_1);
void __cdecl FUN_004b3280(int param_1);
void __cdecl FUN_004b0b60(unsigned int * param_1, unsigned int param_2);
int __cdecl FUN_004b2680(int * param_1, int param_2, int param_3, int param_4);
void __stdcall FUN_004079e0(void);

void * __cdecl FUN_00407e40(unsigned int param_1, unsigned int param_2, unsigned int * param_3, unsigned int param_4)
{
  void * puVar1;
  int iVar2;
  int * piVar3;
  int local_d8[0x36];

  puVar1 = FUN_004079f0(param_4);
  if (*(int *)((char *)puVar1 + 0x78) != 0) {
    FUN_004b3280((int)puVar1);
    return puVar1;
  }
  piVar3 = local_d8;
  for (iVar2 = 0x36; iVar2 != 0; iVar2 = iVar2 - 1) {
    *piVar3 = 0;
    piVar3 = piVar3 + 1;
  }
  local_d8[23] = param_4;
  local_d8[0] = 0;
  local_d8[3] = param_1;
  local_d8[4] = param_2;
  local_d8[6] = param_1;
  local_d8[9] = 8;
  local_d8[8] = 0;
  FUN_004b0b60(param_3, 0x100);
  iVar2 = FUN_004b2680(local_d8, (int)puVar1, 0, 0);
  if (iVar2 == 0) {
    FUN_004079e0();
    return (void *)0x0;
  }
  return puVar1;
}
