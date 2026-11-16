/* SPDX-License-Identifier: MIT */
/* z_xvt 00490a00 */
extern int DAT_006a8810;
extern int DAT_006a86b0;

void FUN_00490be0(char *, int, unsigned int, char *);

void __stdcall FUN_00490a00(void)
{
    int iVar5 = 0;
    if (DAT_006a8810 > 0) {
        unsigned int *puVar6 = (unsigned int *)&DAT_006a86b0;
        do {
            unsigned int *puVar1 = puVar6 + 3;
            unsigned int *puVar2 = puVar6 + 1;
            int *piVar3 = (int *)(puVar6 + 2);
            char *pcVar4 = (char *)(puVar6 - 0x11);
            puVar6 += 0x15;
            iVar5++;
            FUN_00490be0(pcVar4, *piVar3, *puVar2, (char *)*puVar1);
        } while (iVar5 < DAT_006a8810);
    }
    DAT_006a8810 = 0;
    return;
}
