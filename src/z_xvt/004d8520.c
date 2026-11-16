/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8520 */
extern int DAT_00532a34;
extern int DAT_00520278;
extern int DAT_0052027c;
extern int DAT_00520280;
extern int DAT_00520284;
extern unsigned char DAT_0085e6f0;
extern char DAT_0085e7c5;
extern unsigned int DAT_00758824;
extern unsigned char DAT_00757e40[];
extern char s_newpsound_00532acc[];

unsigned int FUN_004cc540(int a, int b, int c, int d, void *buf, unsigned int count, unsigned int flag);
int FUN_004d84f0(int a, int b, int c, int d);
void FUN_004de190(const char *name, int a, int b, int c);
void FUN_004d8900(void);

unsigned int __stdcall FUN_004d8520(void)
{
    int bVar3;
    unsigned int sVar4;
    unsigned int sVar13;
    int iVar5;
    int iVar6;
    int iVar1;
    unsigned int local_60c;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned int uVar11;
    int *piVar9;
    int *piVar15;
    unsigned char *pg;
    unsigned char *pb;
    unsigned char *pk;
    unsigned char *ps;
    unsigned char *pd;
    unsigned char *pz;
    char *pc;
    char *pc2;
    char *src;
    unsigned char *dst;
    int g0, g1, g2, g3;
    int zero;
    unsigned short zw;
    int local_buf[384];
    int *local_5e0;

    local_5e0 = local_buf + 8;
    zero = 0;
    zw = 0;
    bVar3 = 0;
    if (DAT_00532a34 != -1) {
        return 0;
    }
    sVar4 = FUN_004cc540(DAT_00520278, DAT_0052027c, DAT_00520280, DAT_00520284,
                         local_buf, 0x20, (unsigned int)DAT_0085e6f0);
    local_60c = 0;
    if (DAT_00758824 != 0) {
        pg = DAT_00757e40 + 0x20;
        do {
            __builtin_memcpy(&g0, pg + 0, 4);
            __builtin_memcpy(&g1, pg + 4, 4);
            __builtin_memcpy(&g2, pg + 8, 4);
            __builtin_memcpy(&g3, pg + 12, 4);
            iVar5 = FUN_004d84f0(g0, g1, g2, g3);
            if (iVar5 != 0) {
                sVar13 = 0;
                if (sVar4 != 0) {
                    piVar15 = local_5e0;
                    do {
                        iVar6 = FUN_004d84f0(*piVar15, piVar15[1], piVar15[2], piVar15[3]);
                        if ((iVar6 != 0) && (iVar6 == iVar5)) {
                            local_5e0[sVar13 * 0xc] = 0;
                            local_5e0[sVar13 * 0xc + 1] = 0;
                            local_5e0[sVar13 * 0xc + 2] = 0;
                            local_5e0[sVar13 * 0xc + 3] = 0;
                            break;
                        }
                        piVar15 = piVar15 + 0xc;
                        sVar13 = sVar13 + 1;
                    } while (sVar13 < sVar4);
                }
                if (sVar13 == sVar4) {
                    pb = pg - 0x20;
                    for (iVar5 = 0xe; iVar5 != 0; iVar5 = iVar5 + -1) {
                        __builtin_memcpy(pb, &zero, 4);
                        pb = pb + 4;
                    }
                    __builtin_memcpy(pb, &zw, 2);
                    pb[2] = 0;
                }
            }
            pg = pg + 0x3b;
            local_60c = local_60c + 1;
        } while (local_60c < DAT_00758824);
    }
    if (sVar4 != 0) {
        piVar9 = local_5e0;
        do {
            iVar5 = FUN_004d84f0(*piVar9, piVar9[1], piVar9[2], piVar9[3]);
            if (iVar5 != 0) {
                iVar5 = DAT_00758824 * 0x3b;
                uVar7 = 0xffffffff;
                pc = (char *)(piVar9 - 8);
                do {
                    pc2 = pc;
                    if (uVar7 == 0) break;
                    uVar7 = uVar7 - 1;
                    pc2 = pc + 1;
                    iVar6 = *pc;
                    pc = pc2;
                } while ((char)iVar6 != '\0');
                uVar7 = ~uVar7;
                src = pc2 - uVar7;
                dst = DAT_00757e40 + iVar5;
                for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                    __builtin_memcpy(dst, src, 4);
                    src = src + 4;
                    dst = dst + 4;
                }
                for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                    *dst = (unsigned char)*src;
                    src = src + 1;
                    dst = dst + 1;
                }
                iVar6 = piVar9[1];
                iVar1 = piVar9[2];
                bVar3 = 1;
                g0 = piVar9[0];
                g3 = piVar9[3];
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x20, &g0, 4);
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x24, &iVar6, 4);
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x28, &iVar1, 4);
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x2c, &g3, 4);
                g0 = 9;
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x30, &g0, 4);
                __builtin_memcpy(DAT_00757e40 + iVar5 + 0x34, &zero, 4);
                DAT_00757e40[iVar5 + 0x38] = 2;
                DAT_00757e40[iVar5 + 0x39] = 0;
                DAT_00758824 = DAT_00758824 + 1;
            }
            piVar9 = piVar9 + 0xc;
            sVar4 = sVar4 - 1;
        } while (sVar4 != 0);
    }
    if ((bVar3) && (DAT_0085e7c5 != '\0')) {
        FUN_004de190(s_newpsound_00532acc, 1, 0, 0xff);
    }
    uVar7 = 0;
    if (DAT_00758824 != 0) {
        pg = DAT_00757e40 + 0x20;
        do {
            __builtin_memcpy(&g0, pg + 0, 4);
            __builtin_memcpy(&g1, pg + 4, 4);
            __builtin_memcpy(&g2, pg + 8, 4);
            __builtin_memcpy(&g3, pg + 12, 4);
            iVar5 = FUN_004d84f0(g0, g1, g2, g3);
            uVar8 = DAT_00758824;
            if ((iVar5 == 0) && (uVar11 = uVar7 + 1, uVar11 < DAT_00758824)) {
                pk = DAT_00757e40 + 0x20 + uVar11 * 0x3b;
LAB_004d878f:
                __builtin_memcpy(&g0, pk + 0, 4);
                __builtin_memcpy(&g1, pk + 4, 4);
                __builtin_memcpy(&g2, pk + 8, 4);
                __builtin_memcpy(&g3, pk + 12, 4);
                iVar5 = FUN_004d84f0(g0, g1, g2, g3);
                uVar8 = DAT_00758824;
                if (iVar5 == 0) goto code_r0x004d87b0;
                ps = DAT_00757e40 + uVar11 * 0x3b;
                pd = pg - 0x20;
                for (iVar5 = 0xe; iVar5 != 0; iVar5 = iVar5 + -1) {
                    __builtin_memcpy(pd, ps, 4);
                    ps = ps + 4;
                    pd = pd + 4;
                }
                __builtin_memcpy(pd, ps, 2);
                pd[2] = ps[2];
                pz = DAT_00757e40 + uVar11 * 0x3b;
                for (iVar5 = 0xe; iVar5 != 0; iVar5 = iVar5 + -1) {
                    __builtin_memcpy(pz, &zero, 4);
                    pz = pz + 4;
                }
                __builtin_memcpy(pz, &zw, 2);
                pz[2] = 0;
            }
LAB_004d87ea:
            pg = pg + 0x3b;
            uVar7 = uVar7 + 1;
        } while (uVar7 < uVar8);
    }
    DAT_00758824 = 0;
    do {
        iVar5 = DAT_00758824 * 0x3b;
        __builtin_memcpy(&g0, DAT_00757e40 + iVar5 + 0x20, 4);
        __builtin_memcpy(&g1, DAT_00757e40 + iVar5 + 0x24, 4);
        __builtin_memcpy(&g2, DAT_00757e40 + iVar5 + 0x28, 4);
        __builtin_memcpy(&g3, DAT_00757e40 + iVar5 + 0x2c, 4);
        iVar5 = FUN_004d84f0(g0, g1, g2, g3);
        if (iVar5 == 0) break;
        DAT_00758824 = DAT_00758824 + 1;
    } while (DAT_00758824 < 0x20);
    FUN_004d8900();
    return DAT_00758824;
code_r0x004d87b0:
    pk = pk + 0x3b;
    uVar11 = uVar11 + 1;
    if (DAT_00758824 <= uVar11) goto LAB_004d87ea;
    goto LAB_004d878f;
}
