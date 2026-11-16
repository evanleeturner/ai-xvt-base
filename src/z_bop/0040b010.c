/* SPDX-License-Identifier: MIT */
/* z_bop 0040b010 */
extern int DAT_00999450;
extern int DAT_00999462;
extern int DAT_00999470;
extern int DAT_0099949e;
extern int DAT_009994a2;
extern int DAT_00999458;
extern int DAT_00999466;
extern int DAT_00999474;
extern int DAT_00999498;
extern int DAT_0054f990;
extern int DAT_0054f994;
extern int DAT_0052f868;
extern int DAT_0054f9a0;
extern undefined4 *DAT_0053f970;
extern undefined4 *DAT_0054f988;
extern char DAT_005270b4;

void __cdecl FUN_00470140(int param_1);
void __stdcall FUN_00408140(void);
void __stdcall FUN_004b1c10(void);
void __stdcall FUN_004b1c90(void);
void __cdecl FUN_004b1d10(undefined4 *param_1, unsigned int param_2);
void __cdecl FUN_004b1d70(void);
void __cdecl FUN_004b1de0(undefined4 *param_1, unsigned int param_2);
void __stdcall FUN_004b2020(void);
void __cdecl FUN_004b1c50(void);
void __cdecl FUN_00408110(void);
void __cdecl FUN_00408bc0(int *param_1);
void __cdecl FUN_004084e0(int *param_1);
void __cdecl FUN_00408e70(int *param_1);

void __cdecl FUN_0040b010(undefined4 *param_1)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  
  iVar1 = DAT_00999450;
  DAT_00999462 = 0;
  DAT_00999470 = 0;
  DAT_00999450 = iVar1;
  if ((((DAT_0099949e != DAT_009994a2) && ((int)param_1[0x23] + DAT_00999450 <= DAT_00999458)) &&
      ((int)param_1[0x1e] <= DAT_00999466)) && ((int)param_1[0x24] <= DAT_00999474)) {
    puVar4 = (undefined4 *)(DAT_009994a2 * 0xd0 + DAT_00999498);
    for (iVar2 = 0x34; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar4 = *param_1;
      param_1 = param_1 + 1;
      puVar4 = puVar4 + 1;
    }
    piVar3 = (int *)(DAT_009994a2 * 0xd0 + DAT_00999498);
    FUN_00470140((int)piVar3);
    iVar2 = piVar3[0x31];
    if (iVar2 != 0) {
      if ((DAT_0054f990 < (int)(DAT_0054f994 + iVar2 * 8)) ||
         (DAT_0052f868 < (int)(iVar2 * 2 + DAT_0054f9a0))) {
        FUN_00408140();
        FUN_004b1c10();
        FUN_004b1c90();
        FUN_004b1d10(DAT_0053f970,DAT_0054f994);
        FUN_004b1d70();
        FUN_004b1de0(DAT_0054f988,DAT_0054f9a0);
        FUN_004b2020();
        FUN_004b1c50();
        FUN_00408110();
        DAT_0054f9a0 = 0;
        DAT_0054f994 = 0;
      }
      if (DAT_005270b4 == '\0') {
        FUN_004084e0(piVar3);
      }
      else {
        FUN_00408bc0(piVar3);
      }
      FUN_00408e70(piVar3);
      DAT_00999450 = iVar1;
    }
  }
  return;
}
