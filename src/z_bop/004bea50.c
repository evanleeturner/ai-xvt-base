/* SPDX-License-Identifier: MIT */
/* z_bop 004bea50 */
/* Unresolved: the string at 0x0052b340 has a Ghidra name containing
   non-identifier characters; it is declared here as s_______0052b340.
   The types of FUN_004db640's return value and of the DAT_00bb2810 / DAT_0052b0e0
   globals are inferred from usage only.
   FUN_004de5a0: the stack-check shows 24 argument bytes pushed at 0x4bebf7 and
   0x4bedb3 and 16 at 0x4bee54; it is declared variadic after the four known
   parameters. The two extra values (0, 0) at the first two sites are unresolved
   placeholders. */

extern int           DAT_0052b0e0;
extern int           DAT_00664f2c;
extern unsigned int  DAT_00664fa8;
extern unsigned int  DAT_00664fac;
extern unsigned int  DAT_00664fb0;
extern unsigned short DAT_00664fb4;
extern unsigned int *DAT_00b69e28;
extern int           DAT_00b69cc0;
extern char          DAT_00b6a2e0;
/* packed globals (address not a multiple of 4): accessed only via __builtin_memcpy */
extern unsigned int  DAT_00b6a5a6;
extern unsigned int  DAT_00b6a5aa;
extern unsigned char DAT_00b6a5ae[24];
extern unsigned int  DAT_00b6a606;
extern unsigned int  DAT_00b6a60a;
extern int           DAT_00b6d962;
extern unsigned int  DAT_00b6d96a;
extern unsigned int  DAT_00b6d96e;
extern unsigned int  DAT_00b6d972[6];
extern unsigned int  DAT_00b6d9aa;
extern unsigned int  DAT_00b6d9ae;
extern unsigned int  DAT_00bb2810;
extern char          DAT_00bb73a7;
extern char          s_logonsound_0052b350[];
extern char          s_buttonsound_0052a68c[];
extern char          s_______0052b340[];

void  __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *__cdecl FUN_004ddc10(int param_1);
void  __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
void  __cdecl FUN_004dcb30(void);
void  __cdecl FUN_004bf100(int *param_1);
int   __cdecl FUN_004d9d70(int *param_1, int param_2, int param_3, int param_4, int param_5, unsigned int param_6, int param_7);
int   __cdecl FUN_004bef80(int *param_1, int param_2);
int   __cdecl __strnicmp(const char *param_1, const char *param_2, unsigned int param_3);
int   __cdecl FUN_004cb0c0(char *param_1);
int   __cdecl FUN_004de5a0(char *param_1, int param_2, unsigned int param_3, unsigned int param_4, ...);
void  __cdecl FUN_004caf60(void);
int   __cdecl FUN_004dd220(unsigned int *param_1);
int   __cdecl FUN_004da380(int *param_1, char *param_2, int param_3, int param_4, int param_5, char *param_6);
int   __cdecl __strcmpi(const char *param_1, const char *param_2);
void  __cdecl FUN_004bf3a0(char *param_1);
int   __cdecl FUN_004da650(int *param_1, char *param_2, int param_3, unsigned int param_4, int param_5, char *param_6);
unsigned char __cdecl FUN_004dca90(int param_1);
int   __cdecl FUN_004dcb90(char *param_1, char *param_2, char *param_3, char *param_4, char *param_5);
void  __cdecl FUN_004bf260(void);

