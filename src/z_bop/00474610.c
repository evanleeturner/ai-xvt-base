/* SPDX-License-Identifier: MIT */
/* z_bop 00474610 */
/* Prototypes for every callee */
unsigned int  __cdecl FUN_004ac6e0(unsigned int a);
unsigned int  __cdecl FUN_004ac5f0(unsigned int a, int b);
unsigned int  __cdecl FUN_0049eb60(int a);
int *         __cdecl FUN_004ac720(unsigned int h);
void          __cdecl FUN_00471ff0(int *p);
void          __cdecl FUN_004ac740(void);
int           __cdecl FUN_00474960(int *a, int *b, int c, int d, unsigned int *e);

extern unsigned int   DAT_005272b8;
extern unsigned short DAT_005272b4;
extern unsigned short DAT_005272a8;
extern unsigned int   DAT_005272ac;
extern unsigned int   DAT_0060f1c0;
extern unsigned int   DAT_0060f1e8;
extern unsigned int   _DAT_0060f208;
extern unsigned int   _DAT_0060f1f0;
extern unsigned int   DAT_0060f1d8;
extern unsigned int   DAT_0060f204;
extern unsigned int   DAT_0060f200;
extern unsigned int   DAT_0060f1f4;
extern unsigned int   DAT_0060f1cc;

int __cdecl FUN_00474610(unsigned int param_1)
{
    unsigned int uVar1;
    unsigned short uVar8;
    unsigned int uVar2;
    int *piVar3;
    int *piVar4;
    unsigned int uVar5;
    int *piVar6;
    int iVar7;
    int iVar9;
    unsigned int *puVar10;
    int local_d8;
    int local_d4;
    unsigned int local_d0[52];
    int tmp6;
    int tmp10;
    int cnt;
    int newcnt;
    int ptrval;
    int srcptr;

    uVar1 = DAT_005272b8;
    if (((int)DAT_005272b8 < (int)param_1) && (DAT_005272b4 != 0)) {
        uVar1 = FUN_004ac6e0((unsigned int)DAT_005272b4);
        DAT_005272b4 = 0;
        DAT_005272b8 = 0;
    }
    uVar8 = (unsigned short)(uVar1 >> 0x10);
    if (DAT_005272b4 == 0) {
        uVar2 = FUN_004ac5f0(param_1, 0);
        DAT_005272b4 = (unsigned short)uVar2;
        if (DAT_005272b4 == 0) {
            uVar2 = FUN_0049eb60(0);
        }
        uVar8 = (unsigned short)(uVar2 >> 0x10);
        DAT_005272b8 = param_1;
    }
    piVar3 = FUN_004ac720(((unsigned int)uVar8 << 16) | (unsigned int)DAT_005272b4);
    piVar4 = FUN_004ac720((((unsigned int)piVar3 >> 0x10) << 16) | (unsigned int)DAT_005272a8);
    piVar6 = piVar3;
    for (uVar1 = param_1 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
        *piVar6 = *piVar4;
        piVar4 = piVar4 + 1;
        piVar6 = piVar6 + 1;
    }
    for (uVar1 = param_1 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
        *(char *)piVar6 = *(char *)piVar4;
        piVar4 = (int *)((int)piVar4 + 1);
        piVar6 = (int *)((int)piVar6 + 1);
    }
    if ((int *)*piVar3 != piVar3) {
        FUN_00471ff0(piVar3);
    }
    uVar1 = param_1 * 2;
    FUN_004ac740();
    uVar5 = DAT_005272ac;
    if ((uVar1 - DAT_005272ac != 0 && (int)DAT_005272ac <= (int)uVar1) && (DAT_005272a8 != 0)) {
        uVar5 = FUN_004ac6e0((unsigned int)DAT_005272a8);
        DAT_005272a8 = 0;
        DAT_005272ac = 0;
    }
    uVar8 = (unsigned short)(uVar5 >> 0x10);
    if (DAT_005272a8 == 0) {
        uVar2 = FUN_004ac5f0(uVar1, 0);
        DAT_005272a8 = (unsigned short)uVar2;
        if (DAT_005272a8 == 0) {
            uVar2 = FUN_0049eb60(0);
        }
        uVar8 = (unsigned short)(uVar2 >> 0x10);
        DAT_005272ac = uVar1;
    }
    piVar6 = FUN_004ac720(((unsigned int)uVar8 << 16) | (unsigned int)DAT_005272a8);
    *piVar6 = *piVar3;
    piVar6[1] = piVar3[1];
    piVar6[2] = piVar3[2];
    *(short *)(piVar6 + 3) = (short)piVar3[3];
    *piVar6 = (int)piVar6;
    ptrval = (int)((unsigned int)(int)piVar6 + 0xe);
    __builtin_memcpy((char *)piVar6 + 10, &ptrval, 4);
    __builtin_memcpy(&cnt, (char *)piVar3 + 6, 4);
    piVar4 = (int *)((unsigned int)(int)piVar6 + 0xe + (unsigned int)cnt * 4);
    local_d4 = (int)((unsigned int)cnt * 4 + 0xe);
    puVar10 = local_d0;
    for (iVar9 = 0x34; iVar9 != 0; iVar9 = iVar9 + -1) {
        *puVar10 = 0;
        puVar10 = puVar10 + 1;
    }
    DAT_0060f1c0 = 0;
    DAT_0060f1e8 = 0;
    _DAT_0060f208 = 0;
    _DAT_0060f1f0 = 0;
    DAT_0060f1d8 = 0;
    DAT_0060f204 = 0;
    tmp6 = 0;
    __builtin_memcpy((char *)piVar6 + 6, &tmp6, 4);
    local_d8 = 0;
    __builtin_memcpy(&cnt, (char *)piVar3 + 6, 4);
    if (0 < cnt) {
        iVar9 = 0;
        do {
            iVar9 = (int)((unsigned int)iVar9 + 4);
            __builtin_memcpy(&tmp6, (char *)piVar6 + 6, 4);
            newcnt = (int)((unsigned int)tmp6 + 1);
            __builtin_memcpy((char *)piVar6 + 6, &newcnt, 4);
            __builtin_memcpy(&tmp10, (char *)piVar6 + 10, 4);
            ptrval = (int)piVar4;
            __builtin_memcpy((char *)((unsigned int)tmp10 - 4 + (unsigned int)iVar9), &ptrval, 4);
            DAT_0060f200 = 0;
            DAT_0060f1f4 = 0;
            DAT_0060f1cc = 0;
            __builtin_memcpy(&tmp10, (char *)piVar3 + 10, 4);
            __builtin_memcpy(&srcptr, (char *)((unsigned int)tmp10 - 4 + (unsigned int)iVar9), 4);
            iVar7 = FUN_00474960(piVar4, (int *)srcptr, (int)piVar3, (int)piVar6, local_d0);
            local_d4 = (int)((unsigned int)local_d4 + (unsigned int)iVar7);
            piVar4 = (int *)((unsigned int)(int)piVar4 + (unsigned int)iVar7);
            local_d8 = local_d8 + 1;
            __builtin_memcpy(&cnt, (char *)piVar3 + 6, 4);
        } while (local_d8 < cnt);
    }
    return local_d4;
}
