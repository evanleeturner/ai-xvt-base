/* SPDX-License-Identifier: MIT */
/* z_bop 004d7020 */
/* Globals: widths taken from the P-code; no struct layout is known. */
extern int DAT_00665438;
extern int DAT_00aa6cf4;
extern int DAT_00bb2810;
extern int DAT_0052bf50;
extern unsigned short DAT_00b69ccc;
extern unsigned short DAT_00b6a2c0;
extern unsigned short DAT_00b69e38;
extern unsigned short DAT_00b6a2d8;
extern int DAT_00b6a2a0;
extern char DAT_00bb73a7;
extern char DAT_00b69d20[];
extern char DAT_0052b460[];
extern char DAT_0052bf70[];
extern char s__s_v______0052bf64[];
extern char s__s_v___d__d_0052bf58[];
extern char s_jewelsound_0052ad74[];
/* Entry table: stride 0x3e, so int fields are unaligned; accessed as bytes. */
extern unsigned char DAT_00aa62e8[];

DWORD __stdcall GetTickCount(void);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004d6460(int *, int, int, int, int);
char *__cdecl FUN_004ddc10(int);
void __cdecl FUN_004db7d0(int, char *, int *, int, int, unsigned int);
void __cdecl FUN_004de090(int *, int *);
int __cdecl FUN_004d9d70(int *, int, int, int, int, int, int);
void __cdecl FUN_004d67e0(int *, int, int, short, int);
void __cdecl FUN_004b5570(char *, int, int);
void __cdecl FUN_004d6480(int *, int *);
void __cdecl FUN_004c9bf0(unsigned int);
int __cdecl FUN_004d6b90(int *, int, int);
void __cdecl FUN_004d6950(int *, int, int, short);
unsigned char __cdecl FUN_004dc220(void);
unsigned char __cdecl FUN_004dc240(void);
void __cdecl FUN_004de5a0(char *, int, int, int);
void __cdecl FUN_004d64a0(int *, int, int);

int __cdecl FUN_004d7020(int param_1)
{
    char *pcVar2;
    int iVar3;
    unsigned short uVar4;
    unsigned char *piVar5;
    char *piVar6;
    int iVar7;
    int iVar8;
    int local_30[4];
    int local_20;
    DWORD local_1c;
    int local_18;
    int local_14[5];
    int v_type;
    int v_m8;
    int v_m4;

    if (param_1 == 0) {
        DAT_00665438 = 0;
    }
    local_1c = GetTickCount();
    FUN_004d6460(local_30, 0x58, 0x5e, 0x1a0, 0x6d);
    pcVar2 = FUN_004ddc10(0x10);
    FUN_004db7d0(0xc, pcVar2, local_30, 0, 1, 0xffffU);
    FUN_004de090(local_14, &local_18);
    if (6 < DAT_00aa6cf4) {
        FUN_004d6460(local_30, 0x1a5, 0x72, 0x1ae, 0xcc);
        DAT_00665438 = FUN_004d9d70(local_30, DAT_00665438, DAT_00aa6cf4, 0, 5, DAT_00bb2810, 5);
    }
    FUN_004d6460(local_30, 0x58, 0x72, 0x1a3, 0x80);
    local_20 = -1;
    if (DAT_00665438 < (int)((unsigned int)DAT_00665438 + 6U)) {
        piVar5 = DAT_00aa62e8 + (unsigned int)DAT_00665438 * 0x3eU;
        iVar7 = DAT_00665438;
        do {
            if (DAT_00aa6cf4 <= iVar7) {
                return local_20;
            }
            if (iVar7 == DAT_0052bf50) {
                FUN_004d67e0(local_30, 0, 0, (short)DAT_00bb2810, 1);
            }
            uVar4 = DAT_00b69ccc;
            __builtin_memcpy(&v_type, piVar5, 4);
            __builtin_memcpy(&v_m8, piVar5 - 8, 4);
            if (v_type == 0x65) {
                if (v_m8 == 0) {
                    uVar4 = DAT_00b6a2c0;
                    if (*(char *)(piVar5 + 5) != '\0') {
                        uVar4 = DAT_00b69e38;
                    }
                } else {
                    uVar4 = 0xffff;
                    if ((unsigned int)v_m8 < 9) {
                        uVar4 = DAT_00b6a2d8;
                    }
                }
            }
            if (*(char *)(piVar5 + 4) != '\0') {
                FUN_004b5570(DAT_0052bf70, (int)((unsigned int)local_30[0] - 6U), (int)((unsigned int)local_30[1] + 1U));
            }
            __builtin_memcpy(&iVar8, piVar5, 4);
            if (iVar8 == 0x65) {
                piVar6 = (char *)(piVar5 - 0x38);
            } else {
                if (iVar8 == 0) {
                    _sprintf(DAT_00b69d20, s__s_v______0052bf64, (char *)(piVar5 - 0x38));
                } else {
                    _sprintf(DAT_00b69d20, s__s_v___d__d_0052bf58, (char *)(piVar5 - 0x38),
                             ((unsigned int)iVar8 - 1U) / 100U + 1U,
                             ((unsigned int)iVar8 - 1U) % 100U);
                }
                piVar6 = DAT_00b69d20;
            }
            FUN_004db7d0(0xc, piVar6, local_30, 0, 1, (unsigned int)uVar4);
            __builtin_memcpy(&v_m8, piVar5 - 8, 4);
            if ((unsigned int)v_m8 < 9) {
                FUN_004d6480(local_14 + 1, local_30);
                local_14[1] = 0x11d;
                _sprintf(DAT_00b69d20, DAT_0052b460, v_m8);
                FUN_004db7d0(0xc, DAT_00b69d20, local_14 + 1, 0, 1, (unsigned int)uVar4);
                local_14[1] = 0x176;
                __builtin_memcpy(&v_m4, piVar5 - 4, 4);
                FUN_004c9bf0((local_1c - (unsigned int)v_m4) / 1000U);
                FUN_004db7d0(0xc, DAT_00b69d20, local_14 + 1, 0, 1, (unsigned int)uVar4);
            }
            iVar3 = FUN_004d6b90(local_30, local_14[0], local_18);
            iVar8 = local_20;
            if (iVar3 != 0) {
                FUN_004d6950(local_30, 0, 0, (short)DAT_00b6a2a0);
                if (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                    iVar8 = iVar7;
                    if (DAT_00bb73a7 != '\0') {
                        FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff);
                    }
                }
            }
            local_20 = iVar8;
            piVar5 = piVar5 + 0x3e;
            iVar7 = iVar7 + 1;
            FUN_004d64a0(local_30, 0, 0xf);
        } while (iVar7 < (int)((unsigned int)DAT_00665438 + 6U));
    }
    return local_20;
}
