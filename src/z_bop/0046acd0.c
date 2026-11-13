/* SPDX-License-Identifier: MIT */
/* z_bop 0046acd0 */
extern unsigned char s_LOCATARGET_00525280[];

int __cdecl FUN_0046acd0(unsigned char *param_1)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    int iVar3;
    int iVar4;
    unsigned char *pcVar5;
    unsigned char *pbVar6;
    unsigned char *pbVar7;
    int bVar8;
    
    iVar3 = 0;
    if (s_LOCATARGET_00525280[0] != '\0') {
        pcVar5 = s_LOCATARGET_00525280;
        pbVar7 = param_1;
        pbVar6 = pcVar5;
LAB_0046acec:
        do {
            bVar2 = *pcVar5;
            bVar8 = bVar2 < *pbVar7;
            if (bVar2 == *pbVar7) {
                if (bVar2 != 0) {
                    bVar2 = pcVar5[1];
                    bVar8 = bVar2 < pbVar7[1];
                    if (bVar2 != pbVar7[1]) goto LAB_0046ad0c;
                    pcVar5 = pcVar5 + 2;
                    pbVar7 = pbVar7 + 2;
                    if (bVar2 != 0) goto LAB_0046acec;
                }
                iVar4 = 0;
            }
            else {
LAB_0046ad0c:
                iVar4 = (1 - (unsigned int)bVar8) - (unsigned int)(bVar8 != 0);
            }
            if (iVar4 == 0) {
                return iVar3;
            }
            pbVar1 = pbVar6 + 0x52;
            pcVar5 = pbVar1;
            iVar3 = iVar3 + 1;
            pbVar7 = param_1;
            pbVar6 = pcVar5;
        } while (*pbVar1 != 0);
    }
    return iVar3;
}
