/* SPDX-License-Identifier: MIT */
/* z_bop 0046adf0 */
size_t _fread(void *, size_t, size_t, HFILE);

int FUN_0046adf0(char *param_1, HFILE param_2) {
    size_t sVar1;
    int iVar2;
    char local_1;

    *param_1 = '\0';
    while (1) {
        do {
            sVar1 = _fread(&local_1, 1, 1, param_2);
            if (sVar1 != 1) {
                return 1;
            }
        } while (local_1 == ' ' || local_1 == '\t' || local_1 == '\n' || local_1 == ',' || local_1 == '\n');
        if (local_1 != ';') {
            break;
        }
        do {
            sVar1 = _fread(&local_1, 1, 1, param_2);
            if (sVar1 != 1) {
                return 1;
            }
        } while (local_1 != '\n');
    }
    iVar2 = 1;
    *param_1 = local_1;
    while ((sVar1 = _fread(&local_1, 1, 1, param_2), sVar1 == 1 && local_1 != ',' && local_1 != '\n' && local_1 != ' ' && local_1 != '\t' && local_1 != '\n')) {
        param_1[iVar2] = local_1;
        iVar2++;
    }
    param_1[iVar2] = '\0';
    return 1;
}
