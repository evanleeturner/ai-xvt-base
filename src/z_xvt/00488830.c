/* SPDX-License-Identifier: MIT */
/* z_xvt 00488830 */
extern unsigned short DAT_0052a544;
extern int            DAT_0052a548;
extern unsigned short DAT_0052a53c;
extern unsigned int   DAT_0052a540;
extern int            DAT_00602678;
extern int            DAT_006026a0;
extern int            _DAT_006026c0;
extern int            _DAT_006026a8;
extern int            DAT_00602690;
extern int            DAT_006026bc;

unsigned int  FUN_0048f4b0(unsigned int param_1);
unsigned int  FUN_0048f3c0(unsigned int param_1, int param_2);
unsigned int  FUN_0047f010(int param_1);
void         *FUN_0048f4f0(unsigned int param_1);
void          FUN_00486000(int *param_1, int param_2);
void          FUN_0048f510(void);
int           FUN_00488bc0(int *param_1, int *param_2, int param_3, int param_4, unsigned int *param_5);

int FUN_00488830(unsigned int param_1)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int *puVar3;
    unsigned int *puVar4;
    unsigned int uVar5;
    unsigned short uVar8;
    unsigned int uVar6;
    unsigned int *puVar7;
    int *piVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int local_d8;
    int local_d4;
    unsigned int local_d0[52];
    int t10;
    int t6;
    int t6b;
    int tv;
    unsigned short ts;

    uVar2 = param_1;
    if (((int)DAT_0052a548 < (int)param_1) && (DAT_0052a544 != 0)) {
        uVar2 = FUN_0048f4b0((unsigned int)DAT_0052a544);
        DAT_0052a544 = 0;
        DAT_0052a548 = 0;
    }
    if (DAT_0052a544 == 0) {
        DAT_0052a544 = (unsigned short)FUN_0048f3c0(param_1, 0);
        if (DAT_0052a544 == 0) {
            FUN_0047f010(0);
        }
        DAT_0052a548 = (int)param_1;
        uVar2 = param_1;
    }
    puVar3 = (unsigned int *)FUN_0048f4f0(((uVar2 >> 16) << 16) | (unsigned int)DAT_0052a544);
    puVar4 = (unsigned int *)FUN_0048f4f0((((unsigned int)puVar3 >> 16) << 16) | (unsigned int)DAT_0052a53c);
    puVar7 = puVar3;
    for (uVar2 = param_1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar7 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar7 = puVar7 + 1;
    }
    for (uVar2 = param_1 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
        *(unsigned char *)puVar7 = *(unsigned char *)puVar4;
        puVar4 = (unsigned int *)((int)puVar4 + 1);
        puVar7 = (unsigned int *)((int)puVar7 + 1);
    }
    puVar4 = (unsigned int *)*puVar3;
    if (puVar3 != puVar4) {
        iVar12 = 0;
        iVar11 = (int)((unsigned int)puVar3 - (unsigned int)puVar4);
        *puVar3 = (unsigned int)puVar4 + (unsigned int)iVar11;
        __builtin_memcpy(&t10, (unsigned char *)puVar3 + 10, 4);
        if (t10 != 0) {
            tv = (int)((unsigned int)t10 + (unsigned int)iVar11);
            __builtin_memcpy((unsigned char *)puVar3 + 10, &tv, 4);
            __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
            if (0 < t6) {
                iVar10 = 0;
                do {
                    __builtin_memcpy(&t10, (unsigned char *)puVar3 + 10, 4);
                    piVar9 = (int *)((unsigned int)t10 + (unsigned int)iVar10);
                    __builtin_memcpy(&iVar1, piVar9, 4);
                    if (iVar1 != 0) {
                        tv = (int)((unsigned int)iVar1 + (unsigned int)iVar11);
                        __builtin_memcpy(piVar9, &tv, 4);
                        __builtin_memcpy(&t10, (unsigned char *)puVar3 + 10, 4);
                        __builtin_memcpy(&tv, (unsigned char *)((unsigned int)t10 + (unsigned int)iVar10), 4);
                        FUN_00486000((int *)tv, iVar11);
                    }
                    iVar10 = (int)((unsigned int)iVar10 + 4u);
                    iVar12 = (int)((unsigned int)iVar12 + 1u);
                    __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
                } while (iVar12 < t6);
            }
        }
    }
    uVar2 = param_1 * 2;
    FUN_0048f510();
    uVar5 = DAT_0052a540;
    if (((int)DAT_0052a540 < (int)uVar2) && (DAT_0052a53c != 0)) {
        uVar5 = FUN_0048f4b0((unsigned int)DAT_0052a53c);
        DAT_0052a53c = 0;
        DAT_0052a540 = 0;
    }
    uVar8 = (unsigned short)(uVar5 >> 16);
    if (DAT_0052a53c == 0) {
        uVar6 = FUN_0048f3c0(uVar2, 0);
        DAT_0052a53c = (unsigned short)uVar6;
        if (DAT_0052a53c == 0) {
            uVar6 = FUN_0047f010(0);
        }
        uVar8 = (unsigned short)(uVar6 >> 16);
        DAT_0052a540 = uVar2;
    }
    puVar7 = (unsigned int *)FUN_0048f4f0(((unsigned int)uVar8 << 16) | (unsigned int)DAT_0052a53c);
    *puVar7 = *puVar3;
    puVar7[1] = puVar3[1];
    puVar7[2] = puVar3[2];
    ts = *(unsigned short *)(puVar3 + 3);
    *(unsigned short *)(puVar7 + 3) = ts;
    *puVar7 = (unsigned int)puVar7;
    tv = (int)((unsigned int)puVar7 + 0xeu);
    __builtin_memcpy((unsigned char *)puVar7 + 10, &tv, 4);
    __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
    piVar9 = (int *)((unsigned int)puVar7 + 0xeu + (unsigned int)t6 * 4u);
    __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
    local_d4 = (int)((unsigned int)t6 * 4u + 0xeu);
    puVar4 = local_d0;
    for (iVar11 = 0x34; iVar11 != 0; iVar11 = iVar11 + -1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    iVar11 = 0;
    DAT_00602678 = 0;
    DAT_006026a0 = 0;
    _DAT_006026c0 = 0;
    _DAT_006026a8 = 0;
    DAT_00602690 = 0;
    DAT_006026bc = 0;
    tv = 0;
    __builtin_memcpy((unsigned char *)puVar7 + 6, &tv, 4);
    local_d8 = 0;
    __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
    if (0 < t6) {
        do {
            iVar11 = (int)((unsigned int)iVar11 + 4u);
            __builtin_memcpy(&t6b, (unsigned char *)puVar7 + 6, 4);
            tv = (int)((unsigned int)t6b + 1u);
            __builtin_memcpy((unsigned char *)puVar7 + 6, &tv, 4);
            __builtin_memcpy(&t10, (unsigned char *)puVar7 + 10, 4);
            __builtin_memcpy((unsigned char *)((unsigned int)t10 - 4u + (unsigned int)iVar11), &piVar9, 4);
            __builtin_memcpy(&t10, (unsigned char *)puVar3 + 10, 4);
            __builtin_memcpy(&tv, (unsigned char *)((unsigned int)t10 - 4u + (unsigned int)iVar11), 4);
            iVar12 = FUN_00488bc0(piVar9, (int *)tv, (int)puVar3, (int)puVar7, local_d0);
            local_d4 = (int)((unsigned int)local_d4 + (unsigned int)iVar12);
            piVar9 = (int *)((unsigned int)piVar9 + (unsigned int)iVar12);
            local_d8 = local_d8 + 1;
            __builtin_memcpy(&t6, (unsigned char *)puVar3 + 6, 4);
        } while (local_d8 < t6);
    }
    return local_d4;
}
