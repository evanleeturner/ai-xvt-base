/* SPDX-License-Identifier: MIT */
/* z_bop 004b84e0 */
void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9090(char *param_1, int *param_2, int param_3);
extern char DAT_00bb72c4;
extern char DAT_00bb72c6;

void __cdecl FUN_004b84e0(int param_1)
{
  char cVar1;
  char *pcVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  int local_10[4];
  
  FUN_004d6460(local_10, 0x58, 0x6f, 0x14c, 0x7d);
  piVar3 = local_10;
  uVar6 = 0xffff;
  iVar5 = 1;
  iVar4 = 0;
  pcVar2 = FUN_004ddc10(0xdc);
  FUN_004db7d0(0xc, pcVar2, piVar3, iVar4, iVar5, uVar6);
  FUN_004d64a0(local_10, 0, 0xf);
  pcVar2 = (char *)&DAT_00bb72c4 + param_1;
  cVar1 = *pcVar2;
  FUN_004b9090(pcVar2, local_10, 0xdd);
  if (*pcVar2 != cVar1) {
    *(char *)((char *)&DAT_00bb72c6 + param_1) = *pcVar2;
  }
  return;
}
