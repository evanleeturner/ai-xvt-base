/* SPDX-License-Identifier: MIT */
/* z_xvt 0045c050 */
extern int * __cdecl FUN_0045bcb0(unsigned int param_1);
extern void __cdecl FUN_004b696b(int * param_1);
extern int * __cdecl FUN_004b88a0(int param_1, int param_2, int param_3);
extern void __cdecl FUN_0045bbe0(const char *param_1, ...);
extern int __cdecl FUN_004b63ea(int param_1, int param_2, int * param_3, void * param_4);
extern int DAT_006cca80;

int * __cdecl FUN_0045c050(int param_1, int param_2, unsigned int param_3, unsigned char * param_4)
{
    int * piVar1;
    int iVar2;
    int iVar3;
    void * unaff_EBP = 0;
    unsigned short * puVar4;
    unsigned char * pbVar5;
    int unaff_EDI = 0;
    unsigned int * puVar6;
    int * piVar7;
    unsigned int local_6c[4];
    unsigned short * puStack_5c = 0;
    int iStack_10 = 0;
    int iStack_8 = 0;

    piVar1 = (int *)FUN_0045bcb0((unsigned int)param_4);
    iVar2 = DAT_006cca80;
    if (piVar1[2] != 0) {
        FUN_004b696b(piVar1);
        return piVar1;
    }
    piVar1 = FUN_004b88a0(param_1, param_2, DAT_006cca80);
    puVar6 = local_6c;
    for (iVar3 = 0x1b; iVar3 != 0; iVar3 = iVar3 - 1) {
        *puVar6 = 0;
        puVar6 = puVar6 + 1;
    }
    piVar7 = (int *)0x0;
    local_6c[0] = 0x6c;
    iVar3 = ((int (*)(int *, int, unsigned int *, int))(*(int *)piVar1[3] + 100))((int *)piVar1[3], 0, local_6c, 1);
    if (iVar3 != 0) {
        FUN_0045bbe0((const char *)0x00527acc, iVar3);
    }
    while (param_2 != 0) {
        param_2 = param_2 - 1;
        iVar3 = iStack_10;
        puVar4 = puStack_5c;
        pbVar5 = param_4;
        if (0 < iStack_10) {
            do {
                param_4 = pbVar5 + 1;
                iVar3 = iVar3 - 1;
                *puVar4 = *(unsigned short *)(iStack_8 + (unsigned int)*pbVar5 * 2);
                puVar4 = puVar4 + 1;
                pbVar5 = param_4;
            } while (iVar3 != 0);
        }
        puStack_5c = (unsigned short *)((int)puStack_5c + iVar2);
    }
    iVar2 = FUN_004b63ea(unaff_EDI, 0, piVar7, unaff_EBP);
    if (iVar2 != 0) {
        return piVar7;
    }
    FUN_0045bbe0((const char *)0x00527aa8);
    return (int *)0x0;
}
