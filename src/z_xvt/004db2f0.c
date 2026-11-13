/* SPDX-License-Identifier: MIT */
/* z_xvt 004db2f0 */
extern int DAT_0075d846[];

int __cdecl FUN_004db2f0(char *param_1, int param_2)
{
    int iVar3;
    int iVar4;
    int iVar5;
    unsigned char cVar2;

    if (param_1 == 0) {
        return 0;
    }
    if (param_2 >= 0 && param_2 < 0x100) {
        iVar3 = DAT_0075d846[param_2];
        if (iVar3 != 0) {
            iVar4 = 0;
            iVar5 = 0;
            cVar2 = (unsigned char)*param_1;
            while (cVar2 != '\0') {
                if ((unsigned char)param_1[iVar5] > 6) {
                    iVar4 = iVar4 + (unsigned int)(unsigned char)*(unsigned char *)(iVar3 + 0x504 + (unsigned char)param_1[iVar5]) +
                            (unsigned int)(unsigned char)*(unsigned char *)(iVar3 + 0x609);
                }
                iVar5 = iVar5 + 1;
                cVar2 = (unsigned char)param_1[iVar5];
            }
            return iVar4 - (unsigned int)(unsigned char)*(unsigned char *)(iVar3 + 0x609);
        }
        return 0;
    }
    return 0;
}
