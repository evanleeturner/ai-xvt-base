/* SPDX-License-Identifier: MIT */
/* z_bop 00443c30 */
void __cdecl _free(void *);

void __cdecl FUN_00443c30(void *param_1)
{
  void *ptr;
  if (param_1 != (void *)0x0) {
    if (*(void **)((int)param_1 + 0x1014) != (void *)0x0) {
      _free(*(void **)((int)param_1 + 0x1014));
    }
    if (*(void **)((int)param_1 + 0x1018) != (void *)0x0) {
      _free(*(void **)((int)param_1 + 0x1018));
    }
    if (*(void **)((int)param_1 + 0x1044) != (void *)0x0) {
      _free(*(void **)((int)param_1 + 0x1044));
    }
    if (*(void **)((int)param_1 + 0x1048) != (void *)0x0) {
      _free(*(void **)((int)param_1 + 0x1048));
    }
    if (*(void **)((int)param_1 + 0x104c) != (void *)0x0) {
      _free(*(void **)((int)param_1 + 0x104c));
    }
    __builtin_memcpy(&ptr, (void *)((int)param_1 + 0x1082), 4);
    if (ptr != (void *)0x0) {
      _free(ptr);
    }
    __builtin_memcpy(&ptr, (void *)((int)param_1 + 0x1086), 4);
    if (ptr != (void *)0x0) {
      _free(ptr);
    }
    __builtin_memcpy(&ptr, (void *)((int)param_1 + 0x109a), 4);
    if (ptr != (void *)0x0) {
      _free(ptr);
    }
    __builtin_memcpy(&ptr, (void *)((int)param_1 + 0x18ae), 4);
    if (ptr != (void *)0x0) {
      _free(ptr);
    }
    __builtin_memcpy(&ptr, (void *)((int)param_1 + 0x18b2), 4);
    if (ptr != (void *)0x0) {
      _free(ptr);
    }
    _free(param_1);
  }
  return;
}
