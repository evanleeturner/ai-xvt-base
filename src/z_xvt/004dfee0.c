/* SPDX-License-Identifier: MIT */
/* z_xvt 004dfee0 */
void __cdecl FUN_004dfee0(unsigned int *param_1, unsigned int *param_2)
{
    unsigned char bVar1;
    unsigned int *puVar2;
    int iVar3;
    unsigned char *pbVar4;
    unsigned char *pbVar5;
    unsigned int bVar6;

    puVar2 = (unsigned int *)*param_1;
    pbVar5 = (unsigned char *)*param_2;
    pbVar4 = (unsigned char *)*puVar2;
    do {
        bVar1 = *pbVar5;
        bVar6 = bVar1 < *pbVar4;
        if (bVar1 != *pbVar4) {
LAB_004dff11:
            iVar3 = (1 - (unsigned int)bVar6) - (unsigned int)(bVar6 != 0);
            goto LAB_004dff16;
        }
        if (bVar1 == 0) break;
        bVar1 = pbVar5[1];
        bVar6 = bVar1 < pbVar4[1];
        if (bVar1 != pbVar4[1]) goto LAB_004dff11;
        pbVar5 = pbVar5 + 2;
        pbVar4 = pbVar4 + 2;
    } while (bVar1 != 0);
    iVar3 = 0;
LAB_004dff16:
    if (iVar3 < 0) {
        param_2[1] = (unsigned int)puVar2;
        *param_1 = (unsigned int)param_2;
        param_1[1] = param_1[1] + 1;
        return;
    }
    iVar3 = puVar2[1];
    do {
        if (iVar3 == 0) {
            puVar2[1] = (unsigned int)param_2;
            param_1[1] = param_1[1] + 1;
            return;
        }
        pbVar5 = (unsigned char *)*param_2;
        pbVar4 = (unsigned char *)*(unsigned int *)puVar2[1];
        do {
            bVar1 = *pbVar5;
            bVar6 = bVar1 < *pbVar4;
            if (bVar1 != *pbVar4) {
LAB_004dff58:
                iVar3 = (1 - (unsigned int)bVar6) - (unsigned int)(bVar6 != 0);
                goto LAB_004dff5d;
            }
            if (bVar1 == 0) break;
            bVar1 = pbVar5[1];
            bVar6 = bVar1 < pbVar4[1];
            if (bVar1 != pbVar4[1]) goto LAB_004dff58;
            pbVar5 = pbVar5 + 2;
            pbVar4 = pbVar4 + 2;
        } while (bVar1 != 0);
        iVar3 = 0;
LAB_004dff5d:
        if (iVar3 < 0) {
            param_2[1] = puVar2[1];
            puVar2[1] = (unsigned int)param_2;
            param_1[1] = param_1[1] + 1;
            return;
        }
        puVar2 = (unsigned int *)puVar2[1];
        iVar3 = puVar2[1];
    } while (1);
}
