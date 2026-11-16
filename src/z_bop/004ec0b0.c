/* SPDX-License-Identifier: MIT */
/* z_bop 004ec0b0 */
extern char DAT_0052c778[];

extern int DAT_00b69e30;
extern unsigned char DAT_00b6a5aa[4];   /* packed int, address is 2 mod 4 */
extern unsigned char DAT_00b6a606[4];   /* packed int, address is 2 mod 4 */
extern unsigned char DAT_00aa615e[];    /* packed int entries, stride 0x2a */
extern unsigned char DAT_00aa62ae[];
extern int DAT_00aa6104;
extern unsigned char DAT_00a91d1a[];
extern int DAT_0052c5d8[];
extern int DAT_00a91ca0[];
extern char DAT_00bb7394;
extern char DAT_00bb739b;
extern char DAT_00bb739d;
extern int DAT_00aa60f0;
extern int DAT_00aa6108;
extern int DAT_00aa60ec;
extern int DAT_00aa60e8;
extern int DAT_00aa60d4;
extern int DAT_00aa60dc;
extern int DAT_00aa60e0;
extern unsigned int DAT_00b6a2c0;
extern unsigned int _DAT_00b69ccc;
extern char DAT_00b69d20[];

void FUN_004d6460(int *rect, int a, int b, int c, int d);
void __cdecl FUN_0042b090(int a, int b, int c);
unsigned int FUN_004ee8e0(int a);
void FUN_0042b0d0(unsigned int a);
void FUN_0042b0e0(int a, int b, int c);
int __cdecl FUN_0042a380(int a, int b, int c, int d, int e);
int FUN_004d11e0(void);
char *FUN_004ddc10(unsigned int a);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void FUN_004db7d0(int a, char *s, int *rect, int b, int c, unsigned int d);
void FUN_004d64a0(int *rect, int a, int b);
int FUN_004ee510(int a);
int FUN_004dbfa0(char *s, int a);
char FUN_004ee650(int a);
char FUN_004ee7c0(int a);
int FUN_004cfee0(void);

