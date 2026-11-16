/* SPDX-License-Identifier: MIT */
/* z_xvt 004f663a */
extern void FUN_004dbea0(void);
extern void FUN_004be1f0(void *);
extern void *FUN_004be160(char *, char *);
extern int FUN_004f6ad0(int *, int *, int *, int *);
extern void FUN_004db670(void *, void *);
extern void FUN_004db490(int);
extern unsigned char FUN_004dbe00(int);
extern void FUN_004d0e60(char *, int, int);
extern void FUN_004d52b0(void);
extern void FUN_004d1750(char *, int, int);
extern void FUN_004d0580(char *, int, int);
extern void FUN_004d5350(int);
extern void FUN_004db4e0(void);
extern void FUN_004da7c0(int, char *, int, int, unsigned int);
extern void FUN_004db500(void);
extern void FUN_004cff00(void);
extern void FUN_004d5420(void);
extern void FUN_004d00f0(int, int, int);
extern unsigned int FUN_004dbe40(void);
extern unsigned char FUN_004db570(void);
extern unsigned char FUN_004db590(void);
extern void FUN_004d50a0(void);
extern void FUN_004bef20(void);
extern char LAB_004bee00[];

extern char s_credits_txt_00533fd0[];
extern char DAT_00533fdc[];
extern char s_comp01_00533f6c[];
extern char s_artists_00533f18[];
extern char s_testers_00533f50[];
extern char s_lakota_00533f34[];
extern char s_background_00532a4c[];
extern char s_totallylogo_00533f8c[];
extern char s_leclogo_00533fb0[];

extern int DAT_00533f00;
extern void *DAT_006509e4;
extern unsigned short DAT_006509e0;
extern int DAT_006509d4, DAT_00650a08, DAT_00650a04, DAT_006509e8, DAT_006509c8;
extern int DAT_006509c0, DAT_006509ec, DAT_006509cc, DAT_006509c4;
extern int DAT_006509d0;
extern int DAT_006509f0, DAT_006509d8, DAT_006509b8, DAT_006509f4;
extern int DAT_006509dc, DAT_006509bc, DAT_006509f8, DAT_006509fc;
extern int DAT_00650a00, DAT_00650a0c;
extern int DAT_00650a10;
extern char DAT_00650a18[];
extern char DAT_00652a18[];
extern unsigned short DAT_00654a18[];
extern unsigned short DAT_00654a58[];
extern unsigned short DAT_00654a98[];

