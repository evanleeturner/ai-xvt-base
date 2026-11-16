/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5b20 */
extern int DAT_0081bfb6;
extern int DAT_007579d0[];
extern int DAT_00757680[];
extern int DAT_00757870[];
extern short DAT_0075766c;
extern char DAT_007438e4[];
extern int DAT_00758624;
extern int DAT_00758628;
extern int _DAT_00758620;
extern int DAT_0075862c;

int __cdecl FUN_004cb830(int param_1, unsigned int *param_2, unsigned int param_3);

int __stdcall FUN_004f5b20(void)
{
    int local_8 = 0;
    int count = DAT_007579d0[DAT_0081bfb6];

    if (0 < count) {
        do {
            int iVar3 = DAT_0081bfb6 * 8;
            int iVar2 = local_8 + iVar3;
            int iVar5 = 0;

            if (DAT_00757680[iVar2] == -1 && DAT_00757870[iVar2] != 0 && 0 < DAT_0075766c) {
                char *pcVar6 = DAT_007438e4;

                do {
                    if (*pcVar6 != '\0' && (unsigned char)pcVar6[-0xc] == DAT_0081bfb6) {
                        int iVar4 = 0;
                        int iVar1 = DAT_007579d0[DAT_0081bfb6];

                        if (0 < iVar1) {
                            do {
                                if (DAT_00757680[iVar3 + iVar4] == iVar5) break;
                                iVar4++;
                            } while (iVar4 < iVar1);
                        }

                        DAT_00757680[iVar2] = iVar5;

                        if (iVar4 == iVar1) {
                            DAT_00758624 = DAT_0081bfb6;
                            DAT_00758628 = local_8;
                            _DAT_00758620 = 0x6e;
                            DAT_0075862c = iVar5;
                            FUN_004cb830(0, (unsigned int *)&_DAT_00758620, 0x10);
                            break;
                        }
                    }
                    pcVar6 += 0x562;
                    iVar5++;
                } while (iVar5 < DAT_0075766c);
            }
            local_8++;
        } while (local_8 < count);
    }
    return 1;
}
