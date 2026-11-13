/* SPDX-License-Identifier: MIT */
/* z_bop 004658c0 */
extern int DAT_009a8c2c;
extern int DAT_00556f08;
extern int DAT_00557360;
extern unsigned char *DAT_00a60a18;
extern unsigned char *DAT_00a60a1c;

extern int __cdecl FUN_004aa7f0(void);
extern unsigned int __cdecl FUN_004aa870(void);
extern void __cdecl FUN_00463d00(void);
extern int __cdecl FUN_004ac760(void);
extern void __cdecl FUN_0046d300(unsigned int *param_1, unsigned int param_2);
extern void __cdecl FUN_00448f90(int param_1, unsigned char *param_2, int param_3);
extern int __cdecl FUN_0046f660(int param_1);
extern void __cdecl FUN_00463c50(int param_1);

int __cdecl FUN_004658c0(int param_1, int param_2)
{
  int bVar1;
  int iVar2;
  int iVar3;
  unsigned int uVar4;
  int *piVar5;
  unsigned char *pbVar6;
  int iVar7;
  int local_c;
  int local_8;
  int local_4;
  
  bVar1 = 0;
  local_4 = 0;
  local_8 = 0;
  local_c = 0x14;
  do {
    iVar7 = 0;
    iVar3 = local_8;
    do {
      if (iVar7 > 0xeb) break;
      iVar3 = FUN_004aa7f0();
      if ((iVar3 != 0) && (uVar4 = FUN_004aa870(), uVar4 == 0x1b)) {
        iVar7 = 0xec;
        local_c = 1;
        iVar3 = local_8;
        break;
      }
      iVar2 = DAT_009a8c2c;
      FUN_00463d00();
      iVar3 = FUN_004ac760();
      DAT_009a8c2c = DAT_009a8c2c + iVar3;
      iVar7 = (iVar7 - iVar2) + DAT_009a8c2c;
      iVar3 = 0;
      if (param_2 > 0) {
        piVar5 = &DAT_00556f08;
        do {
          if (*piVar5 == 0) break;
          piVar5 = piVar5 + 1;
          iVar3 = iVar3 + 1;
        } while (iVar3 < param_2);
      }
      DAT_009a8c2c = iVar2;
    } while (iVar3 != param_2);
    if (iVar3 == param_2) break;
    local_4 = local_4 + iVar7;
    if (local_4 > 0xeb) {
      local_4 = 0;
      DAT_00557360 = 0x1d;
      FUN_0046d300((unsigned int *)&DAT_00557360, 4);
      bVar1 = !bVar1;
      pbVar6 = DAT_00a60a18;
      if (bVar1) {
        pbVar6 = DAT_00a60a1c;
      }
      FUN_00448f90(3, pbVar6, 0x30);
    }
    if (local_8 == iVar3) {
      local_c = local_c - 1;
    }
    else {
      local_c = 0x14;
    }
    local_8 = iVar3;
  } while (local_c != 0);
  if (local_c != 0) {
    return 1;
  }
  iVar3 = FUN_0046f660(param_1);
  FUN_00463c50(iVar3);
  return 0;
}
