/* SPDX-License-Identifier: MIT */
/* z_xvt 0042e660 */
extern int __stdcall FUN_004a53f0(void);
extern unsigned int __stdcall FUN_004a5470(void);
extern void __cdecl FUN_0042c710(void);
extern int __stdcall FUN_0048f530(void);
extern void __cdecl FUN_004310c0(unsigned int *param_1, unsigned int param_2);
extern void __cdecl FUN_00438c30(int param_1, unsigned char *param_2, int param_3);
extern int __cdecl FUN_00432c10(int param_1);

extern int DAT_006db5a8;
extern int DAT_00536f78;
extern unsigned int DAT_005373d0;
extern int DAT_005373d4;
extern unsigned char *DAT_006a89b8;
extern unsigned char *DAT_006a89bc;

int __cdecl FUN_0042e660(int param_1, int param_2)
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
  
  local_c = 0x14;
  bVar1 = 0;
  local_4 = 0;
  local_8 = 0;
  do {
    iVar7 = 0;
    iVar3 = local_8;
    do {
      if (0xeb < iVar7) break;
      iVar3 = FUN_004a53f0();
      if ((iVar3 != 0) && (uVar4 = FUN_004a5470(), uVar4 == 0x1b)) {
        iVar7 = 0xec;
        local_c = 1;
        iVar3 = local_8;
        break;
      }
      iVar2 = DAT_006db5a8;
      FUN_0042c710();
      iVar3 = FUN_0048f530();
      DAT_006db5a8 = DAT_006db5a8 + iVar3;
      iVar7 = (iVar7 - iVar2) + DAT_006db5a8;
      iVar3 = 0;
      if (0 < param_2) {
        piVar5 = &DAT_00536f78;
        do {
          if (*piVar5 == 0) break;
          piVar5 = piVar5 + 1;
          iVar3 = iVar3 + 1;
        } while (iVar3 < param_2);
      }
      DAT_006db5a8 = iVar2;
    } while (iVar3 != param_2);
    if (iVar3 == param_2) break;
    local_4 = local_4 + iVar7;
    if (0xeb < local_4) {
      local_4 = 0;
      DAT_005373d0 = 0x1d;
      FUN_004310c0((unsigned int *)&DAT_005373d0, 4);
      bVar1 = !bVar1;
      pbVar6 = DAT_006a89b8;
      if (bVar1) {
        pbVar6 = DAT_006a89bc;
      }
      FUN_00438c30(3, pbVar6, 0x30);
    }
    if (local_8 == iVar3) {
      local_c = local_c + -1;
    }
    else {
      local_c = 0x14;
    }
    local_8 = iVar3;
  } while (local_c != 0);
  if (local_c != 0) {
    return 1;
  }
  DAT_005373d4 = FUN_00432c10(param_1);
  DAT_005373d0 = 0x15;
  FUN_004310c0((unsigned int *)&DAT_005373d0, 8);
  return 0;
}
