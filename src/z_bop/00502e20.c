/* SPDX-License-Identifier: MIT */
/* z_bop 00502e20 */
/* Prototypes for every callee. Calling conventions and exact types are not
   confirmed by the evidence; the decompiler's signatures are used. */
extern char FUN_004dca90(int a);
extern void FUN_004b5570(char *s, int x, int y);
extern void FUN_004d6460(int *r, int a, int b, int c, int d);
extern char *FUN_004ddc10(int id);
extern void FUN_004db640(int font, char *s, int *r, unsigned int color);
extern void FUN_004d64a0(int *p, int v, int n);
extern int _sprintf(char *buf, const char *fmt, ...);
extern int FUN_004dbfa0(char *s, int font);
extern void FUN_004b4cc0(char *name, int *r);
extern void FUN_004db4e0(int font, char *s, int x, int y, unsigned int color);

/* Strings */
extern char s__c_s___c_s_0052adb0[];          /* "%c%s: %c%s" */
extern char s__s__s____s_0052d504[];          /* "%s %s - %s" */
extern char s_medlvl_d_0052b49c[];            /* "medlvl%d" */
extern char s__c_s__c_d_0052c518[];           /* "%c%s %c%d" */
extern char s__c_s__c_d__c_s__c_d_0052c308[]; /* "%c%s %c%d   %c%s %c%d" */
extern char DAT_0052ada8[];                   /* format used by the final loop (string not in evidence) */
extern char DAT_0052d510[];

/* Globals. Those at addresses not a multiple of 4 (0xb6bba6, 0xb6bbae,
   0xb6bbb2, 0xb6d962) are packed: declared as byte arrays, accessed with
   __builtin_memcpy. */
extern unsigned char DAT_00b6bba6[4];   /* uint, packed */
extern unsigned char DAT_00b6bbae[4];   /* int, packed */
extern unsigned char DAT_00b6bbb2[];    /* int[10], packed; also read at +0x50 per entry */
extern unsigned char DAT_00b6d962[4];   /* int, packed */
extern unsigned char DAT_00b6da1e[];    /* byte elements, stride 0x4278 (width unconfirmed) */
extern char DAT_00a91b90[];
extern char DAT_00b69d20[];
extern int DAT_00b6a2c0;
extern int DAT_00aa6114;

