/* SPDX-License-Identifier: MIT */
/* z_xvt 004b91c0 */
void __cdecl FUN_004dcfd0(void *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(uint param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, uint param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9eb0(void *param_1, int *param_2, int param_3, int param_4, int param_5, int param_6, char param_7);
extern char DAT_0085e6ea;

void __cdecl FUN_004b91c0(int param_1)
{
  char *pcVar1;
  int local_buf[0x16];
  int iVar3;
  int iVar4;
  uint uVar5;
  
  FUN_004dcfd0(local_buf, 0x58, 0xf3, 0x14c, 0x101);
  uVar5 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004dcfa0(0xf2);
  FUN_004daac0(0xc, pcVar1, local_buf, iVar3, iVar4, uVar5);
  FUN_004dd030(local_buf, 0, 0xf);
  FUN_004b9eb0(&DAT_0085e6ea + param_1, local_buf, 8, 0xf3, 1, local_buf[0], ((char *)local_buf)[4]);
  return;
}
