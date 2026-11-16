/* SPDX-License-Identifier: MIT */
/* z_xvt 004b8a50 */
extern int DAT_0064ef50;
extern int DAT_0064ef54;
extern int DAT_0064ef58;
extern int DAT_0064ef5c;
extern int DAT_0064ef60;
extern int DAT_0064ef64;
extern int DAT_0085e6c0;
extern char DAT_00859c40[];
extern char DAT_0085e7a0[];
extern char DAT_0081bcf0[];
extern char DAT_0081e082[];
extern char DAT_0081b740[];
extern int DAT_0081bcd0;
extern int DAT_0081b850;
extern char DAT_0085e7c6;
extern int DAT_00532ad8;
extern unsigned int DAT_00758620;
extern unsigned int DAT_00758624;
extern unsigned int DAT_00758628;
extern unsigned int DAT_0075862c;
extern unsigned int DAT_00758630;
extern unsigned int DAT_00758634;
extern unsigned int DAT_00758638;
extern unsigned int DAT_0075863c;
extern unsigned int DAT_00758640;
extern unsigned int DAT_00758644;
extern unsigned int DAT_00758648;
extern unsigned int DAT_0075864c;
extern unsigned int DAT_00758650;
extern unsigned int DAT_00758654;
extern unsigned int DAT_00758658;
extern unsigned int DAT_0075865c;
extern unsigned int DAT_00758660;
extern unsigned char DAT_0085e7b4;
extern unsigned char DAT_0085e7b5;
extern unsigned char DAT_0085e7b6;
extern unsigned char DAT_0085e7b7;
extern unsigned char DAT_0085e7b8;
extern unsigned char DAT_0085e7b9;
extern unsigned char DAT_0085e7ba;
extern unsigned char DAT_0085e7bb;
extern unsigned char DAT_0085e7bc;
extern unsigned char DAT_0085e7bd;
extern unsigned char DAT_0085e7be;
extern unsigned char DAT_0085e7bf;
extern unsigned char DAT_0085e78f;
extern unsigned char DAT_0085e7c4;
extern unsigned char DAT_0085e790;

extern char s_frontres_configb_bmp_00531360[];
extern char s_backconfig_00531378[];
extern char s_frame_00531358[];
extern char s_alloff_00531350[];
extern char s_configoverlay_00531340[];
extern char s__c_s__c_s_00531334[];
extern char s_rebtiny_d_00531328[];
extern char s_imptiny_d_0053131c[];
extern char s_buttonsound_00531310[];
extern char s_leaveup_005312fc[];
extern char s_leavedown_00531304[];

void __cdecl FUN_004ddc90(int a, int b);
void __cdecl FUN_004d8fb0(void);
void __cdecl FUN_004d8f30(void);
void __cdecl FUN_004dbea0(void);
void __cdecl FUN_004bdd20(void);
void __cdecl FUN_004d0290(char *a, char *b);
void __cdecl FUN_004d52b0(void);
void __cdecl FUN_004d1750(char *a, int b, int c);
void __cdecl FUN_004d0e60(char *a, int b, int c);
void __cdecl FUN_004dcfd0(int *r, int a, int b, int c, int d);
void __cdecl FUN_004d0580(char *a, int b, int c);
void __cdecl FUN_004d5350(int a);
void __cdecl FUN_004db490(int a);
void __cdecl FUN_004bbd50(void);
void __cdecl FUN_004b8ed0(void);
void __cdecl FUN_004bcc20(void);
void __cdecl FUN_004bd330(void);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void __cdecl FUN_004da920(int a, char *b, int *c, int d);
void __cdecl FUN_004bb3c0(void);
int __cdecl FUN_004c0750(int a);
void __cdecl FUN_004da030(void);
char * __cdecl FUN_004dcfa0(int a);
void __cdecl FUN_004da060(char *a);
unsigned int __cdecl FUN_004d9a80(int *r, char *a, char *b, char *c, int d, int e, int f, char *g);
void __cdecl FUN_004da050(void);
unsigned char __cdecl FUN_004dc470(void);
unsigned int __cdecl FUN_004cc8e0(void);
void __cdecl FUN_004bacf0(void);
void __cdecl FUN_004dbb40(void);
void __cdecl FUN_004db520(void);
void __cdecl FUN_004d0430(char *a);
void __cdecl FUN_004db4c0(void);
void __cdecl FUN_004d8fe0(void);
int __cdecl _rand(void);
void __cdecl FUN_004cb830(int a, unsigned int *b, int c);

