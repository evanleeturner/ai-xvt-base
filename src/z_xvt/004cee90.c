/* SPDX-License-Identifier: MIT */
/* z_xvt 004cee90 */
extern int DAT_00815ac6;
extern int DAT_00815aca;
extern int DAT_00815ad2;
extern int DAT_00815ada;
extern int DAT_00815ace;
extern int DAT_00815ad6;
extern int DAT_00815ade;
extern int DAT_00815ae2;
extern int DAT_00815ae6;
extern int DAT_00815ce6;
extern int DAT_00815cea;
extern int DAT_00815cfa;
extern int DAT_00815cee;
extern int DAT_00815cf2;
extern int DAT_00815cf6;
extern int DAT_0081b5be;
extern const char s_SAdding_new_net_sequence__u_005324f8[];

DWORD __stdcall GetTickCount(void);
int __cdecl _sprintf(char *dst, const char *fmt, ...);

uint __cdecl FUN_004cee90(int param_1)
{
    uint uVar3 = 0;
    int *piVar1;
    DWORD DVar2;
    char local_100[256];

    if (DAT_0081b5be != 0) {
        piVar1 = &DAT_00815ac6;
        do {
            if (*piVar1 == param_1) {
                return uVar3;
            }
            piVar1 += 0x8e;
            uVar3++;
        } while (uVar3 < (uint)DAT_0081b5be);
    }

    if ((uint)DAT_0081b5be == uVar3 && uVar3 < 0x28) {
        (&DAT_00815ac6)[uVar3 * 0x8e] = param_1;
        (&DAT_00815aca)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ad2)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ada)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ace)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ad6)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ade)[uVar3 * 0x8e] = 0x7f;
        (&DAT_00815ae2)[uVar3 * 0x8e] = 0;
        (&DAT_00815ae6)[uVar3 * 0x8e] = 0x39;
        (&DAT_00815ce6)[uVar3 * 0x8e] = 1;
        DVar2 = GetTickCount();
        (&DAT_00815cea)[uVar3 * 0x8e] = DVar2;
        DVar2 = GetTickCount();
        (&DAT_00815cfa)[uVar3 * 0x8e] = DVar2;
        (&DAT_00815cee)[uVar3 * 0x8e] = 0;
        (&DAT_00815cf2)[uVar3 * 0x8e] = 0;
        (&DAT_00815cf6)[uVar3 * 0x8e] = 0;
        DAT_0081b5be = DAT_0081b5be + 1;
        _sprintf(local_100, s_SAdding_new_net_sequence__u_005324f8, param_1);
    }
    return uVar3;
}
