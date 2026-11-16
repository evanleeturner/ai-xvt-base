/* SPDX-License-Identifier: MIT */
/* z_xvt 00427480 */
void *__cdecl FID_conflict___expand(void *, int);

void __cdecl FUN_00427480(int param_1)
{
    unsigned char bVar1;
    char *pcVar2;
    void *pvVar3;
    char *pcVar4;
    char *pcVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int t;
    char *base;

    base = (char *)param_1;
    uVar7 = 0;
    if (param_1 != 0) {
        __builtin_memcpy(&pcVar5, base + 0x1086, 4);
        bVar1 = (unsigned char)pcVar5[3];
        t = 0;
        __builtin_memcpy(base + 0x108e, &t, 4);
        uVar6 = (unsigned int)bVar1 + 1u;
        __builtin_memcpy(base + 0x1092, &uVar6, 4);
        pcVar5[3] = (char)-1;
        if (*(int *)(base + 0x1028) == 0) {
            uVar7 = 0;
            pcVar4 = pcVar5;
            if ((unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030) != 0) {
                do {
                    __builtin_memcpy(&t, base + 0x1092, 4);
                    if (t == 0) {
                        pcVar2 = pcVar4 + 6;
                        uVar6 = (unsigned int)(unsigned char)pcVar4[9];
                    }
                    else {
                        uVar6 = t - 1u;
                        pcVar2 = pcVar4;
                    }
                    __builtin_memcpy(base + 0x1092, &uVar6, 4);
                    if ((((*pcVar5 == *pcVar2) && (pcVar5[1] == pcVar2[1])) && (pcVar5[2] == pcVar2[2])) &&
                        (pcVar5[3] != (char)-1)) {
                        pcVar5[3] = (char)(pcVar5[3] + 1);
                    }
                    else {
                        __builtin_memcpy(&t, base + 0x108e, 4);
                        if (t != 0) {
                            pcVar5 = pcVar5 + 6;
                        }
                        __builtin_memcpy(&t, base + 0x108e, 4);
                        t = t + 1u;
                        __builtin_memcpy(base + 0x108e, &t, 4);
                        __builtin_memcpy(pcVar5, pcVar2, 4);
                        __builtin_memcpy(pcVar5 + 4, pcVar2 + 4, 2);
                        pcVar5[3] = 0;
                    }
                    uVar7 = uVar7 + 1;
                    uVar6 = (unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030);
                    pcVar4 = pcVar2;
                } while (uVar7 <= uVar6 && uVar6 - uVar7 != 0);
            }
        }
        else {
            pcVar4 = pcVar5;
            if ((unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030) != 0) {
                do {
                    __builtin_memcpy(&t, base + 0x1092, 4);
                    if (t == 0) {
                        pcVar2 = pcVar5 + 6;
                        uVar6 = (unsigned int)(unsigned char)pcVar5[9];
                    }
                    else {
                        uVar6 = t - 1u;
                        pcVar2 = pcVar5;
                    }
                    __builtin_memcpy(base + 0x1092, &uVar6, 4);
                    if (((*pcVar4 == *pcVar2) && (pcVar4[1] == pcVar2[1])) &&
                        ((pcVar4[2] == pcVar2[2] &&
                          ((*(short *)(pcVar4 + 4) == *(short *)(pcVar2 + 4) && (pcVar4[3] != (char)-1)))))) {
                        pcVar4[3] = (char)(pcVar4[3] + 1);
                    }
                    else {
                        __builtin_memcpy(&t, base + 0x108e, 4);
                        if (t != 0) {
                            pcVar4 = pcVar4 + 6;
                        }
                        __builtin_memcpy(&t, base + 0x108e, 4);
                        t = t + 1u;
                        __builtin_memcpy(base + 0x108e, &t, 4);
                        __builtin_memcpy(pcVar4, pcVar2, 4);
                        __builtin_memcpy(pcVar4 + 4, pcVar2 + 4, 2);
                        pcVar4[3] = 0;
                    }
                    uVar7 = uVar7 + 1;
                    uVar6 = (unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030);
                    pcVar5 = pcVar2;
                } while (uVar7 <= uVar6 && uVar6 - uVar7 != 0);
            }
        }
        {
            void *buf;
            int cnt;
            __builtin_memcpy(&buf, base + 0x1086, 4);
            __builtin_memcpy(&cnt, base + 0x108e, 4);
            pvVar3 = FID_conflict___expand(buf, (int)((unsigned int)cnt * 6u));
        }
        __builtin_memcpy(base + 0x1086, &pvVar3, 4);
        __builtin_memcpy(&t, base + 0x108e, 4);
        if (*(int *)(base + 0x1024) == 1) {
            if (((unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030) * 3u) >> 2 <= t) {
                *(int *)(base + 0x102c) = 1;
                return;
            }
        }
        else if (((unsigned int)*(int *)(base + 0x1034) * (unsigned int)*(int *)(base + 0x1030)) >> 1 <= t) {
            *(int *)(base + 0x102c) = 1;
        }
    }
    return;
}
