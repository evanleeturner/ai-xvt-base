/* SPDX-License-Identifier: MIT */
/* z_bop 004d23b0 */
extern int DAT_00b69a89;
extern int DAT_00b63f91;
extern int DAT_00b63f95;
extern int DAT_00b63f9d;
extern int DAT_00b63fa5;
extern int DAT_00b63f99;
extern int DAT_00b63fa1;
extern int DAT_00b63fa9;
extern int DAT_00b63fad;
extern int DAT_00b63fb1;
extern int DAT_00b641b1;
extern int DAT_00b641b5;
extern int DAT_00b641c5;
extern int DAT_00b641b9;
extern int DAT_00b641bd;
extern int DAT_00b641c1;
extern const char s_SAdding_new_net_sequence__u_0052b9f0[];

DWORD GetTickCount(void);
int _sprintf(char *dst, const char *fmt, ...);

uint __cdecl FUN_004d23b0(int param_1)
{
  int *piVar1;
  DWORD DVar2;
  uint uVar3;
  char local_100[256];
  
  uVar3 = 0;
  if (DAT_00b69a89 != 0) {
    piVar1 = &DAT_00b63f91;
    do {
      if (*piVar1 == param_1) {
        return uVar3;
      }
      piVar1 = piVar1 + 0x8e;
      uVar3 = uVar3 + 1;
    } while (uVar3 < (uint)DAT_00b69a89);
  }
  if (((uint)DAT_00b69a89 == uVar3) && (uVar3 < 0x28)) {
    (&DAT_00b63f91)[uVar3 * 0x8e] = param_1;
    (&DAT_00b63f95)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63f9d)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63fa5)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63f99)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63fa1)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63fa9)[uVar3 * 0x8e] = 0x7f;
    (&DAT_00b63fad)[uVar3 * 0x8e] = 0;
    (&DAT_00b63fb1)[uVar3 * 0x8e] = 0x39;
    (&DAT_00b641b1)[uVar3 * 0x8e] = 1;
    DVar2 = GetTickCount();
    (&DAT_00b641b5)[uVar3 * 0x8e] = DVar2;
    DVar2 = GetTickCount();
    (&DAT_00b641c5)[uVar3 * 0x8e] = DVar2;
    (&DAT_00b641b9)[uVar3 * 0x8e] = 0;
    (&DAT_00b641bd)[uVar3 * 0x8e] = 0;
    (&DAT_00b641c1)[uVar3 * 0x8e] = 0;
    DAT_00b69a89 = DAT_00b69a89 + 1;
    _sprintf(local_100, s_SAdding_new_net_sequence__u_0052b9f0, param_1);
  }
  return uVar3;
}
