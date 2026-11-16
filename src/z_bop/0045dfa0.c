/* SPDX-License-Identifier: MIT */
/* z_bop 0045dfa0 */
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_009a73c0;
extern char DAT_009a73d8;
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern unsigned short DAT_00a07cec;
extern unsigned char DAT_00521f2e[];

unsigned short __cdecl FUN_00432fa0(unsigned short a, unsigned short b, unsigned short c);
void __cdecl FUN_00403540(unsigned int a, unsigned int b);

#define RD32(dst, addr) __builtin_memcpy(&(dst), (const void *)(unsigned int)(addr), 4)
#define RD16(dst, addr) __builtin_memcpy(&(dst), (const void *)(unsigned int)(addr), 2)

int __cdecl
FUN_0045dfa0(unsigned short param_1, unsigned short param_2, short param_3,
             unsigned short param_4, unsigned short param_5)
{
  int iVar1;
  unsigned char bVar2;
  char cVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  unsigned short uVar6;
  int iVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int bVar10;
  unsigned short local_6;
  unsigned int p, ptr1, q, t, a, b, v1;
  unsigned int r;
  short s;

  local_6 = DAT_00a07cec;
  iVar1 = 0;
  iVar7 = 0;
  if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
    do {
      iVar1 = iVar7;
      uVar9 = (unsigned int)local_6;
      iVar7 = (int)((unsigned int)DAT_009a1fe8 + uVar9 * 0x23);
      if ((*(char *)(iVar7 + 3) != '\0') &&
          (uVar5 = (unsigned short)*(unsigned char *)(iVar7 + 0x16), DAT_009a73c0 != uVar5)) {
        uVar4 = FUN_00432fa0(uVar5, param_1, param_2);
        uVar5 = FUN_00432fa0(uVar5, param_4, param_5);
        if (param_3 == 1) {
          uVar4 = uVar4 | uVar5;
        } else {
          uVar4 = uVar4 & uVar5;
        }
        if (uVar4 != 0) {
          if (local_6 == 0xffff) {
            iVar7 = 0;
          } else {
            p = (unsigned int)DAT_009a1fe8 + uVar9 * 0x23;
            RD32(ptr1, p + 0x1f);
            if (ptr1 == 0) {
              bVar10 = *(char *)(p + 3) == '\0';
LAB_a:
              iVar7 = (int)(1u - bVar10);
            } else if (*(char *)(p + 3) == '\0') {
              iVar7 = 0;
            } else {
              RD32(q, ptr1 + 0xa9);
              if (q == 0) {
                iVar7 = 1;
              } else if ((*(char *)(q + 0x5e) == '\x15') && (*(char *)(q + 0x5f) != '\0')) {
                iVar7 = 0;
              } else {
                cVar3 = *(char *)(q + 7);
                if ((cVar3 == '\x01') || ((cVar3 == '\x03' || (cVar3 == '\x04')))) {
                  iVar7 = 0;
                } else if ((*(char *)(q + 0x23d) == '\x03') && (*(char *)(q + 0x241) != '\0')) {
                  FUN_00403540(uVar9, (unsigned int)DAT_009a73b0);
                  cVar3 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + (unsigned int)DAT_009a1fe8);
                  if ((cVar3 == '\0') || ((cVar3 == '\x01' || (cVar3 == '\x02')))) {
                    if (DAT_009a1ff8 < 0x8001) goto LAB_a2;
                    iVar7 = 0;
                  } else {
                    if (DAT_009a1ff8 < 0x10001) goto LAB_a2;
                    iVar7 = 0;
                  }
                } else {
LAB_a2:
                  p = (unsigned int)DAT_009a1fe8 + uVar9 * 0x23;
                  RD32(v1, p + 0x1b);
                  if ((int)v1 == -1) {
                    RD32(ptr1, p + 0x1f);
                    RD32(t, ptr1 + 0xa9);
                    RD32(a, t + 0xcb);
                    RD32(b, q + 0xc3);
                    bVar10 = a < b;
                    goto LAB_a;
                  }
                  iVar7 = 1;
                }
              }
            }
          }
          if (iVar7 != 0) {
            RD32(iVar7, (unsigned int)DAT_009a1fe8 + 0x1f + uVar9 * 0x23);
            RD32(iVar8, (unsigned int)iVar7 + 0xa9);
            if (DAT_009a73d8 == '\0') {
              iVar1 = (int)((unsigned int)iVar1 + 1u);
            } else {
              RD16(s, (unsigned int)iVar8 + 0x1e5);
              if (s != 0) {
                if (DAT_009a73d8 == '\0' || *(char *)((unsigned int)iVar8 + 0x1eb) == '\0') {
                  iVar1 = (int)((unsigned int)iVar1 + 1u);
                } else {
                  RD32(r, (unsigned int)DAT_009a73b0 * 0x23 + 0x1f + (unsigned int)DAT_009a1fe8);
                  if (*(char *)(r + 0x89) != *(char *)((unsigned int)iVar7 + 0x89)) {
                    iVar1 = (int)((unsigned int)iVar1 + 1u);
                  }
                }
              }
            }
          }
        }
      }
      local_6 = local_6 + 1;
      iVar7 = iVar1;
    } while ((int)(unsigned int)local_6 < DAT_009a1fe0);
  }
  uVar5 = (unsigned short)DAT_00a07ce8;
  if ((int)(DAT_00a07ce8 & 0xffff) < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
    do {
      uVar9 = (unsigned int)uVar5;
      iVar7 = (int)(uVar9 * 0x23 + (unsigned int)DAT_009a1fe8);
      bVar2 = *(unsigned char *)(iVar7 + 3);
      if ((bVar2 != 0) && ((DAT_00521f2e[(unsigned int)bVar2 * 0x1a] & 2) != 0)) {
        bVar2 = *(unsigned char *)(iVar7 + 0x16);
        uVar4 = FUN_00432fa0((unsigned short)bVar2, param_1, param_2);
        uVar6 = FUN_00432fa0((unsigned short)bVar2, param_4, param_5);
        if (param_3 == 1) {
          uVar4 = uVar4 | uVar6;
        } else {
          uVar4 = uVar4 & uVar6;
        }
        if (uVar4 != 0) {
          if (uVar5 == 0xffff) {
            iVar7 = 0;
          } else {
            p = (unsigned int)DAT_009a1fe8 + uVar9 * 0x23;
            RD32(ptr1, p + 0x1f);
            if (ptr1 == 0) {
              bVar10 = *(char *)(p + 3) == '\0';
LAB_b:
              iVar7 = (int)(1u - bVar10);
            } else if (*(char *)(p + 3) == '\0') {
              iVar7 = 0;
            } else {
              RD32(q, ptr1 + 0xa9);
              if (q == 0) {
                iVar7 = 1;
              } else if ((*(char *)(q + 0x5e) == '\x15') && (*(char *)(q + 0x5f) != '\0')) {
                iVar7 = 0;
              } else {
                cVar3 = *(char *)(q + 7);
                if ((cVar3 == '\x01') || ((cVar3 == '\x03' || (cVar3 == '\x04')))) {
                  iVar7 = 0;
                } else if ((*(char *)(q + 0x23d) == '\x03') && (*(char *)(q + 0x241) != '\0')) {
                  FUN_00403540(uVar9, (unsigned int)DAT_009a73b0);
                  cVar3 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + (unsigned int)DAT_009a1fe8);
                  if ((cVar3 == '\0') || ((cVar3 == '\x01' || (cVar3 == '\x02')))) {
                    if (DAT_009a1ff8 < 0x8001) goto LAB_b2;
                    iVar7 = 0;
                  } else {
                    if (DAT_009a1ff8 < 0x10001) goto LAB_b2;
                    iVar7 = 0;
                  }
                } else {
LAB_b2:
                  p = (unsigned int)DAT_009a1fe8 + uVar9 * 0x23;
                  RD32(v1, p + 0x1b);
                  if ((int)v1 == -1) {
                    RD32(ptr1, p + 0x1f);
                    RD32(t, ptr1 + 0xa9);
                    RD32(a, t + 0xcb);
                    RD32(b, q + 0xc3);
                    bVar10 = a < b;
                    goto LAB_b;
                  }
                  iVar7 = 1;
                }
              }
            }
          }
          if (iVar7 != 0) {
            iVar1 = (int)((unsigned int)iVar1 + 1u);
          }
        }
      }
      uVar5 = uVar5 + 1;
    } while ((int)(unsigned int)uVar5 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8));
  }
  if (iVar1 == 0) {
    return 0xffff;
  }
  return iVar1;
}
