/* SPDX-License-Identifier: MIT */
/* z_xvt 004e78b0 */
extern int DAT_0081b850;
extern int DAT_0081bfba;
extern int DAT_0081fb96;
extern char *s_frontres_gametr_bmp_00533304;
extern char *s_frontres_gameti_bmp_005332f0;
extern char *s_frontres_gamecr_bmp_005332dc;
extern char *s_frontres_gameci_bmp_005332c8;
extern char *s_frontres_gamebr_bmp_005332b4;
extern char *s_frontres_gamebi_bmp_005332a0;
extern char *s_frontres_gamemn_bmp_0053328c;
extern int DAT_0075766c;
extern char DAT_007438e4;
extern char DAT_007438d7;
extern char *s_frontres_gamecn_bmp_00533260;
extern char *s_frontres_gamebn_bmp_0053324c;
extern char *s_frontres_gametrn_bmp_00533274;
extern char *s_background_00532a4c;
extern char *s_frame_00531358;
extern int DAT_0081bc68;
extern char *s_clientactive_00531eac;
extern char *s_allactive_00531ebc;
extern char *s_chatbox_00532a84;
extern char *s_gameoverlay_00533240;

extern int __cdecl FUN_004d0290(char *param_1, char *param_2);
extern void __cdecl FUN_004d52b0(void);
extern void __cdecl FUN_004d1750(char *param_1, int param_2, int param_3);
extern void __cdecl FUN_004d0e60(char *param_1, int param_2, int param_3);
extern void __cdecl FUN_004d0580(char *param_1, int param_2, int param_3);
extern int __cdecl FUN_004d5350(int param_1);

int __stdcall FUN_004e78b0(void)
{
    int iVar1;
    char *pcVar2;

    if (DAT_0081b850 == 2) {
        if (DAT_0081bfba == 0) {
            if (DAT_0081fb96 == 0) {
                pcVar2 = s_frontres_gametr_bmp_00533304;
            }
            else {
                pcVar2 = s_frontres_gameti_bmp_005332f0;
            }
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 3) {
            if (DAT_0081fb96 == 0) {
                pcVar2 = s_frontres_gamecr_bmp_005332dc;
            }
            else {
                pcVar2 = s_frontres_gameci_bmp_005332c8;
            }
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 4) {
            if (DAT_0081fb96 == 0) {
                pcVar2 = s_frontres_gamebr_bmp_005332b4;
            }
            else {
                pcVar2 = s_frontres_gamebi_bmp_005332a0;
            }
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 1) {
            pcVar2 = s_frontres_gamemn_bmp_0053328c;
            goto LAB_004e79fb;
        }
    }
    else {
        if (DAT_0081bfba == 0) {
            iVar1 = 0;
            if (0 < DAT_0075766c) {
                pcVar2 = &DAT_007438e4;
                do {
                    if (*pcVar2 != '\0') break;
                    pcVar2 = pcVar2 + 0x562;
                    iVar1 = iVar1 + 1;
                } while (iVar1 < DAT_0075766c);
            }
            if (((char *)&DAT_007438d7)[iVar1 * 0x562] == '\0') {
                pcVar2 = s_frontres_gametr_bmp_00533304;
            }
            else {
                pcVar2 = s_frontres_gameti_bmp_005332f0;
            }
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 3) {
            pcVar2 = s_frontres_gamecn_bmp_00533260;
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 4) {
            pcVar2 = s_frontres_gamebn_bmp_0053324c;
            goto LAB_004e79fb;
        }
        if (DAT_0081bfba == 1) {
            pcVar2 = s_frontres_gamemn_bmp_0053328c;
            goto LAB_004e79fb;
        }
    }
    pcVar2 = s_frontres_gametrn_bmp_00533274;
LAB_004e79fb:
    FUN_004d0290(pcVar2, s_background_00532a4c);
    FUN_004d52b0();
    FUN_004d1750(s_background_00532a4c, 0, 0);
    FUN_004d0e60(s_frame_00531358, 0, 0);
    if (DAT_0081bc68 == 0) {
        pcVar2 = s_clientactive_00531eac;
    }
    else {
        pcVar2 = s_allactive_00531ebc;
    }
    FUN_004d0e60(pcVar2, 0, 0);
    if (DAT_0081b850 != 2) {
        FUN_004d0580(s_chatbox_00532a84, 0, 0);
    }
    FUN_004d0580(s_gameoverlay_00533240, 0, 0);
    FUN_004d5350(1);
    return 1;
}
