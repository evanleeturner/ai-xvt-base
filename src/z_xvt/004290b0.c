/* SPDX-License-Identifier: MIT */
/* z_xvt 004290b0 */
void * __cdecl FUN_004272f0(void);
void __cdecl FUN_00427480(int);
void __cdecl FUN_004282d0(int);
void __cdecl FUN_00427660(void *);
void * __cdecl _malloc(unsigned int);
extern int DAT_00525750[];
extern int DAT_00525740[];

void __cdecl FUN_004290b0(int param_1, int param_2, int param_3, int param_4, int param_5)
{
  void *ctx;
  unsigned char *pbVar10;
  unsigned char bVar6;
  int iVar9;
  unsigned char *out_ptr;

  ctx = FUN_004272f0();
  if (ctx != NULL) {
    *(int *)((int)ctx + 0x1028) = 0;
    *(int *)((int)ctx + 0x1030) = param_3;
    *(int *)((int)ctx + 0x1034) = param_4;
    { int tmp = param_3 * param_4; __builtin_memcpy((void *)((int)ctx + 0x108e), &tmp, 4); }
    
    out_ptr = (unsigned char *)_malloc((unsigned int)param_3 * (unsigned int)param_4 * 6);
    { unsigned char *tmp_ptr = out_ptr; __builtin_memcpy((void *)((int)ctx + 0x1086), &tmp_ptr, 4); }
    
    pbVar10 = (unsigned char *)(param_1 + 0x10);
    iVar9 = 0;
    
    for (bVar6 = *pbVar10; bVar6 != 0xff; pbVar10++) {
      while (bVar6 != 0xfe) {
        unsigned char local_4 = *pbVar10;
        if (local_4 == 0xfb) {
          pbVar10 += 3;
          iVar9 = (unsigned int)pbVar10[-1] * 0x100 + (unsigned int)pbVar10[-2];
        } else if (local_4 == 0xfc) {
          unsigned char count = pbVar10[1];
          pbVar10 += 2;
          do {
            *out_ptr++ = 0x80;
            *out_ptr++ = 0x80;
            *out_ptr++ = 0x80;
            *out_ptr++ = 0;
            *out_ptr++ = 0;
            *out_ptr++ = 0;
          } while (--count != 0);
        } else {
          unsigned int uVar7;
          if (local_4 == 0xfd) {
            uVar7 = (unsigned int)pbVar10[2];
            local_4 = pbVar10[1];
            pbVar10 += 3;
          } else {
            pbVar10++;
            uVar7 = (unsigned int)(local_4 >> (DAT_00525750[param_5] & 0x1f)) + iVar9;
            local_4 = DAT_00525740[param_5] & local_4;
          }
          int iVar8 = uVar7 * 4;
          do {
            *out_ptr++ = *(unsigned char *)(iVar8 + param_2);
            *out_ptr++ = *(unsigned char *)(iVar8 + 1 + param_2);
            *out_ptr++ = *(unsigned char *)(iVar8 + 2 + param_2);
            *out_ptr++ = 0;
            *out_ptr++ = 0;
            *out_ptr++ = 0;
          } while (--local_4 != 0);
        }
        bVar6 = *pbVar10;
      }
    }
    FUN_00427480((int)ctx);
    FUN_004282d0((int)ctx);
    FUN_00427660(ctx);
  }
  return;
}
