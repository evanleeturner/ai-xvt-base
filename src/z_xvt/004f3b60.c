/* SPDX-License-Identifier: MIT */
/* z_xvt 004f3b60 */
/* Globals (types inferred from the decompile and P-code widths) */
extern short DAT_00650904[];
extern short DAT_00650914[];
extern short DAT_00650924[];
extern short DAT_00743d02[];
extern short DAT_00743d2e[];
extern short DAT_00743d86[];
extern short DAT_00650936[];
extern short DAT_00650946[];
extern short DAT_00650956[];
extern short DAT_00650966[];
extern short DAT_00650976[];
extern short DAT_00650986[];
extern unsigned char DAT_006507a2[];   /* array of char *, base address is unaligned (0x6507a2) */
extern int DAT_00650434;
extern short DAT_0075766c;
extern unsigned char DAT_0081bfb6[4];  /* 4-byte value at unaligned address 0x81bfb6 */
extern unsigned char DAT_007438d7[];
extern unsigned char DAT_007438d2[];
extern unsigned char DAT_007438d8[];
extern unsigned char DAT_007438e4[];
extern int DAT_00533800[];
extern char DAT_005339a8[];
extern char DAT_005339ac[];
extern char DAT_005339b0[];
extern char DAT_005339b4[];
extern char DAT_0081b740[];
extern char s_mapicon_d_00533e94[];
extern char DAT_00533e90[];

/* Prototypes of callees */
void __cdecl FUN_004dd000(int *param_1, unsigned int *param_2);
void __cdecl FUN_004f33c0(int *param_1, short param_2, short param_3, short *param_4, short *param_5);
void __cdecl FUN_004f4410(unsigned int *param_1, unsigned int param_2, short param_3, short param_4);
void __cdecl FUN_004f4200(char *param_1, int param_2, short param_3, short param_4, short param_5, short param_6);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void __cdecl FUN_004d0920(char *param_1, int *param_2, int param_3, int param_4);
void __cdecl FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, int param_5);

