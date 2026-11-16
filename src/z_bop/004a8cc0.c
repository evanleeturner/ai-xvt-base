/* SPDX-License-Identifier: MIT */
/* z_bop 004a8cc0 */
extern int DAT_009a1fe8;
extern unsigned char DAT_00521f1c[];
extern unsigned char DAT_0051c641[];
extern unsigned char DAT_0051c645[];
extern unsigned char DAT_0051c649[];
extern int DAT_00a00870[];
extern int DAT_00a00874[];
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;

int  __cdecl FUN_0041f410(int a, int b, int c);
void __cdecl FUN_00427bd0(int a, int b, int c);
void __cdecl FUN_00427d30(int a, int b, int c);
int  __cdecl FUN_004adc40(unsigned int a);
int  __cdecl FUN_004ae2a0(unsigned int a, int b);
int  __cdecl FUN_004ae5a0(unsigned int a, int b);
int  __cdecl FUN_004ae6e0(unsigned int a, int b);
int  __cdecl FUN_004ae640(unsigned int a, int b);
void __cdecl FUN_004037b0(int a, int b, int c, int d);

/* unaligned loads/stores via __builtin_memcpy into/out of a typed local */
#define LD32(dst, addr) __builtin_memcpy(&(dst), (const void *)(addr), 4)
#define LD16(dst, addr) __builtin_memcpy(&(dst), (const void *)(addr), 2)
#define FIX(v) ((((unsigned int)(v)) >> 0xf) | \
                (((unsigned int)(((unsigned long long)(v)) >> 0x20)) << 0x11))

