/* SPDX-License-Identifier: MIT */
/* z_xvt 00429230 */
extern unsigned short DAT_00703abc;
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732e54;

int __cdecl FUN_0042b150(int);
void FUN_004296b0(unsigned short);
void FUN_00429570(void);

void __stdcall FUN_00429230(void)
{
    unsigned short uVar2;
    unsigned short uVar4;
    int iVar3;
    char *pcVar1;
    int tmp_int;
    unsigned short tmp_short;

    uVar2 = DAT_00703abc;
    uVar4 = DAT_00732a3c;
    if ((int)(unsigned int)uVar4 < DAT_006d4980) {
        do {
            iVar3 = DAT_006d4988 + (unsigned int)uVar4 * 0x23;
            if (*(char *)(iVar3 + 3) != '\0') {
                __builtin_memcpy(&pcVar1, (void *)(iVar3 + 0x1f), 4);
                if (*pcVar1 == '\0') {
                    DAT_00732e54 = *(int *)(pcVar1 + 0xa7);
                    iVar3 = FUN_0042b150(DAT_00732e54);
                    if (*(char *)(DAT_00732e54 + 7) != '\x03' &&
                        *(char *)(DAT_00732e54 + 7) != '\x04') {
                        __builtin_memcpy(&tmp_int, (void *)(iVar3 + 0x12), 4);
                        if (tmp_int < 1) {
                            __builtin_memcpy(&tmp_int, (void *)(DAT_006d4988 + 0x1b + (unsigned int)uVar4 * 0x23), 4);
                            if (tmp_int == -1) {
                                FUN_004296b0(uVar4);
                                __builtin_memcpy(&tmp_short, (void *)(iVar3 + 0x16), 2);
                                DAT_00703abc = tmp_short;
                                FUN_00429570();
                                __builtin_memcpy((void *)(iVar3 + 0x16), &DAT_00703abc, 2);
                            }
                            __builtin_memcpy(&tmp_int, (void *)(iVar3 + 0x12), 4);
                            {
                                int tmp_int2;
                                __builtin_memcpy(&tmp_int2, (void *)(iVar3 + 0xe), 4);
                                tmp_int = tmp_int + tmp_int2;
                                __builtin_memcpy((void *)(iVar3 + 0x12), &tmp_int, 4);
                            }
                        }
                    }
                }
            }
            uVar4 = uVar4 + 1;
        } while ((int)(unsigned int)uVar4 < DAT_006d4980);
    }
    DAT_00703abc = uVar2;
    return;
}
