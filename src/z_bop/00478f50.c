/* SPDX-License-Identifier: MIT */
/* z_bop 00478f50 */
extern int DAT_0060f204;
extern int _DAT_0060f208;
extern int _DAT_0060f1f0;
extern int DAT_0060f1d8;

int __cdecl FUN_0047a430(char *param_1);
int __cdecl FUN_00478f50(unsigned int param_1, unsigned int *param_2, unsigned int *param_3);

int __cdecl FUN_00478f50(unsigned int param_1, unsigned int *param_2, unsigned int *param_3)
{
    int *piVar1;
    char cVar2;
    int *piVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    int *piVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    char *pcVar13;
    unsigned int *puVar14;
    int iStack_d4;
    unsigned int auStack_d0[52];

    if (param_2 == (unsigned int *)0x0) {
        return 0;
    }
    iVar8 = 0x18;
    if ((char *)*param_2 != (char *)0x0) {
        uVar5 = 0xffffffff;
        pcVar13 = (char *)*param_2;
        do {
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            cVar2 = *pcVar13;
            pcVar13 = pcVar13 + 1;
        } while (cVar2 != '\0');
        iVar8 = (int)(~uVar5 + 0x18u);
    }
    iVar10 = (int)param_2[5];
    if (((iVar10 != 0) && (param_2[4] != 0)) && (piVar7 = *(int **)(iVar10 + 8), piVar7 != (int *)0x0)) {
        switch (param_2[1]) {
        case 1:
            iVar6 = 0;
            iVar4 = 0;
            if (0 < *(int *)(iVar10 + 4)) {
                do {
                    piVar3 = piVar7 + iVar4 + 1;
                    iVar12 = (int)((unsigned int)iVar4 + 1u);
                    do {
                        iVar4 = iVar12;
                        iVar6 = (int)((unsigned int)iVar6 + 1u);
                        if (piVar3[2] == -1) {
                            iVar4 = (int)((unsigned int)iVar4 + 3u);
                            goto LAB_00478ffd;
                        }
                        piVar1 = piVar3 + 3;
                        piVar3 = piVar3 + 2;
                        iVar12 = (int)((unsigned int)iVar4 + 2u);
                    } while (*piVar1 != -1);
                    iVar4 = (int)((unsigned int)iVar4 + 4u);
LAB_00478ffd:
                    ;
                } while (iVar4 < *(int *)(iVar10 + 4));
            }
            iVar8 = (int)((unsigned int)iVar8 + 4u + (unsigned int)iVar6 * 100u + (unsigned int)DAT_0060f204 * 0xcu);
            break;
        case 2:
            iVar8 = (int)((unsigned int)iVar8 + 0x30u);
            break;
        case 3:
            DAT_0060f204 = *(int *)(iVar10 + 4);
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)DAT_0060f204 * 0xcu);
            break;
        case 4:
            iVar8 = (int)((unsigned int)iVar8 + 0xcu);
            break;
        case 5:
            iVar8 = (int)((unsigned int)iVar8 + 0x24u);
            break;
        case 6:
            iVar8 = (int)((unsigned int)iVar8 + 0xcu);
            break;
        case 7:
            uVar5 = 0xffffffff;
            pcVar13 = (char *)piVar7;
            do {
                if (uVar5 == 0) break;
                uVar5 = uVar5 - 1;
                cVar2 = *pcVar13;
                pcVar13 = pcVar13 + 1;
            } while (cVar2 != '\0');
            iVar8 = (int)((unsigned int)iVar8 + ~uVar5);
            break;
        case 9:
            iVar6 = *(int *)(iVar10 + 4);
            if (*(int *)(iVar10 + 4) < *(int *)(iVar10 + 0x10)) {
                iVar6 = *(int *)(iVar10 + 0x10);
            }
            if (iVar6 < *(int *)(iVar10 + 0x1c)) {
                iVar6 = *(int *)(iVar10 + 0x1c);
            }
            if (iVar6 < *(int *)(iVar10 + 0x28)) {
                iVar6 = *(int *)(iVar10 + 0x28);
            }
            if (iVar6 < *(int *)(iVar10 + 0x34)) {
                iVar6 = *(int *)(iVar10 + 0x34);
            }
            if (iVar6 < *(int *)(iVar10 + 0x40)) {
                iVar6 = *(int *)(iVar10 + 0x40);
            }
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar6 * 0x38u);
            break;
        case 0xb:
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)*(int *)(iVar10 + 4) * 0xcu);
            _DAT_0060f208 = iVar10;
            param_3[0x21] = (unsigned int)iVar10;
            break;
        case 0xd:
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)*(int *)(iVar10 + 4) * 8u);
            break;
        case 0xf:
            if (*(int **)(iVar10 + 0x14) != (int *)0x0) {
                if (*(int **)(iVar10 + 0x20) != (int *)0x0) {
                    iVar8 = (int)((unsigned int)iVar8 + 4u +
                                  (((unsigned int)(**(int **)(iVar10 + 0x20)) - 1u) *
                                   ((unsigned int)(**(int **)(iVar10 + 0x14)) - 1u)) * 100u +
                                  (unsigned int)DAT_0060f204 * 0xcu);
                }
            }
            break;
        case 0x10:
            iVar4 = 0;
            iVar6 = 0;
            iStack_d4 = *(int *)(iVar10 + 4);
            if (0 < iStack_d4) {
                do {
                    iVar10 = *piVar7;
                    iVar12 = iVar4;
                    if (2 < iVar10) {
                        iVar11 = (int)((unsigned int)iVar4 + 1u);
                        iVar9 = (int)((unsigned int)iVar11 - (unsigned int)iVar4);
                        while ((iVar12 = iVar11, iVar9 < iVar10 &&
                                ((iVar6 = (int)((unsigned int)iVar6 + 1u),
                                  (int)((unsigned int)iVar11 + 2u - (unsigned int)iVar10) == iVar4 ||
                                  (iVar12 = (int)((unsigned int)iVar11 + 3u),
                                   (int)((unsigned int)iVar12 - (unsigned int)iVar10) != iVar4))))) {
                            iVar11 = (int)((unsigned int)iVar11 + 2u);
                            iVar9 = (int)((unsigned int)iVar11 - (unsigned int)iVar4);
                        }
                    }
                    piVar7 = piVar7 + 1;
                    iStack_d4 = (int)((unsigned int)iStack_d4 - 1u);
                    iVar4 = iVar12;
                } while (iStack_d4 != 0);
            }
            iVar8 = (int)((unsigned int)iVar8 + 4u + (unsigned int)iVar6 * 100u + (unsigned int)DAT_0060f204 * 0xcu);
            break;
        case 0x11:
            piVar7 = *(int **)(iVar10 + 0x14);
            if (piVar7 != (int *)0x0) {
                iVar6 = 0;
                iVar10 = *(int *)(iVar10 + 0x10);
                if (0 < iVar10) {
                    do {
                        iVar4 = *piVar7;
                        piVar7 = piVar7 + 1;
                        iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar4 - 2u);
                        iVar10 = (int)((unsigned int)iVar10 - 1u);
                    } while (iVar10 != 0);
                }
                iVar8 = (int)((unsigned int)iVar8 + 4u + (unsigned int)iVar6 * 100u + (unsigned int)DAT_0060f204 * 0xcu);
            }
            break;
        case 0x14:
            iVar10 = FUN_0047a430((char *)piVar7);
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar10);
            break;
        case 0x15:
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)*(int *)(iVar10 + 4) * 4u);
            break;
        case 0x16:
            iVar8 = (int)((unsigned int)iVar8 + 0x10u);
            break;
        case 0x17:
            iVar8 = (int)((unsigned int)iVar8 + 0x30u);
            break;
        case 0x19:
            iVar8 = (int)((unsigned int)iVar8 + 0x48u);
        }
    }
    iVar10 = 0;
    if (param_2[2] != 0) {
        puVar14 = auStack_d0;
        for (iVar6 = 0x34; iVar6 != 0; iVar6 = (int)((unsigned int)iVar6 - 1u)) {
            *puVar14 = *param_3;
            param_3 = param_3 + 1;
            puVar14 = puVar14 + 1;
        }
        iVar6 = 0;
        _DAT_0060f208 = 0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1d8 = 0;
        iVar8 = (int)((unsigned int)iVar8 + param_2[2] * 4u);
        if (0 < (int)param_2[2]) {
            do {
                iVar6 = (int)((unsigned int)iVar6 + 1u);
                iVar10 = (int)((unsigned int)iVar10 + 4u);
                iVar4 = FUN_00478f50(param_1, *(unsigned int **)(param_2[3] - 4u + (unsigned int)iVar10), auStack_d0);
                iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar4);
            } while (iVar6 < (int)param_2[2]);
        }
    }
    return iVar8;
}
