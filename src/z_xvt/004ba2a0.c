/* SPDX-License-Identifier: MIT */
/* z_xvt 004ba2a0 */
extern unsigned char DAT_0085e6d0[0xf7];
extern char DAT_0081b740[0x100];
extern char DAT_0052a330[];
extern char *PTR_s_lastpilot_00531160[];
extern char s_config_cfg_005313c4[];

int FUN_004df3a0(int a);
void *FUN_004be160(const char *name, const char *mode);
void FUN_004be1f0(void *f);
char *_fgets(char *buf, int n, void *f);
int _atoi(const char *s);

/* Offsets into DAT_0085e6d0 (base 0x85e6d0) */

void __stdcall FUN_004ba2a0(void)
{
    char **ppuVar1;
    char cVar2;
    unsigned char bVar3;
    void *_File;
    char *pcVar5;
    char **ppuVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned char *pbVar9;
    int iVar10;
    int iVar11;
    char *pcVar12;
    unsigned char *pbVar13;
    unsigned char *puVar14;
    int bVar15;
    int n;

    puVar14 = &DAT_0085e6d0[0];
    for (iVar7 = 0x3d; iVar7 != 0; iVar7 = iVar7 + -1) {
        puVar14[0] = 0;
        puVar14[1] = 0;
        puVar14[2] = 0;
        puVar14[3] = 0;
        puVar14 = puVar14 + 4;
    }
    puVar14[0] = 0;
    puVar14[1] = 0;
    puVar14[2] = 0;
    iVar7 = 0;
    do {
        DAT_0085e6d0[0x00 + iVar7] = 1;
        iVar11 = iVar7 + 1;
        DAT_0085e6d0[0x02 + iVar7] = 2;
        DAT_0085e6d0[0x04 + iVar7] = 1;
        DAT_0085e6d0[0x06 + iVar7] = 1;
        DAT_0085e6d0[0x08 + iVar7] = 1;
        DAT_0085e6d0[0x0a + iVar7] = 1;
        DAT_0085e6d0[0x0c + iVar7] = 1;
        DAT_0085e6d0[0x0e + iVar7] = 2;
        DAT_0085e6d0[0x10 + iVar7] = 10;
        DAT_0085e6d0[0x12 + iVar7] = 10;
        DAT_0085e6d0[0x14 + iVar7] = 2;
        DAT_0085e6d0[0x16 + iVar7] = 2;
        DAT_0085e6d0[0x18 + iVar7] = 0;
        DAT_0085e6d0[0x1a + iVar7] = 2;
        DAT_0085e6d0[0x1c + iVar7] = 1;
        DAT_0085e6d0[0x1e + iVar7] = 1;
        iVar7 = iVar11;
    } while (iVar11 < 2);
    iVar7 = 0;
    DAT_0085e6d0[0x20] = 0;     /* 6f0 */
    DAT_0085e6d0[0xc1] = 1;     /* 791 */
    DAT_0085e6d0[0xc2] = 1;     /* 792 */
    DAT_0085e6d0[0xc3] = 1;     /* 793 */
    DAT_0085e6d0[0xf5] = 1;     /* 7c5 */
    DAT_0085e6d0[0xc4] = 2;     /* 794 */
    DAT_0085e6d0[0xc5] = 2;     /* 795 */
    DAT_0085e6d0[0xc6] = 1;     /* 796 */
    DAT_0085e6d0[0xc7] = 1;     /* 797 */
    DAT_0085e6d0[0xc8] = 1;     /* 798 */
    DAT_0085e6d0[0xcf] = 1;     /* 79f */
    DAT_0085e6d0[0xc9] = 9;     /* 799 */
    DAT_0085e6d0[0xca] = 9;
    DAT_0085e6d0[0xcb] = 9;
    DAT_0085e6d0[0xcc] = 9;
    DAT_0085e6d0[0xcd] = 9;     /* 79d */
    DAT_0085e6d0[0xe4] = 1;     /* 7b4 */
    DAT_0085e6d0[0xe5] = 1;
    DAT_0085e6d0[0xe6] = 1;
    DAT_0085e6d0[0xe7] = 0;
    DAT_0085e6d0[0xe8] = 1;
    DAT_0085e6d0[0xe9] = 0;
    DAT_0085e6d0[0xea] = 0;
    DAT_0085e6d0[0xeb] = 1;
    DAT_0085e6d0[0xec] = 1;
    DAT_0085e6d0[0xed] = 1;     /* 7bd */
    DAT_0085e6d0[0xef] = 1;     /* 7bf */
    DAT_0085e6d0[0xf0] = 0;     /* 7c0 (int) */
    DAT_0085e6d0[0xf1] = 0;
    DAT_0085e6d0[0xf2] = 0;
    DAT_0085e6d0[0xf3] = 0;
    DAT_0085e6d0[0xbf] = 0;     /* 78f */
    DAT_0085e6d0[0xf4] = 0;     /* 7c4 */
    DAT_0085e6d0[0xf6] = 1;     /* 7c6 */
    DAT_0085e6d0[0xce] = 5;     /* 79e */
    DAT_0085e6d0[0xee] = 0xff;  /* 7be */
    DAT_0085e6d0[0xc0] = 8;     /* 790 */

    n = FUN_004df3a0(0);
    if (n != 0 && -1 < (n >> 8)) {
        do {
            if (0xf < iVar7) break;
            switch (iVar7) {
            case 0: DAT_0085e6d0[0xd0 + iVar7] = 0x98; break;
            case 1: DAT_0085e6d0[0xd0 + iVar7] = 0x99; break;
            case 2: DAT_0085e6d0[0xd0 + iVar7] = 0x72; break;
            case 3: DAT_0085e6d0[0xd0 + iVar7] = 0x2e; break;
            case 4: DAT_0085e6d0[0xd0 + iVar7] = 0x65; break;
            case 5: DAT_0085e6d0[0xd0 + iVar7] = 0x69; break;
            case 6: DAT_0085e6d0[0xd0 + iVar7] = 0x5b; break;
            case 7: DAT_0085e6d0[0xd0 + iVar7] = 8; break;
            case 8: DAT_0085e6d0[0xd0 + iVar7] = 0xd; break;
            case 9: DAT_0085e6d0[0xd0 + iVar7] = 0x5d; break;
            }
            iVar7 = iVar7 + 1;
        } while (iVar7 < n);
    }

    _File = FUN_004be160(s_config_cfg_005313c4, DAT_0052a330);
    if (_File != 0) {
        while (_fgets(DAT_0081b740, 0x100, _File) != 0) {
            uVar8 = 0xffffffff;
            pcVar5 = DAT_0081b740;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar2 = *pcVar5;
                pcVar5 = pcVar5 + 1;
            } while (cVar2 != '\0');
            if (DAT_0081b740[(int)(~uVar8) - 2] == '\n') {
                uVar8 = 0xffffffff;
                pcVar5 = DAT_0081b740;
                do {
                    if (uVar8 == 0) break;
                    uVar8 = uVar8 - 1;
                    cVar2 = *pcVar5;
                    pcVar5 = pcVar5 + 1;
                } while (cVar2 != '\0');
                DAT_0081b740[(int)(~uVar8) - 2] = 0;
            }
            pcVar5 = 0;
            iVar7 = 0;
            uVar8 = 0xffffffff;
            pcVar12 = DAT_0081b740;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar2 = *pcVar12;
                pcVar12 = pcVar12 + 1;
            } while (cVar2 != '\0');
            if (0 < (int)(~uVar8 - 1)) {
                do {
                    if (DAT_0081b740[iVar7] == ' ') {
                        DAT_0081b740[iVar7] = 0;
                        pcVar5 = &DAT_0081b740[iVar7 + 1];
                        break;
                    }
                    iVar7 = iVar7 + 1;
                    uVar8 = 0xffffffff;
                    pcVar12 = DAT_0081b740;
                    do {
                        if (uVar8 == 0) break;
                        uVar8 = uVar8 - 1;
                        cVar2 = *pcVar12;
                        pcVar12 = pcVar12 + 1;
                    } while (cVar2 != '\0');
                } while (iVar7 < (int)(~uVar8 - 1));
            }
            if (pcVar5 != 0) {
                iVar11 = 0;
                iVar7 = -1;
                if (*PTR_s_lastpilot_00531160[0] != '\0') {
                    ppuVar6 = PTR_s_lastpilot_00531160;
                    do {
                        pbVar9 = (unsigned char *)*ppuVar6;
                        pbVar13 = (unsigned char *)DAT_0081b740;
                        do {
                            bVar3 = *pbVar9;
                            bVar15 = bVar3 < *pbVar13;
                            if (bVar3 != *pbVar13) {
LAB_004ba58c:
                                iVar10 = (1 - (unsigned int)bVar15) - (unsigned int)(bVar15 != 0);
                                goto LAB_004ba591;
                            }
                            if (bVar3 == 0) break;
                            bVar3 = pbVar9[1];
                            bVar15 = bVar3 < pbVar13[1];
                            if (bVar3 != pbVar13[1]) goto LAB_004ba58c;
                            pbVar9 = pbVar9 + 2;
                            pbVar13 = pbVar13 + 2;
                        } while (bVar3 != 0);
                        iVar10 = 0;
LAB_004ba591:
                        iVar7 = iVar11;
                        if (iVar10 == 0) break;
                        ppuVar1 = ppuVar6 + 1;
                        ppuVar6 = ppuVar6 + 1;
                        iVar11 = iVar11 + 1;
                        iVar7 = -1;
                    } while (**ppuVar1 != '\0');
                }
                switch (iVar7) {
                case 0:
                    DAT_0085e6d0[0xa1] = (unsigned char)pcVar5[0];
                    DAT_0085e6d0[0xa2] = (unsigned char)pcVar5[1];
                    DAT_0085e6d0[0xa3] = (unsigned char)pcVar5[2];
                    DAT_0085e6d0[0xa4] = (unsigned char)pcVar5[3];
                    DAT_0085e6d0[0xa5] = (unsigned char)pcVar5[4];
                    DAT_0085e6d0[0xa6] = (unsigned char)pcVar5[5];
                    DAT_0085e6d0[0xa7] = (unsigned char)pcVar5[6];
                    DAT_0085e6d0[0xa8] = (unsigned char)pcVar5[7];
                    DAT_0085e6d0[0xa9] = (unsigned char)pcVar5[8];
                    DAT_0085e6d0[0xaa] = (unsigned char)pcVar5[9];
                    DAT_0085e6d0[0xab] = (unsigned char)pcVar5[10];
                    DAT_0085e6d0[0xac] = (unsigned char)pcVar5[11];
                    DAT_0085e6d0[0xad] = (unsigned char)pcVar5[0xc];
                    break;
                case 1:  DAT_0085e6d0[0x00] = (unsigned char)_atoi(pcVar5); break;
                case 2:  DAT_0085e6d0[0x02] = (unsigned char)_atoi(pcVar5); break;
                case 3:  DAT_0085e6d0[0x04] = (unsigned char)_atoi(pcVar5); break;
                case 4:  DAT_0085e6d0[0x06] = (unsigned char)_atoi(pcVar5); break;
                case 5:  DAT_0085e6d0[0x08] = (unsigned char)_atoi(pcVar5); break;
                case 6:  DAT_0085e6d0[0x0a] = (unsigned char)_atoi(pcVar5); break;
                case 7:  DAT_0085e6d0[0x0c] = (unsigned char)_atoi(pcVar5); break;
                case 8:  DAT_0085e6d0[0x0e] = (unsigned char)_atoi(pcVar5); break;
                case 9:  DAT_0085e6d0[0x10] = (unsigned char)_atoi(pcVar5); break;
                case 10: DAT_0085e6d0[0x12] = (unsigned char)_atoi(pcVar5); break;
                case 0xb: DAT_0085e6d0[0x14] = (unsigned char)_atoi(pcVar5); break;
                case 0xc: DAT_0085e6d0[0x16] = (unsigned char)_atoi(pcVar5); break;
                case 0xd: DAT_0085e6d0[0x18] = (unsigned char)_atoi(pcVar5); break;
                case 0xe: DAT_0085e6d0[0x1a] = (unsigned char)_atoi(pcVar5); break;
                case 0xf: DAT_0085e6d0[0x01] = (unsigned char)_atoi(pcVar5); break;
                case 0x10: DAT_0085e6d0[0x03] = (unsigned char)_atoi(pcVar5); break;
                case 0x11: DAT_0085e6d0[0x05] = (unsigned char)_atoi(pcVar5); break;
                case 0x12: DAT_0085e6d0[0x07] = (unsigned char)_atoi(pcVar5); break;
                case 0x13: DAT_0085e6d0[0x09] = (unsigned char)_atoi(pcVar5); break;
                case 0x14: DAT_0085e6d0[0x0b] = (unsigned char)_atoi(pcVar5); break;
                case 0x15: DAT_0085e6d0[0x0d] = (unsigned char)_atoi(pcVar5); break;
                case 0x16: DAT_0085e6d0[0x0f] = (unsigned char)_atoi(pcVar5); break;
                case 0x17: DAT_0085e6d0[0x11] = (unsigned char)_atoi(pcVar5); break;
                case 0x18: DAT_0085e6d0[0x13] = (unsigned char)_atoi(pcVar5); break;
                case 0x19: DAT_0085e6d0[0x15] = (unsigned char)_atoi(pcVar5); break;
                case 0x1a: DAT_0085e6d0[0x17] = (unsigned char)_atoi(pcVar5); break;
                case 0x1b: DAT_0085e6d0[0x19] = (unsigned char)_atoi(pcVar5); break;
                case 0x1c: DAT_0085e6d0[0x1b] = (unsigned char)_atoi(pcVar5); break;
                case 0x1d: DAT_0085e6d0[0x20] = (unsigned char)_atoi(pcVar5); break;
                case 0x1e:
                    puVar14 = &DAT_0085e6d0[0x21];
                    for (iVar7 = 0x10; iVar7 != 0; iVar7 = iVar7 + -1) {
                        puVar14[0] = (unsigned char)pcVar5[0];
                        puVar14[1] = (unsigned char)pcVar5[1];
                        puVar14[2] = (unsigned char)pcVar5[2];
                        puVar14[3] = (unsigned char)pcVar5[3];
                        pcVar5 = pcVar5 + 4;
                        puVar14 = puVar14 + 4;
                    }
                    break;
                case 0x1f:
                    puVar14 = &DAT_0085e6d0[0x61];
                    for (iVar7 = 0x10; iVar7 != 0; iVar7 = iVar7 + -1) {
                        puVar14[0] = (unsigned char)pcVar5[0];
                        puVar14[1] = (unsigned char)pcVar5[1];
                        puVar14[2] = (unsigned char)pcVar5[2];
                        puVar14[3] = (unsigned char)pcVar5[3];
                        pcVar5 = pcVar5 + 4;
                        puVar14 = puVar14 + 4;
                    }
                    break;
                case 0x20: DAT_0085e6d0[0xc1] = (unsigned char)_atoi(pcVar5); break;
                case 0x21: DAT_0085e6d0[0xc2] = (unsigned char)_atoi(pcVar5); break;
                case 0x22: DAT_0085e6d0[0xc3] = (unsigned char)_atoi(pcVar5); break;
                case 0x23: DAT_0085e6d0[0xf5] = (unsigned char)_atoi(pcVar5); break;
                case 0x24: DAT_0085e6d0[0xc4] = (unsigned char)_atoi(pcVar5); break;
                case 0x25: DAT_0085e6d0[0xc5] = (unsigned char)_atoi(pcVar5); break;
                case 0x26: DAT_0085e6d0[0xc6] = (unsigned char)_atoi(pcVar5); break;
                case 0x27: DAT_0085e6d0[0xc7] = (unsigned char)_atoi(pcVar5); break;
                case 0x28: DAT_0085e6d0[0xc8] = (unsigned char)_atoi(pcVar5); break;
                case 0x29: DAT_0085e6d0[0xc9] = (unsigned char)_atoi(pcVar5); break;
                case 0x2a: DAT_0085e6d0[0xca] = (unsigned char)_atoi(pcVar5); break;
                case 0x2b: DAT_0085e6d0[0xcb] = (unsigned char)_atoi(pcVar5); break;
                case 0x2c: DAT_0085e6d0[0xcc] = (unsigned char)_atoi(pcVar5); break;
                case 0x2d: DAT_0085e6d0[0xcd] = (unsigned char)_atoi(pcVar5); break;
                case 0x2e: DAT_0085e6d0[0xce] = (unsigned char)_atoi(pcVar5); break;
                case 0x2f: case 0x30: case 0x31: case 0x32: case 0x33:
                case 0x34: case 0x35: case 0x36: case 0x37: case 0x38:
                case 0x39: case 0x3a: case 0x3b: case 0x3c: case 0x3d:
                case 0x3e: case 0x3f: case 0x40: case 0x41: case 0x42:
                    iVar11 = _atoi(pcVar5);
                    DAT_0085e6d0[0xa1 + iVar7] = (unsigned char)iVar11;
                    break;
                case 0x4f: DAT_0085e6d0[0xe4] = (unsigned char)_atoi(pcVar5); break;
                case 0x50: DAT_0085e6d0[0xe5] = (unsigned char)_atoi(pcVar5); break;
                case 0x51: DAT_0085e6d0[0xe6] = (unsigned char)_atoi(pcVar5); break;
                case 0x52: DAT_0085e6d0[0xe7] = (unsigned char)_atoi(pcVar5); break;
                case 0x53: DAT_0085e6d0[0xe8] = (unsigned char)_atoi(pcVar5); break;
                case 0x54: DAT_0085e6d0[0xe9] = (unsigned char)_atoi(pcVar5); break;
                case 0x55: DAT_0085e6d0[0xea] = (unsigned char)_atoi(pcVar5); break;
                case 0x56: DAT_0085e6d0[0xeb] = (unsigned char)_atoi(pcVar5); break;
                case 0x57: DAT_0085e6d0[0xec] = (unsigned char)_atoi(pcVar5); break;
                case 0x58: DAT_0085e6d0[0xed] = (unsigned char)_atoi(pcVar5); break;
                case 0x59: DAT_0085e6d0[0xee] = (unsigned char)_atoi(pcVar5); break;
                case 0x5a: DAT_0085e6d0[0xef] = (unsigned char)_atoi(pcVar5); break;
                case 0x5b:
                    iVar7 = _atoi(pcVar5);
                    DAT_0085e6d0[0xf0] = (unsigned char)iVar7;
                    DAT_0085e6d0[0xf1] = (unsigned char)((unsigned int)iVar7 >> 8);
                    DAT_0085e6d0[0xf2] = (unsigned char)((unsigned int)iVar7 >> 16);
                    DAT_0085e6d0[0xf3] = (unsigned char)((unsigned int)iVar7 >> 24);
                    break;
                case 0x5c:
                    DAT_0085e6d0[0xaf] = (unsigned char)pcVar5[0];
                    DAT_0085e6d0[0xb0] = (unsigned char)pcVar5[1];
                    DAT_0085e6d0[0xb1] = (unsigned char)pcVar5[2];
                    DAT_0085e6d0[0xb2] = (unsigned char)pcVar5[3];
                    DAT_0085e6d0[0xb3] = (unsigned char)pcVar5[4];
                    DAT_0085e6d0[0xb4] = (unsigned char)pcVar5[5];
                    DAT_0085e6d0[0xb5] = (unsigned char)pcVar5[6];
                    DAT_0085e6d0[0xb6] = (unsigned char)pcVar5[7];
                    DAT_0085e6d0[0xb7] = (unsigned char)pcVar5[8];
                    DAT_0085e6d0[0xb8] = (unsigned char)pcVar5[9];
                    DAT_0085e6d0[0xb9] = (unsigned char)pcVar5[10];
                    DAT_0085e6d0[0xba] = (unsigned char)pcVar5[11];
                    DAT_0085e6d0[0xbb] = (unsigned char)pcVar5[12];
                    DAT_0085e6d0[0xbc] = (unsigned char)pcVar5[13];
                    DAT_0085e6d0[0xbd] = (unsigned char)pcVar5[14];
                    DAT_0085e6d0[0xbe] = (unsigned char)pcVar5[15];
                    break;
                case 0x5d: DAT_0085e6d0[0xbf] = (unsigned char)_atoi(pcVar5); break;
                case 0x5e: DAT_0085e6d0[0xf4] = (unsigned char)_atoi(pcVar5); break;
                case 0x5f: DAT_0085e6d0[0xf6] = (unsigned char)_atoi(pcVar5); break;
                case 0x60: DAT_0085e6d0[0xcf] = (unsigned char)_atoi(pcVar5); break;
                case 0x61: DAT_0085e6d0[0xc0] = (unsigned char)_atoi(pcVar5); break;
                case 0x62: DAT_0085e6d0[0x1c] = (unsigned char)_atoi(pcVar5); break;
                case 99:   DAT_0085e6d0[0x1e] = (unsigned char)_atoi(pcVar5); break;
                case 100:  DAT_0085e6d0[0x1d] = (unsigned char)_atoi(pcVar5); break;
                case 0x65: DAT_0085e6d0[0x1f] = (unsigned char)_atoi(pcVar5); break;
                }
            }
        }
        FUN_004be1f0(_File);
    }
    return;
}
