/* SPDX-License-Identifier: MIT */
/* z_bop 004fa810 */
extern short DAT_00aa5aac;
extern unsigned char DAT_00b6a5a6;
extern int DAT_00aa5cb0[];
extern int DAT_00aa5ac0[];
extern int DAT_00aa5e10[];
extern int DAT_00b69e30;
extern unsigned int DAT_00b6a2a0;
extern unsigned int DAT_00b6a2c0;
extern unsigned char DAT_00b6a270[];
extern char DAT_00a91d24[];
extern char DAT_00b69d20[];
extern char DAT_0052d0b0[];
extern char s__c_s__c_s_0052a6b0[];
extern char s__u___s_0052d0a8[];
extern unsigned char DAT_00aa6150[];
extern unsigned char DAT_00aa615e[];
extern unsigned char DAT_00aa6162[];
extern unsigned char DAT_00aa62aa[];

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(int param_1);
void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
int __cdecl FUN_004d11e0(void);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d4cf0(unsigned int *param_1);
void __cdecl FUN_004d4d20(void *param_1);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);

unsigned int __cdecl FUN_004fa810(int param_1)
{
  char *pcVar1;
  int iVar2;
  int iVar3;
  unsigned char *piVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  char *local_34;
  int local_30[4];
  int local_20[4];
  unsigned int local_10[4];
  int tmp;
  unsigned int utmp;

  FUN_004d6460(local_30, 0x58, 0x14f, 0x1bb, 0x15d);
  uVar9 = 0xffff;
  pcVar1 = FUN_004ddc10(0x273);
  FUN_004db640(0xc, pcVar1, local_30, uVar9);
  iVar2 = FUN_004d11e0();
  if ((iVar2 == DAT_00aa5cb0[DAT_00b6a5a6 * 8]) || (DAT_00b69e30 == 2)) {
    FUN_004d6460(local_30, 0x58, 0x1a6, 0x1bb, 0x1b4);
    uVar9 = DAT_00b6a2a0;
    pcVar1 = FUN_004ddc10(0x2e5);
    FUN_004db640(0xc, pcVar1, local_30, uVar9);
  }
  iVar7 = 0;
  iVar2 = 0;
  FUN_004d6460(local_30, 0x58, 0x160, 0x108, 0x170);
  if (DAT_00aa5aac < 1) {
    return 1;
  }
  local_34 = DAT_00a91d24;
LAB_004fa8f7:
  if ((*local_34 != '\0') && ((unsigned char)local_34[-0xc] == DAT_00b6a5a6)) {
    iVar5 = 0;
    if (0 < DAT_00aa5e10[DAT_00b6a5a6]) {
      do {
        if (DAT_00aa5ac0[DAT_00b6a5a6 * 8 + iVar5] == iVar2) {
          iVar6 = 0;
          piVar4 = DAT_00aa615e;
          goto LAB_004fa958;
        }
        iVar5 = iVar5 + 1;
      } while (iVar5 < DAT_00aa5e10[DAT_00b6a5a6]);
    }
    goto LAB_004faa91;
  }
  goto LAB_004fab1a;
  while (1) {
    piVar4 = piVar4 + 0x2a;
    iVar6 = iVar6 + 1;
    if ((unsigned char *)(DAT_00aa62aa + 3) < piVar4) break;
LAB_004fa958:
    __builtin_memcpy(&tmp, piVar4, 4);
    if (tmp == DAT_00aa5cb0[DAT_00b6a5a6 * 8 + iVar5]) {
      _sprintf(DAT_00b69d20, DAT_0052d0b0, iVar7 + 1);
      FUN_004d6480(local_20, local_30);
      local_20[2] = (int)((unsigned int)local_20[0] + 0xf);
      FUN_004db7d0(0xc, DAT_00b69d20, local_20, 0, 1, 0xffff);
      local_20[0] = (int)((unsigned int)local_30[0] + 0xf);
      local_20[2] = local_30[2];
      iVar3 = (int)(DAT_00aa6150 + iVar6 * 0x2a);
      uVar8 = 1;
      __builtin_memcpy(&utmp, DAT_00aa6162 + iVar6 * 0x2a, 4);
      pcVar1 = FUN_004ddc10((int)(utmp + 0x9a));
      _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar1, uVar8, (char *)iVar3);
      FUN_004d4cf0(local_10);
      FUN_004d4d20(local_30);
      iVar3 = FUN_004d11e0();
      __builtin_memcpy(&tmp, DAT_00aa615e + iVar6 * 0x2a, 4);
      if ((iVar3 == tmp) ||
          (uVar9 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
        uVar9 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2);
      }
      FUN_004db7d0(0xc, DAT_00b69d20, local_20, 0, 1, uVar9);
      FUN_004d4d20(local_10);
      break;
    }
  }
LAB_004faa91:
  if (DAT_00aa5e10[DAT_00b6a5a6] == iVar5) {
    pcVar1 = FUN_004ddc10(0x321);
    _sprintf(DAT_00b69d20, s__u___s_0052d0a8, iVar7 + 1, pcVar1);
    FUN_004db7d0(0xc, DAT_00b69d20, local_30, 0, 1, 0xffff);
  }
  iVar7 = iVar7 + 1;
  if (iVar7 == 4) {
    FUN_004d6460(local_30, 0x10b, 0x160, 0x1bb, 0x170);
  }
  else {
    FUN_004d64a0(local_30, 0, 0x11);
  }
LAB_004fab1a:
  if (DAT_00aa5e10[DAT_00b6a5a6] <= iVar7) {
    return 1;
  }
  local_34 = local_34 + 0x562;
  iVar2 = iVar2 + 1;
  if (DAT_00aa5aac <= iVar2) {
    return 1;
  }
  goto LAB_004fa8f7;
}
