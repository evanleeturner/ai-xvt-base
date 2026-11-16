/* SPDX-License-Identifier: MIT */
/* z_bop 0041dc30 */
extern int DAT_00523410;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_009d113c;
extern int DAT_009d1308;
extern unsigned char DAT_009d6940[2];
extern char DAT_009d6954;
extern unsigned short DAT_009ec5e6;
extern int DAT_009ecc34;
extern unsigned char DAT_009fe7d0;
extern unsigned short DAT_00a00860;
extern unsigned short DAT_00a07be0;
extern unsigned short DAT_00a07be2;
extern int DAT_00a07ce8;
extern int DAT_00a07cec;
extern unsigned short DAT_00a08294;
extern char DAT_00bb7375;

extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c89[];
extern unsigned char DAT_009d9154[];
extern unsigned char DAT_009d9155[];
extern unsigned short DAT_009d8b82[];
extern unsigned char DAT_009d8b84[];
extern unsigned char DAT_009a20e6[];
extern unsigned char DAT_009a2110[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009e967e[];
extern unsigned char DAT_009e9681[];
extern unsigned char DAT_009fd458[];
extern unsigned short DAT_0051bf60[];
extern unsigned char DAT_0051bf78[];
extern unsigned short DAT_0051bf88[];
extern unsigned char DAT_0051bfa0[];
extern unsigned char DAT_0051c581[];
extern unsigned char DAT_0051c591[];
extern unsigned char DAT_00521f1a[];
extern unsigned char DAT_00521f1c[];
extern unsigned int DAT_00a00870[];
extern unsigned int DAT_00a00874[];

unsigned int __cdecl FUN_0041fac0(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_004aef60(unsigned int a, int b);
unsigned int __cdecl FUN_004aeec0(unsigned int a, int b);
unsigned int __cdecl FUN_004a6990(unsigned short a, unsigned short b, unsigned int c, unsigned int d);
int __cdecl FUN_0042fb70(int a, unsigned int b, int c, int d, unsigned int e, unsigned short f);
int __cdecl FUN_00430200(int a, int b, unsigned int c, unsigned int d);
unsigned int __cdecl FUN_00425cb0(void);
unsigned int __cdecl FUN_0042e4d0(unsigned int a, unsigned int b, int c);
unsigned int __cdecl FUN_00425ba0(unsigned int a, unsigned int b);
void __cdecl FUN_00450650(int a, int b);
void __cdecl FUN_00451c20(unsigned int a, void *b, int c);
unsigned int __cdecl FUN_00425d10(void);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_0043f140(void);
void __cdecl FUN_004abe50(void);
void __cdecl FUN_00434500(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_00435250(unsigned int a, int b);
void __cdecl FUN_0045acd0(int a);
void __cdecl FUN_00484a30(int a, unsigned int b, unsigned int c);
void __cdecl FUN_00433d30(unsigned short a, unsigned short b, int c);
int __cdecl FUN_004af5b0(unsigned int a);
int __cdecl FUN_004adc40(unsigned int a);
int __cdecl FUN_004ae2a0(unsigned int a, unsigned int b);
int __cdecl FUN_004ae5a0(unsigned int a, unsigned int b);
unsigned short __cdecl FUN_004591c0(unsigned int a, char b);

unsigned int __cdecl
FUN_0041dc30(unsigned int param_1, unsigned short param_2, unsigned int param_3,
             unsigned int param_4, unsigned int param_5, unsigned short param_6,
             unsigned int param_7, unsigned short param_8, unsigned int param_9)
{
  short *psVar1;
  int *piVar2;
  int *piVar3;
  char cVar4;
  unsigned char bVar5;
  unsigned int *puVar6;
  char *pcVar7;
  _Bool bVar8;
  unsigned short uVar9;
  unsigned short uVar10;
  short sVar11;
  unsigned short uVar12;
  unsigned short uVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned int uVar16;
  int iVar17;
  unsigned int uVar18;
  unsigned short uVar19;
  unsigned int uVar20;
  int iVar21;
  int iVar22;
  int iVar23;
  _Bool bVar24;
  _Bool bVar25;
  unsigned char local_1f;
  unsigned short local_1c;
  unsigned int local_10;
  unsigned short local_c;
  /* scratch for unaligned (packed) accesses */
  int m_i0;
  int m_i1;
  unsigned int m_u0;
  unsigned int m_u1;
  unsigned short m_s0;
  short m_h0;
  short m_h1;
  unsigned char *m_p0;

  uVar15 = param_4;
  uVar9 = DAT_009d113c;
  bVar8 = 0;
  bVar25 = 0;
  local_1f = 1;
  if (DAT_00523410 != 0) {
    return 1;
  }
  uVar14 = param_1 & 0xffff;
  iVar23 = uVar14 * 0x23;
  uVar20 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar23 + 0x16);
  iVar17 = uVar20 * 0x566;
  if ((DAT_009d8c84[uVar20 * 0x566] == 0x14) || (DAT_009d9155[iVar17] == 0x14)) {
    return ((unsigned int)(unsigned short)((unsigned int)iVar17 >> 0x10) << 0x10) | 1;
  }
  uVar20 = param_3 & 0xffff;
  if ((((int)DAT_00a07cec <= (int)uVar20) && ((int)uVar20 < DAT_009a1fe0)) &&
      ((iVar17 = (unsigned int)*(unsigned char *)(uVar20 * 0x23 + 0x16 + DAT_009a1fe8) * 0x566,
        DAT_009d8c84[iVar17] == 0x14) || (DAT_009d9155[iVar17] == 0x14))) {
    return ((unsigned int)(unsigned short)((unsigned int)iVar17 >> 0x10) << 0x10) | 1;
  }
  __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + iVar23 + 0x1f), 4);
  __builtin_memcpy(&puVar6, (unsigned char *)(m_u0 + 0xa9), 4);
  DAT_009d113c = *(unsigned short *)(puVar6 + 0x10);
  if ((short)param_3 == -2) {
    bVar25 = 1;
    param_3 = (param_3 & 0xffff0000) | 0xffff;
    local_1c = 0x35;
    param_4 = 0;
    uVar15 = uVar15 & 0xffff;
  }
  else if ((short)param_3 == -1) {
    local_1c = 0x35;
LAB_0041de09:
    uVar15 = 0x20000;
  }
  else {
    iVar21 = uVar20 * 0x23 + DAT_009a1fe8;
    __builtin_memcpy(&iVar17, (unsigned char *)((unsigned int)iVar21 + 0x1f), 4);
    if (iVar17 == 0) {
      local_1c = (unsigned short)*(unsigned char *)((unsigned int)iVar21 + 3);
      __builtin_memcpy(&m_u0, DAT_00521f1c + (unsigned int)local_1c * 0x1a, 4);
      if (0x7fff < m_u0) goto LAB_0041de09;
      __builtin_memcpy(&m_u0, DAT_00521f1c + (unsigned int)local_1c * 0x1a, 4);
      uVar15 = m_u0 << 2;
    }
    else {
      local_1c = (unsigned short)*(unsigned char *)((unsigned int)iVar21 + 3);
      __builtin_memcpy(&m_i0, (unsigned char *)((unsigned int)iVar17 + 0xa9), 4);
      if (m_i0 == 0) {
        __builtin_memcpy(&uVar15, (unsigned char *)((unsigned int)iVar17 + 0x7d), 4);
      }
      else {
        uVar15 = FUN_0041fac0(param_1, param_3);
      }
    }
  }
  cVar4 = *(char *)(DAT_009a1fe8 + 2 + iVar23);
  if ((cVar4 == '\x04') || (cVar4 == '\x05')) {
    uVar15 = uVar15 >> 4;
  }
  if (cVar4 == '\x03') {
    uVar15 = uVar15 >> 2;
  }
  if ((DAT_00a08294 == 0xe) &&
      ((cVar4 = *(char *)(DAT_009a1fe8 + iVar23 + 3), cVar4 == '%' || (cVar4 == '&')))) {
    uVar15 = uVar15 >> 2;
  }
  uVar20 = 0xffff;
  __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xd3, 4);
  m_u0 = (unsigned int)m_i0 + uVar15;
  __builtin_memcpy((unsigned char *)puVar6 + 0xd3, &m_u0, 4);
  if ((short)param_3 == -1) {
LAB_0041dfdf:
    if ((short)param_3 == -1) {
      m_i0 = 1;
    }
    else {
      iVar17 = (param_3 & 0xffff) * 0x23 + DAT_009a1fe8;
      __builtin_memcpy(&m_i1, (unsigned char *)((unsigned int)iVar17 + 0x1f), 4);
      m_i0 = (m_i1 != 0);
    }
    if (m_i0 != 0) {
      if (bVar25) {
        __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xdf, 4);
        m_u0 = (unsigned int)m_i0 + uVar15;
        __builtin_memcpy((unsigned char *)puVar6 + 0xdf, &m_u0, 4);
      }
    }
    else if (*(char *)((unsigned int)iVar17 + 2) == '\n') {
      __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xdb, 4);
      m_u0 = (unsigned int)m_i0 + uVar15;
      __builtin_memcpy((unsigned char *)puVar6 + 0xdb, &m_u0, 4);
    }
  }
  else {
    iVar17 = (param_3 & 0xffff) * 0x23;
    iVar21 = iVar17 + DAT_009a1fe8;
    __builtin_memcpy(&m_p0, (unsigned char *)((unsigned int)iVar21 + 0x1f), 4);
    if ((char *)m_p0 == (char *)0x0) goto LAB_0041dfdf;
    cVar4 = *(char *)m_p0;
    if (cVar4 == '\0') {
      uVar20 = param_3 & 0xffff;
      __builtin_memcpy(&iVar22, (unsigned char *)((unsigned int)iVar21 + 0x1b), 4);
    }
    else {
      __builtin_memcpy(&uVar10, m_p0 + 0x85, 2);
      uVar20 = (unsigned int)uVar10;
      __builtin_memcpy(&iVar22,
                       (unsigned char *)((uVar20 * 8 - (unsigned int)uVar10) * 5 + 0x1b + DAT_009a1fe8), 4);
      if ((cVar4 == '\x01') &&
          (__builtin_memcpy(&pcVar7, m_p0 + 0xa5, 4), pcVar7 != (char *)0x0)) {
        iVar22 = (int)*pcVar7;
      }
    }
    if (iVar22 == -1) {
      if (cVar4 == '\0') {
        __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xdb, 4);
        m_u0 = (unsigned int)m_i0 + uVar15;
        __builtin_memcpy((unsigned char *)puVar6 + 0xdb, &m_u0, 4);
      }
      else {
        iVar17 = uVar20 * 0x23 + DAT_009a1fe8;
        if (*(char *)((unsigned int)iVar17 + 2) == '\0') {
          piVar2 = (int *)((unsigned char *)puVar6 +
                           (unsigned int)DAT_009d8c89[(unsigned int)*(unsigned char *)((unsigned int)iVar17 + 0x16) * 0x566] * 4 +
                           0x1c7);
          __builtin_memcpy(&m_i0, piVar2, 4);
          m_i0 = (int)((unsigned int)m_i0 + uVar15);
          __builtin_memcpy(piVar2, &m_i0, 4);
        }
        else if (((int)uVar20 < DAT_00a07ce8) ||
                 ((int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8) <= (int)uVar20)) {
          __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xdf, 4);
          m_u0 = (unsigned int)m_i0 + uVar15;
          __builtin_memcpy((unsigned char *)puVar6 + 0xdf, &m_u0, 4);
        }
        else {
          __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xe3, 4);
          m_u0 = (unsigned int)m_i0 + uVar15;
          __builtin_memcpy((unsigned char *)puVar6 + 0xe3, &m_u0, 4);
        }
      }
    }
    else {
      __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + (unsigned int)iVar22 * 4 + 0x1a7, 4);
      m_u0 = (unsigned int)m_i0 + uVar15;
      __builtin_memcpy((unsigned char *)puVar6 + (unsigned int)iVar22 * 4 + 0x1a7, &m_u0, 4);
      __builtin_memcpy(&m_p0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar17), 4);
      if (*(char *)m_p0 == '\0') {
        __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xdb, 4);
        m_u0 = (unsigned int)m_i0 + uVar15;
        __builtin_memcpy((unsigned char *)puVar6 + 0xdb, &m_u0, 4);
      }
    }
    piVar2 = (int *)((unsigned char *)puVar6 +
                     (unsigned int)*(unsigned char *)(uVar20 * 0x23 + 0x16 + DAT_009a1fe8) * 4 + 0xe7);
    __builtin_memcpy(&m_i0, piVar2, 4);
    m_i0 = (int)((unsigned int)m_i0 + uVar15);
    __builtin_memcpy(piVar2, &m_i0, 4);
  }
  __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
  if (m_i0 != -1) {
    __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xd7, 4);
    m_u0 = (unsigned int)m_i0 + uVar15;
    __builtin_memcpy((unsigned char *)puVar6 + 0xd7, &m_u0, 4);
  }
  uVar10 = (unsigned short)param_1;
  if (param_2 != 0xffff) {
    __builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0x215, 4);
    __builtin_memcpy(&m_i1, (unsigned char *)puVar6 + 0x211, 4);
    if ((int)((unsigned int)m_i0 + (unsigned int)m_i1) == 0) {
      if (DAT_009d6954 == '\0') {
LAB_0041e090:
        uVar16 = FUN_004aef60((unsigned int)*(unsigned char *)(DAT_009a1fe8 + 3 + iVar23), param_2 - 1);
      }
      else {
        uVar16 = FUN_004aef60((unsigned int)*(unsigned char *)(DAT_009a1fe8 + 3 + iVar23), param_2 - 1);
      }
      if (uVar16 != 0) {
        uVar15 = FUN_004a6990(uVar10, param_2, uVar15, uVar20);
      }
    }
    else if (DAT_009d6954 == '\0') goto LAB_0041e090;
  }
  cVar4 = *(char *)(DAT_009a1fe8 + 2 + iVar23);
  if (cVar4 == '\x0e') {
    uVar15 = 0;
  }
  piVar2 = (int *)((unsigned char *)puVar6 + (param_4 & 0xffff) * 4 + 0x211);
  __builtin_memcpy(&iVar17, piVar2, 4);
  if ((int)uVar15 < iVar17) {
    m_i0 = (int)((unsigned int)iVar17 - uVar15);
    __builtin_memcpy(piVar2, &m_i0, 4);
    __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
    if (iVar17 != -1) {
      DAT_009d8b82[(unsigned int)iVar17 * 8] = 0x3b;
      DAT_009fe7d0 = (unsigned char)param_4;
      uVar15 = (param_4 & 0xffff) ^ 1;
      __builtin_memcpy(&iVar17, (unsigned char *)puVar6 + uVar15 * 4 + 0x211, 4);
      __builtin_memcpy(&m_i0, piVar2, 4);
      if (m_i0 < iVar17) {
        __builtin_memcpy(&m_i0, piVar2, 4);
        iVar21 = (int)((unsigned int)iVar17 - (unsigned int)m_i0) / 2;
        m_i1 = (int)((unsigned int)iVar17 - (unsigned int)iVar21);
        __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 4 + 0x211, &m_i1, 4);
        m_i1 = (int)((unsigned int)m_i0 + (unsigned int)iVar21);
        __builtin_memcpy(piVar2, &m_i1, 4);
        *((unsigned char *)puVar6 + 0x21a) = 1;
      }
    }
    __builtin_memcpy(&m_u0, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
    if (m_u0 != uVar14) {
      FUN_0042fb70(DAT_009ecc34, uVar14, 3, -1, uVar14, 0x2000);
    }
LAB_0041ea1a:
    if (DAT_00523410 == 0) {
      uVar15 = 0;
      if ((*((unsigned char *)puVar6 + 7) != '\0') && (*((unsigned char *)puVar6 + 7) != '\x05')) {
        goto LAB_0041f2c3;
      }
      __builtin_memcpy(&uVar15, (unsigned char *)puVar6 + 0xc3, 4);
      __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xcb, 4);
      if (uVar15 < m_u0) goto LAB_0041f2c3;
      if ((short)param_3 != -1) {
        __builtin_memcpy(&pcVar7, (unsigned char *)((param_3 & 0xffff) * 0x23 + 0x1f + DAT_009a1fe8), 4);
        if (pcVar7 != (char *)0x0) {
          if (*pcVar7 != '\0') {
            __builtin_memcpy(&m_s0, pcVar7 + 0x85, 2);
            param_3 = ((unsigned int)(unsigned short)((unsigned int)pcVar7 >> 0x10) << 0x10) | m_s0;
          }
          FUN_00434500(param_3, param_1);
        }
      }
      uVar15 = FUN_00435250(uVar14, 0);
      __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
      if (iVar17 != -1) {
        FUN_0045acd0(iVar17);
      }
      if (DAT_009d6940[1] == '\0') {
        __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
        if (iVar17 != -1) {
          FUN_00484a30(iVar17, uVar20, uVar15);
        }
      }
      else {
        __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
        if (m_i0 == DAT_009ecc34) {
          DAT_009d6940[0] = 1;
        }
      }
      __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + iVar23 + 0x1b), 4);
      if (m_i0 != -1) {
        DAT_009ec5e6 = 0;
      }
      FUN_00433d30(uVar10, (unsigned short)*(unsigned char *)(DAT_009a1fe8 + iVar23 + 0x16), 2);
      uVar19 = (unsigned short)DAT_00a07cec;
      if ((int)((unsigned int)DAT_00a07cec & 0xffff) < DAT_009a1fe0) {
        do {
          iVar17 = (unsigned int)uVar19 * 0x23 + DAT_009a1fe8;
          if ((*(char *)((unsigned int)iVar17 + 3) != '\0') &&
              (__builtin_memcpy(&m_u0, (unsigned char *)((unsigned int)iVar17 + 0x1f), 4),
               __builtin_memcpy(&iVar17, (unsigned char *)(m_u0 + 0xa9), 4),
               __builtin_memcpy(&m_s0, (unsigned char *)((unsigned int)iVar17 + 0x66), 2),
               m_s0 == uVar10)) {
            m_s0 = 0xffff;
            __builtin_memcpy((unsigned char *)((unsigned int)iVar17 + 0x66), &m_s0, 2);
          }
          uVar19 = uVar19 + 1;
        } while ((int)(unsigned int)uVar19 < DAT_009a1fe0);
      }
      uVar19 = (unsigned short)DAT_00a07cec;
      if ((int)((unsigned int)DAT_00a07cec & 0xffff) < DAT_009a1fe0) {
        do {
          iVar17 = (unsigned int)uVar19 * 0x23 + DAT_009a1fe8;
          if ((*(char *)((unsigned int)iVar17 + 3) != '\0') &&
              (__builtin_memcpy(&m_u0, (unsigned char *)((unsigned int)iVar17 + 0x1f), 4),
               __builtin_memcpy(&iVar17, (unsigned char *)(m_u0 + 0xa9), 4),
               __builtin_memcpy(&m_s0, (unsigned char *)((unsigned int)iVar17 + 0x68), 2),
               m_s0 == uVar10)) {
            m_s0 = 0xffff;
            __builtin_memcpy((unsigned char *)((unsigned int)iVar17 + 0x68), &m_s0, 2);
          }
          uVar19 = uVar19 + 1;
        } while ((int)(unsigned int)uVar19 < DAT_009a1fe0);
      }
      FUN_00451c20(param_1, puVar6, 0x88);
      __builtin_memcpy(&uVar10, DAT_009e967e + (unsigned int)DAT_009ecc34 * 0x5bd, 2);
      if (*(unsigned char *)(DAT_009a1fe8 + iVar23 + 0x16) == uVar10) {
        iVar17 = FUN_0042fb70(DAT_009ecc34, uVar14, 6, -1, uVar14, 0xc000);
        if (iVar17 == 0) {
          uVar10 = 0xc000;
          iVar17 = 0x10;
          uVar20 = 0xffffffff;
LAB_0041ecf2:
          FUN_0042fb70(DAT_009ecc34, uVar20, iVar17, -1, uVar14, uVar10);
        }
      }
      else if ((uVar20 != 0xffffffff) && ((int)uVar20 < DAT_009a1fe0) &&
               (__builtin_memcpy(&m_u0, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4),
                m_u0 != uVar20) &&
               (*(unsigned char *)(uVar20 * 0x23 + 0x16 + DAT_009a1fe8) == uVar10)) {
        cVar4 = *(char *)(DAT_009a1fe8 + iVar23 + 2);
        if ((cVar4 == '\0') || (cVar4 == '\x01')) {
          uVar10 = 0xffff;
          iVar17 = 9;
        }
        else {
          uVar10 = 0xa000;
          iVar17 = 10;
        }
        goto LAB_0041ecf2;
      }
      iVar17 = FUN_00430200(1, 0x1b, uVar14, 0xffff);
      if (iVar17 != 0) {
        __builtin_memcpy(&m_s0, DAT_009e967c + (unsigned int)DAT_009ecc34 * 0x5bd, 2);
        uVar15 = (unsigned int)m_s0;
        if (DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar23) * 0x566] == uVar15) {
          bVar25 = 0;
        }
        else {
          bVar25 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar23) * 0x566] +
                                uVar15 * 0x1e5] == '\0';
        }
        if (!bVar25) {
          __builtin_memcpy(&iVar17, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
          if (iVar17 != -1) {
            cVar4 = *(char *)((unsigned int)iVar17 * 0x23 + 3 + DAT_009a1fe8);
            if (((cVar4 == '\x01') || (((cVar4 == '\x02') || (cVar4 == '\x03')) || (cVar4 == '\x0e'))) ||
                (cVar4 == '\x04')) {
              bVar25 = 1;
            }
            else {
              bVar25 = 0;
            }
            if (bVar25) {
              uVar15 = 0x57;
            }
            else {
              uVar15 = 0x24;
            }
            FUN_0042e4d0(uVar15, 0xffffffff, DAT_009ecc34);
          }
        }
      }
      bVar5 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar23);
      uVar15 = (unsigned int)bVar5;
      iVar17 = FUN_004af5b0(uVar15);
      if (iVar17 == 0) {
        bVar5 = *((unsigned char *)puVar6 + 4);
        uVar10 = FUN_00425cb0();
        for (uVar10 = (uVar10 & 0x3fff) + 0x2000;
             (__builtin_memcpy(&m_s0, DAT_0051c591 + (unsigned int)bVar5 * 0x101, 2), m_s0 < uVar10);
             uVar10 = uVar10 >> 1) {
        }
        uVar19 = FUN_00425cb0();
        if (uVar19 < 0x8000) {
          uVar10 = (unsigned short)(0U - (unsigned int)uVar10);
        }
        __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4);
        __builtin_memcpy((unsigned char *)(m_u0 + 0x77), &uVar10, 2);
        iVar23 = (int)(DAT_009a1fe8 + (unsigned int)iVar23);
        if (DAT_009d9154[(unsigned int)*(unsigned char *)((unsigned int)iVar23 + 0x16) * 0x566] == 0) {
          uVar15 = FUN_00425cb0();
          sVar11 = (short)(((short)(uVar15 & 0xffff0007) + 8) * 0xec);
          uVar15 = ((unsigned int)(unsigned short)((uVar15 & 0xffff0007) >> 0x10) << 0x10) |
                   (unsigned short)sVar11;
          __builtin_memcpy(&m_u0, (unsigned char *)((unsigned int)iVar23 + 0x1f), 4);
          __builtin_memcpy((unsigned char *)(m_u0 + 0x81), &sVar11, 2);
          *((unsigned char *)puVar6 + 7) = 3;
        }
        else {
          __builtin_memcpy(&uVar15, (unsigned char *)((unsigned int)iVar23 + 0x1f), 4);
          m_s0 = (unsigned short)((unsigned short)DAT_009d9154[(unsigned int)*(unsigned char *)((unsigned int)iVar23 + 0x16) * 0x566] * 0x49c + -0x49b);
          __builtin_memcpy((unsigned char *)(uVar15 + 0x81), &m_s0, 2);
          *((unsigned char *)puVar6 + 7) = 3;
        }
      }
      else if (((__builtin_memcpy(&m_i0, DAT_00521f1c + (unsigned int)local_1c * 0x1a, 4), m_i0 < 0x448) ||
                (DAT_00521f1a[(unsigned int)local_1c * 0x1a] != '\0')) &&
               ((local_1c != 0x3a) &&
                (__builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4),
                 __builtin_memcpy(&m_h0, (unsigned char *)(m_u0 + 0x79), 2), m_h0 != 0))) {
        uVar10 = FUN_00425cb0();
        __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + iVar23 + 0x1b), 4);
        if ((uVar10 < 0x4000) && (m_i0 != -1)) {
          __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + iVar23 + 0x1f), 4);
          m_s0 = 1;
          __builtin_memcpy((unsigned char *)(m_u0 + 0x81), &m_s0, 2);
          uVar15 = FUN_0042e4d0(0x15, uVar14, DAT_009ecc34);
          local_1f = 0;
          *((unsigned char *)puVar6 + 7) = 4;
        }
        else {
          if (bVar5 < 0x49) {
            uVar10 = *(unsigned short *)(DAT_00a00870 + uVar15 * 0x65);
          }
          else {
            iVar17 = FUN_004adc40(uVar15);
            uVar10 = (unsigned short)iVar17;
          }
          local_1c = 0;
          local_c = 0;
          uVar19 = 0;
          if (1 < uVar10) {
            uVar19 = FUN_00425cb0();
            local_10 = 0;
            uVar19 = uVar19 & 1;
            if (uVar10 != 0) {
              do {
                if (bVar5 < 0x49) {
                  uVar20 = local_10;
                  if ((int)DAT_00a00870[uVar15 * 0x65] <= (int)local_10) {
                    uVar20 = DAT_00a00870[uVar15 * 0x65] - 1;
                  }
                  iVar17 = (int)DAT_00a00874[uVar20 + uVar15 * 0x65];
                }
                else {
                  iVar17 = FUN_004ae2a0(uVar15, local_10);
                }
                if ((*((unsigned char *)puVar6 + local_10 + 0x2d8) == '\0') &&
                    ((iVar17 == 2) || (iVar17 == 0x14))) {
                  if (uVar19 == 0) {
                    iVar17 = FUN_004ae5a0(uVar15, local_10);
                    if (0 < iVar17) break;
                  }
                  else {
                    iVar17 = FUN_004ae5a0(uVar15, local_10);
                    if (iVar17 < 0) break;
                  }
                }
                uVar12 = (unsigned short)local_10 + 1;
                local_10 = (unsigned int)uVar12;
              } while (uVar12 < uVar10);
            }
            if (((unsigned short)local_10 < uVar10) &&
                (uVar12 = FUN_004591c0(param_1, (char)local_10), uVar12 != 0xffff)) {
              uVar13 = FUN_00425cb0();
              local_c = (uVar13 & 0x3fff) + 0x4000;
              uVar13 = FUN_00425cb0();
              local_1c = (uVar13 & 0x7ff) + 0x800;
              uVar13 = local_1c;
              if (uVar19 != 0) {
                uVar13 = (unsigned short)(0U - (unsigned int)local_1c);
                local_c = (unsigned short)(0U - (unsigned int)local_c);
              }
              iVar21 = (unsigned int)uVar12 * 0x23;
              __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar21), 4);
              __builtin_memcpy((unsigned char *)(m_u0 + 0x77), &local_c, 2);
              psVar1 = (short *)(DAT_009a1fe8 + 0x10 + iVar21);
              __builtin_memcpy(&m_h0, psVar1, 2);
              m_h0 = (short)(m_h0 + uVar13);
              __builtin_memcpy(psVar1, &m_h0, 2);
              __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar21), 4);
              *(unsigned char *)(m_u0 + 0x92) = 1;
              __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar21), 4);
              *(unsigned char *)((unsigned int)iVar17 + 0x8b) = *(unsigned char *)((unsigned int)iVar17 + 0x92);
              *(unsigned char *)(DAT_009a1fe8 + 0x1a + iVar21) = 2;
              *((unsigned char *)puVar6 + local_10 + 0x2d8) = 4;
              uVar15 = FUN_00425d10();
              if ((uVar15 & 0x8000) == 0) {
                uVar15 = 0x1b;
              }
              else {
                uVar15 = 0x1a;
              }
              FUN_0042e4d0(uVar15, uVar14, DAT_009ecc34);
              local_1f = 0;
            }
          }
          bVar5 = *((unsigned char *)puVar6 + 4);
          uVar12 = FUN_00425cb0();
          for (uVar12 = (uVar12 & 0x3fff) + 0x2000;
               (__builtin_memcpy(&m_s0, DAT_0051c591 + (unsigned int)bVar5 * 0x101, 2), m_s0 < uVar12);
               uVar12 = uVar12 >> 1) {
          }
          if (local_c < 0x8000) {
            uVar12 = (unsigned short)(0U - (unsigned int)uVar12);
          }
          __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4);
          __builtin_memcpy((unsigned char *)(m_u0 + 0x77), &uVar12, 2);
          if (local_1c != 0) {
            local_1c = local_1c >> 1;
            if (uVar19 == 0) {
              local_1c = (unsigned short)(0U - (unsigned int)local_1c);
            }
            psVar1 = (short *)(DAT_009a1fe8 + 0x10 + iVar23);
            __builtin_memcpy(&m_h0, psVar1, 2);
            m_h0 = (short)(m_h0 + local_1c);
            __builtin_memcpy(psVar1, &m_h0, 2);
            __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4);
            *(unsigned char *)(m_u0 + 0x92) = 1;
            __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4);
            *(unsigned char *)((unsigned int)iVar17 + 0x8b) = *(unsigned char *)((unsigned int)iVar17 + 0x92);
          }
          *((unsigned char *)puVar6 + 7) = 3;
          iVar23 = (int)(DAT_009a1fe8 + (unsigned int)iVar23);
          __builtin_memcpy(&m_i0, (unsigned char *)((unsigned int)iVar23 + 0x1b), 4);
          if (m_i0 == -1) {
            uVar19 = FUN_00425cb0();
            uVar12 = FUN_00425cb0();
            __builtin_memcpy(&m_u0, (unsigned char *)((unsigned int)iVar23 + 0x1f), 4);
            m_s0 = (unsigned short)(((uVar19 & 3) + (uVar12 & 7) + 1) * 0xec);
            __builtin_memcpy((unsigned char *)(m_u0 + 0x81), &m_s0, 2);
          }
          else {
            __builtin_memcpy(&m_u0, (unsigned char *)((unsigned int)iVar23 + 0x1f), 4);
            m_s0 = 0x2c4;
            __builtin_memcpy((unsigned char *)(m_u0 + 0x81), &m_s0, 2);
          }
          uVar15 = (unsigned int)uVar10;
          *((unsigned char *)puVar6 + uVar15 + 0x2d8) = 2;
        }
      }
      else {
        __builtin_memcpy(&m_u0, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar23), 4);
        m_s0 = 1;
        __builtin_memcpy((unsigned char *)(m_u0 + 0x81), &m_s0, 2);
        uVar15 = FUN_0042e4d0(0x15, uVar14, DAT_009ecc34);
        local_1f = 0;
        *((unsigned char *)puVar6 + 7) = 4;
      }
      goto LAB_0041f2c3;
    }
  }
  else {
    if ((((DAT_00bb7375 == '\x02') && (cVar4 != '\0')) && (iVar17 != 0)) &&
        (__builtin_memcpy(&m_i0, (unsigned char *)puVar6 + 0xc3, 4), m_i0 == 0)) {
      FUN_00430200(1, 0x10, uVar14, 0xffff);
    }
    __builtin_memcpy(&m_i0, piVar2, 4);
    uVar15 = uVar15 - (unsigned int)m_i0;
    m_i0 = 0;
    __builtin_memcpy(piVar2, &m_i0, 4);
    __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
    if (m_i0 != -1) {
      if ((((short)param_4 == 0) && (!bVar25)) && (uVar19 = FUN_00425cb0(), uVar19 < 0x1000)) {
        __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
        FUN_0042e4d0(0x59, 0xffffffff, m_i0);
      }
      uVar16 = (param_4 & 0xffff) ^ 1;
      __builtin_memcpy(&iVar17, (unsigned char *)puVar6 + uVar16 * 4 + 0x211, 4);
      piVar3 = (int *)((unsigned char *)puVar6 + uVar16 * 4 + 0x211);
      if (iVar17 != 0) {
        iVar17 = iVar17 / 2;
        __builtin_memcpy(&m_i0, piVar3, 4);
        m_i0 = (int)((unsigned int)m_i0 - (unsigned int)iVar17);
        __builtin_memcpy(piVar3, &m_i0, 4);
        __builtin_memcpy(&m_i0, piVar2, 4);
        m_i0 = (int)((unsigned int)m_i0 + (unsigned int)iVar17);
        __builtin_memcpy(piVar2, &m_i0, 4);
        *((unsigned char *)puVar6 + 0x21a) = 1;
      }
    }
    if (uVar15 == 0) goto LAB_0041ea1a;
    if (((local_1c == 0x8d) || (local_1c == 0x8e)) || (local_1c == 0x93)) {
      __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0xcf, 2);
      if (m_s0 < 1000) {
        if (local_1c == 0x8d) {
          __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0xcf, 2);
          m_s0 = (unsigned short)(m_s0 + 1);
          __builtin_memcpy((unsigned char *)puVar6 + 0xcf, &m_s0, 2);
        }
        if (local_1c == 0x8e) {
          __builtin_memcpy(&m_h0, (unsigned char *)puVar6 + 0xcf, 2);
          m_h0 = (short)(m_h0 + 2);
          __builtin_memcpy((unsigned char *)puVar6 + 0xcf, &m_h0, 2);
        }
        if (local_1c == 0x93) {
          __builtin_memcpy(&m_h0, (unsigned char *)puVar6 + 0xcf, 2);
          m_h0 = (short)(m_h0 + 4);
          __builtin_memcpy((unsigned char *)puVar6 + 0xcf, &m_h0, 2);
        }
      }
      __builtin_memcpy(&m_h0, DAT_0051c581 + (unsigned int)*((unsigned char *)puVar6 + 4) * 0x101, 2);
      __builtin_memcpy(&m_h1, (unsigned char *)puVar6 + 0xcf, 2);
      sVar11 = (short)(m_h0 - m_h1);
      __builtin_memcpy(&m_h0, (unsigned char *)puVar6 + 0x1e5, 2);
      if ((m_h0 != 0) && (sVar11 < 0xb)) {
        if (0 < (int)uVar15) {
          uVar15 = (uVar15 + 199) / 200;
          do {
            uVar16 = 0;
            do {
              __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0x1e5, 2);
              if ((m_s0 & DAT_0051bf60[uVar16]) != 0) {
                bVar8 = 1;
                m_s0 = (unsigned short)((DAT_0051bf60[uVar16] ^ 0x3ff) & m_s0);
                __builtin_memcpy((unsigned char *)puVar6 + 0x1e5, &m_s0, 2);
                break;
              }
              uVar19 = (unsigned short)((short)uVar16 + 1);
              uVar16 = (unsigned int)uVar19;
            } while (uVar19 < 10);
            __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
            if (((unsigned short)uVar16 < 10) && (m_i0 != -1)) {
              m_s0 = 0;
              __builtin_memcpy((unsigned char *)puVar6 + uVar16 * 2 + 0x2ac, &m_s0, 2);
              m_s0 = DAT_0051bf88[uVar16];
              __builtin_memcpy((unsigned char *)puVar6 + uVar16 * 2 + 0x2c2, &m_s0, 2);
            }
            uVar15 = uVar15 - 1;
          } while (uVar15 != 0);
        }
        uVar19 = 0;
        __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
        if ((sVar11 < 1) && (m_i0 != -1)) {
          do {
            uVar15 = (unsigned int)uVar19;
            __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0x1e5, 2);
            if ((m_s0 & DAT_0051bf60[uVar15]) != 0) {
              m_s0 = (unsigned short)((DAT_0051bf60[uVar15] ^ 0x3ff) & m_s0);
              __builtin_memcpy((unsigned char *)puVar6 + 0x1e5, &m_s0, 2);
              m_s0 = 0;
              __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 2 + 0x2ac, &m_s0, 2);
              m_s0 = DAT_0051bf88[uVar15];
              __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 2 + 0x2c2, &m_s0, 2);
            }
            uVar19 = uVar19 + 1;
          } while (uVar19 < 10);
          m_s0 = 0;
          __builtin_memcpy((unsigned char *)puVar6 + 0x1e5, &m_s0, 2);
        }
        bVar24 = 0;
        __builtin_memcpy(&m_h0, (unsigned char *)puVar6 + 0x1e5, 2);
        if (m_h0 == 0) {
          __builtin_memcpy(&m_s0, DAT_0051c581 + (unsigned int)*((unsigned char *)puVar6 + 4) * 0x101, 2);
          __builtin_memcpy((unsigned char *)puVar6 + 0xcf, &m_s0, 2);
          m_u0 = 0;
          __builtin_memcpy((unsigned char *)puVar6 + 0x211, &m_u0, 4);
          __builtin_memcpy((unsigned char *)puVar6 + 0x215, &m_u0, 4);
          FUN_00451c20(param_1, puVar6, 0x89);
          iVar17 = FUN_00430200(1, 0x1c, uVar14, 0xffff);
          if (iVar17 != 0) {
            __builtin_memcpy(&m_s0, DAT_009e967c + (unsigned int)DAT_009ecc34 * 0x5bd, 2);
            uVar15 = (unsigned int)m_s0;
            if (DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar23) * 0x566] != uVar15) {
              bVar24 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar23) * 0x566] +
                                    uVar15 * 0x1e5] == '\0';
            }
            if (!bVar24) {
              __builtin_memcpy(&iVar17, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
              if (iVar17 != -1) {
                cVar4 = *(char *)((unsigned int)iVar17 * 0x23 + 3 + DAT_009a1fe8);
                if (((cVar4 == '\x01') ||
                     (((cVar4 == '\x02') || (cVar4 == '\x03')) || (cVar4 == '\x0e'))) ||
                    (cVar4 == '\x04')) {
                  bVar24 = 1;
                }
                else {
                  bVar24 = 0;
                }
                if (bVar24) {
                  uVar15 = 0x57;
                }
                else {
                  uVar15 = 0x24;
                }
                FUN_0042e4d0(uVar15, 0xffffffff, DAT_009ecc34);
              }
            }
          }
          __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
          if (m_i0 == -1) {
            uVar15 = 0;
            if (*((unsigned char *)puVar6 + 0x21d) != '\0') {
              do {
                if (*(char *)(puVar6 + uVar15 + 0xdc) == '\x02') {
                  m_s0 = 0xffff;
                  __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 4 + 0x3b2, &m_s0, 2);
                }
                uVar15 = uVar15 + 1;
              } while (uVar15 < *((unsigned char *)puVar6 + 0x21d));
            }
            uVar15 = 0;
            if (*((unsigned char *)puVar6 + 0x21b) != '\0') {
              do {
                *((unsigned char *)puVar6 + uVar15 + 0x220) = 0;
                uVar15 = uVar15 + 1;
              } while (uVar15 < *((unsigned char *)puVar6 + 0x21b));
            }
          }
          uVar15 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar23);
          iVar17 = uVar15 * 0x126;
          __builtin_memcpy(&m_h0, DAT_009a20e6 + iVar17, 2);
          m_h0 = (short)(m_h0 + 1);
          __builtin_memcpy(DAT_009a20e6 + iVar17, &m_h0, 2);
          if (DAT_009d8c80[uVar15 * 0x566] == *(char *)(puVar6 + 0x2c)) {
            DAT_009a2110[iVar17] = 1;
          }
        }
      }
      __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
      if (m_i0 == DAT_009ecc34) {
        FUN_0042e4d0(0x21, uVar14, DAT_009ecc34);
      }
    }
    else {
      if ((param_2 != 0xffff) &&
          (uVar16 = FUN_004aeec0((unsigned int)*(unsigned char *)(DAT_009a1fe8 + 3 + iVar23), param_2 - 1),
           uVar16 != 0)) {
        uVar15 = FUN_004a6990(uVar10, param_2, uVar15, uVar20);
      }
      __builtin_memcpy(&uVar16, (unsigned char *)puVar6 + 0xc3, 4);
      m_u0 = uVar16 + uVar15;
      __builtin_memcpy((unsigned char *)puVar6 + 0xc3, &m_u0, 4);
      if ((DAT_00bb7375 == '\x02') && (*(char *)(DAT_009a1fe8 + 2 + iVar23) != '\0')) {
        __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xcb, 4);
        uVar15 = FUN_00425ba0(m_u0, 0xf333);
        __builtin_memcpy(&m_u1, (unsigned char *)puVar6 + 0xc3, 4);
        if ((uVar16 < uVar15) && (uVar15 <= m_u1)) {
          iVar17 = 0x16;
        }
        else {
          __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xcb, 4);
          uVar15 = FUN_00425ba0(m_u0, 0xc000);
          __builtin_memcpy(&m_u1, (unsigned char *)puVar6 + 0xc3, 4);
          if ((uVar16 < uVar15) && (uVar15 <= m_u1)) {
            iVar17 = 0x15;
          }
          else {
            __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xcb, 4);
            uVar15 = FUN_00425ba0(m_u0, 0x4000);
            __builtin_memcpy(&m_u1, (unsigned char *)puVar6 + 0xc3, 4);
            if ((uVar15 <= uVar16) || (m_u1 < uVar15)) goto LAB_0041e3a6;
            iVar17 = 0x14;
          }
        }
        FUN_00430200(1, iVar17, uVar14, 0xffff);
      }