unsigned int __stdcall FUN_00502e20(void)
{
    char bVar1;
    char *pcVar2;
    char *pcVar3;
    char *pcVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    unsigned int tmpU;
    int tmpI;
    int idx;
    int tmpV;
    unsigned char *piVar8;
    /* L[0..3] = local_54 rect, memset spans 0x14 bytes into L[4];
       local_44 = &L[4], local_3c = L[6] */
    int L[7];
    unsigned char local_34[52];

    iVar7 = 0;
    iVar9 = 0;
    bVar1 = FUN_004dca90(0x12);
    if (((bVar1 != 0) &&
         (bVar1 = FUN_004dca90(0x10), bVar1 != 0)) &&
        (bVar1 = FUN_004dca90(0x78), bVar1 != 0)) {
        FUN_004b5570(DAT_0052d510, 0xaf, 0x156);
    }
    FUN_004d6460(&L[0], 0x54, 0x5a, 0x1b2, 0x6a);
    uVar13 = 0xffff;
    pcVar2 = FUN_004ddc10(0x13a);
    FUN_004db640(0xf, pcVar2, &L[0], uVar13);
    FUN_004d6460(&L[0], 0x54, 0x6e, 0x1b2, 0x73);
    FUN_004db640(0xc, DAT_00a91b90, &L[0], (unsigned int)DAT_00b6a2c0);
    FUN_004d64a0(&L[0], 0, 0x14);
    uVar13 = 0;
    do {
        if (9 < uVar13) break;
        __builtin_memcpy(&tmpV, DAT_00b6bbb2 + uVar13 * 4, 4);
        if (tmpV == 0) {
            iVar7 = (int)((unsigned int)iVar7 + 1u);
        } else if (tmpV == 1) {
            iVar9 = (int)((unsigned int)iVar9 + 1u);
        }
        uVar13 = uVar13 + 1;
        __builtin_memcpy(&tmpU, DAT_00b6bba6, 4);
    } while (uVar13 <= tmpU);

    __builtin_memcpy(&tmpI, DAT_00b6bbae, 4);
    if (iVar7 == tmpI) {
        pcVar2 = FUN_004ddc10(0x159);
        uVar11 = 5;
        pcVar3 = FUN_004ddc10(0x2c5);
        _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 4, pcVar3, uVar11, pcVar2);
        FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
        __builtin_memcpy(&idx, DAT_00b6d962, 4);
        if (DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)] != 0) {
            FUN_004d64a0(&L[0], 0, 0xf);
            __builtin_memcpy(&idx, DAT_00b6d962, 4);
            pcVar2 = FUN_004ddc10(DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)] + 0x17d);
            pcVar3 = FUN_004ddc10(0x17d);
            pcVar4 = FUN_004ddc10(0x179);
            _sprintf(DAT_00b69d20, s__s__s____s_0052d504, pcVar4, pcVar3, pcVar2);
            iVar5 = FUN_004dbfa0(DAT_00b69d20, 0xc);
            __builtin_memcpy(&idx, DAT_00b6d962, 4);
            _sprintf((char *)local_34, s_medlvl_d_0052b49c,
                     DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)]);
            FUN_004b4cc0((char *)local_34, &L[4]);
            iVar6 = (int)((((unsigned int)((int)((unsigned int)L[2] - (unsigned int)L[0]) >> 1)) -
                           (((((unsigned int)iVar5 + 10u) - (unsigned int)L[4]) + (unsigned int)L[6] + 0xbU) >> 1)) +
                          (unsigned int)L[0]);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar6, L[1], 0xffff);
            FUN_004b5570((char *)local_34,
                         (int)((unsigned int)iVar6 + (unsigned int)iVar5 + 10u + 10u), L[1]);
        }
    } else if (iVar9 == tmpI) {
        pcVar2 = FUN_004ddc10(0x15a);
        uVar11 = 3;
        pcVar3 = FUN_004ddc10(0x2c5);
        _sprintf(DAT_00b69d20, s__c_s___c_s_0052adb0, 4, pcVar3, uVar11, pcVar2);
        FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
        __builtin_memcpy(&idx, DAT_00b6d962, 4);
        if (DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)] != 0) {
            FUN_004d64a0(&L[0], 0, 0xf);
            __builtin_memcpy(&idx, DAT_00b6d962, 4);
            pcVar2 = FUN_004ddc10(DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)] + 0x17d);
            pcVar3 = FUN_004ddc10(0x17d);
            pcVar4 = FUN_004ddc10(0x179);
            _sprintf(DAT_00b69d20, s__s__s____s_0052d504, pcVar4, pcVar3, pcVar2);
            iVar5 = FUN_004dbfa0(DAT_00b69d20, 0xc);
            __builtin_memcpy(&idx, DAT_00b6d962, 4);
            _sprintf((char *)local_34, s_medlvl_d_0052b49c,
                     DAT_00b6da1e[(int)((unsigned int)idx * 0x4278u)]);
            FUN_004b4cc0((char *)local_34, &L[4]);
            iVar6 = (int)((((unsigned int)((int)((unsigned int)L[2] - (unsigned int)L[0]) >> 1)) -
                           (((((unsigned int)iVar5 + 10u) - (unsigned int)L[4]) + (unsigned int)L[6] + 0xbU) >> 1)) +
                          (unsigned int)L[0]);
            FUN_004db4e0(0xc, DAT_00b69d20, iVar6, L[1], 0xffff);
            FUN_004b5570((char *)local_34,
                         (int)((unsigned int)iVar6 + (unsigned int)iVar5 + 10u + 10u), L[1]);
        }
    } else {
        uVar11 = 1;
        iVar5 = tmpI;
        pcVar2 = FUN_004ddc10(0x155);
        _sprintf(DAT_00b69d20, s__c_s__c_d_0052c518, 4, pcVar2, uVar11, iVar5);
        FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
    }
    FUN_004d64a0(&L[0], 0, 0x14);
    uVar12 = 1;
    pcVar2 = FUN_004ddc10(0x157);
    uVar10 = 3;
    uVar11 = 1;
    pcVar3 = FUN_004ddc10(0x156);
    _sprintf(DAT_00b69d20, s__c_s__c_d__c_s__c_d_0052c308, 5, pcVar3, uVar11, iVar7, uVar10, pcVar2,
             uVar12, iVar9);
    FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
    uVar13 = 0;
    FUN_004d64a0(&L[0], 0, 0x14);
    piVar8 = DAT_00b6bbb2;
    do {
        if (0xb6bbd9u < (unsigned int)piVar8) {
            return 1;
        }
        __builtin_memcpy(&iVar7, piVar8, 4);
        if (iVar7 == 0) {
            local_34[0] = 5;
        } else if (iVar7 == 1) {
            local_34[0] = 3;
        } else if (iVar7 == 2) {
            local_34[0] = 1;
        }
        uVar13 = uVar13 + 1;
        __builtin_memcpy(&tmpV, piVar8 + 0x50, 4);
        _sprintf(DAT_00b69d20, DAT_0052ada8, 4,
                 (int)((unsigned int)DAT_00aa6114 + (unsigned int)tmpV * 0x148u + 0x40u));
        FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
        FUN_004d64a0(&L[0], 0, 0xf);
        __builtin_memcpy(&tmpV, piVar8, 4);
        pcVar2 = FUN_004ddc10((int)((unsigned int)tmpV + 0x159u));
        _sprintf(DAT_00b69d20, DAT_0052ada8, (unsigned int)local_34[0], pcVar2);
        FUN_004db640(0xc, DAT_00b69d20, &L[0], 0xffff);
        FUN_004d64a0(&L[0], 0, 0xf);
        piVar8 = piVar8 + 4;
        __builtin_memcpy(&tmpU, DAT_00b6bba6, 4);
    } while (uVar13 <= tmpU);
    return 1;
}