void __stdcall FUN_004ec0b0(void)
{
  int bVar1;
  int bVar2;
  char cVar3;
  unsigned int uVar4;
  int iVar5;
  char *pcVar6;
  char *pcVar7;
  unsigned int uVar8;
  int iVar9;
  unsigned char *piVar10;
  unsigned int uVar11;
  char *pcVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  int local_10[4];
  int tmp;
  int v5aa;
  int v606;

  if (DAT_00b69e30 == 2) {
    uVar14 = 0x197;
    uVar13 = 0x1ae;
    uVar11 = 0x54;
  }
  else {
    uVar14 = 0x14d;
    uVar13 = 0x172;
    uVar11 = 0x90;
  }
  FUN_004d6460(local_10, uVar11, 0x6b, uVar13, uVar14);
  FUN_0042b090(0, 0, 0);
  uVar4 = FUN_004ee8e0(-1);
  __builtin_memcpy(&v5aa, DAT_00b6a5aa, 4);
  if ((v5aa == 1) || (v5aa == 2)) {
    switch (uVar4) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 0xe:
    case 0x10:
      uVar8 = (unsigned int)DAT_00a91d1a[(int)((unsigned int)DAT_00aa6104 * 0x562u)];
      break;
    default:
      uVar8 = (unsigned int)DAT_00a91d1a[(int)((unsigned int)DAT_00aa6104 * 0x562u)] + 1u;
      if (3 < uVar8) {
        uVar8 = 0;
      }
    }
    FUN_0042b0d0(uVar8);
  }
  FUN_0042b0e0(DAT_0052c5d8[uVar4 * 3],
               DAT_0052c5d8[uVar4 * 3 + 1],
               DAT_0052c5d8[uVar4 * 3 + 2]);
  iVar9 = 0;
  piVar10 = DAT_00aa615e;
  FUN_0042a380(local_10[0], local_10[1],
               (int)((unsigned int)local_10[2] - (unsigned int)local_10[0] + 1u),
               (int)((unsigned int)local_10[3] - (unsigned int)local_10[1] + 1u),
               0);
  do {
    iVar5 = FUN_004d11e0();
    __builtin_memcpy(&tmp, piVar10, 4);
    if (iVar5 == tmp) break;
    piVar10 = piVar10 + 0x2a;
    iVar9 = iVar9 + 1;
  } while (piVar10 < DAT_00aa62ae);

  if ((DAT_00a91ca0[iVar9] != 0) && (DAT_00b69e30 != 2)) {
    FUN_004d6460(local_10, 0x58, 0x6f, 0xea, 0x7d);
    uVar4 = FUN_004ee8e0(-1);
    pcVar6 = FUN_004ddc10(uVar4 + 0x15);
    pcVar7 = FUN_004ddc10(0x25d);
    _sprintf(DAT_00b69d20, "%s: %s", pcVar7, pcVar6);
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
    FUN_004d64a0(local_10, 0, 0xf);
    uVar11 = (unsigned int)DAT_00aa60ec;
    pcVar6 = FUN_004ddc10(0x23d);
    _sprintf(DAT_00b69d20, "%s %u", pcVar6, uVar11);
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
    FUN_004d64a0(local_10, 0, 0xf);
    if (DAT_00bb739d == '\x02') {
      pcVar6 = FUN_004ddc10(0x1bf);
      pcVar7 = FUN_004ddc10(0x10b);
      pcVar12 = "%s %s";
      goto LAB_004ec34f;
    }
    if (DAT_00bb739d == '\0') {
      pcVar6 = FUN_004ddc10(0x10b);
      _sprintf(DAT_00b69d20, DAT_0052c778, pcVar6);
      goto LAB_004ec354;
    }
    pcVar6 = (char *)(unsigned long)((unsigned int)DAT_00aa60e8 + 1u);
    pcVar7 = FUN_004ddc10(0x10b);
    pcVar12 = "%s %u";
LAB_004ec34f:
    _sprintf(DAT_00b69d20, pcVar12, pcVar7, pcVar6);
LAB_004ec354:
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
    FUN_004d6460(local_10, 0x10e, 0x6f, 0x1aa, 0x7d);
    FUN_004d6460(local_10, 0x113, 0x6f, 0x1af, 0x7d);
    if (DAT_00aa60d4 == 0) {
      uVar4 = 0x111;
    }
    else {
      iVar9 = FUN_004ee510(-1);
      uVar4 = (unsigned int)iVar9 + 0x111u;
    }
    pcVar6 = FUN_004ddc10(uVar4);
    pcVar7 = FUN_004ddc10(0x108);
    _sprintf(DAT_00b69d20, "%s %s", pcVar7, pcVar6);
    iVar9 = local_10[2];
    local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
    local_10[0] = (int)((unsigned int)iVar9 - (unsigned int)local_10[0]);
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
    FUN_004d64a0(local_10, 0, 0xf);
    bVar1 = 1;
    if (DAT_00aa60dc == 0) {
      pcVar6 = FUN_004ddc10(0x111);
      pcVar7 = FUN_004ddc10(0x10a);
      _sprintf(DAT_00b69d20, "%s %s", pcVar7, pcVar6);
      uVar4 = FUN_004ee8e0(-1);
      if (((int)uVar4 < 1) || ((5 < (int)uVar4 && (uVar4 != 0xe)))) goto LAB_004ec52d;
    }
    else {
      uVar4 = FUN_004ee8e0(-1);
      if (((int)uVar4 < 1) || ((5 < (int)uVar4 && (uVar4 != 0xe)))) {
        cVar3 = FUN_004ee650(-1);
        pcVar6 = FUN_004ddc10((unsigned int)(int)(unsigned char)cVar3 + 0x11b);
        pcVar7 = FUN_004ddc10(0x10a);
        _sprintf(DAT_00b69d20, "%s %s", pcVar7, pcVar6);
        goto LAB_004ec52d;
      }
      pcVar6 = FUN_004ddc10(0x111);
      pcVar7 = FUN_004ddc10(0x10a);
      _sprintf(DAT_00b69d20, "%s %s", pcVar7, pcVar6);
    }
    bVar1 = 0;
LAB_004ec52d:
    iVar9 = local_10[2];
    if (bVar1) {
      local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
      local_10[0] = (int)((unsigned int)iVar9 - (unsigned int)local_10[0]);
      FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
      FUN_004d64a0(local_10, 0, 0xf);
    }
    if (DAT_00aa60e0 == 0) {
      uVar4 = 0x111;
    }
    else {
      cVar3 = FUN_004ee7c0(-1);
      uVar4 = (unsigned int)(int)(unsigned char)cVar3 + 0x120;
    }
    pcVar6 = FUN_004ddc10(uVar4);
    pcVar7 = FUN_004ddc10(0x109);
    _sprintf(DAT_00b69d20, "%s %s", pcVar7, pcVar6);
    local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
    local_10[0] = (int)((unsigned int)local_10[2] - (unsigned int)local_10[0]);
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, _DAT_00b69ccc);
    return;
  }

  FUN_004d6460(local_10, 0x58, 0x6f, 0xea, 0x7d);
  uVar4 = FUN_004ee8e0(-1);
  __builtin_memcpy(&v5aa, DAT_00b6a5aa, 4);
  __builtin_memcpy(&v606, DAT_00b6a606, 4);
  if (DAT_00b69e30 == 2) {
    if (((v5aa == 0) && (v606 == 1)) && (DAT_00bb7394 != '\x03')) {
      bVar2 = 1;
      bVar1 = 0;
    }
    else {
LAB_004ec6b7:
      bVar2 = 0;
      bVar1 = 1;
    }
  }
  else if ((v5aa == 0) && (v606 == 1)) {
    if (DAT_00bb7394 == '\x03') {
      bVar1 = 1;
      bVar2 = 0;
    }
    else {
      bVar1 = 0;
      bVar2 = 1;
    }
  }
  else {
    bVar1 = 0;
    bVar2 = 0;
    if (DAT_00bb739b == '\0') {
      bVar1 = 0;
    }
    else if ((DAT_00bb739b == '\x01') ||
             ((DAT_00bb739b == '\x02' &&
              ((iVar9 = FUN_004cfee0(), iVar9 != 0 || (DAT_00b69e30 == 2)))))) goto LAB_004ec6b7;
  }
  if (((DAT_00aa60f0 < 2) && (DAT_00aa6108 < 2)) || (cVar3 = '\x01', !bVar1)) {
    cVar3 = '\x04';
  }
  iVar5 = (int)cVar3;
  pcVar6 = FUN_004ddc10(uVar4 + 0x15);
  iVar9 = iVar5;
  pcVar7 = FUN_004ddc10(0x25d);
  _sprintf(DAT_00b69d20, "%c%s: %c%s", 4, pcVar7, iVar9, pcVar6);
  FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, 0xffff);
  FUN_004d64a0(local_10, 0, 0xf);
  iVar9 = iVar5;
  uVar11 = (unsigned int)DAT_00aa60ec;
  pcVar6 = FUN_004ddc10(0x23d);
  _sprintf(DAT_00b69d20, "%c%s %c%u", 4, pcVar6, iVar9, uVar11);
  FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, 0xffff);
  FUN_004d64a0(local_10, 0, 0xf);
  if (DAT_00bb739d == '\x02') {
    pcVar6 = FUN_004ddc10(0x1bf);
    iVar5 = 4;
    pcVar7 = FUN_004ddc10(0x10b);
    pcVar12 = "%c%s %c%s";
LAB_004ec80a:
    _sprintf(DAT_00b69d20, pcVar12, 4, pcVar7, iVar5, pcVar6);
  }
  else {
    if (DAT_00bb739d != '\0') {
      pcVar6 = (char *)(unsigned long)((unsigned int)DAT_00aa60e8 + 1u);
      pcVar7 = FUN_004ddc10(0x10b);
      pcVar12 = "%c%s %c%u";
      goto LAB_004ec80a;
    }
    uVar11 = 4;
    pcVar6 = FUN_004ddc10(0x10b);
    _sprintf(DAT_00b69d20, "%c%s %c1", 4, pcVar6, uVar11);
  }
  FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, 0xffff);
  FUN_004d6460(local_10, 0x113, 0x6f, 0x1af, 0x7d);
  if (DAT_00aa60d4 == 0) {
    pcVar6 = FUN_004ddc10(0x111);
    pcVar7 = FUN_004ddc10(0x108);
    _sprintf(DAT_00b69d20, "%c%s %s", 4, pcVar7, pcVar6);
  }
  else {
    if ((DAT_00aa60d4 == 1) || (bVar2)) {
      iVar9 = FUN_004ee510(-1);
      pcVar6 = FUN_004ddc10((unsigned int)iVar9 + 0x111u);
      uVar11 = 4;
    }
    else {
      iVar9 = FUN_004ee510(-1);
      pcVar6 = FUN_004ddc10((unsigned int)iVar9 + 0x111u);
      uVar11 = 1;
    }
    pcVar7 = FUN_004ddc10(0x108);
    _sprintf(DAT_00b69d20, "%c%s %c%s", 4, pcVar7, uVar11, pcVar6);
  }
  iVar9 = local_10[2];
  local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
  local_10[0] = (int)((unsigned int)iVar9 - (unsigned int)local_10[0]);
  FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, 0xffff);
  FUN_004d64a0(local_10, 0, 0xf);
  bVar1 = 1;
  if (DAT_00aa60dc < 1) {
    pcVar6 = FUN_004ddc10(0x111);
    pcVar7 = FUN_004ddc10(0x10a);
    _sprintf(DAT_00b69d20, "%c%s %s", 4, pcVar7, pcVar6);
    uVar4 = FUN_004ee8e0(-1);
    if (((int)uVar4 < 1) || ((5 < (int)uVar4 && (uVar4 != 0xe)))) goto LAB_004eca25;
  }
  else {
    uVar4 = FUN_004ee8e0(-1);
    if (((int)uVar4 < 1) || ((5 < (int)uVar4 && (uVar4 != 0xe)))) {
      cVar3 = FUN_004ee650(-1);
      pcVar6 = FUN_004ddc10((unsigned int)(int)(unsigned char)cVar3 + 0x11b);
      uVar11 = 1;
      pcVar7 = FUN_004ddc10(0x10a);
      _sprintf(DAT_00b69d20, "%c%s %c%s", 4, pcVar7, uVar11, pcVar6);
      goto LAB_004eca25;
    }
    pcVar6 = FUN_004ddc10(0x111);
    pcVar7 = FUN_004ddc10(0x10a);
    _sprintf(DAT_00b69d20, "%c%s %s", 4, pcVar7, pcVar6);
  }
  bVar1 = 0;
