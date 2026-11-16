/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f140 */
extern unsigned char  DAT_006da9fa;
extern unsigned char  DAT_00525508;
extern unsigned char  DAT_0052ad80[];   /* table at 0x0052ad80 */
extern unsigned short DAT_006ffbf0;
extern short          DAT_0070acc0;
extern short          DAT_00732e58;
extern unsigned char  DAT_005263b8[];
extern void (__cdecl *DAT_006d9984)(unsigned int);

/* original pushes 4 bytes (the pointer) and the caller cleans them */
unsigned long long __cdecl FUN_004b4060(const unsigned char *param_1);

void __thiscall FUN_0048f140(void *this, unsigned char *param_1)
{
    unsigned char bVar1;
    unsigned short uVar2;
    unsigned int uThis;
    unsigned char *pbVar6;
    unsigned long long uVar7;
    unsigned long long uVar8;
    unsigned char local_50[80];

    uThis = (unsigned int)this;
    bVar1 = *param_1;
    do {
        if (bVar1 == 0) {
            return;
        }
        bVar1 = *param_1;
        uThis = (uThis & 0xffffff00u) | (unsigned int)bVar1;
        if (bVar1 == 0xfe) {
            param_1 = param_1 + 1;
            DAT_006da9fa = *param_1;
            if (0x3f < (unsigned int)DAT_006da9fa) {
                uThis = (unsigned int)DAT_00525508;
                if (uThis != (unsigned int)DAT_006da9fa) {
                    DAT_006da9fa = DAT_0052ad80[(unsigned int)DAT_006da9fa];
                }
            }
        }
        else if (bVar1 < 0x10) {
            DAT_006da9fa = bVar1;
            if (0x3f < (unsigned int)bVar1) {
                uThis = (unsigned int)DAT_00525508;
                if (uThis != (unsigned int)bVar1) {
                    DAT_006da9fa = DAT_0052ad80[(unsigned int)bVar1];
                }
            }
        }
        else {
            if ((bVar1 == 0x20) && (DAT_006ffbf0 != 0)) {
                bVar1 = param_1[1];
                pbVar6 = param_1 + 1;
                uVar2 = 0;
                while (bVar1 != 0x20) {
                    bVar1 = *pbVar6;
                    if (bVar1 == 0) break;
                    pbVar6 = pbVar6 + 1;
                    local_50[uVar2] = bVar1;
                    bVar1 = *pbVar6;
                    uVar2 = uVar2 + 1;
                }
                local_50[uVar2] = 0;
                uVar7 = FUN_004b4060(DAT_005263b8);
                uVar8 = FUN_004b4060(local_50);
                if ((int)DAT_0070acc0 + -2 <
                    (int)((((unsigned int)uVar7 & 0xffff) + ((unsigned int)uVar8 & 0xffff)) + (unsigned int)(int)DAT_00732e58)) {
                    uThis = 0xa;
                }
                else {
                    uThis = (unsigned int)*param_1;
                }
            }
            (*DAT_006d9984)(uThis);
        }
        bVar1 = param_1[1];
        param_1 = param_1 + 1;
    } while (1);
}
