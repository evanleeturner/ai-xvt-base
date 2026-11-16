/* SPDX-License-Identifier: MIT */
/* z_bop 0046fae0 */
typedef unsigned int uint;
typedef unsigned long DWORD;
typedef unsigned int undefined4;

extern uint DAT_00999548;
extern uint DAT_0099f6e0;
extern DWORD DAT_00999e0c[];
extern uint DAT_005597a0;
extern uint DAT_005597a4;
extern uint DAT_005597a8;
extern uint DAT_005597ac;
extern DWORD DAT_005597b0;
extern uint DAT_00999bf8[];
extern uint DAT_00999c00[];
extern uint DAT_00999bf0[];
extern uint DAT_00999be8[];
extern const char s_Sending_RRA_0052709c[];

undefined4 * __cdecl FUN_0046dfa0(undefined4 * param_1);
DWORD __stdcall timeGetTime(void);
uint __cdecl FUN_0046fee0(int param_1);
void __cdecl FUN_0046c220(const char * param_1);
void __cdecl FUN_0046f470(uint param_1, uint * param_2, DWORD param_3, DWORD param_4);

undefined4 __stdcall FUN_0046fae0(void)
{
  undefined4 *puVar1;
  DWORD DVar2;
  uint uVar3;
  int *piVar4;
  uint uVar5;
  uint local_4;
  
  uVar5 = 0;
  puVar1 = FUN_0046dfa0(&local_4);
  if (local_4 != 0) {
    piVar4 = (int *)(puVar1 + 8);
    do {
      DVar2 = timeGetTime();
      if (DAT_00999548 != (uint)*piVar4) {
        uVar3 = FUN_0046fee0(*piVar4);
        if (((uVar3 < DAT_0099f6e0) && (uVar3 < 8)) &&
           (3000 < DVar2 - DAT_00999e0c[uVar3 * 0x8e])) {
          DAT_00999e0c[uVar3 * 0x8e] = DVar2;
          FUN_0046c220(s_Sending_RRA_0052709c);
          DAT_005597a0 = 0x37;
          DAT_005597a4 = DAT_00999bf8[uVar3 * 0x8e] + 1;
          if (0x7f < DAT_005597a4) {
            DAT_005597a4 = 0;
          }
          DAT_005597a8 = DAT_00999c00[uVar3 * 0x8e] + 1;
          if (0x7f < DAT_005597a8) {
            DAT_005597a8 = 0;
          }
          DAT_005597ac = DAT_00999bf0[uVar3 * 0x8e] + 1;
          if (0x7f < DAT_005597ac) {
            DAT_005597ac = 0;
          }
          DAT_005597b0 = timeGetTime();
          FUN_0046f470(DAT_00999be8[uVar3 * 0x8e], &DAT_005597a0, 0x14, DAT_005597b0);
        }
      }
      piVar4 = piVar4 + 10;
      uVar5 = uVar5 + 1;
    } while (uVar5 < local_4);
  }
  return 1;
}
