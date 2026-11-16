/* SPDX-License-Identifier: MIT */
/* z_bop 00446960 */
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern short DAT_009a20ce;
extern short DAT_009a20e6;
extern char DAT_0051a44f[];
extern int DAT_009d77fc;
extern int DAT_009ec604;
extern int DAT_009d6828;
extern int DAT_00a081e8;
extern int DAT_009cd260;
extern int DAT_00a08298;
extern int DAT_009ecc34;

void __cdecl FUN_00434500(uint param_1, uint param_2);
void __cdecl FUN_00406d10(ushort param_1, ushort param_2);
ushort __cdecl FUN_00459750(int param_1);
ushort __cdecl FUN_00425d10(void);
void __cdecl FUN_0042e4d0(uint param_1, uint param_2, int param_3);

void __cdecl FUN_00446960(ushort param_1, uint param_2)
{
    int iVar1;
    int iVar2;
    ushort uVar3;
    int iVar4;
    char cVar5;
    uint unaff_ESI = 0;
    uint uVar6;
    int iVar7;
    short *pCnt;
    uint pObj;
    uint uTmp;
    ushort usTmp;
    ushort usZero = 0;

    uVar6 = (unaff_ESI & 0xffff0000u) | (uint)param_1;
    iVar7 = (int)((param_2 & 0xffff) * 0x23);
    iVar4 = (int)((uint)DAT_009a1fe8 + (uint)iVar7);
    if (*(char *)((uint)iVar4 + 2) == '\n') {
        cVar5 = *(char *)((uint)param_1 * 0x23 + 3 + (uint)DAT_009a1fe8);
        if ((cVar5 != -0x73) && (cVar5 != -0x72)) {
            cVar5 = -0x7d;
            goto LAB_00446ab1;
        }
    }
    else {
        if ((int)(uint)param_1 < DAT_009a1fe0) {
            cVar5 = -0x7f;
            pCnt = (short *)((uint)&DAT_009a20ce + (uint)*(unsigned char *)((uint)iVar4 + 0x16) * 0x126);
            *pCnt = (short)((unsigned short)*pCnt + 1);
            *(unsigned char *)((uint)iVar4 + 3) = 0;
            FUN_00434500(uVar6, param_2);
            goto LAB_00446ab1;
        }
        iVar2 = (int)((uint)param_1 * 0x23);
        iVar1 = (int)((uint)DAT_009a1fe8 + (uint)iVar2);
        cVar5 = *(char *)((uint)DAT_009a1fe8 + 3 + (uint)iVar2);
        if ((cVar5 != -0x73) && (cVar5 != -0x72)) {
            pCnt = (short *)((uint)&DAT_009a20ce + (uint)*(unsigned char *)((uint)iVar4 + 0x16) * 0x126);
            *pCnt = (short)((unsigned short)*pCnt + 1);
            cVar5 = -0x7f;
            if (*(char *)((uint)iVar4 + 3) == 'M') {
                __builtin_memcpy(&pObj, (void *)((uint)iVar1 + 0x1f), 4);
                __builtin_memcpy(&usTmp, (void *)(pObj + 0x85), 2);
                FUN_00406d10((ushort)param_2, usTmp);
            }
            *(unsigned char *)((uint)DAT_009a1fe8 + 3 + (uint)iVar7) = 0;
            __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar2), 4);
            __builtin_memcpy(&usTmp, (void *)(pObj + 0x85), 2);
            FUN_00434500((((param_2 >> 16) & 0xffffu) << 16) | (uint)usTmp, param_2);
            goto LAB_00446ab1;
        }
        __builtin_memcpy((void *)((uint)iVar4 + 0x17), &usZero, 2);
        pCnt = (short *)((uint)&DAT_009a20e6 + (uint)*(unsigned char *)((uint)DAT_009a1fe8 + 0x16 + (uint)iVar7) * 0x126);
        *pCnt = (short)((unsigned short)*pCnt + 1);
    }
    cVar5 = -0x7c;
LAB_00446ab1:
    if ((int)(uint)param_1 < DAT_009a1fe0) {
        uVar3 = FUN_00459750(0xd);
        uVar6 = (uint)uVar3;
        if (uVar3 == 0xffff) {
            return;
        }
        *(unsigned char *)(uVar6 * 0x23 + 3 + (uint)DAT_009a1fe8) = 0x89;
    }
    uVar6 = uVar6 & 0xffff;
    iVar7 = (int)(uVar6 * 0x23);

    uTmp = (uint)DAT_009d77fc + (uint)DAT_009ec604;
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 4 + (uint)iVar7), &uTmp, 4);
    uTmp = (uint)DAT_009d6828 + (uint)DAT_00a081e8;
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 8 + (uint)iVar7), &uTmp, 4);
    uTmp = (uint)DAT_009cd260 + (uint)DAT_00a08298;
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 0xc + (uint)iVar7), &uTmp, 4);

    if (DAT_0051a44f[*(unsigned char *)((uint)DAT_009a1fe8 + 3 + (uint)iVar7)] != '\0') {
        cVar5 = -0x7f;
    }
    *(char *)((uint)DAT_009a1fe8 + 3 + (uint)iVar7) = cVar5;
    *(unsigned char *)((uint)DAT_009a1fe8 + 2 + (uint)iVar7) = 0xd;

    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    *(unsigned char *)pObj = 5;
    *(unsigned char *)((uint)DAT_009a1fe8 + 0x19 + (uint)iVar7) = 2;
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    __builtin_memcpy((void *)(pObj + 0x79), &usZero, 2);
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    *(unsigned char *)(pObj + 1) = 0;
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    __builtin_memcpy((void *)(pObj + 0x83), &usZero, 2);
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    __builtin_memcpy((void *)(pObj + 0x81), &usZero, 2);
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 0x12 + (uint)iVar7), &usZero, 2);
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 0x10 + (uint)iVar7), &usZero, 2);
    __builtin_memcpy((void *)((uint)DAT_009a1fe8 + 0x14 + (uint)iVar7), &usZero, 2);
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    *(unsigned char *)(pObj + 0x92) = 1;
    __builtin_memcpy(&pObj, (void *)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar7), 4);
    *(unsigned char *)(pObj + 0x8b) = *(unsigned char *)(pObj + 0x92);

    if ((cVar5 != -0x7d) && (cVar5 != -0x7c)) {
        iVar7 = DAT_009ecc34;
        uVar3 = FUN_00425d10();
        FUN_0042e4d0((uint)(ushort)((uVar3 & 3) + 0x16), uVar6, iVar7);
        return;
    }
    FUN_0042e4d0(0x1c, uVar6, DAT_009ecc34);
    return;
}
