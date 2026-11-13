/* SPDX-License-Identifier: MIT */
/* z_xvt 0048e480 */
int __cdecl __strcmpi(char *s1, char *s2);
void * __cdecl FUN_0048e520(void *param_1, char *param_2);

void * __cdecl FUN_0048e480(int param_1, char *param_2)
{
  int local_4 = 0;
  int count;
  __builtin_memcpy(&count, (char *)param_1 + 6, 4);

  if (count > 0) {
    int local_8 = 0;
    do {
      int base_int;
      __builtin_memcpy(&base_int, (char *)param_1 + 10, 4);
      char *base = (char *)base_int;
      void **puVar3;
      __builtin_memcpy(&puVar3, base + local_8, 4);

      if (puVar3 == NULL) {
        /* LAB_0048e4f2 */
        puVar3 = NULL;
      }
      else if ((char *)*puVar3 == NULL || __strcmpi((char *)*puVar3, param_2) != 0) {
        int iVar1 = 0;
        int sub_count = *(int *)((char *)puVar3 + 8);
        if (sub_count > 0) {
          int iVar4 = 0;
          do {
            int sub_base_int = *(int *)((char *)puVar3 + 12);
            char *sub_base = (char *)sub_base_int;
            void *puVar2;
            __builtin_memcpy(&puVar2, sub_base + iVar4, 4);
            puVar2 = FUN_0048e520(puVar2, param_2);
            if (puVar2 != NULL) {
              return puVar2;
            }
            iVar4 += 4;
            iVar1++;
          } while (iVar1 < sub_count);
        }
        puVar3 = NULL;
      }

      if (puVar3 != NULL) {
        return (void *)puVar3;
      }

      local_8 += 4;
      local_4++;
    } while (local_4 < count);
  }
  return NULL;
}
