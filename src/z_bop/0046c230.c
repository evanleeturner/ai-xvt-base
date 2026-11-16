/* SPDX-License-Identifier: MIT */
/* z_bop 0046c230 */
extern char s_Dial_a_New_Number__00527048[];

extern int *DAT_009994c4;
extern unsigned int DAT_009994c0[];
extern unsigned int DAT_009994cc;
extern unsigned int DAT_009994d0[];
extern unsigned int DAT_009994e0[];
extern unsigned int DAT_009994f0;
extern unsigned int DAT_009994f4;
extern unsigned int DAT_009994f8;
extern unsigned int DAT_009994fc;
extern unsigned int DAT_00999524;
extern unsigned int DAT_00999528[];
extern char DAT_00999538[];
extern unsigned int DAT_00999548;
extern unsigned int DAT_0099954c;
extern int DAT_00999550[];
extern unsigned int DAT_009997d0;
extern unsigned int DAT_009997d4;
extern unsigned int DAT_009999d4;
extern unsigned int DAT_009999d8;
extern unsigned int DAT_009999dc;
extern unsigned int DAT_009999e0;
extern unsigned int DAT_00999be0;
extern unsigned int DAT_00999be4;
extern unsigned int DAT_00999be8;
extern unsigned int DAT_00999bec;
extern unsigned int DAT_0099f4ac;
extern unsigned int DAT_0099f6e0;
extern unsigned int DAT_00527020;
extern unsigned int DAT_0052701c;
extern unsigned int DAT_00527024;
extern unsigned int DAT_00527028;
extern unsigned int DAT_0052702c;
extern unsigned int DAT_005599a0;
extern unsigned int DAT_005597a0;
extern unsigned int DAT_005dd9b8;
extern unsigned int DAT_005ee1b8[];
extern unsigned int DAT_009ec608;

extern void FUN_0046c220(void);
extern int FUN_004d1940(void *, void *, void *, void *, void *, void *, int, void *, void *, void *,
                        void *, void *, void *, void *, void *, void *, void *, void *, void *,
                        void *, void *, void *);
extern char *__cdecl _strncpy(char *, const char *, unsigned int);
extern void FUN_0046c730(void);
extern unsigned long __stdcall timeGetTime(void);
extern void FUN_0046d350(unsigned int, unsigned int *, int);
extern int FUN_0046e040(void);
extern int *FUN_0046f5e0(int *, unsigned int *, int);
extern void FUN_0046f840(int);

