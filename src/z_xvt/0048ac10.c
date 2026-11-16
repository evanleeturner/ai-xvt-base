/* SPDX-License-Identifier: MIT */
/* z_xvt 0048ac10 */
extern int DAT_006a8884;
extern int DAT_006026bc;
extern unsigned char DAT_006a887a[4]; /* packed int at 0x6a887a (not 4-aligned) */
extern int DAT_00602678;
extern int DAT_006026a0;
extern int DAT_006026a8;
extern int *DAT_00602690;
extern int *_DAT_006026c0;

int FUN_0048ac10(unsigned int *param_1, unsigned int *param_2);

int FUN_0048ac10(unsigned int *param_1, unsigned int *param_2)
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
        if (param_1[1] != 0x14) goto switchD_0048ac64_caseD_8;
        iVar3 = (int)((unsigned int)*(volatile int *)0x14 * (unsigned int)*(volatile int *)0x10);
        iVar4 = iVar3;
        if ((int)((unsigned int)iVar3 - (unsigned int)*(volatile int *)0x8) == 0) {
            iVar4 = *(volatile int *)0xc;
        }
        iVar5 = (int)((unsigned int)iVar5 + 0x18u + (unsigned int)iVar4);
        if (*(volatile int *)0x4 != 0) {
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)*(volatile int *)0x4 * 0x300u);
            goto switchD_0048ac64_caseD_8;
        }
        if ((int)((unsigned int)iVar3 - (unsigned int)*(volatile int *)0x8) == 0) {
            iVar3 = *(volatile int *)0xc;
        }
        if (*(volatile int *)0x0 != (int)((unsigned int)iVar3 + 0x18u)) goto switchD_0048ac64_caseD_8;
LAB_0048adfc:
        iVar5 = (int)((unsigned int)iVar5 + 0x3000u);
    } else {
        switch (param_1[1]) {
        case 1:
        case 0xf:
        case 0x10:
        case 0x11:
            if (DAT_006a8884 < *piVar7) {
                DAT_006a8884 = *piVar7;
            }
            iVar5 = (int)((unsigned int)iVar5 + 4u + param_1[4] * 100u);
            if (param_2[0x21] == 0) {
                iVar5 = (int)((unsigned int)iVar5 + (unsigned int)DAT_006026bc * 0xcu);
            }
            break;
        case 2:
            iVar5 = (int)((unsigned int)iVar5 + 0x30u);
            break;
        case 3:
        {
            int tmp3;
            DAT_006026bc = (int)param_1[4];
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)DAT_006026bc * 0xcu);
            __builtin_memcpy(&tmp3, DAT_006a887a, 4);
            if (tmp3 < (int)param_1[4]) {
                tmp3 = (int)param_1[4];
                __builtin_memcpy(DAT_006a887a, &tmp3, 4);
            }
            break;
        }
        case 4:
            iVar5 = (int)((unsigned int)iVar5 + 0xcu);
            break;
        case 5:
            iVar5 = (int)((unsigned int)iVar5 + 0x24u);
            break;
        case 6:
            iVar5 = (int)((unsigned int)iVar5 + 0xcu);
            break;
        case 7:
        {
            char *pcVar7 = (char *)piVar7;
            uVar2 = 0xffffffff;
            do {
                if (uVar2 == 0) break;
                uVar2 = uVar2 - 1;
                cVar1 = *pcVar7;
                pcVar7 = pcVar7 + 1;
            } while (cVar1 != '\0');
            iVar5 = (int)((unsigned int)iVar5 + ~uVar2);
            break;
        }
        case 9:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 0x38u);
            DAT_00602690 = piVar7;
            break;
        case 0xb:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 0xcu);
            _DAT_006026c0 = piVar7;
            param_2[0x21] = (unsigned int)piVar7;
            break;
        case 0xd:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 8u);
            break;
        case 0x13:
            iVar5 = (int)((unsigned int)iVar5 + 0xcu);
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
            if ((char *)*piVar7 != (char *)((unsigned int)piVar7 + (unsigned int)iVar4 + 0x18u)) break;
            goto LAB_0048adfc;
        case 0x15:
            iVar5 = (int)((unsigned int)iVar5 + param_1[4] * 4u);
            break;
        case 0x16:
            iVar5 = (int)((unsigned int)iVar5 + 0x10u);
            break;
        case 0x17:
            iVar5 = (int)((unsigned int)iVar5 + 0x30u);
            break;
        case 0x19:
            iVar5 = (int)((unsigned int)iVar5 + 0x48u);
        }
    }
switchD_0048ac64_caseD_8:
    if (param_1[2] != 0) {
        puVar8 = local_d0;
        for (iVar4 = 0x34; iVar4 != 0; iVar4 = iVar4 + -1) {
            *puVar8 = *param_2;
            param_2 = param_2 + 1;
            puVar8 = puVar8 + 1;
        }
        DAT_00602678 = 0;
        DAT_006026a0 = 0;
        _DAT_006026c0 = (int *)0;
        DAT_006026a8 = 0;
        DAT_00602690 = (int *)0;
        local_d4 = 0;
        iVar5 = (int)((unsigned int)iVar5 + param_1[2] * 4u);
        if (0 < (int)param_1[2]) {
            local_d8 = 0;
            do {
                iVar4 = FUN_0048ac10(*(unsigned int **)(param_1[3] + (unsigned int)local_d8), local_d0);
                iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar4);
                local_d4 = local_d4 + 1;
                local_d8 = local_d8 + 4;
            } while (local_d4 < (int)param_1[2]);
        }
    }
    return iVar5;
}
