/* SPDX-License-Identifier: MIT */
/* z_bop 004dcd30 */
extern char s_dialogbox_0052c0b0[];
extern char s_buttonsound_0052a68c[];
extern char s_dialogok_0052b2c0[];
extern char s_dialogokd_0052c0a4[];
extern char s_dialogcancel_0052c084[];
extern char s_dialogcanceld_0052c094[];

extern char DAT_00665688[];
extern char DAT_00665810[];
extern char DAT_00665890[];
extern char DAT_00665990[];
extern char DAT_00665708[];
extern int DAT_00665808;
extern int DAT_0066580c;
extern int ram0x00aa62a0;

void FUN_004dcb30(void);
void FUN_004de0b0(int, int);
void FUN_004d4e20(void);
void FUN_004b4d40(char *, int, int);
void FUN_004d4ec0(int);
void FUN_004dc140(int);
int FUN_004dd0f0(void);
void FUN_004d6460(int *, int, int, int, int);
void FUN_004db640(int, char *, int *, int);
void FUN_004d64a0(int *, int, int);
char *FUN_004ddc10(int);
int FUN_004da780(int *, char *, char *, char *, int, int, int, char *);
unsigned int FUN_004dcad0(void);
unsigned int FUN_004dcb10(void);
unsigned int FUN_004dcb50(void);

unsigned int __cdecl FUN_004dcd30(int param_1)
{
    _Bool bVar1;
    char *pcVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    char *pcVar7;
    int local_10[4]; /* rect: left, top, right, bottom (local_10, local_c, local_8, local_4) */

    if (param_1 == 0) {
        FUN_004dcb30();
        if ((DAT_00665688[0] == '\0') && (DAT_00665810[0] != '\0')) {
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
    bVar1 = (FUN_004dd0f0() != 0);
    if (bVar1) {
        ram0x00aa62a0 = 0;
    }
    FUN_004d6460(local_10, 0xa6, 0xe1, 0x206, 0xf5);
    local_10[3] = (int)((unsigned int)local_10[1] + 0x14u);
    FUN_004db640(0xf, DAT_00665890, local_10, 0xffff);
    FUN_004d64a0(local_10, 0, 0x14);
    FUN_004db640(0xf, DAT_00665990, local_10, 0xffff);
    FUN_004d64a0(local_10, 0, 0x14);
    FUN_004db640(0xf, DAT_00665708, local_10, 0xffff);
    if (DAT_00665688[0] == '\0') {
        if (DAT_00665810[0] == '\0') {
            FUN_004d6460(local_10, 0xab, 0xf0, 0xca, 0x113);
            pcVar7 = s_buttonsound_0052a68c;
            iVar6 = 0x14;
            uVar4 = 0xffff;
            iVar5 = 0xc;
            pcVar2 = FUN_004ddc10(0x20b);
            iVar5 = FUN_004da780(local_10, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, pcVar2, iVar5,
                                 (int)uVar4, iVar6, pcVar7);
            uVar3 = FUN_004dcad0();
            if ((char)uVar3 == '\r') {
                iVar5 = 1;
            }
            if (iVar5 != 0) {
                bVar1 = 1;
                ram0x00aa62a0 = 1;
                FUN_004de0b0(DAT_00665808, DAT_0066580c);
            }
            goto LAB_004dd0dc;
        }
        FUN_004d6460(local_10, 0x1e1, 0xf0, 0x201, 0x113);
        iVar5 = FUN_004da780(local_10, s_dialogcancel_0052c084, s_dialogcanceld_0052c094, DAT_00665810,
                             0xc, 0xffff, 0x15, s_buttonsound_0052a68c);
        uVar3 = FUN_004dcad0();
        if ((char)uVar3 == '\x1b') {
            iVar5 = 1;
        }
    }
    else {
        if (DAT_00665810[0] == '\0') {
            FUN_004d6460(local_10, 0xab, 0xf0, 0xca, 0x113);
            iVar5 = FUN_004da780(local_10, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, DAT_00665688, 0xc,
                                 0xffff, 0x14, s_buttonsound_0052a68c);
            uVar3 = FUN_004dcad0();
            if ((char)uVar3 == '\r') {
                iVar5 = 1;
            }
            if (iVar5 != 0) {
                bVar1 = 1;
                ram0x00aa62a0 = 1;
            }
            goto LAB_004dd0dc;
        }
        FUN_004d6460(local_10, 0xab, 0xf0, 0xca, 0x113);
        iVar5 = FUN_004da780(local_10, s_dialogok_0052b2c0, s_dialogokd_0052c0a4, DAT_00665688, 0xc,
                             0xffff, 0x14, s_buttonsound_0052a68c);
        uVar3 = FUN_004dcb10();
        if ((char)uVar3 == '\r') {
            iVar5 = 1;
            FUN_004dcad0();
        }
        if (iVar5 != 0) {
            ram0x00aa62a0 = 1;
        }
        bVar1 = (iVar5 != 0) || bVar1;
        FUN_004d6460(local_10, 0x1e1, 0xf0, 0x201, 0x113);
        iVar5 = FUN_004da780(local_10, s_dialogcancel_0052c084, s_dialogcanceld_0052c094, DAT_00665810,
                             0xc, 0xffff, 0x15, s_buttonsound_0052a68c);
        uVar3 = FUN_004dcad0();
        if ((char)uVar3 == '\x1b') {
            iVar5 = 1;
            FUN_004dcb50();
        }
    }
    if (iVar5 != 0) {
        ram0x00aa62a0 = 0;
        bVar1 = 1;
    }
LAB_004dd0dc:
    if (!bVar1) {
        return 0;
    }
    return 1;
}
