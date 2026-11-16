/* SPDX-License-Identifier: MIT */
/* z_bop 004fb670 */
extern int FUN_004dcb30(void);
extern void FUN_004cc590(void *f);
extern void *FUN_004cc4a0(char *name, char *mode);
extern int FUN_004fbaf0(unsigned int *a, int *b, int *c, int *d);
extern void FUN_004dc330(void (*a)(void), void (*b)(void));
extern void FUN_004be1b0(void);
extern void LAB_004be050(void);
extern void FUN_004dc140(int a);
extern unsigned char FUN_004dca90(int a);
extern void FUN_004b5570(char *name, int x, int y);
extern void FUN_004d4e20(void);
extern void FUN_004b5de0(char *name, int x, int y);
extern void FUN_004b4d40(char *name, int x, int y);
extern void FUN_004d4ec0(int a);
extern void FUN_004dc190(void);
extern void FUN_004db4e0(int a, char *s, int x, int y, unsigned int c);
extern void FUN_004dc1b0(void);
extern void FUN_004d3340(void);
extern void FUN_004d4f80(void);
extern void FUN_004d3520(int a, int b, int c);
extern unsigned int FUN_004dcad0(void);
extern unsigned char FUN_004dc220(void);
extern unsigned char FUN_004dc240(void);
extern void FUN_004d4c10(void);

extern char s_credits_txt_0052d198[];
extern char DAT_0052d1a4[];
extern char s_comp01_0052d134[];
extern char s_artists_0052d0e0[];
extern char s_testers_0052d118[];
extern char s_lakota_0052d0fc[];
extern char s_background_0052bf74[];
extern char s_totallylogo_0052d154[];
extern char s_leclogo_0052d178[];

extern void *DAT_006697cc;
extern unsigned short DAT_006697c8;
extern int DAT_0052d0c8;
extern int DAT_006697bc;
extern int DAT_006697f0;
extern int DAT_006697ec;
extern int DAT_006697d0;
extern int DAT_006697b0;
extern int DAT_006697a8;
extern int DAT_006697d4;
extern int DAT_006697b4;
extern int DAT_006697ac;
extern int DAT_006697b8;
extern int DAT_006697d8;
extern int DAT_006697c0;
extern int DAT_006697a0;
extern int DAT_006697dc;
extern int DAT_006697c4;
extern int DAT_006697a4;
extern unsigned int DAT_006697f8;
extern int DAT_006697e0;
extern int DAT_006697e4;
extern int DAT_006697e8;
extern int DAT_006697f4;
extern char DAT_00669800[];
extern char DAT_0066b800[];
extern unsigned short DAT_0066d800[];
extern unsigned short DAT_0066d840[];

