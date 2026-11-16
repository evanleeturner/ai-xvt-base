/* SPDX-License-Identifier: MIT */
/* z_bop 00437860 */
extern unsigned short DAT_009d682c;
extern unsigned short DAT_009a1fe4;

void __cdecl FUN_004aba50(void);
void __cdecl FUN_004377b0(int, int, int, unsigned char);
void __cdecl FUN_004abe50(void);

void __cdecl FUN_00437860(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int local_8;

    iVar1 = (int)((unsigned int)param_2 + (unsigned int)param_4);
    if ((((0 < iVar1) && (0 < (int)((unsigned int)param_1 + (unsigned int)param_3))) &&
         (param_1 < (int)(unsigned int)DAT_009d682c)) &&
        (((param_2 < (int)(unsigned int)DAT_009a1fe4 && (0 < param_4)) && (0 < param_3)))) {
        local_8 = param_4 >> 3;
        iVar5 = param_3 >> 3;
        if (iVar5 < 3) {
            iVar5 = 3;
        }
        if (local_8 < 3) {
            local_8 = 3;
        }
        if (param_3 < iVar5) {
            iVar5 = param_3;
        }
        if (param_4 < local_8) {
            local_8 = param_4;
        }
        FUN_004aba50();
        if (-1 < param_2) {
            uVar3 = (unsigned int)param_1 + (unsigned int)iVar5;
            if ((0 < (int)uVar3) && (uVar2 = (unsigned int)DAT_009d682c, param_1 < (int)uVar2)) {
                iVar4 = param_1;
                if (param_1 < 0) {
                    iVar4 = 0;
                }
                if ((int)uVar2 < (int)uVar3) {
                    uVar3 = uVar2;
                }
                FUN_004377b0(iVar4, (int)uVar3, param_2, param_5);
            }
            iVar4 = (int)(((unsigned int)param_3 - (unsigned int)iVar5) + (unsigned int)param_1);
            if ((0 < (int)((unsigned int)param_1 + (unsigned int)param_3)) &&
                (uVar3 = (unsigned int)DAT_009d682c, iVar4 < (int)uVar3)) {
                if (iVar4 < 0) {
                    iVar4 = 0;
                }
                uVar2 = (unsigned int)param_1 + (unsigned int)param_3;
                if ((int)uVar3 < (int)((unsigned int)param_1 + (unsigned int)param_3)) {
                    uVar2 = uVar3;
                }
                FUN_004377b0(iVar4, (int)uVar2, param_2, param_5);
            }
        }
        if (iVar1 <= (int)(unsigned int)DAT_009a1fe4) {
            uVar3 = (unsigned int)param_1 + (unsigned int)iVar5;
            if ((0 < (int)uVar3) && (uVar2 = (unsigned int)DAT_009d682c, param_1 < (int)uVar2)) {
                iVar4 = param_1;
                if (param_1 < 0) {
                    iVar4 = 0;
                }
                if ((int)uVar2 < (int)uVar3) {
                    uVar3 = uVar2;
                }
                FUN_004377b0(iVar4, (int)uVar3, (int)((unsigned int)iVar1 - 1u), param_5);
            }
            iVar5 = (int)(((unsigned int)param_3 - (unsigned int)iVar5) + (unsigned int)param_1);
            if ((0 < (int)((unsigned int)param_1 + (unsigned int)param_3)) &&
                (uVar3 = (unsigned int)DAT_009d682c, iVar5 < (int)uVar3)) {
                if (iVar5 < 0) {
                    iVar5 = 0;
                }
                uVar2 = (unsigned int)param_1 + (unsigned int)param_3;
                if ((int)uVar3 < (int)((unsigned int)param_1 + (unsigned int)param_3)) {
                    uVar2 = uVar3;
                }
                FUN_004377b0(iVar5, (int)uVar2, (int)((unsigned int)iVar1 - 1u), param_5);
            }
        }
        iVar1 = 1;
        if (1 < local_8) {
            do {
                iVar5 = (int)((unsigned int)param_2 + (unsigned int)iVar1);
                if ((-1 < iVar5) && (iVar5 < (int)(unsigned int)DAT_009a1fe4)) {
                    if (-1 < param_1) {
                        FUN_004377b0(param_1, (int)((unsigned int)param_1 + 1u), iVar5, param_5);
                    }
                    if ((int)((unsigned int)param_1 + (unsigned int)param_3) <= (int)(unsigned int)DAT_009d682c) {
                        FUN_004377b0((int)(((unsigned int)param_1 - 1u) + (unsigned int)param_3),
                                     (int)((unsigned int)param_1 + (unsigned int)param_3), iVar5, param_5);
                    }
                }
                iVar1 = (int)((unsigned int)iVar1 + 1u);
            } while (iVar1 < local_8);
        }
        for (iVar1 = (int)((unsigned int)param_4 - (unsigned int)local_8);
             iVar1 < (int)((unsigned int)param_4 - 1u);
             iVar1 = (int)((unsigned int)iVar1 + 1u)) {
            if (((local_8 <= iVar1) && (iVar5 = (int)((unsigned int)param_2 + (unsigned int)iVar1), -1 < iVar5)) &&
                (iVar5 < (int)(unsigned int)DAT_009a1fe4)) {
                if (-1 < param_1) {
                    FUN_004377b0(param_1, (int)((unsigned int)param_1 + 1u), iVar5, param_5);
                }
                if ((int)((unsigned int)param_1 + (unsigned int)param_3) <= (int)(unsigned int)DAT_009d682c) {
                    FUN_004377b0((int)(((unsigned int)param_1 - 1u) + (unsigned int)param_3),
                                 (int)((unsigned int)param_1 + (unsigned int)param_3), iVar5, param_5);
                }
            }
        }
        FUN_004abe50();
    }
    return;
}
