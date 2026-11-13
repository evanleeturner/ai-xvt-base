/* SPDX-License-Identifier: MIT */
/* z_xvt 004395b0 */
extern uint DAT_005ef2cc;
extern DWORD __stdcall timeGetTime(void);
extern uint __cdecl FUN_004d0070(uint param_1);

uint __cdecl FUN_004395b0(uint param_1, uint param_2, int param_3)
{
    if (DAT_005ef2cc == 0) {
        return 0;
    }
    int bVar5 = (param_1 <= param_2);
    if (param_2 != param_1) {
        uint uVar4;
        if (bVar5) {
            uVar4 = param_2 - param_1;
        }
        else {
            uVar4 = param_1 - param_2;
        }
        DWORD DVar2 = timeGetTime();
        do {
            while (1) {
                DWORD DVar3 = timeGetTime();
                if ((int)((uint)(param_3 << 8) / uVar4 + DVar2) < (int)DVar3) {
                    if (bVar5) {
                        uint uVar1 = param_1 + 0x100;
                        param_1 = 0xffff;
                        if (uVar1 < 0x10000) {
                            param_1 = uVar1;
                        }
                    }
                    else if (param_1 < 0x100) {
                        param_1 = 0;
                    }
                    else {
                        param_1 = param_1 - 0x100;
                    }
                    FUN_004d0070(param_1);
                    DVar2 = DVar3;
                }
                if (!bVar5) break;
                if (param_2 <= param_1) {
                    return 1;
                }
            }
        } while (param_2 < param_1);
        return 1;
    }
    return 1;
}