unsigned int __cdecl FUN_004bea50(int param_1)
{
    unsigned char bVar1;
    char *pcVar2;
    int iVar3;
    int iVar4;
    char *pcVar5;
    char *pcVar6;
    unsigned int *puVar7;
    int iVar8;
    unsigned char *puVar9;
    unsigned int uVar10;
    char *pcVar11;
    unsigned int uVar12;
    int *piVar13;
    char *pcVar14;
    int local_14;
    int local_10[4];
    int local_idx;
    unsigned int local_val;

    FUN_004d6460(local_10, 0x1c3, 0x5a, 0x25d, 0x6a);
    piVar13 = local_10;
    uVar12 = 0xffff;
    pcVar2 = FUN_004ddc10(0x1cf);
    FUN_004db640(0xc, pcVar2, piVar13, uVar12);
    if (param_1 == 0) {
        FUN_004dcb30();
        DAT_00664fa8 = 0;
        DAT_00664fac = 0;
        DAT_00664fb0 = 0;
        DAT_00664fb4 = 0;
        if ((DAT_00b69e28 == (unsigned int *)0) || ((int)DAT_00b69e28[1] < 0xe)) {
            piVar13 = &local_14;
        } else {
            piVar13 = &DAT_0052b0e0;
        }
        FUN_004bf100(piVar13);
    }
    if ((DAT_00b69e28 != (unsigned int *)0) && (0xd < (int)DAT_00b69e28[1])) {
        FUN_004d6460(local_10, 0x254, 0x75, 0x25d, 0x140);
        DAT_0052b0e0 = FUN_004d9d70(local_10, DAT_0052b0e0, DAT_00b69e28[1], 0, 5, DAT_00bb2810, 4);
    }
    FUN_004d6460(local_10, 0x1cd, 0x75, 0x253, 0x140);
    local_14 = FUN_004bef80(local_10, DAT_0052b0e0);
    if (local_14 != 0) {
        iVar3 = __strnicmp((char *)&DAT_00b6a2e0,
                           (char *)((unsigned int)local_14 * 0xeu + (unsigned int)DAT_00b69cc0 + (unsigned int)-0xe),
                           0xc);
        if (iVar3 != 0) {
            puVar7 = (unsigned int *)*DAT_00b69e28;
            iVar3 = local_14 + -1;
            if (0 < iVar3) {
                do {
                    puVar7 = (unsigned int *)puVar7[1];
                    iVar3 = iVar3 + -1;
                } while (iVar3 != 0);
            }
            iVar3 = FUN_004cb0c0((char *)*puVar7);
            if (iVar3 != 0) {
                if (DAT_00bb73a7 != '\0') {
                    FUN_004de5a0(s_logonsound_0052b350, 1, 0, 0xff, 0, 0);
                }
                __builtin_memcpy(&local_idx, &DAT_00b6d962, 4);
                __builtin_memcpy(&local_val, &DAT_00b6d96a + (int)((unsigned int)local_idx * 0x4278u), 4);
                __builtin_memcpy(&DAT_00b6a5a6, &local_val, 4);
                __builtin_memcpy(&local_val, &DAT_00b6d96e + (int)((unsigned int)local_idx * 0x4278u), 4);
                __builtin_memcpy(&DAT_00b6a5aa, &local_val, 4);
                puVar7 = DAT_00b6d972 + (int)((unsigned int)local_idx * 0x4278u);
                puVar9 = DAT_00b6a5ae;
                for (iVar3 = 6; iVar3 != 0; iVar3 = iVar3 + -1) {
                    __builtin_memcpy(puVar9, puVar7, 4);
                    puVar7 = puVar7 + 1;
                    puVar9 = puVar9 + 4;
                }
                __builtin_memcpy(&local_val, &DAT_00b6d9aa + (int)((unsigned int)local_idx * 0x4278u), 4);
                __builtin_memcpy(&DAT_00b6a606, &local_val, 4);
                __builtin_memcpy(&local_val, &DAT_00b6d9ae + (int)((unsigned int)local_idx * 0x4278u), 4);
                __builtin_memcpy(&DAT_00b6a60a, &local_val, 4);
            }
        }
        DAT_00664fa8 = 0;
        DAT_00664fac = 0;
        DAT_00664fb0 = 0;
        DAT_00664fb4 = 0;
        FUN_004bf100(&local_14);
        FUN_004caf60();
        DAT_00664f2c = 1;
    }
    if (DAT_00b69e28[1] == 0) {
        local_14 = FUN_004dd220(&DAT_00664fa8);
    } else {
        FUN_004d6460(local_10, 0x1cd, 0x141, 0x253, 0x154);
        local_14 = FUN_004da380(local_10, (char *)&DAT_00664fa8, 0xd, 0, 0xc, s_______0052b340);
    }
    iVar3 = 0;
    if ((local_14 != 0) && ((char)DAT_00664fa8 != '\0')) {
        local_14 = 0;
        DAT_00664f2c = 1;
        if (DAT_00b69e28 != (unsigned int *)0) {
            iVar8 = 0;
            puVar7 = (unsigned int *)*DAT_00b69e28;
            if ((DAT_00b69cc0 != 0) && (0 < (int)DAT_00b69e28[1])) {
                do {
                    iVar4 = __strcmpi((char *)((unsigned int)DAT_00b69cc0 + (unsigned int)iVar3), (char *)&DAT_00664fa8);
                    if (iVar4 == 0) {
                        iVar3 = FUN_004cb0c0((char *)*puVar7);
                        if (iVar3 != 0) {
                            if (DAT_00bb73a7 != '\0') {
                                FUN_004de5a0(s_logonsound_0052b350, 1, 0, 0xff, 0, 0);
                            }
                            __builtin_memcpy(&local_idx, &DAT_00b6d962, 4);
                            __builtin_memcpy(&local_val, &DAT_00b6d96a + (int)((unsigned int)local_idx * 0x4278u), 4);
                            __builtin_memcpy(&DAT_00b6a5a6, &local_val, 4);
                            __builtin_memcpy(&local_val, &DAT_00b6d96e + (int)((unsigned int)local_idx * 0x4278u), 4);
                            __builtin_memcpy(&DAT_00b6a5aa, &local_val, 4);
                            puVar7 = DAT_00b6d972 + (int)((unsigned int)local_idx * 0x4278u);
                            puVar9 = DAT_00b6a5ae;
                            for (iVar3 = 6; iVar3 != 0; iVar3 = iVar3 + -1) {
                                __builtin_memcpy(puVar9, puVar7, 4);
                                puVar7 = puVar7 + 1;
                                puVar9 = puVar9 + 4;
                            }
                            __builtin_memcpy(&local_val, &DAT_00b6d9aa + (int)((unsigned int)local_idx * 0x4278u), 4);
                            __builtin_memcpy(&DAT_00b6a606, &local_val, 4);
                            __builtin_memcpy(&local_val, &DAT_00b6d9ae + (int)((unsigned int)local_idx * 0x4278u), 4);
                            __builtin_memcpy(&DAT_00b6a60a, &local_val, 4);
                        }
                        break;
                    }
                    iVar8 = iVar8 + 1;
                    puVar7 = (unsigned int *)puVar7[1];
                    iVar3 = iVar3 + 0xe;
                } while (iVar8 < (int)DAT_00b69e28[1]);
            }
        }
        if (local_14 == 0) {
            if (DAT_00bb73a7 != '\0') {
                FUN_004de5a0(s_logonsound_0052b350, 1, 0, 0xff);
            }
            FUN_004bf3a0((char *)&DAT_00664fa8);
            if ((DAT_00b69e28 == (unsigned int *)0) || ((int)DAT_00b69e28[1] < 0xe)) {
                piVar13 = &local_14;
            } else {
                piVar13 = &DAT_0052b0e0;
            }
            FUN_004bf100(piVar13);
        }
        DAT_00664fa8 = 0;
        DAT_00664fac = 0;
        DAT_00664fb0 = 0;
        DAT_00664fb4 = 0;
        FUN_004caf60();
    }
    FUN_004d6460(local_10, 0x1c3, 0x166, 0x25d, 0x17a);
    pcVar14 = s_buttonsound_0052a68c;
    iVar8 = 0x14;
    uVar10 = 0xffff;
    iVar3 = 0xf;
    pcVar2 = FUN_004ddc10(0x1d0);
    local_14 = FUN_004da650(local_10, pcVar2, iVar3, uVar10, iVar8, pcVar14);
    if (((local_14 != 0) || (bVar1 = FUN_004dca90(0x2e), bVar1 != 0)) &&
        ((char)DAT_00b6a2e0 != '\0')) {
        pcVar2 = FUN_004ddc10(0x212);
        pcVar14 = FUN_004ddc10(0x211);
        pcVar11 = (char *)0;
        pcVar5 = FUN_004ddc10(0x210);
        pcVar6 = FUN_004ddc10(0x20f);
        iVar3 = FUN_004dcb90(pcVar6, pcVar5, pcVar11, pcVar14, pcVar2);
        if (iVar3 != 0) {
            FUN_004bf260();
        }
        DAT_0052b0e0 = 0;
    }
    return 1;
}
