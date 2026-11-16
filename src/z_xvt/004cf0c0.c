/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf0c0 */
extern int DAT_0078072e;
extern uint DAT_00780726;
extern int DAT_0078072a;
extern int DAT_00780c7a;
extern int DAT_0078077a;
extern uint DAT_0081b5be;
extern uint DAT_00815cfa;
extern uint DAT_00815ad6;
extern uint DAT_00815ace;
extern uint DAT_008050a6;
extern uint DAT_008050ae;
extern uint DAT_007810a6;
extern uint DAT_007810aa;
extern uint DAT_007810ae;
extern unsigned char DAT_007810b2;
extern unsigned char DAT_007810b3;
extern unsigned char DAT_007810b4;
extern unsigned char DAT_007810b5;
extern uint DAT_007810b6;

DWORD __stdcall GetTickCount(void);
uint __cdecl FUN_004cee90(uint param_1);
void __cdecl FUN_004cb830(int param_1, uint *param_2, int param_3);

uint __stdcall FUN_004cf0c0(void)
{
    int iVar1;
    DWORD DVar2;
    uint uVar3;
    DWORD DVar4;
    uint uVar5;
    int *piVar6;
    uint local_8[2];

    __builtin_memcpy(&iVar1, &DAT_0078072e, 4);
    if (iVar1 == 0) {
        DVar2 = GetTickCount();
        __builtin_memcpy(&local_8[0], &DAT_00780726, 4);
        uVar3 = FUN_004cee90(local_8[0]);
        __builtin_memcpy(&local_8[0], &DAT_0081b5be, 4);
        if (local_8[0] <= uVar3) {
            return 0;
        }
        __builtin_memcpy(&DVar4, &(&DAT_00815cfa)[uVar3 * 0x8e], 4);
        if (45000 < DVar2 - DVar4) {
            DVar2 = GetTickCount();
            __builtin_memcpy(&(&DAT_00815cfa)[uVar3 * 0x8e], &DVar2, 4);
            __builtin_memcpy(&local_8[0], &DAT_008050ae, 4);
            if (local_8[0] < 0x400) {
                __builtin_memcpy(&local_8[1], &DAT_008050a6, 4);
                local_8[0] = 0x46;
                __builtin_memcpy(&(&DAT_007810b6)[local_8[1] * 0x84], &local_8[0], 4);
                __builtin_memcpy(&local_8[0], &DAT_00780726, 4);
                __builtin_memcpy(&(&DAT_007810a6)[local_8[1] * 0x84], &local_8[0], 4);
                local_8[0] = 4;
                __builtin_memcpy(&(&DAT_007810aa)[local_8[1] * 0x84], &local_8[0], 4);
                local_8[0] = 0;
                __builtin_memcpy(&(&DAT_007810ae)[local_8[1] * 0x84], &local_8[0], 4);
                (&DAT_007810b2)[local_8[1] * 0x210] = 0;
                (&DAT_007810b5)[local_8[1] * 0x210] = 0;
                (&DAT_007810b3)[local_8[1] * 0x210] = 0;
                __builtin_memcpy(&uVar5, &(&DAT_00815ad6)[uVar3 * 0x8e], 4);
                uVar5 = uVar5 + 1;
                if (0x7f < uVar5) {
                    uVar5 = 0;
                }
                __builtin_memcpy(&(&DAT_00815ad6)[uVar3 * 0x8e], &uVar5, 4);
                (&DAT_007810b4)[local_8[1] * 0x210] = (unsigned char)uVar5;
                local_8[1] = local_8[1] + 1;
                __builtin_memcpy(&DAT_008050a6, &local_8[1], 4);
                __builtin_memcpy(&local_8[0], &DAT_008050ae, 4);
                local_8[0] = local_8[0] + 1;
                __builtin_memcpy(&DAT_008050ae, &local_8[0], 4);
                if (0x3ff < local_8[1]) {
                    local_8[1] = 0;
                    __builtin_memcpy(&DAT_008050a6, &local_8[1], 4);
                }
            }
        }
    }
    else {
        piVar6 = &DAT_0078077a;
        DVar2 = GetTickCount();
        do {
            __builtin_memcpy(&iVar1, piVar6, 4);
            if (iVar1 != 0) {
                __builtin_memcpy(&local_8[1], &DAT_00780c7a, 4);
                if (local_8[1] != (uint)iVar1) {
                    __builtin_memcpy(&uVar5, &DAT_0078072a, 4);
                    if (uVar5 != (uint)iVar1) {
                        __builtin_memcpy(&local_8[1], piVar6 + 1, 4);
                        if (local_8[1] != 0) {
                            uVar3 = FUN_004cee90(iVar1);
                            __builtin_memcpy(&local_8[1], &DAT_0081b5be, 4);
                            if (uVar3 < local_8[1]) {
                                __builtin_memcpy(&uVar5, &(&DAT_00815cfa)[uVar3 * 0x8e], 4);
                                if (45000 < DVar2 - uVar5) {
                                    DVar4 = GetTickCount();
                                    __builtin_memcpy(&(&DAT_00815cfa)[uVar3 * 0x8e], &DVar4, 4);
                                    __builtin_memcpy(&local_8[1], &DAT_008050ae, 4);
                                    if (local_8[1] < 0x400) {
                                        local_8[0] = 0x5b;
                                        __builtin_memcpy(&iVar1, piVar6, 4);
                                        FUN_004cb830(iVar1, local_8, 4);
                                        __builtin_memcpy(&local_8[1], &DAT_008050a6, 4);
                                        local_8[0] = 0x47;
                                        __builtin_memcpy(&(&DAT_007810b6)[local_8[1] * 0x84], &local_8[0], 4);
                                        __builtin_memcpy(&iVar1, piVar6, 4);
                                        __builtin_memcpy(&(&DAT_007810a6)[local_8[1] * 0x84], &iVar1, 4);
                                        local_8[0] = 4;
                                        __builtin_memcpy(&(&DAT_007810aa)[local_8[1] * 0x84], &local_8[0], 4);
                                        local_8[0] = 0;
                                        __builtin_memcpy(&(&DAT_007810ae)[local_8[1] * 0x84], &local_8[0], 4);
                                        (&DAT_007810b2)[local_8[1] * 0x210] = 0;
                                        (&DAT_007810b5)[local_8[1] * 0x210] = 0;
                                        (&DAT_007810b3)[local_8[1] * 0x210] = 1;
                                        __builtin_memcpy(&uVar5, &(&DAT_00815ace)[uVar3 * 0x8e], 4);
                                        uVar5 = uVar5 + 1;
                                        if (0x7f < uVar5) {
                                            uVar5 = 0;
                                        }
                                        __builtin_memcpy(&(&DAT_00815ace)[uVar3 * 0x8e], &uVar5, 4);
                                        (&DAT_007810b4)[local_8[1] * 0x210] = (unsigned char)uVar5;
                                        local_8[1] = local_8[1] + 1;
                                        __builtin_memcpy(&DAT_008050a6, &local_8[1], 4);
                                        __builtin_memcpy(&local_8[0], &DAT_008050ae, 4);
                                        local_8[0] = local_8[0] + 1;
                                        __builtin_memcpy(&DAT_008050ae, &local_8[0], 4);
                                        if (0x3ff < local_8[1]) {
                                            local_8[1] = 0;
                                            __builtin_memcpy(&DAT_008050a6, &local_8[1], 4);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            piVar6 = piVar6 + 10;
        } while (piVar6 < &DAT_00780c7a);
    }
    return 1;
}
