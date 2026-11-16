/* SPDX-License-Identifier: MIT */
/* z_xvt 00433540 */
extern DWORD __stdcall timeGetTime(void);

extern uint DAT_00739290;
extern int DAT_00733798;
extern int DAT_0073379c;
extern int DAT_007337a4;
extern int DAT_007337ac;
extern int DAT_007337a0;
extern int DAT_007337a8;
extern int DAT_007337b0;
extern int DAT_007337b4;
extern int DAT_007337b8;
extern int DAT_007339b8;
extern int DAT_007339bc;
extern int DAT_007339c0;
extern int DAT_007339c4;

uint __cdecl FUN_00433540(int param_1)
{
  int *piVar1;
  DWORD DVar2;
  uint uVar3;
  
  uVar3 = 0;
  piVar1 = (int *)&DAT_00733798;
  if (DAT_00739290 != 0) {
    do {
      if (*piVar1 == param_1) {
        return uVar3;
      }
      piVar1 = piVar1 + 0x8e;
      uVar3 = uVar3 + 1;
    } while (uVar3 < DAT_00739290);
  }
  if ((DAT_00739290 == uVar3) && (uVar3 < 0x28)) {
    *piVar1 = param_1;
    *(piVar1 + 1) = 0x7f;
    *(piVar1 + 3) = 0x7f;
    *(piVar1 + 5) = 0x7f;
    *(piVar1 + 2) = 0x7f;
    *(piVar1 + 4) = 0x7f;
    *(piVar1 + 6) = 0x7f;
    *(piVar1 + 7) = 0;
    *(piVar1 + 8) = 0x39;
    *(piVar1 + 0x88) = 1;
    DVar2 = timeGetTime();
    *(piVar1 + 0x89) = DVar2;
    *(piVar1 + 0x8a) = 0;
    *(piVar1 + 0x8b) = 0;
    DAT_00739290 = DAT_00739290 + 1;
  }
  return uVar3;
}
