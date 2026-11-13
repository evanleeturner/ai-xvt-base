/* SPDX-License-Identifier: MIT */
/* z_xvt 004832a0 */
extern void FUN_00483410(short, short, int);
extern void FUN_00483540(short, short, int);
extern void FUN_004836a0(void);

extern int DAT_00703c68;
extern int DAT_00703be0;
extern int DAT_00703adc;
extern int DAT_006db700;
extern int DAT_006db6e0;
extern int DAT_006db70c;
extern int DAT_006db7a0;
extern int DAT_006db79c;
extern int DAT_006db7a8;
extern int DAT_007329c4;
extern int DAT_007329c8;
extern int DAT_00732a14;
extern int DAT_00732540;
extern int DAT_00732544;
extern int DAT_00732550;
extern int DAT_00732560;
extern int DAT_00732564;
extern int DAT_00732580;

void __cdecl FUN_004832a0(short param_1, short param_2, short param_3, short param_4, int param_5)
{
    if (param_5 == 0) {
        FUN_00483410(param_2, param_3, 0);
        FUN_00483540(param_1, param_4, 0);
        FUN_004836a0();
        return;
    }

    int base_ptr;
    __builtin_memcpy(&base_ptr, (void *)(param_5 + 0x1f), 4);

    char c_90;
    __builtin_memcpy(&c_90, (void *)(base_ptr + 0x90), 1);
    if (c_90 != '\0') {
        FUN_00483410(param_2, param_3, param_5);
        FUN_00483540(param_1, param_4, param_5);
        FUN_004836a0();
        return;
    }

    short s_91, s_93, s_95, s_97, s_99, s_9b, s_9d, s_9f, s_a1;
    __builtin_memcpy(&s_91, (void *)(base_ptr + 0x91), 2);
    __builtin_memcpy(&s_93, (void *)(base_ptr + 0x93), 2);
    __builtin_memcpy(&s_95, (void *)(base_ptr + 0x95), 2);
    __builtin_memcpy(&s_97, (void *)(base_ptr + 0x97), 2);
    __builtin_memcpy(&s_99, (void *)(base_ptr + 0x99), 2);
    __builtin_memcpy(&s_9b, (void *)(base_ptr + 0x9b), 2);
    __builtin_memcpy(&s_9d, (void *)(base_ptr + 0x9d), 2);
    __builtin_memcpy(&s_9f, (void *)(base_ptr + 0x9f), 2);
    __builtin_memcpy(&s_a1, (void *)(base_ptr + 0xa1), 2);

    DAT_00703c68 = (int)s_91;
    DAT_00703be0 = (int)s_93;
    DAT_00703adc = (int)s_95;
    DAT_006db700 = (int)s_97;
    DAT_006db6e0 = (int)s_99;
    DAT_006db70c = (int)s_9b;
    DAT_006db7a0 = (int)s_9d;
    DAT_006db79c = (int)s_9f;
    DAT_006db7a8 = (int)s_a1;

    DAT_007329c4 = -DAT_00703c68;
    DAT_007329c8 = -DAT_00703be0;
    DAT_00732a14 = -DAT_00703adc;
    DAT_00732540 = DAT_006db6e0;
    DAT_00732544 = DAT_006db70c;
    DAT_00732550 = DAT_006db700;
    DAT_00732560 = DAT_006db79c;
    DAT_00732564 = DAT_006db7a8;
    DAT_00732580 = DAT_006db7a0;

    FUN_004836a0();
    return;
}
