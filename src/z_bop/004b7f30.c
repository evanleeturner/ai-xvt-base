/* SPDX-License-Identifier: MIT */
/* z_bop 004b7f30 */
/* Unresolved: return types of void-called callees and exact parameter types are inferred from call sites only. */

void __cdecl FUN_004de0b0(int a, int b);
void __cdecl FUN_004d9d10(void);
void __cdecl FUN_004d9ca0(void);
void __cdecl FUN_004dcb30(void);
void __cdecl FUN_004bd5f0(void);
void __cdecl FUN_004b4a60(const char *a, const char *b);
void __cdecl FUN_004d4e20(void);
void __cdecl FUN_004b5de0(const char *a, int b, int c);
void __cdecl FUN_004b5570(const char *a, int b, int c);
void __cdecl FUN_004d6460(int *r, int a, int b, int c, int d);
void __cdecl FUN_004b4d40(const char *a, int b, int c);
void __cdecl FUN_004d4ec0(int a);
void __cdecl FUN_004dc140(int a);
void __cdecl FUN_004bb680(void);
void __cdecl FUN_004b83a0(void);
void __cdecl FUN_004bc520(void);
void __cdecl FUN_004bcc10(void);
void __cdecl FUN_004bda30(void);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void __cdecl FUN_004db640(int a, char *b, int *c, unsigned int d);
void __cdecl FUN_004baaf0(void);
int __cdecl FUN_004bf9b0(int a);
void __cdecl FUN_004dad40(void);
char * __cdecl FUN_004ddc10(int a);
void __cdecl FUN_004dad70(char *a);
unsigned int __cdecl FUN_004da780(int *r, const char *a, const char *b, char *c, int d, int e, int f, const char *g);
void __cdecl FUN_004dad60(void);
unsigned int __cdecl FUN_004dd0f0(void);
unsigned int __cdecl FUN_004cffb0(void);
void __cdecl FUN_004ba3c0(void);
void __cdecl FUN_004dc7e0(void);
void __cdecl FUN_004dc1d0(void);
void __cdecl FUN_004b4bf0(const char *a);
void __cdecl FUN_004dc170(void);
void __cdecl FUN_004d9d40(void);
int __cdecl _rand(void);
void __cdecl FUN_004cef70(int a, unsigned int *b, int c);

extern int DAT_00664e88;
extern int DAT_00664e8c;
extern int DAT_00664e90;
extern int DAT_00664e94;
extern int DAT_00664e98;
extern int DAT_00664e9c;
extern int DAT_00bb72a0;
extern char DAT_00bb2820[];
extern char DAT_00bb7380[];
extern char DAT_00b6a2e0[];
extern char DAT_00b69d20[];
extern char DAT_00b6bc9e;
extern unsigned int DAT_00b6a2c0;
extern char DAT_00bb73a8;
extern int DAT_00b69e30;
extern int DAT_0052c000;
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6af4;
extern unsigned int DAT_00aa6af8;
extern unsigned int DAT_00aa6afc;
extern unsigned int DAT_00aa6b00;
extern unsigned int DAT_00aa6b04;
extern unsigned int DAT_00aa6b08;
extern unsigned int DAT_00aa6b0c;
extern unsigned int DAT_00aa6b10;
extern unsigned int DAT_00aa6b14;
extern unsigned int DAT_00aa6b18;
extern unsigned int DAT_00aa6b1c;
extern unsigned int DAT_00aa6b20;
extern unsigned int DAT_00aa6b24;
extern unsigned int DAT_00aa6b28;
extern unsigned int DAT_00aa6b2c;
extern unsigned int DAT_00aa6b30;
extern unsigned int DAT_00aa6b34;
extern unsigned int DAT_00aa6b38;
extern unsigned char DAT_00bb7394;
extern unsigned char DAT_00bb7395;
extern unsigned char DAT_00bb7396;
extern unsigned char DAT_00bb7397;
extern unsigned char DAT_00bb7398;
extern unsigned char DAT_00bb7399;
extern unsigned char DAT_00bb739a;
extern unsigned char DAT_00bb739b;
extern unsigned char DAT_00bb739c;
extern unsigned char DAT_00bb739d;
extern unsigned char DAT_00bb739e;
extern unsigned char DAT_00bb739f;
extern unsigned char DAT_00bb736f;
extern unsigned char DAT_00bb73a4;
extern unsigned char DAT_00bb7370;
extern unsigned char DAT_00bb73a5;
extern unsigned char DAT_00bb73a6;