unsigned int __cdecl FUN_004f663a(unsigned int param_1, int param_2)
{
    unsigned char bVar1;
    unsigned char uVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned short *puVar5;
    unsigned short *puVar6;
    int iVar7;
    char *pcVar8;
    int unaff_retaddr = 0;   /* stands in for the return-address slot (unverified) */
    unsigned char in_ZF = 0; /* UNRESOLVED: ZF set by code before 0x4f663e */

    if (!in_ZF) {
        FUN_004dbea0();
        if (DAT_006509e4 != 0) {
            FUN_004be1f0(DAT_006509e4);
        }
        DAT_006509e0 = 0xffff;
        DAT_00533f00 = 0;
        DAT_006509d4 = 0;
        DAT_00650a08 = 0;
        DAT_00650a04 = 0;
        DAT_006509e8 = 0;
        DAT_006509c8 = 0;
        DAT_006509c0 = 0;
        DAT_006509ec = 0;
        DAT_006509cc = 0;
        DAT_006509c4 = 0;
        DAT_006509e4 = FUN_004be160(s_credits_txt_00533fd0, DAT_00533fdc);
        DAT_006509d0 = 0;
        DAT_006509f0 = 0;
        DAT_006509d8 = 0;
        DAT_006509b8 = 0;
        DAT_006509f4 = 0;
        DAT_006509dc = 0;
        DAT_006509bc = 0;
        iVar3 = FUN_004f6ad0(&DAT_00650a10, &DAT_006509d0, &DAT_006509f8, &DAT_006509fc);
        if (iVar3 == 0) {
            FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
            return 0;
        }
        FUN_004db490(200);
    }
    if (DAT_00533f00 != 0) {
        FUN_004db670((void *)FUN_004bef20, (void *)LAB_004bee00);
        return 0;
    }
    bVar1 = FUN_004dbe00(0x10);
    if (((bVar1 != 0) &&
         (bVar1 = FUN_004dbe00(0x12), bVar1 != 0)) &&
        (bVar1 = FUN_004dbe00(0x7b), bVar1 != 0)) {
        switch (DAT_006509d4) {
        case 0:
            iVar3 = 0x9d;
            pcVar8 = s_comp01_00533f6c;
            break;
        case 1:
            iVar3 = 0xa3;
            pcVar8 = s_artists_00533f18;
            break;
        case 2:
            iVar3 = 0x98;
            pcVar8 = s_testers_00533f50;
            break;
        default:
            goto switchD_004f6785_caseD_3;
        case 4:
            iVar3 = 0x8f;
            pcVar8 = s_lakota_00533f34;
        }
        FUN_004d0e60(pcVar8, 0xa0, iVar3);
    }
switchD_004f6785_caseD_3:
    if ((((DAT_006509e8 == DAT_006509f0) && (DAT_006509b8 == DAT_006509c0)) &&
         ((DAT_006509d8 == DAT_006509c8 &&
           ((DAT_006509ec == DAT_006509f4 && (DAT_006509bc == DAT_006509c4)))))) &&
        (DAT_006509dc == DAT_006509cc)) goto LAB_004f6916;
    DAT_006509f0 = DAT_006509e8;
    DAT_006509b8 = DAT_006509c0;
    DAT_006509d8 = DAT_006509c8;
    DAT_006509f4 = DAT_006509ec;
    DAT_006509bc = DAT_006509c4;
    DAT_006509dc = DAT_006509cc;
    FUN_004d52b0();
    FUN_004d1750(s_background_00532a4c, 0, 0);
    if (DAT_006509e8 == 1) {
        pcVar8 = s_totallylogo_00533f8c;
LAB_004f68c5:
        FUN_004d0580(pcVar8, DAT_006509c0, DAT_006509c8);
    }
    else if (DAT_006509e8 == 2) {
        pcVar8 = s_leclogo_00533fb0;
        goto LAB_004f68c5;
    }
    if (DAT_006509ec == 1) {
        pcVar8 = s_totallylogo_00533f8c;
LAB_004f6904:
        FUN_004d0580(pcVar8, DAT_006509c4, DAT_006509cc);
    }
    else if (DAT_006509ec == 2) {
        pcVar8 = s_leclogo_00533fb0;
        goto LAB_004f6904;
    }
    FUN_004d5350(1);
LAB_004f6916:
    if ((DAT_00650a10 & 1) != 0) {
        FUN_004db4e0();
    }
    iVar3 = DAT_00650a08;
    puVar5 = DAT_00654a18;
    pcVar8 = DAT_00650a18;
    iVar7 = DAT_00650a00;
    do {
        puVar6 = puVar5 + 1;
        FUN_004da7c0(0xf, pcVar8, iVar3, iVar7, (unsigned int)*puVar5);
        puVar5 = puVar6;
        pcVar8 = pcVar8 + 0x100;
        iVar7 = (int)((unsigned int)iVar7 + 0x13u);
    } while (puVar6 < DAT_00654a58);
    if ((DAT_00650a10 & 1) == 0) {
        FUN_004db4e0();
    }
    else {
        FUN_004db500();
    }
    iVar3 = DAT_00650a0c;
    puVar5 = DAT_00654a58;
    pcVar8 = DAT_00652a18;
    iVar7 = DAT_00650a04;
    do {
        puVar6 = puVar5 + 1;
        FUN_004da7c0(0xf, pcVar8, iVar3, iVar7, (unsigned int)*puVar5);
        puVar5 = puVar6;
        pcVar8 = pcVar8 + 0x100;
        iVar7 = (int)((unsigned int)iVar7 + 0x13u);
    } while (puVar6 < DAT_00654a98);
    FUN_004db500();
    if (DAT_006509d0 == 0) {
        if (DAT_006509f8 <= param_2) {
            DAT_00533f00 = 1;
            FUN_004d5420();
            FUN_004d00f0(0x8000, 0x1000, 2000);
        }
    }
    else if (DAT_006509f8 <= param_2) {
        FUN_004f6ad0(&DAT_00650a10, &DAT_006509d0, &unaff_retaddr, &DAT_006509fc);
        DAT_006509f8 = (int)((unsigned int)DAT_006509f8 + (unsigned int)unaff_retaddr);
        FUN_004db490(DAT_006509fc);
        DAT_006509d4 = (int)((unsigned int)DAT_006509d4 + 1u);
        if (DAT_006509d0 == 0) {
            FUN_004cff00();
        }
    }
    uVar4 = FUN_004dbe40();
    uVar4 = uVar4 & 0xff;
    uVar2 = FUN_004db570();
    if (((uVar2 != 0) ||
         (uVar2 = FUN_004db590(), uVar2 != 0)) || (uVar4 == 0x1b) ||
        ((uVar4 == 0xd) || (uVar4 == 0x20))) {
        DAT_00533f00 = 1;
        FUN_004d5420();
        FUN_004d50a0();
        FUN_004d00f0(0x8000, 0x1000, 2000);
    }
    return 0;
}
