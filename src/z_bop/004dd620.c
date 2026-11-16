/* SPDX-License-Identifier: MIT */
/* z_bop 004dd620 */
/* Prototypes (types inferred from the decompile; unresolved parameter types are marked) */
unsigned int FUN_004dcb30(void);              /* return value unused */
void FUN_004de0b0(int a, int b);
void FUN_004d4e20(void);
void FUN_004b4d40(char *name, int a, int b);
void FUN_004d4ec0(int a);
void FUN_004dc140(int a);
void FUN_004d6460(int *rect, int left, int top, int right, int bottom);
void FUN_004db640(int a, char *text, int *rect, int color);
void FUN_004d64a0(int *rect, int dx, int dy);
char *FUN_004ddc10(int id);
int FUN_004da780(int *rect, char *s1, char *s2, char *text, int a, int b, int c, char *snd);
unsigned int FUN_004dcad0(void);
unsigned int FUN_004dcb10(void);
void FUN_004dcb50(void);

extern char DAT_00665688;
extern char DAT_00665810;
extern char DAT_00665890;
extern char DAT_00665990;
extern char DAT_00665708;
extern int DAT_00665808;
extern int DAT_0066580c;
extern int ram0x00aa62a0;

extern char s_dialogbox_0052c0b0[];
extern char s_dialogok_0052b2c0[];
extern char s_dialogokd_0052c0a4[];
extern char s_dialogcancel_0052c084[];
extern char s_dialogcanceld_0052c094[];
extern char s_buttonsound_0052a68c[];

unsigned int __cdecl FUN_004dd620(int param_1)
{
    char *pcVar1;
    unsigned int uVar2;
    int bVar3;
    int iVar5;
    int iVar6;
    char *pcVar7;
    int rect[4];   /* local_10, local_c, (local_8), local_4 */

    bVar3 = 0;
    if (param_1 == 0) {
        FUN_004dcb30();
        if ((DAT_00665688 == '\0') && (DAT_00665810 != '\0')) {
            iVar5 = 0x1f0;
        }
        else {
            iVar5 = 0xb8;
        }
        FUN_004de0b0(iVar5, 0xff);
        FUN_004d4e20();
        FUN_004b4d40(s_dialogbox_0052c0b0, 0, 0);
        FUN_004b4d40(s_dialogbox_0052c0b0, 0, 0);
        FUN_004b4d40(s_dialogbox_0052c0b0, 0, 0);
        FUN_004d4ec0(1);
        FUN_004dc140(0x14);
        return 0;
    }
    FUN_004d6460(rect, 0xa6, 0xe1, 0x206, 0xf5);
    rect[3] = (int)((unsigned int)rect[1] + 0x14u);
    FUN_004db640(0xf, &DAT_00665890, rect, 0xffff);
    FUN_004d64a0(rect, 0, 0x14);
    FUN_004db640(0xf, &DAT_00665990, rect, 0xffff);
    FUN_004d64a0(rect, 0, 0x14);
    FUN_004db640(0xf, &DAT_00665708, rect, 0xffff);
    if (DAT_00665688 == '\0') {
        if (DAT_00665810 == '\0') {
            FUN_004d6460(rect, 0xab, 0xf0, 0xca, 0x113);
            pcVar7 = s_buttonsound_0052a68c;
            iVar6 = 0x14;
            pcVar1 = FUN_004ddc10(0x20b);
            iVar5 = FUN_004da780(rect, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, pcVar1, 0xc, 0xffff, iVar6, pcVar7);
            uVar2 = FUN_004dcad0();
            if ((char)uVar2 == '\r') {
                iVar5 = 1;
            }
            if (iVar5 != 0) {
                bVar3 = 1;
                ram0x00aa62a0 = 1;
                FUN_004de0b0(DAT_00665808, DAT_0066580c);
            }
            goto LAB_004dd9b4;
        }
        FUN_004d6460(rect, 0x1e1, 0xf0, 0x201, 0x113);
        iVar5 = FUN_004da780(rect, s_dialogcancel_0052c084, s_dialogcanceld_0052c094, &DAT_00665810, 0xc, 0xffff, 0x15, s_buttonsound_0052a68c);
        uVar2 = FUN_004dcad0();
        if ((char)uVar2 == '\x1b') {
            iVar5 = 1;
        }
    }
    else {
        if (DAT_00665810 == '\0') {
            FUN_004d6460(rect, 0xab, 0xf0, 0xca, 0x113);
            iVar5 = FUN_004da780(rect, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, &DAT_00665688, 0xc, 0xffff, 0x14, s_buttonsound_0052a68c);
            uVar2 = FUN_004dcad0();
            if ((char)uVar2 == '\r') {
                iVar5 = 1;
            }
            if (iVar5 != 0) {
                bVar3 = 1;
                ram0x00aa62a0 = 1;
            }
            goto LAB_004dd9b4;
        }
        FUN_004d6460(rect, 0xab, 0xf0, 0xca, 0x113);
        iVar5 = FUN_004da780(rect, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, &DAT_00665688, 0xc, 0xffff, 0x14, s_buttonsound_0052a68c);
        uVar2 = FUN_004dcb10();
        if ((char)uVar2 == '\r') {
            iVar5 = 1;
            FUN_004dcad0();
        }
        bVar3 = (iVar5 != 0);
        if (bVar3) {
            ram0x00aa62a0 = 1;
        }
        FUN_004d6460(rect, 0x1e1, 0xf0, 0x201, 0x113);
        iVar5 = FUN_004da780(rect, s_dialogcancel_0052c084, s_dialogcanceld_0052c094, &DAT_00665810, 0xc, 0xffff, 0x15, s_buttonsound_0052a68c);
        uVar2 = FUN_004dcad0();
        if ((char)uVar2 == '\x1b') {
            iVar5 = 1;
            FUN_004dcb50();
        }
    }
    if (iVar5 != 0) {
        ram0x00aa62a0 = 0;
        bVar3 = 1;
    }
LAB_004dd9b4:
    if (!bVar3) {
        return 0;
    }
    return 1;
}