LAB_0041e3a6:
      __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
      if ((iVar17 != -1) && (!bVar25)) {
        if (iVar17 != -1) {
          __builtin_memcpy(&m_h0, DAT_009d8b84 + (unsigned int)iVar17 * 0x10, 2);
          m_h0 = (short)(m_h0 + 0x3b);
          __builtin_memcpy(DAT_009d8b84 + (unsigned int)iVar17 * 0x10, &m_h0, 2);
        }
        uVar19 = FUN_00425cb0();
        if (uVar19 < 0x4000) {
          uVar19 = FUN_00425cb0();
          uVar12 = FUN_00425cb0();
          uVar13 = FUN_00425cb0();
          uVar15 = (unsigned int)(unsigned short)((uVar19 & 7) + (uVar12 & 1) + (uVar13 & 1));
          __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0x1e5, 2);
          if ((m_s0 & DAT_0051bf60[uVar15]) != 0) {
            m_s0 = (unsigned short)((DAT_0051bf60[uVar15] ^ 0x3ff) & m_s0);
            __builtin_memcpy((unsigned char *)puVar6 + 0x1e5, &m_s0, 2);
            DAT_00a07be2 = 0x57;
            DAT_00a07be0 = (unsigned short)DAT_0051bf78[uVar15];
            __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
            if ((m_i0 == DAT_009ecc34) && (DAT_009e9681[(unsigned int)DAT_009ecc34 * 0x5bd] == '\0')) {
              FUN_00450650(0x56, DAT_009ecc34);
            }
            m_s0 = 0;
            __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 2 + 0x2ac, &m_s0, 2);
            m_s0 = DAT_0051bf88[uVar15];
            __builtin_memcpy((unsigned char *)puVar6 + uVar15 * 2 + 0x2c2, &m_s0, 2);
          }
        }
      }
    }
    __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xc3, 4);
    __builtin_memcpy(&m_u1, (unsigned char *)puVar6 + 199, 4);
    if ((m_u0 < m_u1) || (bVar25)) {
      __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
      if ((m_i0 == DAT_009ecc34) && (!bVar25)) {
        uVar15 = FUN_00425d10();
        if ((uVar15 & 0x8000) == 0) {
          uVar15 = 0x20;
        }
        else {
          uVar15 = 0x1f;
        }
        FUN_0042e4d0(uVar15, uVar14, DAT_009ecc34);
        local_1f = 0;
      }
      __builtin_memcpy(&m_u0, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
      if (m_u0 != uVar14) {
        uVar19 = 0x4000;
        iVar17 = 3;
        goto LAB_0041e9cc;
      }
    }
    else {
      uVar15 = FUN_00425cb0();
      uVar19 = *(unsigned short *)(DAT_0051bfa0 + (uVar15 & 0xf) * 2);
      __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0x1df, 2);
      if (((DAT_009d6940[1] == '\0') || (uVar19 != 1)) && ((m_s0 & uVar19) != 0)) {
        __builtin_memcpy(&iVar17, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
        if (iVar17 != -1) {
          FUN_0042e4d0(0x1e, uVar14, iVar17);
          bVar25 = 0;
          local_1f = 0;
          cVar4 = *(char *)(DAT_009a1fe8 + 3 + iVar23);
          if (((cVar4 == '\x01') || (cVar4 == '\x02')) ||
              ((cVar4 == '\x03' || ((cVar4 == '\x0e' || (cVar4 == '\x04')))))) {
            bVar25 = 1;
          }
          if (bVar25) {
            if (uVar19 < 9) {
              if (((uVar19 == 8) || (uVar19 == 2)) || (uVar19 == 4)) {
                uVar19 = 0xe;
              }
            }
            else if (uVar19 < 0x101) {
              if ((uVar19 == 0x100) || (uVar19 == 0x80)) {
                uVar19 = 0x180;
              }
            }
            else if (uVar19 < 0x401) {
              if ((uVar19 == 0x400) || (uVar19 == 0x200)) {
LAB_0041e90f:
                uVar19 = 0xe00;
              }
            }
            else if ((uVar19 == 0x800) || (uVar19 == 0x1000)) goto LAB_0041e90f;
          }
        }
        bVar8 = 1;
        __builtin_memcpy(&m_s0, (unsigned char *)puVar6 + 0x1e1, 2);
        m_s0 = (unsigned short)(m_s0 & ~uVar19);
        __builtin_memcpy((unsigned char *)puVar6 + 0x1e1, &m_s0, 2);
      }
      __builtin_memcpy(&m_u0, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
      if ((*((unsigned char *)puVar6 + 7) == '\0') && (m_u0 != uVar14)) {
        uVar19 = 0x6000;
        iVar17 = 5;
LAB_0041e9cc:
        FUN_0042fb70(DAT_009ecc34, uVar14, iVar17, -1, uVar14, uVar19);
      }
    }
    __builtin_memcpy(&m_i0, (unsigned char *)(DAT_009a1fe8 + 0x1b + iVar23), 4);
    if (((!bVar8) || (m_i0 != DAT_009ecc34)) || (DAT_00a00860 != 0)) goto LAB_0041ea1a;
    if (DAT_00523410 == 0) {
      FUN_004aba50();
      FUN_0043f140();
      FUN_004abe50();
      goto LAB_0041ea1a;
    }
  }
  cVar4 = *((char *)puVar6 + 7);
  uVar15 = (((unsigned int)DAT_00523410 >> 8) << 8) | (unsigned char)cVar4;
  if ((cVar4 == '\0') || (cVar4 == '\x05')) {
    __builtin_memcpy(&uVar15, (unsigned char *)puVar6 + 0xcb, 4);
    __builtin_memcpy(&m_u0, (unsigned char *)puVar6 + 0xc3, 4);
    if (uVar15 <= m_u0) {
      uVar15 = uVar15 - 1;
      __builtin_memcpy((unsigned char *)puVar6 + 0xc3, &uVar15, 4);
    }
  }
LAB_0041f2c3:
  uVar18 = ((unsigned int)(unsigned short)(uVar15 >> 0x10) << 0x10) | DAT_009d113c;
  *(unsigned short *)(puVar6 + 0x10) = DAT_009d113c;
  DAT_009d113c = uVar9;
  return (((uVar18 >> 8) << 8) | local_1f) & 0xffff00ff;
}