void __cdecl FUN_004a8cc0(int param_1, int param_2)
{
  char cVar1;
  int iVar2;
  long long lVar3;
  long long lVar4;
  long long lVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  unsigned int uVar12;
  int iVar13;
  int iVar14;
  int local_58;
  int t0;
  int t1;
  short s0;
  short s1;
  int obj;
  int obj2;
  int st_i;
  short st_s;

  iVar14 = (int)((unsigned int)param_2 * 0x23u + (unsigned int)DAT_009a1fe8);
  LD32(iVar2, DAT_00521f1c + (unsigned int)*(unsigned char *)(iVar14 + 3) * 0x1a);
  iVar6 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)param_1 * 0x23u);
  LD32(t0, iVar6 + 4);
  LD32(t1, iVar14 + 4);
  iVar13 = (int)((unsigned int)t0 - (unsigned int)t1);
  LD32(t0, iVar6 + 8);
  LD32(t1, iVar14 + 8);
  iVar10 = (int)((unsigned int)t0 - (unsigned int)t1);
  LD32(t0, iVar6 + 0xc);
  LD32(t1, iVar14 + 0xc);
  iVar6 = (int)((unsigned int)t0 - (unsigned int)t1);
  iVar7 = FUN_0041f410(iVar13, iVar10, iVar6);
  if ((int)((unsigned int)iVar2 * 2u) < iVar7) {
    return;
  }
  LD32(iVar8, iVar14 + 0x1f);
  if (iVar8 == 0) {
    return;
  }
  LD32(obj2, iVar8 + 0xa9);
  LD16(s0, obj2 + 0x1e5);
  if (s0 == 0) {
    return;
  }
  if (*(char *)(iVar14 + 3) == 'E') {
    if (0x1000 < iVar7) {
      return;
    }
  }
  else {
    if (*(char *)(iVar8 + 0x92) != '\0') {
      LD16(s0, iVar14 + 0x12);
      LD16(s1, iVar14 + 0x10);
      FUN_00427bd0((int)s0, (int)s1, iVar14);
      LD16(s0, iVar14 + 0x14);
      FUN_00427d30((int)s0, 0, iVar14);
    }
    LD32(obj, iVar14 + 0x1f);
    LD16(s0, obj + 0x99);
    lVar3 = (long long)(int)s0 * (long long)iVar13;
    LD16(s0, obj + 0x9b);
    lVar4 = (long long)(int)s0 * (long long)iVar10;
    LD16(s0, obj + 0x9d);
    lVar5 = (long long)(int)s0 * (long long)iVar6;
    iVar7 = (int)(FIX(lVar5) + FIX(lVar3) + FIX(lVar4));

    LD16(s0, obj + 0x93);
    lVar3 = (long long)(int)s0 * (long long)iVar13;
    LD16(s0, obj + 0x95);
    lVar4 = (long long)(int)s0 * (long long)iVar10;
    LD16(s0, obj + 0x97);
    lVar5 = (long long)(int)s0 * (long long)iVar6;
    iVar8 = (int)(FIX(lVar5) + FIX(lVar3) + FIX(lVar4));

    LD16(s0, obj + 0x9f);
    lVar3 = (long long)(int)s0 * (long long)iVar13;
    LD16(s0, obj + 0xa1);
    lVar4 = (long long)(int)s0 * (long long)iVar10;
    LD16(s0, obj + 0xa3);
    lVar5 = (long long)(int)s0 * (long long)iVar6;
    iVar10 = (int)(FIX(lVar5) + FIX(lVar3) + FIX(lVar4));

    LD32(iVar6, obj + 0xa9);
    if (iVar6 == 0) {
      return;
    }
    iVar11 = (int)((unsigned int)*(unsigned char *)(iVar6 + 4) * 0x101u);
    LD32(iVar6, DAT_0051c641 + iVar11);
    LD32(iVar13, DAT_0051c649 + iVar11);
    LD32(iVar11, DAT_0051c645 + iVar11);
    cVar1 = *(char *)(iVar14 + 3);
    if ((((cVar1 == '5') || (cVar1 == '3')) || (cVar1 == '4')) && (0 < iVar10)) {
      return;
    }
    if (cVar1 == '6') {
      iVar10 = (int)((unsigned int)iVar10 - (unsigned int)iVar11);
      if (iVar10 < 0) {
        iVar10 = (int)(0u - (unsigned int)iVar10);
      }
      if (iVar11 < 0) {
        iVar11 = (int)(0u - (unsigned int)iVar11);
      }
      if ((int)((unsigned int)iVar11 - (unsigned int)(iVar11 >> 2)) < iVar10) {
        return;
      }
      iVar6 = FUN_0041f410((int)((unsigned int)iVar7 - (unsigned int)iVar6),
                           (int)((unsigned int)iVar8 - (unsigned int)iVar13), iVar10);
      if ((iVar2 >> 3) < iVar6) {
        return;
      }
    }
    else if (cVar1 == '[') {
      iVar6 = FUN_0041f410((int)((unsigned int)iVar7 - (unsigned int)iVar6),
                           (int)((unsigned int)iVar8 - (unsigned int)iVar13),
                           (int)((unsigned int)iVar10 - (unsigned int)iVar11));
      if (iVar2 / 6 < iVar6) {
        uVar12 = (unsigned int)*(unsigned char *)(iVar14 + 3);
        if (uVar12 < 0x49) {
          local_58 = DAT_00a00870[uVar12 * 0x65];
        }
        else {
          local_58 = FUN_004adc40(uVar12);
        }
        iVar6 = 0;
        if (local_58 < 1) {
          return;
        }
        do {
          if (uVar12 < 0x49) {
            if (iVar6 < 0) {
              iVar13 = 0;
            }
            else {
              iVar13 = iVar6;
              if (DAT_00a00870[uVar12 * 0x65] <= iVar6) {
                iVar13 = (int)((unsigned int)DAT_00a00870[uVar12 * 0x65] - 1u);
              }
              iVar13 = DAT_00a00874[(unsigned int)iVar13 + uVar12 * 0x65];
            }
          }
          else {
            iVar13 = FUN_004ae2a0(uVar12, iVar6);
          }
          if (iVar13 == 0x10) {
            iVar13 = FUN_004ae5a0(uVar12, iVar6);
            iVar11 = FUN_004ae6e0(uVar12, iVar6);
            iVar9 = FUN_004ae640(uVar12, iVar6);
            FUN_004037b0(iVar14, iVar13, iVar11, (int)(0u - (unsigned int)iVar9));
            iVar13 = FUN_0041f410((int)((unsigned int)iVar7 - (unsigned int)DAT_009d8a58),
                                  (int)((unsigned int)iVar8 - (unsigned int)DAT_009d8a54),
                                  (int)((unsigned int)iVar10 - (unsigned int)DAT_009d8b60));
            if (iVar13 < iVar2 / 6) break;
          }
          iVar6 = (int)((unsigned int)iVar6 + 1u);
          if (local_58 <= iVar6) {
            return;
          }
        } while (1);
      }
    }
    else {
      iVar6 = FUN_0041f410((int)((unsigned int)iVar7 - (unsigned int)iVar6),
                           (int)((unsigned int)iVar8 - (unsigned int)iVar13),
                           (int)((unsigned int)iVar10 - (unsigned int)iVar11));
      if (iVar2 / 6 < iVar6) {
        return;
      }
    }
  }
  LD32(iVar2, (unsigned int)DAT_009a1fe8 + 0x1fu + (unsigned int)param_1 * 0x23u);
  if (iVar2 != 0) {
    LD32(iVar2, iVar2 + 0xa9);
    if (iVar2 != 0) {
      st_i = 0x28000;
      __builtin_memcpy((void *)(iVar2 + 0x1d), &st_i, 4);
      st_s = 0;
      __builtin_memcpy((void *)(iVar2 + 0x248), &st_s, 2);
    }
  }
  return;
}