void __cdecl FUN_004f3b60(unsigned int *param_1, unsigned int param_2)
{
    int iVar1;
    char cVar2;
    short sVar3;
    unsigned short uVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    char *pcVar10;
    char *pcVar12;
    short sVar11;
    int iVar13;
    unsigned int uStack_46 = 0;
    short local_42;
    short local_40;
    short sStack_3e;
    int iStack_3c;
    int local_38[4];
    char acStack_28[40];
    int local_81bfb6;
    unsigned int uWord;

    sVar11 = 0;
    FUN_004dd000(local_38, param_1);
    do {
        if (DAT_00650904[sVar11] != 0) {
            sVar3 = DAT_00650914[sVar11];
            FUN_004f33c0(local_38, DAT_00743d02[sVar3 * 0x2b1], DAT_00743d2e[sVar3 * 0x2b1],
                         &local_40, &local_42);
            FUN_004f4410(param_1, param_2, sVar3, DAT_00650924[sVar11]);
        }
        sVar11 = (short)(sVar11 + 1);
    } while (sVar11 < 8);

    sStack_3e = 0;
    do {
        iVar5 = (int)sStack_3e;
        iVar1 = iVar5 * 2;
        if (DAT_00650936[iVar5] != 0) {
            sVar11 = DAT_00650946[iVar5];
            FUN_004f33c0(local_38, DAT_00650956[iVar5], DAT_00650966[iVar5],
                         &local_40, &local_42);

            uVar6 = 0xffffffffu;
            __builtin_memcpy(&pcVar10, (char *)DAT_006507a2 + sVar11 * 4, 4);
            do {
                pcVar12 = pcVar10;
                if (uVar6 == 0) break;
                uVar6 = uVar6 - 1;
                pcVar12 = pcVar10 + 1;
                cVar2 = *pcVar10;
                pcVar10 = pcVar12;
            } while (cVar2 != '\0');
            uVar6 = ~uVar6;
            pcVar10 = pcVar12 + -uVar6;
            pcVar12 = acStack_28;
            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                __builtin_memcpy(&uWord, pcVar10, 4);
                __builtin_memcpy(pcVar12, &uWord, 4);
                pcVar10 = pcVar10 + 4;
                pcVar12 = pcVar12 + 4;
            }
            for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *pcVar12 = *pcVar10;
                pcVar10 = pcVar10 + 1;
                pcVar12 = pcVar12 + 1;
            }

            sVar11 = 0;
            cVar2 = acStack_28[0];
            while (cVar2 != '\0') {
                pcVar10 = acStack_28 + sVar11;
                if (acStack_28[sVar11] == '[') {
                    *pcVar10 = '\x02';
                }
                if (*pcVar10 == ']') {
                    *pcVar10 = '\x01';
                }
                sVar11 = (short)(sVar11 + 1);
                cVar2 = acStack_28[sVar11];
            }
            FUN_004f4200(acStack_28, 1, local_40, local_42, DAT_00650976[iVar5],
                         DAT_00650986[iVar5]);
        }
        sStack_3e = (short)(sStack_3e + 1);
        if (7 < sStack_3e) {
            iStack_3c = 1;
            sStack_3e = 0;
            if (0 < DAT_0075766c) {
                do {
                    iVar8 = (int)sStack_3e;
                    iVar1 = (int)((unsigned int)iVar8 * 0x562u);
                    iVar5 = (int)((unsigned int)iVar8 * 0x2b1u + (unsigned int)DAT_00650434);
                    uVar4 = (unsigned short)DAT_00743d2e[iVar5];
                    uStack_46 = (uStack_46 & 0xffff0000u) | (unsigned int)uVar4;
                    if (DAT_00743d86[iVar5] != 0) {
                        switch (DAT_007438d7[iVar1]) {
                        case 0:
                            uStack_46 = (unsigned int)uVar4;
                            break;
                        case 1:
                            uStack_46 = (1u << 16) | (unsigned int)uVar4;
                            break;
                        case 2:
                            uStack_46 = (2u << 16) | (unsigned int)uVar4;
                            break;
                        case 3:
                            uStack_46 = (3u << 16) | (unsigned int)uVar4;
                            break;
                        case 4:
                            uStack_46 = (1u << 16) | (unsigned int)uVar4;
                            break;
                        case 5:
                            uStack_46 = (4u << 16) | (unsigned int)uVar4;
                        }
                        iVar9 = (int)((unsigned int)DAT_00533800[(short)(unsigned short)DAT_007438d2[iVar1]] * 0x10u);
                        iVar13 = (int)((unsigned int)(*(int *)(DAT_005339b0 + iVar9) - *(int *)(DAT_005339a8 + iVar9)) + 1u);
                        iVar8 = (int)((unsigned int)(*(int *)(DAT_005339b4 + iVar9) - *(int *)(DAT_005339ac + iVar9)) + 1u);
                        FUN_004f33c0(local_38, DAT_00743d02[iVar5], (short)uStack_46,
                                     &local_40, &local_42);
                        local_40 = (short)(local_40 - (short)(iVar13 >> 1));
                        local_42 = (short)(local_42 - (short)(iVar8 >> 1));
                        _sprintf(DAT_0081b740, s_mapicon_d_00533e94, (int)(short)(uStack_46 >> 16));
                        FUN_004d0920(DAT_0081b740, (int *)(DAT_005339a8 + iVar9),
                                     (int)local_40, (int)local_42);
                        __builtin_memcpy(&local_81bfb6, DAT_0081bfb6, 4);
                        if ((DAT_007438e4[iVar1] != '\0') && ((int)DAT_007438d8[iVar1] == local_81bfb6)) {
                            local_40 = (short)(local_40 + (short)iVar13);
                            local_42 = (short)(local_42 + (short)iVar8);
                            _sprintf(DAT_0081b740, DAT_00533e90, iStack_3c);
                            FUN_004da7c0(10, DAT_0081b740, (int)local_40, (int)local_42, 0xffff);
                            iStack_3c = (int)((unsigned int)iStack_3c + 1u);
                        }
                    }
                    sStack_3e = (short)(sStack_3e + 1);
                } while (sStack_3e < DAT_0075766c);
            }
            return;
        }
    } while (1);
}
