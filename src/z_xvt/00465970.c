/* SPDX-License-Identifier: MIT */
/* z_xvt 00465970 */
extern unsigned int DAT_005f1d48;
extern unsigned short DAT_005f1d48_2;
extern unsigned short DAT_005f1d50;
extern unsigned int DAT_005f25d8;
extern unsigned int DAT_005f25dc;
extern unsigned int DAT_005f25c0;
extern unsigned int DAT_005f25b8;
extern unsigned int DAT_005f25bc;
extern unsigned int DAT_006a8820;
extern unsigned short DAT_006a8824;
extern unsigned char DAT_006a8826[4];   /* packed */
extern unsigned char DAT_006a882a[4];   /* packed */
extern unsigned char DAT_006a882e[4];   /* packed */
extern unsigned char DAT_006a8832[4];   /* packed */
extern unsigned short DAT_006a8836;
extern unsigned int DAT_006a8838;
extern unsigned int DAT_006a883c;
extern unsigned int DAT_006a8840;
extern unsigned short DAT_006a8844;
extern unsigned char DAT_006a8846[4];   /* packed */
extern unsigned char DAT_006a884a[4];   /* packed */
extern unsigned short DAT_006a884e;
extern unsigned int DAT_006a8850;
extern unsigned int DAT_006a8858;
extern unsigned short DAT_006a885c;
extern unsigned char DAT_006a8866[4];   /* packed */
extern unsigned short DAT_006a886a;
extern unsigned int DAT_006a8874;
extern unsigned short DAT_006a8878;
extern unsigned char DAT_006a887e[4];   /* packed */
extern unsigned short DAT_006a8882;
extern unsigned int DAT_006a8888;
extern unsigned short DAT_006a888c;
extern unsigned char DAT_006a888e[4];   /* packed */
extern unsigned short DAT_006a8892;
extern unsigned int DAT_006a8894;
extern unsigned short DAT_006a8898;
extern unsigned char DAT_006a889e[4];   /* packed */
extern unsigned int DAT_006a8a0c;
extern float DAT_006a8a08;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_00527e74;
extern unsigned int DAT_006da9f4;
extern unsigned char DAT_007091ae[4];   /* packed */
extern unsigned int DAT_005f1d3c;
extern unsigned int DAT_005f258c;
extern unsigned int DAT_0052ae18;
extern float DAT_005201d0;
extern int DAT_00732f3c;
extern unsigned char DAT_005f2590;

unsigned int __fastcall FUN_0048f4f0(unsigned int param_1);
void FUN_0045f260(void);