unsigned int FUN_004fb670(int param_1)
{
    unsigned char bVar1;
    unsigned char uVar2;
    int iVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned short *puVar6;
    unsigned short *puVar7;
    char *pcVar8;
    int local_4;

    if (param_1 == 0) {
        FUN_004dcb30();
        if (DAT_006697cc != (void *)0) {
            FUN_004cc590(DAT_006697cc);
        }
        DAT_006697c8 = 0xffff;
        DAT_0052d0c8 = 0;
        DAT_006697bc = 0;
        DAT_006697f0 = 0;
        DAT_006697ec = 0;
        DAT_006697d0 = 0;
        DAT_006697b0 = 0;
        DAT_006697a8 = 0;
        DAT_006697d4 = 0;
        DAT_006697b4 = 0;
        DAT_006697ac = 0;
        DAT_006697cc = FUN_004cc4a0(s_credits_txt_0052d198, DAT_0052d1a4);
        DAT_006697b8 = 0;
        DAT_006697d8 = 0;
        DAT_006697c0 = 0;
        DAT_006697a0 = 0;
        DAT_006697dc = 0;
        DAT_006697c4 = 0;
        DAT_006697a4 = 0;
        iVar3 = FUN_004fbaf0(&DAT_006697f8, &DAT_006697b8, &DAT_006697e0, &DAT_006697e4);
        if (iVar3 == 0) {
            FUN_004dc330(FUN_004be1b0, LAB_004be050);
            return 0;
        }
        FUN_004dc140(200);
    }
    if (DAT_0052d0c8 != 0) {
        FUN_004dc330(FUN_004be1b0, LAB_004be050);
        return 0;
    }
    bVar1 = FUN_004dca90(0x10);
    if (((bVar1 != 0) &&
         (bVar1 = FUN_004dca90(0x12), bVar1 != 0)) &&
        (bVar1 = FUN_004dca90(0x7b), bVar1 != 0)) {
        switch (DAT_006697bc) {
        case 0:
            iVar3 = 0x9d;
            pcVar8 = s_comp01_0052d134;
            break;
        case 1:
            iVar3 = 0xa3;
            pcVar8 = s_artists_0052d0e0;
            break;
        case 2:
            iVar3 = 0x98;
            pcVar8 = s_testers_0052d118;
            break;
        default:
            goto switchD_004fb7c3_caseD_3;
        case 4:
            iVar3 = 0x8f;
            pcVar8 = s_lakota_0052d0fc;
        }
        FUN_004b5570(pcVar8, 0xa0, iVar3);
    }
switchD_004fb7c3_caseD_3:
    if ((((DAT_006697d0 == DAT_006697d8) && (DAT_006697a8 == DAT_006697a0)) &&
         ((DAT_006697b0 == DAT_006697c0 &&
           ((DAT_006697d4 == DAT_006697dc && (DAT_006697ac == DAT_006697a4)))))) &&
        (DAT_006697b4 == DAT_006697c4)) goto LAB_004fb948;
    DAT_006697d8 = DAT_006697d0;
    DAT_006697a0 = DAT_006697a8;
    DAT_006697c0 = DAT_006697b0;
    DAT_006697dc = DAT_006697d4;
    DAT_006697a4 = DAT_006697ac;
    DAT_006697c4 = DAT_006697b4;
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    if (DAT_006697d0 == 1) {
        pcVar8 = s_totallylogo_0052d154;
LAB_004fb8f7:
        FUN_004b4d40(pcVar8, DAT_006697a8, DAT_006697b0);
    }
    else if (DAT_006697d0 == 2) {
        pcVar8 = s_leclogo_0052d178;
        goto LAB_004fb8f7;
    }
    if (DAT_006697d4 == 1) {
        pcVar8 = s_totallylogo_0052d154;
LAB_004fb936:
        FUN_004b4d40(pcVar8, DAT_006697ac, DAT_006697b4);
    }
    else if (DAT_006697d4 == 2) {
        pcVar8 = s_leclogo_0052d178;
        goto LAB_004fb936;
    }
    FUN_004d4ec0(1);
LAB_004fb948:
    if ((DAT_006697f8 & 1) != 0) {
        FUN_004dc190();
    }
    iVar3 = DAT_006697f0;
    iVar5 = DAT_006697e8;
    pcVar8 = DAT_00669800;
    puVar6 = DAT_0066d800;
    do {
        puVar7 = puVar6 + 1;
        FUN_004db4e0(0xf, pcVar8, iVar3, iVar5, (unsigned int)*puVar6);
        iVar5 = (int)((unsigned int)iVar5 + 0x13u);
        pcVar8 = pcVar8 + 0x100;
        puVar6 = puVar7;
    } while (puVar7 < DAT_0066d840);
    if ((DAT_006697f8 & 1) == 0) {
        FUN_004dc190();
    }
    else {
        FUN_004dc1b0();
    }
    iVar3 = DAT_006697f4;
    puVar6 = DAT_0066d840;
    pcVar8 = DAT_0066b800;
    iVar5 = DAT_006697ec;
    do {
        puVar7 = puVar6 + 1;
        FUN_004db4e0(0xf, pcVar8, iVar3, iVar5, (unsigned int)*puVar6);
        puVar6 = puVar7;
        pcVar8 = pcVar8 + 0x100;
        iVar5 = (int)((unsigned int)iVar5 + 0x13u);
    } while (puVar7 < DAT_0066d840 + 32);
    FUN_004dc1b0();
    if (DAT_006697b8 == 0) {
        if (DAT_006697e0 <= param_1) {
            DAT_0052d0c8 = 1;
            FUN_004d4f80();
            FUN_004d3520(0x8000, 0x1000, 2000);
        }
    }
    else if (DAT_006697e0 <= param_1) {
        FUN_004fbaf0(&DAT_006697f8, &DAT_006697b8, &local_4, &DAT_006697e4);
        DAT_006697e0 = (int)((unsigned int)DAT_006697e0 + (unsigned int)local_4);
        FUN_004dc140(DAT_006697e4);
        DAT_006697bc = (int)((unsigned int)DAT_006697bc + 1u);
        if (DAT_006697b8 == 0) {
            FUN_004d3340();
        }
    }
    uVar4 = FUN_004dcad0();
    uVar4 = uVar4 & 0xff;
    uVar2 = FUN_004dc220();
    if ((((uVar2 != 0) ||
          (uVar2 = FUN_004dc240(), uVar2 != 0)) || (uVar4 == 0x1b)) ||
        ((uVar4 == 0xd || (uVar4 == 0x20)))) {
        DAT_0052d0c8 = 1;
        FUN_004d4f80();
        FUN_004d4c10();
        FUN_004d3520(0x8000, 0x1000, 2000);
    }
    return 0;
}
