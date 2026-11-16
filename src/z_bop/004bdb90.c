/* SPDX-License-Identifier: MIT */
/* z_bop 004bdb90 */
/* Evidence gaps (not resolved from the supplied bundle):
 * - Return types and argument types of most callees are unknown; Ghidra's
 *   shapes are used and the prototypes may need adjusting.
 * - local_10[0], local_10[1] and the contents of local_10[2], local_10[3]
 *   (local_8, local_4) are written by FUN_004b4cc0; layout is inferred from
 *   the contiguous stack slots, element types assumed to be int.
 * - DAT_00b6a2e0 total size assumed from the clear loop: 0x1214b dwords + 1 word.
 * - Callee calling conventions are assumed cdecl except where noted; no
 *   stack-check FINDING was provided.
 */

void FUN_004c9ee0(void);
void FUN_004d4dd0(void);
void FUN_004d4c00(int param_1);
void FUN_004de100(void);
void FUN_004d4bf0(void);
void FUN_004d4f60(void);
void FUN_004dada0(int param_1);
void FUN_004df880(char *param_1);
void FUN_004df700(char *param_1);
void FUN_004b4cc0(char *param_1, int *param_2);
void FUN_004b49a0(char *param_1, void *param_2);
void FUN_004dd9d0(char *param_1);
void FUN_004df250(char *param_1);
void FUN_004cc060(char *param_1);
int FUN_004d6e30(unsigned char param_1, unsigned char param_2, unsigned char param_3);
void FUN_004b97e0(void);
void FUN_004c9c50(int param_1);
void FUN_004bf8c0(char *param_1);
int FUN_004d2e50(void);
void FUN_004d3330(void);
void FUN_004d34a0(unsigned int param_1);
void FUN_004d3020(int param_1, int param_2, int param_3);
void FUN_004d3140(void);

extern int DAT_0052ba5c;
extern int DAT_00bb2818;
extern char s_frontres_top_lst_0052b188[];
extern char s_frontres_side_lst_0052b174[];
extern char s_frontres_awards_lst_0052b160[];
extern char s_frontres_promo_lst_0052b14c[];
extern char s_frontres_icons_lst_0052b138[];
extern char s_sfx_sfx_lst_0052b12c[];
extern char s_cursor_0052b124[];
extern char s_fronttxt_txt_0052b110[];
extern char s_movies_cutscene_lst_0052b0fc[];
extern char s_frontres_campawds_lst_0052b0e4[];
extern char DAT_0052b120[];
extern void *DAT_00b6a2ac;
extern char DAT_00b69d20[];
extern unsigned char DAT_00b6a2e0[];
extern unsigned int *DAT_00b69cd0;
extern int DAT_00b6a2c4;
extern int _DAT_00b6a2d8;
extern int DAT_00bb2810;
extern int DAT_00b69cc4;
extern int DAT_00b6a2a0;
extern int DAT_00b69e38;
extern int DAT_00b6a2a4;
extern int DAT_00b6a2c0;
extern int _DAT_00b69ccc;
extern int DAT_00b69e24;
extern int DAT_00b6a264;
extern int _DAT_00b69cf0;
extern int _DAT_00b69cf4;
extern int _DAT_00b69cf8;
extern int _DAT_00b69cfc;
extern int _DAT_00b69d00;
extern int _DAT_00b69d04;
extern int _DAT_00b69d08;
extern int _DAT_00b69d0c;
extern int _DAT_00b69d10;
extern int _DAT_00b69d14;
extern int _DAT_00b6a270;
extern int _DAT_00b6a274;
extern int _DAT_00b6a278;
extern int _DAT_00b6a27c;
extern int _DAT_00b6a280;
extern int _DAT_00b6a284;
extern int _DAT_00b6a288;
extern int _DAT_00b6a28c;
extern int _DAT_00b6a290;
extern int _DAT_00b6a294;
extern int _DAT_00b6a298;
extern int _DAT_00b6a29c;
extern int DAT_00b69cac;
extern char DAT_00bb7351[];
extern unsigned int DAT_00664ec4;
extern char DAT_00bb737f;
extern unsigned char DAT_00bb737e;

extern void __cdecl _free(void *p);
extern void *__cdecl _malloc(unsigned int size);
extern int __cdecl _sprintf(char *buf, const char *fmt, ...);

