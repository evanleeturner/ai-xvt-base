/* SPDX-License-Identifier: MIT */
/* z_bop 004b4b20 */
int __cdecl FUN_004b7a70(char *param_1);
void *__cdecl _malloc(unsigned int size);
int __cdecl FUN_004b6fb0(char *param_1, unsigned int *param_2, int param_3, int param_4);
void __cdecl _free(void *ptr);
char *__cdecl _strncpy(char *dst, const char *src, unsigned int count);
void __cdecl FUN_004b7970(char *param_1);

int __cdecl FUN_004b4b20(char *param_1, char *param_2, int param_3, int param_4)
{
  int iVar1;
  unsigned int *puVar2;
  char local_44[64];
  unsigned int *local_4;

  if (*param_1 == '\0') {
    return 0;
  }
  if (*param_2 == '\0') {
    return 0;
  }
  iVar1 = FUN_004b7a70(param_2);
  if (iVar1 != -1) {
    return 0;
  }
  local_4 = (unsigned int *)_malloc(0x414);
  if (local_4 == (unsigned int *)0x0) {
    return 0;
  }
  puVar2 = local_4;
  for (iVar1 = 0x105; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  iVar1 = FUN_004b6fb0(param_1, local_4, param_3, param_4);
  if (iVar1 == 0) {
    _free(local_4);
    return 0;
  }
  _strncpy(local_44, param_2, 0x40);
  FUN_004b7970(local_44);
  return 1;
}
