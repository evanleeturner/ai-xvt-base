/* SPDX-License-Identifier: MIT */
/* z_bop 004b83a0 */
extern int DAT_00664e9c;

void __cdecl FUN_004d6460(int *param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4, unsigned int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db640(unsigned int param_1, char *param_2, int *param_3, unsigned int param_4);
void __cdecl FUN_004b84e0(int param_1);
void __cdecl FUN_004b8570(int param_1);
void __cdecl FUN_004b8600(int param_1);
void __cdecl FUN_004b86e0(int param_1);
void __cdecl FUN_004b8760(int param_1);
void __cdecl FUN_004b87e0(int param_1);
void __cdecl FUN_004b8860(int param_1);
void __cdecl FUN_004b88e0(int param_1);
void __cdecl FUN_004b8960(int param_1);
void __cdecl FUN_004b89e0(int param_1);
void __cdecl FUN_004b8a60(int param_1);
void __cdecl FUN_004b8b20(int param_1);
void __cdecl FUN_004b8ba0(int param_1);
void __cdecl FUN_004b8c80(int param_1);
void __cdecl FUN_004b8d00(int param_1);
void __cdecl FUN_004b8da0(int param_1);

void __stdcall FUN_004b83a0(void)
{
  char *pcVar1;
  unsigned int uVar2;
  int *piVar3;
  unsigned int uVar4;
  int local_10[4];
  
  FUN_004d6460(local_10, 0x54, 0x5a, 0x25c, 0x6a);
  piVar3 = local_10;
  uVar4 = 0xffff;
  if (DAT_00664e9c == 1) {
    uVar2 = 0xdb;
  }
  else {
    uVar2 = 0x281;
  }
  pcVar1 = FUN_004ddc10(uVar2);
  FUN_004db640(0xf, pcVar1, piVar3, uVar4);
  FUN_004b84e0(DAT_00664e9c - 1);
  FUN_004b8570(DAT_00664e9c - 1);
  FUN_004b8600(DAT_00664e9c - 1);
  FUN_004b86e0(DAT_00664e9c - 1);
  FUN_004b8760(DAT_00664e9c - 1);
  FUN_004b87e0(DAT_00664e9c - 1);
  FUN_004b8860(DAT_00664e9c - 1);
  FUN_004b88e0(DAT_00664e9c - 1);
  FUN_004b8960(DAT_00664e9c - 1);
  FUN_004b89e0(DAT_00664e9c - 1);
  FUN_004b8a60(DAT_00664e9c - 1);
  FUN_004b8b20(DAT_00664e9c - 1);
  FUN_004b8ba0(DAT_00664e9c - 1);
  FUN_004b8c80(DAT_00664e9c - 1);
  FUN_004b8d00(DAT_00664e9c - 1);
  FUN_004b8da0(DAT_00664e9c - 1);
  return;
}
