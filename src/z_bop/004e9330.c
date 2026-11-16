/* SPDX-License-Identifier: MIT */
/* z_bop 004e9330 */
extern int DAT_00b69e30;
extern int DAT_00aa5aac;
extern int DAT_00b6a250;
extern char DAT_00a91d24[];
extern char DAT_00a91d17[];

extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6d962[4];

extern char s_frontres_gametr_bmp_0052c468[];
extern char s_frontres_gameti_bmp_0052c454[];
extern char s_frontres_gamecr_bmp_0052c440[];
extern char s_frontres_gameci_bmp_0052c42c[];
extern char s_frontres_gamebr_bmp_0052c418[];
extern char s_frontres_gamebi_bmp_0052c404[];
extern char s_frontres_gamemn_bmp_0052c3f0[];
extern char s_frontres_gamecar_bmp_0052c3d8[];
extern char s_frontres_gamecai_bmp_0052c3c0[];
extern char s_frontres_gamecn_bmp_0052c394[];
extern char s_frontres_gamebn_bmp_0052c380[];
extern char s_frontres_gametrn_bmp_0052c3a8[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_gameoverlay_0052c374[];

unsigned char FUN_004e9560(void);
void FUN_004b4a60(char *a, char *b);
void FUN_004d4e20(void);
void FUN_004b5de0(char *a, int b, int c);
void FUN_004b5570(char *a, int b, int c);
void FUN_004b4d40(char *a, int b, int c);
void FUN_004d4ec0(int a);

unsigned int __stdcall FUN_004e9330(void)
{
    unsigned char bVar1;
    int iVar2;
    char *pcVar3;
    int local_a;
    int local_b;

    __builtin_memcpy(&local_a, DAT_00b6a5aa, 4);
    __builtin_memcpy(&local_b, DAT_00b6d962, 4);

    if (DAT_00b69e30 == 2) {
        if (local_a == 0) {
            if (local_b == 0) {
                pcVar3 = s_frontres_gametr_bmp_0052c468;
            } else {
                pcVar3 = s_frontres_gameti_bmp_0052c454;
            }
            goto LAB_004e94cb;
        }
        if (local_a == 3) {
            if (local_b == 0) {
                pcVar3 = s_frontres_gamecr_bmp_0052c440;
            } else {
                pcVar3 = s_frontres_gameci_bmp_0052c42c;
            }
            goto LAB_004e94cb;
        }
        if (local_a == 4) {
            if (local_b == 0) {
                pcVar3 = s_frontres_gamebr_bmp_0052c418;
            } else {
                pcVar3 = s_frontres_gamebi_bmp_0052c404;
            }
            goto LAB_004e94cb;
        }
        if (local_a == 1) {
            pcVar3 = s_frontres_gamemn_bmp_0052c3f0;
            goto LAB_004e94cb;
        }
        if (local_a == 5) {
            if (local_b == 0) {
                pcVar3 = s_frontres_gamecar_bmp_0052c3d8;
            } else {
                pcVar3 = s_frontres_gamecai_bmp_0052c3c0;
            }
            goto LAB_004e94cb;
        }
    } else {
        if (local_a == 0) {
            iVar2 = 0;
            if (0 < DAT_00aa5aac) {
                pcVar3 = DAT_00a91d24;
                do {
                    if (*pcVar3 != '\0') break;
                    pcVar3 = pcVar3 + 0x562;
                    iVar2 = iVar2 + 1;
                } while (iVar2 < DAT_00aa5aac);
            }
            if (DAT_00a91d17[iVar2 * 0x562] == '\0') {
                pcVar3 = s_frontres_gametr_bmp_0052c468;
            } else {
                pcVar3 = s_frontres_gameti_bmp_0052c454;
            }
            goto LAB_004e94cb;
        }
        if (local_a == 3) {
            pcVar3 = s_frontres_gamecn_bmp_0052c394;
            goto LAB_004e94cb;
        }
        if (local_a == 4) {
            pcVar3 = s_frontres_gamebn_bmp_0052c380;
            goto LAB_004e94cb;
        }
        if (local_a == 1) {
            pcVar3 = s_frontres_gamemn_bmp_0052c3f0;
            goto LAB_004e94cb;
        }
        if (local_a == 5) {
            bVar1 = FUN_004e9560();
            if (bVar1 == 0) {
                pcVar3 = s_frontres_gamecai_bmp_0052c3c0;
            } else {
                pcVar3 = s_frontres_gamecar_bmp_0052c3d8;
            }
            goto LAB_004e94cb;
        }
    }
    pcVar3 = s_frontres_gametrn_bmp_0052c3a8;
LAB_004e94cb:
    FUN_004b4a60(pcVar3, s_background_0052bf74);
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    if (DAT_00b6a250 == 0) {
        pcVar3 = s_clientactive_0052b69c;
    } else {
        pcVar3 = s_allactive_0052b6ac;
    }
    FUN_004b5570(pcVar3, 0, 0);
    if (DAT_00b69e30 != 2) {
        FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
    }
    FUN_004b4d40(s_gameoverlay_0052c374, 0, 0);
    FUN_004d4ec0(1);
    return 1;
}
