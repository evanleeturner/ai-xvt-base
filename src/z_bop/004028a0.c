/* SPDX-License-Identifier: MIT */
/* z_bop 004028a0 */
extern unsigned short DAT_009d113c;
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_00a08104;

void __cdecl FUN_00402cb0(unsigned short);
void __cdecl FUN_00402b60(void);

void __stdcall FUN_004028a0(void)
{
    unsigned short uVar2;
    unsigned short uVar4;
    int iVar3;
    int iVar1;
    int local_ptr;
    int local_val;

    uVar2 = DAT_009d113c;
    uVar4 = DAT_00a07cec;
    if ((int)(unsigned int)uVar4 < DAT_009a1fe0) {
        do {
            iVar3 = (unsigned int)uVar4 * 0x23 + DAT_009a1fe8;
            if (*(char *)(iVar3 + 3) != '\0') {
                __builtin_memcpy(&local_ptr, (void *)(iVar3 + 0x1f), 4);
                if (*(char *)local_ptr != '\0') {
                    __builtin_memcpy(&iVar1, (void *)(local_ptr + 0xa9), 4);
                    DAT_00a08104 = iVar1;
                    if (*(char *)(iVar1 + 7) != '\x03' &&
                        (*(char *)(iVar1 + 7) != '\x04' && *(int *)(iVar1 + 0x3c) >= 1)) {
                        __builtin_memcpy(&local_val, (void *)(iVar3 + 0x1b), 4);
                        if (local_val == -1) {
                            FUN_00402cb0(uVar4);
                            DAT_009d113c = *(unsigned short *)(iVar1 + 0x40);
                            FUN_00402b60();
                            *(unsigned short *)(iVar1 + 0x40) = DAT_009d113c;
                        }
                        *(int *)(iVar1 + 0x3c) = *(int *)(iVar1 + 0x3c) + *(int *)(iVar1 + 0x38);
                    }
                }
            }
            uVar4 = uVar4 + 1;
        } while ((int)(unsigned int)uVar4 < DAT_009a1fe0);
    }
    DAT_009d113c = uVar2;
    return;
}
