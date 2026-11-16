/* SPDX-License-Identifier: MIT */
/* z_bop 00476810 */
extern int DAT_00999488;
extern unsigned char DAT_0099947e[4]; /* unaligned (0x99947e): accessed via memcpy */
extern int DAT_0060f204;
extern int *DAT_0060f1d8;
extern int *_DAT_0060f208;
extern int DAT_0060f1c0;
extern int DAT_0060f1e8;
extern int _DAT_0060f1f0;

int __cdecl FUN_00476810(unsigned int *param_1, unsigned int *param_2);

int __cdecl FUN_00476810(unsigned int *param_1, unsigned int *param_2)
{
    char cVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    char *pcVar6;
    int *piVar7;
    unsigned int *puVar8;
    int local_d8;
    int local_d4;
    unsigned int local_d0[52];
    int tmp99947e;

    if (param_1 == (unsigned int *)0) {
        return 0;
    }
    iVar5 = 0x18;
    if ((char *)*param_1 != (char *)0) {
        uVar2 = 0xffffffff;
        pcVar6 = (char *)*param_1;
        do {
            if (uVar2 == 0) break;
            uVar2 = uVar2 - 1;
            cVar1 = *pcVar6;
            pcVar6 = pcVar6 + 1;
        } while (cVar1 != '\0');
        iVar5 = (int)(~uVar2 + 0x18u);
    }
    piVar7 = (int *)param_1[5];
    if (piVar7 == (int *)0) {
        /* original reads fixed low addresses (null + offset) here */
        if (param_1[1] != 0x14) goto switchD_00476863_caseD_8;
        iVar3 = (int)((unsigned int)piVar7[5] * (unsigned int)piVar7[4]);
        iVar4 = iVar3;
        if ((int)((unsigned int)iVar3 - (unsigned int)piVar7[2]) == 0) {
            iVar4 = piVar7[3];
        }
        iVar5 = (int)((unsigned int)iVar5 + 0x18u + (unsigned int)iVar4);
        if (piVar7[1] != 0) {
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)piVar7[1] * 0x300u);
            goto switchD_00476863_caseD_8;
        }
        if ((int)((unsigned int)iVar3 - (unsigned int)piVar7[2]) == 0) {
            iVar3 = piVar7[3];
        }
        if ((char *)piVar7[0] != (char *)((unsigned int)iVar3 + 0x18u)) goto switchD_00476863_caseD_8;
LAB_00476a01:
        iVar5 = (int)((unsigned int)iVar5 + 0x3000u);
    }
    else {
        switch (param_1[1]) {
        case 1:
        case 0xf:
        case 0x10:
        case 0x11:
            if (DAT_00999488 < *piVar7) {
                DAT_00999488 = *piVar7;
            }
            iVar5 = (int)((unsigned int)iVar5 + 4u + param_1[4] * 100u);
            if (param_2[0x21] == 0) {
                iVar5 = (int)((unsigned int)iVar5 + (unsigned int)DAT_0060f204 * 0xcu);
            }
            break;
        case 2:
            iVar5 = iVar5 + 0x30;
            break;
        case 3:
            DAT_0060f204 = (int)param_1[4];
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)DAT_0060f204 * 0xcu);
            __builtin_memcpy(&tmp99947e, DAT_0099947e, 4);
            if (tmp99947e < (int)param_1[4]) {
                tmp99947e = (int)param_1[4];
                __builtin_memcpy(DAT_0099947e, &tmp99947e, 4);
            }
            break;
        case 4:
            iVar5 = iVar5 + 0xc;
            break;
        case 5:
            iVar5 = iVar5 + 0x24;
            break;
        case 6:
            iVar5 = iVar5 + 0xc;
            break;
        case 7:
            uVar2 = 0xffffffff;
            pcVar6 = (char *)piVar7;
            do {
                if (uVar2 == 0) break;
                uVar2 = uVar2 - 1;
                cVar1 = *pcVar6;
                pcVar6 = pcVar6 + 1;
            } while (cVar1 != '\0');
            iVar5 = (int)((unsigned int)iVar5 + ~uVar2);
            break;
        case 9:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 0x38u);
            DAT_0060f1d8 = piVar7;
            break;
        case 0xb:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 0xcu);
            _DAT_0060f208 = piVar7;
            param_2[0x21] = (unsigned int)piVar7;
            break;
        case 0xd:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 8u);
            break;
        case 0x13:
            iVar5 = iVar5 + 0xc;
            break;
        case 0x14:
            iVar4 = (int)((unsigned int)piVar7[5] * (unsigned int)piVar7[4]);
            iVar3 = iVar4;
            if ((int)((unsigned int)iVar4 - (unsigned int)piVar7[2]) == 0) {
                iVar3 = piVar7[3];
            }
            iVar5 = (int)((unsigned int)iVar5 + 0x18u + (unsigned int)iVar3);
            if (piVar7[1] != 0) {
                iVar5 = (int)((unsigned int)iVar5 + (unsigned int)piVar7[1] * 0x300u);
                break;
            }
            if ((int)((unsigned int)iVar4 - (unsigned int)piVar7[2]) == 0) {
                iVar4 = piVar7[3];
            }
            if ((char *)*piVar7 != (char *)((int)piVar7 + iVar4 + 0x18)) break;
            goto LAB_00476a01;
        case 0x15:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 4u);
            break;
        case 0x16:
            iVar5 = iVar5 + 0x10;
            break;
        case 0x17:
            iVar5 = iVar5 + 0x30;
            break;
        case 0x19:
            iVar5 = iVar5 + 0x48;
        }
    }
switchD_00476863_caseD_8:
    if (param_1[2] != 0) {
        puVar8 = local_d0;
        for (iVar4 = 0x34; iVar4 != 0; iVar4 = iVar4 + -1) {
            *puVar8 = *param_2;
            param_2 = param_2 + 1;
            puVar8 = puVar8 + 1;
        }
        DAT_0060f1c0 = 0;
        DAT_0060f1e8 = 0;
        _DAT_0060f208 = (int *)0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1d8 = (int *)0;
        local_d4 = 0;
        iVar5 = (int)((unsigned int)iVar5 + param_1[2] * 4u);
        if (0 < (int)param_1[2]) {
            local_d8 = 0;
            do {
                iVar4 = FUN_00476810(*(unsigned int **)(param_1[3] + local_d8), local_d0);
                iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar4);
                local_d4 = local_d4 + 1;
                local_d8 = local_d8 + 4;
            } while (local_d4 < (int)param_1[2]);
        }
    }
    return iVar5;
}