unsigned int __cdecl FUN_0046c230(char *param_1, char *param_2, int param_3, int param_4,
                                  unsigned int param_5, int param_6)
{
    unsigned int *puVar1;
    int iVar2;
    int *piVar3;
    int iVar4;
    int *piVar5;
    unsigned int *puVar6;
    unsigned int local_48[10];
    char local_20[32];
    int stack0xffffffa8[15];
    unsigned int stack0xffffffa4;

    __builtin_memcpy(local_20, s_Dial_a_New_Number__00527048, 19);
    __builtin_memset(local_20 + 19, 0, 13);

    FUN_0046c220();

    puVar1 = DAT_009994c0;
    for (iVar4 = 0x190e; iVar4 != 0; iVar4 = iVar4 - 1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
    }
    DAT_009ec608 = 1;
    DAT_005599a0 = 0;
    DAT_009994cc = param_5;
    DAT_009997d0 = 0;
    DAT_009999d8 = 1;
    *(unsigned char *)&DAT_009997d4 = 0x39;
    DAT_009999d4 = 1;
    DAT_009999dc = 0;
    DAT_00999be4 = 1;
    *(unsigned char *)&DAT_009999e0 = 0x39;
    DAT_00999be0 = 1;
    DAT_00999524 = 0;

    puVar1 = &DAT_00999bec;
    do {
        *puVar1 = 0x7f;
        puVar6 = puVar1 + 0x8e;
        puVar1[2] = 0x7f;
        puVar1[4] = 0x7f;
        puVar1[1] = 0x7f;
        puVar1[3] = 0x7f;
        puVar1[5] = 0x7f;
        puVar1[6] = 0;
        puVar1[-1] = 0;
        *(unsigned char *)(puVar1 + 7) = 0x39;
        puVar1[0x87] = 1;
        puVar1[0x88] = 0;
        puVar1[0x89] = 0;
        puVar1[0x8a] = 0;
        puVar1 = puVar6;
    } while (puVar6 < &DAT_0099f4ac);

    DAT_00527020 = 0;
    puVar1 = DAT_005ee1b8;
    for (iVar4 = 0x8400; iVar4 != 0; iVar4 = iVar4 - 1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
    }
    DAT_0052702c = 0;
    DAT_0099f6e0 = 0;

    if ((param_6 == 1) && (param_3 == 1)) {
        FUN_004d1940(&DAT_009994c4, DAT_009994d0, DAT_009994e0, &DAT_009994f0,
                     &DAT_009994f8, DAT_00999528, 0x5599a8, &DAT_00527028, &DAT_0052702c,
                     &DAT_00527024, &DAT_00999be8, &DAT_0099f6e0, &DAT_009997d0,
                     &DAT_009997d4, &DAT_009999d4, &DAT_009999d8, &DAT_009999dc,
                     &DAT_009999e0, &DAT_00999be0, &DAT_00999be4, &DAT_005dd9b8,
                     &DAT_0052701c);
        if (DAT_009994c4 == (int *)0) {
            DAT_00999548 = 1;
            DAT_0099954c = 1;
        } else {
            puVar1 = local_48;
            for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 - 1) {
                *puVar1 = 0;
                puVar1 = puVar1 + 1;
            }
            local_48[0] = 0x28;
            (*(int (__stdcall **)(int *, unsigned int *, int))(*DAT_009994c4 + 0x38))(
                DAT_009994c4, local_48, 0);
        }
        _strncpy((char *)DAT_00999528, param_1, 0x10);
        _strncpy(DAT_00999538, param_2, 0x10);
        puVar1 = DAT_00999528;
        puVar6 = (unsigned int *)DAT_00999550;
        for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 - 1) {
            *puVar6 = *puVar1;
            puVar1 = puVar1 + 1;
            puVar6 = puVar6 + 1;
        }
        DAT_009994f4 = param_3;
        DAT_009994fc = 1;
        return 1;
    }

    FUN_004d1940(&DAT_009994c4, DAT_009994d0, DAT_009994e0, &DAT_009994f0,
                 &DAT_009994f8, DAT_00999528, 0x5599a8, &DAT_00527028, &DAT_0052702c,
                 &DAT_00527024, &DAT_00999be8, &DAT_0099f6e0, &DAT_009997d0,
                 &DAT_009997d4, &DAT_009999d4, &DAT_009999d8, &DAT_009999dc,
                 &DAT_009999e0, &DAT_00999be0, &DAT_00999be4, &DAT_005dd9b8,
                 &DAT_0052701c);
    puVar1 = local_48;
    for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 - 1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
    }
    local_48[0] = 0x28;
    (*(int (__stdcall **)(int *, unsigned int *, int))(*DAT_009994c4 + 0x38))(
        DAT_009994c4, local_48, 0);
    DAT_009994fc = 0;
    DAT_009994f4 = param_3;
    FUN_0046c730();
    DAT_009ec608 = 1;
    timeGetTime();
    DAT_005597a0 = 0xd;
    FUN_0046d350(DAT_009994f8, &DAT_005597a0, 4);
    DAT_005597a0 = 0x39;
    FUN_0046d350(DAT_009994f8, &DAT_005597a0, 4);

    iVar2 = FUN_0046e040();
    iVar4 = param_6;
    if (iVar2 != 0) {
        while (iVar4 != 0) {
            piVar3 = FUN_0046f5e0(stack0xffffffa8, &stack0xffffffa4, 0x3c);
            if (piVar3 == (int *)0) {
                DAT_009994c4 = (int *)0;
                return 0;
            }
            if (*piVar3 == 0xd) {
                iVar4 = (int)((unsigned int)iVar4 - 1u);
            }
        }
    }

    iVar4 = FUN_0046e040();
    if (iVar4 == 0) {
        if (param_4 == 0) {
            do {
                piVar3 = FUN_0046f5e0(stack0xffffffa8, &stack0xffffffa4, 0x3c);
                if (piVar3 == (int *)0) {
                    DAT_009994c4 = (int *)0;
                    return 0;
                }
            } while (*piVar3 != 0xe);
            DAT_009994fc = piVar3[1];
            iVar4 = 0;
            if (0 < (int)DAT_009994fc) {
                do {
                    do {
                        piVar3 = FUN_0046f5e0(stack0xffffffa8, &stack0xffffffa4, 0x3c);
                        if (piVar3 == (int *)0) {
                            return 0;
                        }
                    } while (*piVar3 != 0xf);
                    iVar4 = iVar4 + 1;
                    piVar5 = piVar3 + 2;
                    piVar3 = DAT_00999550 + (int)((unsigned int)piVar3[1] * 10u);
                    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 - 1) {
                        *piVar3 = *piVar5;
                        piVar5 = piVar5 + 1;
                        piVar3 = piVar3 + 1;
                    }
                } while (iVar4 < (int)DAT_009994fc);
            }
        }
    } else if (1 < param_6) {
        FUN_0046f840(0);
        DAT_005597a0 = 0x39;
        FUN_0046d350(0, &DAT_005597a0, 4);
    }
    return 1;
}