int __stdcall FUN_004bdb90(void)
{
    int iVar1;
    unsigned int *puVar2;
    int local_10[4];

    DAT_0052ba5c = 1;
    DAT_00bb2818 = 0;
    FUN_004c9ee0();
    FUN_004d4dd0();
    FUN_004d4c00(0);
    FUN_004de100();
    FUN_004d4bf0();
    FUN_004d4f60();
    FUN_004dada0(0xf);
    FUN_004dada0(0xc);
    FUN_004dada0(10);
    FUN_004df880(s_frontres_top_lst_0052b188);
    FUN_004df880(s_frontres_side_lst_0052b174);
    FUN_004df880(s_frontres_awards_lst_0052b160);
    FUN_004df880(s_frontres_promo_lst_0052b14c);
    FUN_004df880(s_frontres_icons_lst_0052b138);
    FUN_004df700(s_sfx_sfx_lst_0052b12c);
    FUN_004b4cc0(s_cursor_0052b124, local_10);
    if (DAT_00b6a2ac != (void *)0x0) {
        _free(DAT_00b6a2ac);
        DAT_00b6a2ac = (void *)0x0;
    }
    DAT_00b6a2ac = _malloc((unsigned int)(((unsigned int)(local_10[2] + 1) * (unsigned int)(local_10[3] + 1)) * 2u));
    FUN_004b49a0(s_cursor_0052b124, DAT_00b6a2ac);
    _sprintf(DAT_00b69d20, DAT_0052b120, DAT_00b6a2e0);
    puVar2 = (unsigned int *)DAT_00b6a2e0;
    for (iVar1 = 0x1214b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    *(unsigned short *)puVar2 = 0;
    FUN_004dd9d0(s_fronttxt_txt_0052b110);
    FUN_004df250(s_movies_cutscene_lst_0052b0fc);
    FUN_004cc060(s_frontres_campawds_lst_0052b0e4);
    DAT_00b69cd0 = (unsigned int *)_malloc(0x400);
    DAT_00b6a2c4 = 0;
    if (DAT_00b69cd0 != (unsigned int *)0x0) {
        puVar2 = DAT_00b69cd0;
        for (iVar1 = 0x100; iVar1 != 0; iVar1 = iVar1 + -1) {
            *puVar2 = 0;
            puVar2 = puVar2 + 1;
        }
    }
    _DAT_00b6a2d8 = FUN_004d6e30(0, 0xff, 0);
    DAT_00bb2810 = FUN_004d6e30(0, 0, 0x80);
    DAT_00b69cc4 = FUN_004d6e30(0x40, 0x80, 0x40);
    DAT_00b6a2a0 = FUN_004d6e30(0, 0xff, 0);
    DAT_00b69e38 = FUN_004d6e30(0xff, 0, 0);
    DAT_00b6a2a4 = FUN_004d6e30(0, 0, 0xff);
    DAT_00b6a2c0 = FUN_004d6e30(0xff, 0xff, 0);
    _DAT_00b69ccc = FUN_004d6e30(0x60, 0x60, 0x60);
    DAT_00b69e24 = FUN_004d6e30(0xc4, 0xfc, 0xfc);
    DAT_00b6a264 = FUN_004d6e30(0x30, 0x6f, 0x7b);
    _DAT_00b69cf0 = DAT_00b69e38;
    _DAT_00b69cf4 = DAT_00bb2810;
    _DAT_00b69cf8 = DAT_00b6a2a4;
    _DAT_00b69cfc = DAT_00b6a2c0;
    _DAT_00b69d00 = FUN_004d6e30(0x80, 0, 0xff);
    _DAT_00b69d04 = FUN_004d6e30(0, 0xff, 0x80);
    _DAT_00b69d08 = DAT_00b69cc4;
    _DAT_00b69d0c = FUN_004d6e30(0, 0xff, 0xff);
    _DAT_00b69d10 = FUN_004d6e30(0, 0x80, 0xff);
    _DAT_00b69d14 = FUN_004d6e30(0xff, 0x80, 0);
    _DAT_00b6a270 = FUN_004d6e30(0x80, 0x80, 0);
    _DAT_00b6a274 = FUN_004d6e30(0x95, 0x95, 0);
    _DAT_00b6a278 = FUN_004d6e30(0xaa, 0xaa, 0);
    _DAT_00b6a27c = FUN_004d6e30(0xc0, 0xc0, 0);
    _DAT_00b6a280 = FUN_004d6e30(0xd5, 0xd5, 0);
    _DAT_00b6a284 = FUN_004d6e30(0xea, 0xea, 0);
    _DAT_00b6a288 = FUN_004d6e30(0xff, 0xff, 0);
    _DAT_00b6a28c = FUN_004d6e30(0xea, 0xea, 0);
    _DAT_00b6a290 = FUN_004d6e30(0xd5, 0xd5, 0);
    _DAT_00b6a294 = FUN_004d6e30(0xc0, 0xc0, 0);
    _DAT_00b6a298 = FUN_004d6e30(0xaa, 0xaa, 0);
    _DAT_00b6a29c = FUN_004d6e30(0x95, 0x95, 0);
    FUN_004b97e0();
    FUN_004c9c50(DAT_00b69cac);
    FUN_004bf8c0(DAT_00bb7351);
    iVar1 = FUN_004d2e50();
    DAT_00664ec4 = (unsigned int)(iVar1 == 0);
    FUN_004d3330();
    if (DAT_00bb737f != '\0') {
        FUN_004d34a0(((unsigned int)DAT_00bb737e * 0xffffu) / 9u);
        FUN_004d3020(7, 0, 0);
        return 0;
    }
    FUN_004d3140();
    return 0;
}
