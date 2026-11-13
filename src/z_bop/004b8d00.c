/* SPDX-License-Identifier: MIT */
/* z_bop 004b8d00 */
void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b8ea0(unsigned char *param_1, int *param_2, int param_3);

extern unsigned char DAT_00bb72cc;
extern unsigned char DAT_00bb72c8;

void __cdecl FUN_004b8d00(int param_1)
{
  unsigned char *pbVar1;
  unsigned char bVar2;
  char *pcVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  unsigned int uVar7;
  int local_10[4];
  
  FUN_004d6460(local_10, 0x164, 0x167, 600, 0x175);
  piVar4 = local_10;
  uVar7 = 0xffff;
  iVar6 = 1;
  iVar5 = 0;
  pcVar3 = FUN_004ddc10(0x322);
  FUN_004db7d0(0xc, pcVar3, piVar4, iVar5, iVar6, uVar7);
  FUN_004d64a0(local_10, 0, 0xf);
  pbVar1 = &DAT_00bb72cc + param_1;
  bVar2 = *pbVar1;
  FUN_004b8ea0(pbVar1, local_10, 0xec);
  if ((*pbVar1 != bVar2) && (*pbVar1 != 0)) {
    (&DAT_00bb72c8)[param_1] = 1;
  }
  return;
}