void __fastcall FUN_00465970(unsigned int param_1)
{
    unsigned int *puVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned short extraout_var = 0;
    unsigned short extraout_var_00 = 0;
    unsigned short extraout_var_01 = 0;
    unsigned short extraout_var_02 = 0;
    unsigned short extraout_var_03 = 0;
    unsigned short extraout_var_04 = 0;
    unsigned short extraout_var_05 = 0;
    unsigned short extraout_var_06 = 0;
    unsigned short extraout_var_07 = 0;
    unsigned short extraout_var_08 = 0;
    int iVar4;
    unsigned char *pbVar5;
    unsigned char *pbVar6;
    unsigned char bVar7;
    unsigned int *puVar8;
    unsigned int uVar9;
    unsigned short in_FPUControlWord = 0;
    unsigned int local_c;
    unsigned int t0;
    unsigned int t1;

    DAT_005f1d48 = (((unsigned int)DAT_005f1d48_2) << 16) | (unsigned int)in_FPUControlWord;
    DAT_005f1d48 = DAT_005f1d48 & 0xfffffcffu;
    DAT_005f25d8 = 0x749dc5aeu;
    DAT_005f25dc = 0x749dc5aeu;
    DAT_005f25c0 = 0x749dc5aeu;
    DAT_005f25b8 = 0;
    DAT_005f25bc = 0;
    DAT_005f1d50 = in_FPUControlWord;

    DAT_006a8820 = FUN_0048f4f0((((unsigned int)(unsigned short)(param_1 >> 0x10)) << 16) | (unsigned int)DAT_006a8824);
    t0 = FUN_0048f4f0((((unsigned int)extraout_var) << 16) | (unsigned int)DAT_006a8836);
    __builtin_memcpy(DAT_006a8832, &t0, 4);
    DAT_006a883c = DAT_006a8838;
    t0 = FUN_0048f4f0((((unsigned int)extraout_var_08) << 16) | (unsigned int)DAT_006a884e);
    __builtin_memcpy(DAT_006a884a, &t0, 4);
    DAT_006a8858 = FUN_0048f4f0((((unsigned int)extraout_var_00) << 16) | (unsigned int)DAT_006a885c);
    t0 = FUN_0048f4f0((((unsigned int)extraout_var_01) << 16) | (unsigned int)DAT_006a886a);
    __builtin_memcpy(DAT_006a8866, &t0, 4);
    DAT_006a8874 = FUN_0048f4f0((((unsigned int)extraout_var_02) << 16) | (unsigned int)DAT_006a8878);
    t0 = FUN_0048f4f0((((unsigned int)extraout_var_03) << 16) | (unsigned int)DAT_006a8882);
    __builtin_memcpy(DAT_006a887e, &t0, 4);
    DAT_006a8888 = FUN_0048f4f0((((unsigned int)extraout_var_04) << 16) | (unsigned int)DAT_006a888c);
    t0 = FUN_0048f4f0((((unsigned int)extraout_var_05) << 16) | (unsigned int)DAT_006a8892);
    __builtin_memcpy(DAT_006a888e, &t0, 4);
    DAT_006a8840 = FUN_0048f4f0((((unsigned int)extraout_var_06) << 16) | (unsigned int)DAT_006a8844);
    DAT_006a8894 = FUN_0048f4f0((((unsigned int)extraout_var_07) << 16) | (unsigned int)DAT_006a8898);

    t0 = DAT_006a8820;
    __builtin_memcpy(DAT_006a882a, &t0, 4);
    __builtin_memcpy(&t1, DAT_006a8826, 4);
    t0 = DAT_006a8820 + (t1 * 6u + 0xfffffffau) * 4u;
    __builtin_memcpy(DAT_006a882e, &t0, 4);

    local_c = 0;
    if (DAT_006d4984 != 0) {
        iVar4 = 0;
        pbVar6 = (unsigned char *)((unsigned int)DAT_00527e74 + DAT_006da9f4);
        do {
            uVar9 = 0;
            pbVar5 = pbVar6 + 1;
            __builtin_memcpy(&t0, DAT_006a888e, 4);
            *(unsigned int *)(t0 + (unsigned int)iVar4) = 0;
            bVar7 = *pbVar6;
            __builtin_memcpy(&t0, DAT_006a888e, 4);
            puVar8 = *(unsigned int **)(t0 + (unsigned int)iVar4);
            pbVar6 = pbVar5;
            __builtin_memcpy(&t1, DAT_007091ae, 4);
            if (t1 != 0) {
                do {
                    uVar3 = (unsigned int)*pbVar6;
                    pbVar5 = pbVar6 + 1;
                    if (uVar3 == 0) {
                        uVar3 = (unsigned int)*pbVar5;
                        pbVar5 = pbVar6 + 2;
                        if (uVar3 == 0) {
                            bVar2 = *pbVar5;
                            pbVar5 = pbVar6 + 3;
                            uVar3 = (unsigned int)bVar2 + 0x100u;
                        }
                        uVar3 = uVar3 + 0xffu;
                    }
                    if ((char)bVar7 < '\0') {
                        if (puVar8 == (unsigned int *)0) {
                            __builtin_memcpy(&t0, DAT_006a888e, 4);
                            __builtin_memcpy(&t1, DAT_006a882a, 4);
                            *(unsigned int *)(t0 + (unsigned int)iVar4) = t1;
                        } else {
                            __builtin_memcpy(&t1, DAT_006a882a, 4);
                            *puVar8 = t1;
                        }
                        __builtin_memcpy(&t0, DAT_006a882a, 4);
                        puVar8 = (unsigned int *)t0;
                        __builtin_memcpy(&t0, DAT_006a882a, 4);
                        puVar1 = (unsigned int *)(t0 + 4u);
                        __builtin_memcpy(&t0, DAT_006a882a, 4);
                        t0 = t0 + 0x18u;
                        __builtin_memcpy(DAT_006a882a, &t0, 4);
                        *puVar1 = uVar9;
                        puVar8[2] = uVar9 + uVar3;
                        puVar8[5] = (unsigned int)&DAT_005f2590;
                        *puVar8 = 0;
                    }
                    bVar7 = (unsigned char)(0u - (unsigned int)bVar7);
                    uVar9 = uVar9 + uVar3;
                    pbVar6 = pbVar5;
                    __builtin_memcpy(&t1, DAT_007091ae, 4);
                } while (uVar9 < t1);
            }
            iVar4 = (int)((unsigned int)iVar4 + 4u);
            local_c = local_c + 1;
            pbVar6 = pbVar5;
        } while (local_c < (unsigned int)DAT_006d4984);
    }

    __builtin_memcpy(&t1, DAT_007091ae, 4);
    DAT_006a8a0c = ((t1 - 1u) + DAT_005f1d3c) / DAT_005f1d3c;
    DAT_005f258c = DAT_005f258c + (unsigned int)DAT_006d4984;
    DAT_006a8850 = 0;
    t0 = 0;
    __builtin_memcpy(DAT_006a8846, &t0, 4);
    __builtin_memcpy(DAT_006a889e, &t0, 4);
    DAT_006a8a08 = DAT_005201d0 / (float)DAT_00732f3c;
    if (DAT_0052ae18 != 0) {
        FUN_0045f260();
    }
    return;
}
