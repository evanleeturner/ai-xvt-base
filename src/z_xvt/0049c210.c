/* SPDX-License-Identifier: MIT */
/* z_xvt 0049c210 */
extern int DAT_0052accc;
extern int DAT_00645a88;
extern int DAT_0052acc8;
extern int DAT_0052acc4;
extern void __cdecl FUN_0049b8a0(unsigned int *param_1);

void __stdcall FUN_0049c210(void)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int *puVar3;
    int iVar4;
    int iVar5;
    unsigned char *pbVar6;
    unsigned char *pbVar7;
    
    iVar4 = 0;
    while (DAT_0052accc != 0) {
        DAT_0052accc = DAT_0052accc - 1;
        puVar3 = (unsigned int *)(DAT_00645a88 + iVar4);
        iVar4 = iVar4 + 9;
        pbVar6 = (unsigned char *)((int)puVar3 + 9);
        for (uVar2 = (unsigned int)*(unsigned char *)(puVar3 + 2); uVar2 != 0; uVar2 = uVar2 - 1) {
            iVar4 = iVar4 + 1;
            uVar1 = (unsigned int)*pbVar6;
            pbVar6 = pbVar6 + 1;
            for (; uVar1 != 0; uVar1 = uVar1 - 1) {
                pbVar7 = pbVar6 + 1;
                iVar5 = iVar4 + 1;
                if ((*pbVar6 & 0x7f) == 0x7f) {
                    pbVar7 = pbVar6 + 5;
                    iVar5 = iVar4 + 5;
                }
                if ((*pbVar6 & 0x80) != 0) {
                    pbVar7 = pbVar7 + 1;
                    iVar5 = iVar5 + 1;
                }
                pbVar6 = pbVar7 + 2;
                iVar4 = iVar5 + 2;
            }
        }
        puVar3[1] = puVar3[1] & 0x7fffffff;
        FUN_0049b8a0(puVar3);
    }
    DAT_0052acc8 = DAT_0052acc4;
    return;
}