LAB_004eca25:
  iVar9 = local_10[2];
  if (bVar1) {
    local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
    local_10[0] = (int)((unsigned int)iVar9 - (unsigned int)local_10[0]);
    uVar4 = DAT_00b6a2c0;
    if ((DAT_00aa60dc != 1) && (!bVar2)) {
      uVar4 = 0xffff;
    }
    FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, uVar4);
    FUN_004d64a0(local_10, 0, 0xf);
  }
  if (DAT_00aa60e0 == 0) {
    pcVar6 = FUN_004ddc10(0x111);
    pcVar7 = FUN_004ddc10(0x109);
    _sprintf(DAT_00b69d20, "%c%s %s", 4, pcVar7, pcVar6);
  }
  else {
    if ((DAT_00aa60e0 == 1) || (bVar2)) {
      cVar3 = FUN_004ee7c0(-1);
      pcVar6 = FUN_004ddc10((unsigned int)(int)(unsigned char)cVar3 + 0x120);
      uVar11 = 4;
    }
    else {
      cVar3 = FUN_004ee7c0(-1);
      pcVar6 = FUN_004ddc10((unsigned int)(int)(unsigned char)cVar3 + 0x120);
      uVar11 = 1;
    }
    pcVar7 = FUN_004ddc10(0x109);
    _sprintf(DAT_00b69d20, "%c%s %c%s", 4, pcVar7, uVar11, pcVar6);
  }
  local_10[0] = FUN_004dbfa0(DAT_00b69d20, 0xc);
  local_10[0] = (int)((unsigned int)local_10[2] - (unsigned int)local_10[0]);
  FUN_004db7d0(0xc, DAT_00b69d20, local_10, 0, 1, 0xffff);
  return;
}