unsigned int __cdecl FUN_004b8a50(int *param_1, int *param_2, int param_3, char *param_4,
                                  int *param_5, int *param_6, int *param_7, unsigned int param_8,
                                  unsigned int param_9, int param_10, int param_11,
                                  unsigned int param_12, unsigned int param_13)
{
    unsigned char bVar1;
    char *pcVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int uVar6;
    int iVar7;
    char *pcVar8;
    int local_rect[4];

    iVar5 = 0;
    if (param_1 == (int *)0) {
        FUN_004ddc90(0x20, 0x7f);
        FUN_004d8fb0();
        FUN_004d8f30();
        DAT_0064ef50 = 0;
        DAT_0064ef54 = 0;
        DAT_0064ef64 = 0;
        DAT_0064ef58 = 0;
        DAT_0064ef60 = 1;
        FUN_004dbea0();
        FUN_004bdd20();
        if (0 < DAT_0085e6c0) {
            pcVar2 = DAT_00859c40;
            do {
                if (*pcVar2 == DAT_0085e7a0[DAT_0064ef58]) {
                    DAT_0064ef5c = iVar5;
                }
                pcVar2 = pcVar2 + 0x95;
                iVar5 = iVar5 + 1;
            } while (iVar5 < DAT_0085e6c0);
        }
        FUN_004d0290(s_frontres_configb_bmp_00531360, s_backconfig_00531378);
        FUN_004d52b0();
        FUN_004d1750(s_backconfig_00531378, 0, 0);
        FUN_004d0e60(s_frame_00531358, 0, 0);
        FUN_004d0e60(s_alloff_00531350, 0, 0);
        FUN_004dcfd0(local_rect, 0x54, 0x6b, 0x25c, 0x1b1);
        FUN_004d0580(s_configoverlay_00531340, 0, 0);
        FUN_004d5350(1);
        FUN_004db490(0x14);
    }
    switch (DAT_0064ef64) {
    case 0:
        FUN_004bbd50();
        if (DAT_0064ef60 != 0) {
            FUN_004d52b0();
            FUN_004d5350(1);
        }
        break;
    case 1:
        FUN_004b8ed0();
        if (DAT_0064ef60 != 0) {
            FUN_004d52b0();
            FUN_004d5350(1);
        }
        break;
    case 2:
        FUN_004b8ed0();
        if (DAT_0064ef60 != 0) {
            FUN_004d52b0();
            FUN_004d5350(1);
        }
        break;
    case 3:
        FUN_004bcc20();
        if (DAT_0064ef60 != 0) {
            FUN_004d52b0();
            FUN_004d5350(1);
        }
        break;
    case 4:
        FUN_004bd330();
    }
    FUN_004dcfd0(local_rect, 200, 0x1c4, 0x1b4, 0x1d0);
    if (DAT_0081bcf0[0] != '\0') {
        _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, DAT_0081e082, 1, DAT_0081bcf0);
        FUN_004da920(0xc, DAT_0081b740, local_rect, DAT_0081bcd0);
        uVar4 = (unsigned int)((int)param_1 >> 0x1f);
        iVar5 = (int)((((((unsigned int)param_1 ^ uVar4) - uVar4) & 0x1f) ^ uVar4) - uVar4) >> 1;
        _sprintf(DAT_0081b740, s_rebtiny_d_00531328, iVar5);
        FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
        _sprintf(DAT_0081b740, s_imptiny_d_0053131c, iVar5);
        FUN_004d0e60(DAT_0081b740, 0x1a4, 0x1c5);
    }
    DAT_0064ef60 = 0;
    FUN_004bb3c0();
    iVar5 = FUN_004c0750(2);
    if (iVar5 != 1) {
        FUN_004dcfd0(local_rect, 0x55, 0x1bf, 0xb0, 0x1d7);
        if (DAT_0085e7c6 != '\0') {
            FUN_004da030();
        }
        pcVar2 = FUN_004dcfa0(0xc1);
        FUN_004da060(pcVar2);
        pcVar8 = s_buttonsound_00531310;
        iVar7 = 8;
        uVar6 = 0;
        iVar5 = 0xc;
        pcVar2 = FUN_004dcfa0(0xc1);
        uVar4 = FUN_004d9a80(local_rect, s_leaveup_005312fc, s_leavedown_00531304, pcVar2,
                             iVar5, (int)uVar6, iVar7, pcVar8);
        FUN_004da050();
        bVar1 = FUN_004dc470();
        uVar4 = uVar4 | (unsigned int)bVar1;
        if (DAT_0081b850 == 4) {
            uVar3 = FUN_004cc8e0();
            uVar4 = uVar4 | uVar3;
        }
        if (uVar4 != 0) {
            FUN_004bacf0();
            DAT_00532ad8 = 0;
            FUN_004dbea0();
            FUN_004dbb40();
            FUN_004db520();
            FUN_004d0430(s_backconfig_00531378);
            FUN_004db4c0();
            FUN_004d8fe0();
            if (DAT_0081b850 == 4) {
                DAT_00758620 = 0x59;
                DAT_00758624 = (unsigned int)DAT_0085e7b4;
                DAT_00758628 = (unsigned int)DAT_0085e7b5;
                DAT_0075862c = (unsigned int)DAT_0085e7b6;
                DAT_00758630 = (unsigned int)DAT_0085e7b7;
                DAT_00758634 = (unsigned int)DAT_0085e7b8;
                DAT_00758638 = (unsigned int)DAT_0085e7b9;
                DAT_0075863c = (unsigned int)DAT_0085e7ba;
                DAT_00758640 = (unsigned int)DAT_0085e7bb;
                DAT_00758644 = (unsigned int)DAT_0085e7bc;
                DAT_00758648 = (unsigned int)DAT_0085e7bd;
                DAT_0075864c = (unsigned int)DAT_0085e7be;
                DAT_00758650 = (unsigned int)DAT_0085e7bf;
                DAT_00758654 = (unsigned int)_rand();
                DAT_00758658 = (unsigned int)DAT_0085e78f;
                DAT_0075865c = (unsigned int)DAT_0085e7c4;
                DAT_00758660 = (unsigned int)DAT_0085e790;
                FUN_004cb830(0, &DAT_00758620, 0x44);
            }
        }
        return 0;
    }
    return 1;
}
