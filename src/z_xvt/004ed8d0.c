/* SPDX-License-Identifier: MIT */
/* z_xvt 004ed8d0 */
extern void *DAT_00757c78;
extern char DAT_00533790[];
extern int __cdecl _fscanf(void *stream, const char *format, ...);
extern char * __cdecl _fgets(char *buffer, int count, void *stream);

unsigned int __cdecl FUN_004ed8d0(char *param_1, char *param_2, char *param_3)
{
  char cVar1;
  int iVar2;
  char *pcVar3;
  unsigned int uVar4;
  unsigned int local_4;
  
  if (DAT_00757c78 == (void *)0x0) {
    return 0;
  }
  iVar2 = _fscanf(DAT_00757c78, DAT_00533790, &local_4);
  if ((iVar2 == 0) || (iVar2 == -1)) {
    *param_1 = '\0';
    return 0xffff;
  }
  pcVar3 = _fgets(param_1, 0x100, DAT_00757c78);
  if (pcVar3 == (char *)0x0) {
LAB_004ed95a:
    *param_1 = '\0';
  }
  else {
    uVar4 = 0xffffffff;
    pcVar3 = param_1;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    if (param_1[~uVar4 - 2] == '\n') {
      uVar4 = 0xffffffff;
      pcVar3 = param_1;
      do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
      } while (cVar1 != '\0');
      param_1[~uVar4 - 2] = '\0';
    }
    if (*param_1 == '.') goto LAB_004ed95a;
  }
  pcVar3 = _fgets(param_2, 0x100, DAT_00757c78);
  if (pcVar3 != (char *)0x0) {
    uVar4 = 0xffffffff;
    pcVar3 = param_2;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    if (param_2[~uVar4 - 2] == '\n') {
      uVar4 = 0xffffffff;
      pcVar3 = param_2;
      do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
      } while (cVar1 != '\0');
      param_2[~uVar4 - 2] = '\0';
    }
    if (*param_2 != '.') goto LAB_004ed9a9;
  }
  *param_2 = '\0';
LAB_004ed9a9:
  pcVar3 = _fgets(param_3, 0x100, DAT_00757c78);
  if (pcVar3 == (char *)0x0) {
    *param_3 = '\0';
    return local_4;
  }
  uVar4 = 0xffffffff;
  pcVar3 = param_3;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  if (param_3[~uVar4 - 2] == '\n') {
    uVar4 = 0xffffffff;
    pcVar3 = param_3;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    param_3[~uVar4 - 2] = '\0';
  }
  if (*param_3 == '.') {
    *param_3 = '\0';
  }
  return local_4;
}
