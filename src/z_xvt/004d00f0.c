/* SPDX-License-Identifier: MIT */
/* z_xvt 004d00f0 */
extern unsigned int DAT_00759b08;
extern unsigned long GetTickCount(void);
extern unsigned int FUN_004d0070(unsigned int param_1);

unsigned int __cdecl FUN_004d00f0(unsigned int param_1, unsigned int param_2, int param_3)
{
    unsigned int uVar1;
    unsigned long DVar2;
    unsigned long DVar3;
    unsigned int uVar4;
    int bVar5;

    if (DAT_00759b08 == 0) {
        return 0;
    }
    bVar5 = param_1 <= param_2;
    if (param_2 != param_1) {
        if (bVar5) {
            uVar4 = param_2 - param_1;
        }
        else {
            uVar4 = param_1 - param_2;
        }
        DVar2 = GetTickCount();
        do {
            while (1) {
                DVar3 = GetTickCount();
                if ((int)((unsigned int)(param_3 << 8) / uVar4 + DVar2) < (int)DVar3) {
                    if (bVar5) {
                        uVar1 = param_1 + 0x100;
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