unsigned int __cdecl FUN_004b7f30(unsigned int param_1)
{
    char *pcVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int uVar6;
    int iVar7;
    const char *pcVar8;
    int local_10[4];

    iVar5 = 0;
    if (param_1 == 0) {
        FUN_004de0b0(0x20, 0x7f);
        FUN_004d9d10();
        FUN_004d9ca0();
        DAT_00664e88 = 0;
        DAT_00664e8c = 0;
        DAT_00664e9c = 0;
        DAT_00664e90 = 0;
        DAT_00664e98 = 1;
        FUN_004dcb30();
        FUN_004bd5f0();
        if (0 < DAT_00bb72a0) {
            pcVar2 = DAT_00bb2820;
            do {
                if (*pcVar2 == DAT_00bb7380[DAT_00664e90]) {
                    DAT_00664e94 = iVar5;
                }
                pcVar2 = pcVar2 + 0x95;
                iVar5 = iVar5 + 1;
            } while (iVar5 < DAT_00bb72a0);
        }
        FUN_004b4a60("frontres\\configb.bmp", "backconfig");
        FUN_004d4e20();
        FUN_004b5de0("backconfig", 0, 0);
        FUN_004b5570("frame", 0, 0);
        FUN_004b5570("alloff", 0, 0);
        FUN_004d6460(local_10, 0x54, 0x6b, 0x25c, 0x1b1);
        FUN_004b4d40("configoverlay", 0, 0);
        FUN_004d4ec0(1);
        FUN_004dc140(0x14);
    }
    switch (DAT_00664e9c) {
    case 0:
        FUN_004bb680();
        goto after_switch;
    case 1:
        FUN_004b83a0();
        if (DAT_00664e98 == 0) goto switch_default;
        goto LAB_004b80a4;
    case 2:
        FUN_004b83a0();
        if (DAT_00664e98 == 0) goto switch_default;
        goto LAB_004b80a4;
    case 3:
        FUN_004bc520();
        goto after_switch;
    case 4:
        FUN_004bcc10();
        goto switch_default;
    case 5:
        FUN_004bda30();
        goto after_switch;
    default:
        goto switch_default;
    }
after_switch:
    if (DAT_00664e98 != 0) {
LAB_004b80a4:
        FUN_004d4e20();
        FUN_004d4ec0(1);
    }
switch_default:
    FUN_004d6460(local_10, 200, 0x1c4, 0x1b4, 0x1d0);
    if (DAT_00b6a2e0[0] != '\0') {
        _sprintf(DAT_00b69d20, "%c%s %c%s", 6, &DAT_00b6bc9e, 1, DAT_00b6a2e0);
        FUN_004db640(0xc, DAT_00b69d20, local_10, DAT_00b6a2c0);
        uVar4 = (unsigned int)((int)param_1 >> 0x1f);
        iVar5 = (int)((((param_1 ^ uVar4) - uVar4 & 0x1f) ^ uVar4) - uVar4) >> 1;
        _sprintf(DAT_00b69d20, "rebtiny%d", iVar5);
        FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
        _sprintf(DAT_00b69d20, "imptiny%d", iVar5);
        FUN_004b5570(DAT_00b69d20, 0x1a4, 0x1c5);
    }
    DAT_00664e98 = 0;
    FUN_004baaf0();
    iVar5 = FUN_004bf9b0(2);
    if (iVar5 == 1) {
        return 1;
    }
    FUN_004d6460(local_10, 0x55, 0x1bf, 0xb0, 0x1d7);
    if (DAT_00bb73a8 != '\0') {
        FUN_004dad40();
    }
    pcVar2 = FUN_004ddc10(0xce);
    FUN_004dad70(pcVar2);
    pcVar8 = "buttonsound";
    iVar7 = 8;
    uVar6 = 0;
    iVar5 = 0xc;
    pcVar2 = FUN_004ddc10(0xce);
    uVar4 = FUN_004da780(local_10, "leaveup", "leavedown", pcVar2, iVar5, (int)uVar6, iVar7, pcVar8);
    FUN_004dad60();
    uVar4 = uVar4 | FUN_004dd0f0();
    if (DAT_00b69e30 == 4) {
        uVar3 = FUN_004cffb0();
        uVar4 = uVar4 | uVar3;
    }
    if (uVar4 != 0) {
        FUN_004ba3c0();
        DAT_0052c000 = 0;
        FUN_004dcb30();
        FUN_004dc7e0();
        FUN_004dc1d0();
        FUN_004b4bf0("backconfig");
        FUN_004dc170();
        FUN_004d9d40();
        if (DAT_00b69e30 == 4) {
            DAT_00aa6af0 = 0x59;
            DAT_00aa6af4 = (unsigned int)DAT_00bb7394;
            DAT_00aa6af8 = (unsigned int)DAT_00bb7395;
            DAT_00aa6afc = (unsigned int)DAT_00bb7396;
            DAT_00aa6b00 = (unsigned int)DAT_00bb7397;
            DAT_00aa6b04 = (unsigned int)DAT_00bb7398;
            DAT_00aa6b08 = (unsigned int)DAT_00bb7399;
            DAT_00aa6b0c = (unsigned int)DAT_00bb739a;
            DAT_00aa6b10 = (unsigned int)DAT_00bb739b;
            DAT_00aa6b14 = (unsigned int)DAT_00bb739c;
            DAT_00aa6b18 = (unsigned int)DAT_00bb739d;
            DAT_00aa6b1c = (unsigned int)DAT_00bb739e;
            DAT_00aa6b20 = (unsigned int)DAT_00bb739f;
            DAT_00aa6b24 = (unsigned int)_rand();
            DAT_00aa6b28 = (unsigned int)DAT_00bb736f;
            DAT_00aa6b2c = (unsigned int)DAT_00bb73a4;
            DAT_00aa6b30 = (unsigned int)DAT_00bb7370;
            DAT_00aa6b34 = (unsigned int)DAT_00bb73a5;
            DAT_00aa6b38 = (unsigned int)DAT_00bb73a6;
            FUN_004cef70(0, &DAT_00aa6af0, 0x4c);
        }
    }
    return 0;
}
