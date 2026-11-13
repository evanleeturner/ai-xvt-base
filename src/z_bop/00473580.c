/* SPDX-License-Identifier: MIT */
/* z_bop 00473580 */
extern int DAT_005272a4;
extern short DAT_009a7ed0[];
extern int DAT_0060f1c0;
extern int DAT_0060f1e8;
extern int DAT_0060f208;
extern int DAT_0060f1f0;
extern int DAT_0060f1d8;
extern int DAT_0060f204;

extern void FUN_004ac740(void);
extern void *FUN_004ac720(int);
extern void FUN_00471ff0(int *);
extern int FUN_004736b0(int, int *, int *, float *, float *);

int __cdecl FUN_00473580(int param_1, float *param_2, float *param_3)
{
    int unaff_ESI = 0;
    short uVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int *piVar6;
    int aiStack_d0[0x34];
    int count;
    
    if (DAT_005272a4 == 0) {
        return 0;
    }
    
    uVar1 = DAT_009a7ed0[*(unsigned char *)((char *)param_1 + 3)];
    FUN_004ac740();
    piVar2 = (int *)FUN_004ac720((unaff_ESI & 0xFFFF0000) | (unsigned short)uVar1);
    
    if (*piVar2 != (int)piVar2) {
        FUN_00471ff0(piVar2);
    }
    
    piVar6 = aiStack_d0;
    for (iVar4 = 0x34; iVar4 != 0; iVar4--) {
        *piVar6 = 0;
        piVar6++;
    }
    
    iVar4 = 0;
    aiStack_d0[0] = param_1;
    aiStack_d0[1] = 0x3f800000;
    aiStack_d0[2] = 0;
    aiStack_d0[3] = 0;
    aiStack_d0[4] = 0;
    aiStack_d0[5] = 0x3f800000;
    aiStack_d0[6] = 0;
    aiStack_d0[7] = 0;
    aiStack_d0[8] = 0;
    aiStack_d0[9] = 0x3f800000;
    aiStack_d0[10] = 0x3f800000;
    aiStack_d0[11] = 0;
    aiStack_d0[12] = 0;
    aiStack_d0[13] = 0;
    aiStack_d0[14] = 0x3f800000;
    aiStack_d0[15] = 0;
    aiStack_d0[16] = 0;
    aiStack_d0[17] = 0;
    aiStack_d0[18] = 0x3f800000;
    
    DAT_0060f1c0 = 0;
    DAT_0060f1e8 = 0;
    DAT_0060f208 = 0;
    DAT_0060f1f0 = 0;
    DAT_0060f1d8 = 0;
    DAT_0060f204 = 0;
    
    __builtin_memcpy(&count, (char *)piVar2 + 6, 4);
    if (0 < count) {
        iVar5 = 0;
        do {
            int base;
            __builtin_memcpy(&base, (char *)piVar2 + 10, 4);
            int *item = *(int **)(base + iVar5);
            iVar3 = FUN_004736b0((int)piVar2, item, aiStack_d0, param_2, param_3);
            if (iVar3 != 0) {
                return 1;
            }
            iVar5 += 4;
            iVar4++;
            __builtin_memcpy(&count, (char *)piVar2 + 6, 4);
        } while (iVar4 < count);
    }
    return 0;
}
