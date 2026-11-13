/* SPDX-License-Identifier: MIT */
/* z_xvt 004342f0 */
extern int DAT_006da4f8;
extern int *DAT_006db59c;

void __stdcall FUN_004342f0(void) {
    int iVar1;
    int *piVar2;
    int *piVar3;
    int *piVar4;
    int iVar5;
    int *piVar6;
    int *piVar7;
    int *piVar8;
    int bVar9;
    int *local_8 = 0;
    int local_4;

    iVar5 = 1;
    if (1 < DAT_006da4f8) {
        do {
            piVar6 = (int *)0;
            local_4 = 0;
            piVar3 = DAT_006db59c;
            piVar8 = local_8;
            if (0 < DAT_006da4f8) {
                while (1) {
                    iVar1 = 0;
                    piVar4 = piVar3;
                    local_8 = piVar8;
                    if (0 < iVar5) {
                        do {
                            piVar8 = piVar4;
                            if (piVar8 == (int *)0) goto LAB_004343d7;
                            iVar1 = iVar1 + 1;
                            piVar4 = (int *)piVar8[2];
                            local_8 = piVar8;
                        } while (iVar1 < iVar5);
                    }
                    local_8 = piVar8;
                    if (piVar4 == (int *)0) break;
                    iVar1 = 0;
                    if (0 < iVar5) {
                        do {
                            do {
                                piVar2 = piVar3;
                                piVar3 = piVar2;
                                piVar7 = piVar6;
                                if (*piVar4 < *piVar2) break;
                                piVar3 = (int *)piVar2[2];
                                piVar6 = piVar2;
                                piVar7 = piVar2;
                            } while (piVar8 != piVar2);
                            piVar2 = piVar4;
                            piVar6 = piVar8;
                            if (piVar8 == piVar7) goto joined_r0x004343a6;
                            piVar8[2] = piVar4[2];
                            if (piVar7 == (int *)0) {
                                DAT_006db59c = piVar4;
                                piVar4[2] = (int)piVar3;
                                piVar6 = DAT_006db59c;
                            } else {
                                piVar7[2] = (int)piVar4;
                                piVar4[2] = (int)piVar3;
                                piVar6 = piVar4;
                            }
                            piVar4 = (int *)piVar8[2];
                        } while ((piVar4 != (int *)0) && (iVar1 = iVar1 + 1, iVar1 < iVar5));
                    }
                    bVar9 = piVar8 == piVar6;
                    piVar6 = piVar8;
                    piVar2 = piVar4;
                    if (bVar9) {
                        joined_r0x004343a6:
                        while ((piVar4 = piVar2, iVar1 < iVar5 && (piVar4 != (int *)0))) {
                            iVar1 = iVar1 + 1;
                            piVar6 = piVar4;
                            piVar2 = (int *)piVar4[2];
                        }
                    }
                    local_8 = piVar6;
                    if ((piVar4 == (int *)0) ||
                       (local_4 = local_4 + iVar5 * 2, piVar3 = piVar4, piVar8 = piVar6,
                        DAT_006da4f8 <= local_4)) break;
                }
            }
        LAB_004343d7:
            iVar5 = iVar5 * 2;
        } while (iVar5 < DAT_006da4f8);
    }
    return;
}
