/* SPDX-License-Identifier: MIT */
/* z_bop 004d92b0 */
extern int DAT_0052bf50;
extern int DAT_00518278;
extern int DAT_0051827c;
extern int DAT_00518280;
extern int DAT_00518284;
extern unsigned char DAT_00bb72d0;
extern char DAT_00bb73a7;
extern unsigned int DAT_00aa6cf4;
extern unsigned char DAT_00aa62b0;
extern unsigned char DAT_00aa62d0;
extern char s_newpsound_0052bff4[];

unsigned int FUN_004cfc20(int a, int b, int c, int d, void *buf, unsigned int cap, unsigned int flag);
int FUN_004d9280(int a, int b, int c, int d);
void FUN_004de5a0(const char *name, int a, int b, int c);
void FUN_004d9680(void);

unsigned int __stdcall FUN_004d92b0(void)
{
    int bVar3;
    int iVar1;
    int iVar2;
    unsigned int sVar4;
    int iVar5;
    int iVar6;
    unsigned char *pbVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned char *piVar10;
    unsigned char *puVar11;
    int *piVar12;
    unsigned char *pbVar12;
    unsigned int sVar13;
    unsigned char *puVar14;
    unsigned int uVar15;
    unsigned char *pbVar16;
    unsigned int local_608;
    int local_600[0x180];
    int *local_5e0;
    int g[4];
    int iZero;
    short sZero;
    int iTmp;

    local_5e0 = local_600 + 8;
    iZero = 0;
    sZero = 0;
    bVar3 = 0;
    if (DAT_0052bf50 != -1) {
        return 0;
    }
    sVar4 = FUN_004cfc20(DAT_00518278, DAT_0051827c, DAT_00518280, DAT_00518284,
                         local_600, 0x20, (unsigned int)DAT_00bb72d0);
    local_608 = 0;
    if (DAT_00aa6cf4 != 0) {
        piVar10 = &DAT_00aa62d0;
        do {
            __builtin_memcpy(g, piVar10, 16);
            iVar5 = FUN_004d9280(g[0], g[1], g[2], g[3]);
            if (iVar5 != 0) {
                sVar13 = 0;
                if (sVar4 != 0) {
                    piVar12 = local_5e0;
                    do {
                        iVar6 = FUN_004d9280(*piVar12, piVar12[1], piVar12[2], piVar12[3]);
                        if ((iVar6 != 0) && (iVar6 == iVar5)) {
                            local_5e0[sVar13 * 0xc] = 0;
                            local_5e0[sVar13 * 0xc + 1] = 0;
                            local_5e0[sVar13 * 0xc + 2] = 0;
                            local_5e0[sVar13 * 0xc + 3] = 0;
                            break;
                        }
                        piVar12 = piVar12 + 0xc;
                        sVar13 = sVar13 + 1;
                    } while (sVar13 < sVar4);
                }
                if (sVar13 == sVar4) {
                    pbVar12 = piVar10 - 0x20;
                    for (iVar5 = 0xf; iVar5 != 0; iVar5 = iVar5 + -1) {
                        __builtin_memcpy(pbVar12, &iZero, 4);
                        pbVar12 = pbVar12 + 4;
                    }
                    __builtin_memcpy(pbVar12, &sZero, 2);
                }
            }
            piVar10 = piVar10 + 0x3e;
            local_608 = local_608 + 1;
        } while (local_608 < DAT_00aa6cf4);
    }
    if (sVar4 != 0) {
        piVar10 = (unsigned char *)local_5e0;
        do {
            iVar5 = FUN_004d9280(((int *)piVar10)[0], ((int *)piVar10)[1],
                                 ((int *)piVar10)[2], ((int *)piVar10)[3]);
            if (iVar5 != 0) {
                uVar8 = 0xffffffff;
                iVar5 = DAT_00aa6cf4 * 0x3e;
                pbVar12 = piVar10 - 0x20;
                do {
                    pbVar7 = pbVar12;
                    if (uVar8 == 0) break;
                    uVar8 = uVar8 - 1;
                    pbVar7 = pbVar12 + 1;
                    iVar6 = *pbVar12;
                    pbVar12 = pbVar7;
                } while ((char)iVar6 != '\0');
                uVar8 = ~uVar8;
                puVar11 = pbVar7 - uVar8;
                puVar14 = &DAT_00aa62b0 + iVar5;
                for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    __builtin_memcpy(puVar14, puVar11, 4);
                    puVar11 = puVar11 + 4;
                    puVar14 = puVar14 + 4;
                }
                for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                    *puVar14 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar14 = puVar14 + 1;
                }
                iVar6 = ((int *)piVar10)[1];
                iVar1 = ((int *)piVar10)[2];
                iTmp = *(int *)piVar10;
                __builtin_memcpy(&DAT_00aa62d0 + iVar5, &iTmp, 4);
                DAT_00aa6cf4 = DAT_00aa6cf4 + 1;
                iVar2 = ((int *)piVar10)[3];
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 4, &iVar6, 4);
                bVar3 = 1;
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 8, &iVar1, 4);
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 0xc, &iVar2, 4);
                iTmp = 9;
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 0x10, &iTmp, 4);
                iTmp = 0;
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 0x14, &iTmp, 4);
                iTmp = 0x65;
                __builtin_memcpy(&DAT_00aa62d0 + iVar5 + 0x18, &iTmp, 4);
                (&DAT_00aa62d0)[iVar5 + 0x1c] = 0;
            }
            piVar10 = piVar10 + 0x30;
            sVar4 = sVar4 - 1;
        } while (sVar4 != 0);
    }
    if ((bVar3) && (DAT_00bb73a7 != '\0')) {
        FUN_004de5a0(s_newpsound_0052bff4, 1, 0, 0xff);
    }
    uVar8 = 0;
    if (DAT_00aa6cf4 != 0) {
        piVar10 = &DAT_00aa62d0;
        do {
            __builtin_memcpy(g, piVar10, 16);
            iVar5 = FUN_004d9280(g[0], g[1], g[2], g[3]);
            uVar9 = DAT_00aa6cf4;
            if ((iVar5 == 0) && (uVar15 = uVar8 + 1, uVar15 < DAT_00aa6cf4)) {
                pbVar12 = &DAT_00aa62d0 + uVar15 * 0x3e;
LAB_004d950c:
                __builtin_memcpy(g, pbVar12, 16);
                iVar5 = FUN_004d9280(g[0], g[1], g[2], g[3]);
                uVar9 = DAT_00aa6cf4;
                if (iVar5 == 0) goto code_r0x004d952d;
                pbVar7 = &DAT_00aa62b0 + uVar15 * 0x3e;
                puVar11 = pbVar7;
                pbVar16 = piVar10 - 0x20;
                for (iVar5 = 0xf; iVar5 != 0; iVar5 = iVar5 + -1) {
                    __builtin_memcpy(pbVar16, puVar11, 4);
                    puVar11 = puVar11 + 4;
                    pbVar16 = pbVar16 + 4;
                }
                __builtin_memcpy(pbVar16, puVar11, 2);
                for (iVar5 = 0xf; iVar5 != 0; iVar5 = iVar5 + -1) {
                    __builtin_memcpy(pbVar7, &iZero, 4);
                    pbVar7 = pbVar7 + 4;
                }
                __builtin_memcpy(pbVar7, &sZero, 2);
            }
LAB_004d9560:
            piVar10 = piVar10 + 0x3e;
            uVar8 = uVar8 + 1;
        } while (uVar8 < uVar9);
    }
    DAT_00aa6cf4 = 0;
    do {
        iVar5 = DAT_00aa6cf4 * 0x3e;
        __builtin_memcpy(g, &DAT_00aa62d0 + iVar5, 16);
        iVar5 = FUN_004d9280(g[0], g[1], g[2], g[3]);
        if (iVar5 == 0) break;
        DAT_00aa6cf4 = DAT_00aa6cf4 + 1;
    } while (DAT_00aa6cf4 < 0x20);
    FUN_004d9680();
    return DAT_00aa6cf4;
code_r0x004d952d:
    pbVar12 = pbVar12 + 0x3e;
    uVar15 = uVar15 + 1;
    if (DAT_00aa6cf4 <= uVar15) goto LAB_004d9560;
    goto LAB_004d950c;
}
