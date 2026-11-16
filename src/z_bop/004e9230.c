/* SPDX-License-Identifier: MIT */
/* z_bop 004e9230 */
extern char *_fgets(char *, int, void *);
extern char DAT_00b69d20[];
extern char DAT_00b69d1e;

int __cdecl FUN_004e9230(void *param_1)
{
  char cVar1;
  char *pcVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  char *pcVar5;
  int local_84;
  char local_80[128];
  
  local_84 = 0;
  do {
    while (1) {
      do {
        pcVar2 = _fgets((char *)&DAT_00b69d20, 0x100, param_1);
        if (pcVar2 == (char *)0x0) {
          return local_84;
        }
      } while ((DAT_00b69d20[0] == '/') && (DAT_00b69d20[1] == '/'));
      if (DAT_00b69d20[0] == '[') break;
      pcVar2 = _fgets((char *)&DAT_00b69d20, 0x100, param_1);
      if (pcVar2 == (char *)0x0) {
        return local_84;
      }
      pcVar2 = _fgets((char *)&DAT_00b69d20, 0x100, param_1);
      if (pcVar2 == (char *)0x0) {
        return local_84;
      }
      local_84 = local_84 + 1;
    }
    uVar3 = 0xffffffff;
    pcVar2 = (char *)&DAT_00b69d20;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    (&DAT_00b69d1e)[~uVar3] = 0;
    uVar3 = 0xffffffff;
    pcVar2 = (char *)((int)&DAT_00b69d20 + 1);
    do {
      pcVar5 = pcVar2;
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      pcVar5 = pcVar2 + 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar5;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar2 = pcVar5 + -uVar3;
    pcVar5 = local_80;
    for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
      *(unsigned int *)pcVar5 = *(unsigned int *)pcVar2;
      pcVar2 = pcVar2 + 4;
      pcVar5 = pcVar5 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar5 = *pcVar2;
      pcVar2 = pcVar2 + 1;
      pcVar5 = pcVar5 + 1;
    }
  } while (1);
}
