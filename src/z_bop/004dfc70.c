/* SPDX-License-Identifier: MIT */
/* z_bop 004dfc70 */
void __cdecl FUN_004dfc70(undefined4 *param_1, undefined4 *param_2)
{
    undefined4 *puVar2 = (undefined4 *)*param_1;
    byte *pbVar5 = (byte *)*param_2;
    byte *pbVar4 = (byte *)*puVar2;
    int iVar3;
    int bVar6;
    byte bVar1;

    do {
        bVar1 = *pbVar5;
        bVar6 = bVar1 < *pbVar4;
        if (bVar1 != *pbVar4) {
LAB_004dfca1:
            iVar3 = (1 - (unsigned int)bVar6) - (unsigned int)(bVar6 != 0);
            goto LAB_004dfca6;
        }
        if (bVar1 == 0) break;
        bVar1 = pbVar5[1];
        bVar6 = bVar1 < pbVar4[1];
        if (bVar1 != pbVar4[1]) goto LAB_004dfca1;
        pbVar5 = pbVar5 + 2;
        pbVar4 = pbVar4 + 2;
    } while (bVar1 != 0);
    iVar3 = 0;
LAB_004dfca6:
    if (iVar3 < 0) {
        param_2[1] = (undefined4)puVar2;
        *param_1 = (undefined4)param_2;
        param_1[1] = param_1[1] + 1;
        return;
    }
    iVar3 = puVar2[1];
    do {
        if (iVar3 == 0) {
            puVar2[1] = (undefined4)param_2;
            param_1[1] = param_1[1] + 1;
            return;
        }
        pbVar5 = (byte *)*param_2;
        pbVar4 = *(byte **)puVar2[1];
        do {
            bVar1 = *pbVar5;
            bVar6 = bVar1 < *pbVar4;
            if (bVar1 != *pbVar4) {
LAB_004dfce3:
                iVar3 = (1 - (unsigned int)bVar6) - (unsigned int)(bVar6 != 0);
                goto LAB_004dfce8;
            }
            if (bVar1 == 0) break;
            bVar1 = pbVar5[1];
            bVar6 = bVar1 < pbVar4[1];
            if (bVar1 != pbVar4[1]) goto LAB_004dfce3;
            pbVar5 = pbVar5 + 2;
            pbVar4 = pbVar4 + 2;
        } while (bVar1 != 0);
        iVar3 = 0;
LAB_004dfce8:
        if (iVar3 < 0) {
            param_2[1] = (undefined4)puVar2[1];
            puVar2[1] = (undefined4)param_2;
            param_1[1] = param_1[1] + 1;
            return;
        }
        puVar2 = (undefined4 *)puVar2[1];
        iVar3 = puVar2[1];
    } while (1);
}
