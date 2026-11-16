/* SPDX-License-Identifier: MIT */
/* z_bop 00443ef0 */
void  _free(void *);
void *_malloc(unsigned int);
void  _exit(int);
void  FUN_004438b0(const char *, const char *);
void  FUN_004445e0(int);
int   FUN_004446c0(int);
void  FUN_004444f0(int);
void  FUN_00443890(const char *);

extern int DAT_00556928;
extern int DAT_00556930;
extern unsigned char DAT_00556934;
extern unsigned char DAT_00556935;
extern unsigned char DAT_00556936;
/* packed (unaligned) globals */
extern unsigned char DAT_0055693d[4];
extern unsigned char DAT_00556941[4];
extern unsigned char DAT_00556945[4];
extern unsigned char DAT_00556965[2];

void __cdecl FUN_00443ef0(int param_1, int param_2, int param_3, int param_4)
{
    unsigned char *p = (unsigned char *)param_1;
    int iVar1;
    void *pvVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned char bVar5;
    short sVar6;
    unsigned char *pbVar7;
    int bVar8;
    unsigned int local_4;
    void *tmpPtr;
    unsigned int tmpCount;
    unsigned int zero32;
    unsigned int val41080000;
    unsigned short us;
    int node;
    int next;

    if (*(void **)(p + 0x104c) != (void *)0) {
        _free(*(void **)(p + 0x104c));
    }
    pvVar2 = _malloc((unsigned int)DAT_00556930 * 9u);
    *(void **)(p + 0x104c) = pvVar2;
    if (pvVar2 == (void *)0) {
        FUN_004438b0("Unable to quantize image", "Memory allocation failed");
        _exit(1);
    }
    __builtin_memcpy(&tmpPtr, p + 0x1082, 4);
    if (tmpPtr != (void *)0) {
        __builtin_memcpy(&tmpPtr, p + 0x1082, 4);
        _free(tmpPtr);
    }
    zero32 = 0;
    __builtin_memcpy(p + 0x1082, &zero32, 4);
    tmpPtr = *(void **)(p + 0x104c);
    __builtin_memcpy(DAT_0055693d, &tmpPtr, 4);
    DAT_00556930 = 0;
    FUN_004445e0(DAT_00556928);

    if ((param_2 == 2) && (param_4 == 2)) {
        pbVar7 = *(unsigned char **)(p + 0x104c);
        uVar4 = (unsigned int)(((((unsigned int)pbVar7[9] * 0x4du + (unsigned int)pbVar7[0xb] * 0x1du) + (unsigned int)pbVar7[10] * 0x96u) &
                                0xffffff00u) <
                               ((((unsigned int)pbVar7[2] * 0x1du + (unsigned int)pbVar7[1] * 0x96u) + (unsigned int)*pbVar7 * 0x4du) &
                                0xffffff00u));
        iVar3 = (int)(uVar4 * 9u);
        pbVar7[iVar3] = 0;
        *(unsigned char *)(*(int *)(p + 0x104c) + 1 + iVar3) = 0;
        iVar1 = (int)((unsigned int)(uVar4 == 0) * 9u);
        *(unsigned char *)(*(int *)(p + 0x104c) + 2 + iVar3) = 0;
        *(unsigned char *)(*(int *)(p + 0x104c) + iVar1) = 0xff;
        *(unsigned char *)(*(int *)(p + 0x104c) + 1 + iVar1) = 0xff;
        *(unsigned char *)(*(int *)(p + 0x104c) + 2 + iVar1) = 0xff;
    }
    if (param_4 != 3) {
        *(int *)(p + 0x1028) = 0;
        *(int *)(p + 0x1024) = 2;
    }
    *(int *)(p + 0x1054) = DAT_00556930;
    bVar8 = 0;
    if (param_3 != 0) {
        iVar3 = FUN_004446c0(param_1);
        bVar8 = (iVar3 == 0);
    }
    __builtin_memcpy(&pbVar7, p + 0x1086, 4);
    __builtin_memcpy(&tmpCount, p + 0x108e, 4);
    if ((!bVar8) && (local_4 = 0, tmpCount != 0)) {
        do {
            sVar6 = 7;
            iVar3 = DAT_00556928;
            do {
                bVar5 = (unsigned char)sVar6;
                uVar4 = ((((unsigned int)(*pbVar7 >> (bVar5 & 0x1f)) & 1u) * 2u) |
                         ((unsigned int)(pbVar7[1] >> (bVar5 & 0x1f)) & 1u)) * 2u |
                        ((unsigned int)(pbVar7[2] >> (bVar5 & 0x1f)) & 1u);
                if ((((unsigned int)1 << (signed char)uVar4) & (unsigned int)*(unsigned char *)(iVar3 + 2)) == 0) {
                    break;
                }
                sVar6 = (short)(sVar6 - 1);
                __builtin_memcpy(&next, (unsigned char *)(iVar3 + 0x32 + (int)(uVar4 * 4u)), 4);
                iVar3 = next;
            } while (sVar6 != 0);
            DAT_00556934 = *pbVar7;
            DAT_00556935 = pbVar7[1];
            DAT_00556936 = pbVar7[2];
            zero32 = 0;
            __builtin_memcpy(DAT_00556941, &zero32, 4);
            val41080000 = 0x41080000u;
            __builtin_memcpy(DAT_00556945, &val41080000, 4);
            __builtin_memcpy(&node, (unsigned char *)(iVar3 + 0x2e), 4);
            FUN_004444f0(node);
            if (*(int *)(p + 0x1024) == 2) {
                __builtin_memcpy(&us, DAT_00556965, 2);
                __builtin_memcpy(pbVar7 + 4, &us, 2);
            } else {
                __builtin_memcpy(&us, DAT_00556965, 2);
                iVar3 = (int)((unsigned int)us * 9u);
                *pbVar7 = *(unsigned char *)(*(int *)(p + 0x104c) + iVar3);
                pbVar7[1] = *(unsigned char *)(*(int *)(p + 0x104c) + 1 + iVar3);
                pbVar7[2] = *(unsigned char *)(*(int *)(p + 0x104c) + 2 + iVar3);
            }
            pbVar7 = pbVar7 + 6;
            __builtin_memcpy(&tmpCount, p + 0x108e, 4);
            if ((tmpCount - local_4 == 1) ||
                (local_4 % *(unsigned int *)(p + 0x1034) == 0)) {
                FUN_00443890("  Assigning image colors...  ");
            }
            local_4 = local_4 + 1;
            __builtin_memcpy(&tmpCount, p + 0x108e, 4);
        } while (local_4 < tmpCount);
    }
    return;
}
