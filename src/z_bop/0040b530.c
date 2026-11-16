/* SPDX-License-Identifier: MIT */
/* z_bop 0040b530 */
/* Unresolved evidence:
 * - unaff_EBX / unaff_ESI are incoming caller register values (FUN_004422c0);
 *   they are captured from EBX / ESI with empty inline asm, no value is invented.
 * - uStack_78 is an uninitialised stack slot passed to the vtable+0x80 call;
 *   the empty asm only marks it as defined for the compiler.
 * - The string argument for FUN_004079e0 is the literal at 0x0051a750.
 */

extern int *DAT_00a68748;
extern unsigned int DAT_0066ddc4;
extern unsigned int DAT_005233e0;
extern unsigned int DAT_0066dde0;
extern unsigned int DAT_005233e8;
extern unsigned int DAT_009a8e3c;
extern unsigned int DAT_009a7b60;
extern unsigned short DAT_0052747c;
extern unsigned int DAT_009a8074;
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_009d682c;

void __cdecl FUN_004079e0(const char *param_1, int param_2);

void __stdcall FUN_0040b530(void)
{
    unsigned char bVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char bVar7;
    unsigned int unaff_EBX;
    unsigned int *puVar8;
    unsigned int *puVar9;
    unsigned int unaff_ESI;
    unsigned char *pbVar10;
    unsigned char *pbVar11;
    int *piVar12;
    unsigned int *puVar13;
    unsigned char uVar14;
    unsigned int uStack_80;
    unsigned int uStack_7c;
    unsigned int uStack_78;
    int aiStack_6c[27];

    __asm__ __volatile__("" : "=b"(unaff_EBX), "=S"(unaff_ESI));
    __asm__ __volatile__("" : "=m"(uStack_78));

    piVar12 = aiStack_6c;
    for (iVar4 = 0x1b; iVar4 != 0; iVar4 = iVar4 + -1) {
        *piVar12 = 0;
        piVar12 = piVar12 + 1;
    }
    aiStack_6c[0] = 0x6c;
    do {
        iVar4 = (*(int (__stdcall **)(int *, int, int *, int, int))
                    ((unsigned int)*DAT_00a68748 + 100))(DAT_00a68748, 0, aiStack_6c, 0, 0);
        if (iVar4 == 0) {
            piVar12 = aiStack_6c;
            for (iVar4 = 0x1b; iVar4 != 0; iVar4 = iVar4 + -1) {
                *piVar12 = 0;
                piVar12 = piVar12 + 1;
            }
            aiStack_6c[0] = 0x6c;
            (*(int (__stdcall **)(int *, int *))
                ((unsigned int)*DAT_00a68748 + 0x58))(DAT_00a68748, aiStack_6c);
            uStack_7c = 0;
            puVar8 = (unsigned int *)
                     (unaff_EBX + ((DAT_0066ddc4 - DAT_005233e0) & 0xfffffffeU) + DAT_009a8e3c * 2 +
                      (((DAT_0066dde0 - DAT_005233e8) >> 1) + DAT_009a7b60) * (unsigned int)aiStack_6c[2]);
            pbVar10 = (unsigned char *)((unsigned int)DAT_0052747c + DAT_009a8074);
            if (DAT_009a1fe4 != 0) {
                do {
                    bVar7 = *pbVar10;
                    uStack_80 = 0;
                    pbVar10 = pbVar10 + 1;
                    puVar9 = puVar8;
                    pbVar11 = pbVar10;
                    if (DAT_009d682c != 0) {
                        do {
                            uVar6 = (unsigned int)*pbVar11;
                            pbVar10 = pbVar11 + 1;
                            if (uVar6 == 0) {
                                uVar6 = (unsigned int)*pbVar10;
                                pbVar10 = pbVar11 + 2;
                                if (uVar6 == 0) {
                                    bVar1 = *pbVar10;
                                    pbVar10 = pbVar11 + 3;
                                    uVar6 = (unsigned int)bVar1 + 0x100;
                                }
                                uVar6 = uVar6 + 0xff;
                            }
                            puVar13 = puVar9;
                            if ((char)bVar7 < 0) {
                                uVar14 = (unsigned char)(unaff_ESI >> 0x10);
                                uVar2 = (unsigned short)(((unsigned int)uVar14 << 8) | uVar14);
                                uVar3 = ((unsigned int)uVar2 << 16) | uVar2;
                                for (uVar5 = uVar6 >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *puVar13 = uVar3;
                                    puVar13 = puVar13 + 1;
                                }
                            } else {
                                uVar14 = (unsigned char)(unaff_ESI >> 0x18);
                                uVar2 = (unsigned short)(((unsigned int)uVar14 << 8) | uVar14);
                                uVar3 = ((unsigned int)uVar2 << 16) | uVar2;
                                for (uVar5 = uVar6 >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *puVar13 = uVar3;
                                    puVar13 = puVar13 + 1;
                                }
                            }
                            puVar9 = (unsigned int *)((unsigned int)puVar9 + uVar6 * 2);
                            for (uVar5 = (uVar6 * 2) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(char *)puVar13 = (char)uVar3;
                                puVar13 = (unsigned int *)((unsigned int)puVar13 + 1);
                            }
                            bVar7 = (unsigned char)(0 - bVar7);
                            uStack_80 = uStack_80 + uVar6;
                            pbVar11 = pbVar10;
                        } while (uStack_80 < DAT_009d682c);
                    }
                    puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)aiStack_6c[2]);
                    uStack_7c = uStack_7c + 1;
                } while (uStack_7c < DAT_009a1fe4);
            }
            (*(int (__stdcall **)(int *, unsigned int))
                ((unsigned int)*DAT_00a68748 + 0x80))(DAT_00a68748, uStack_78);
            return;
        }
    } while (iVar4 == -0x7789fde4);
    FUN_004079e0("ERROR!(%x) failed to lock D3D z buffer!\n", iVar4);
    return;
}
