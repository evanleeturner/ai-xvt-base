/* SPDX-License-Identifier: MIT */
/* z_bop 00417f10 */
/* Packed/unaligned accessors: records use odd strides (0x23, 0x5bd, 0x12, 6)
   and odd field offsets, so every multi-byte access goes through memcpy. */
#define LD32(p) ({ int t_; __builtin_memcpy(&t_, (p), 4); t_; })
#define LD16(p) ({ short t_; __builtin_memcpy(&t_, (p), 2); t_; })
#define ST32(p, v) do { int t_ = (int)(v); __builtin_memcpy((p), &t_, 4); } while (0)
#define ST16(p, v) do { short t_ = (short)(v); __builtin_memcpy((p), &t_, 2); } while (0)
#define REC(i) ((char *)DAT_009a1fe8 + (int)((unsigned)(i) * 0x23u))
#define INNER(r) ((char *)LD32((r) + 0x1f))

extern int DAT_00a080f8;
extern int DAT_00523410;
extern int DAT_00523414;
extern int DAT_00a0085c;
extern int DAT_009e9658;
extern char *DAT_009a1fe8;
extern short DAT_009ec5fe;
extern short DAT_009d8112;
extern int DAT_009ec5d0;
extern unsigned char *DAT_009ecc34;
extern unsigned char DAT_009a7b70[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009ed670[];
extern int DAT_009a8db0[];

void __cdecl FUN_004186e0(int, int *);
void __stdcall FUN_004ace80(void);
void __stdcall FUN_00445570(void);
void __cdecl FUN_0047a5e0(unsigned char *);

void __cdecl FUN_00417f10(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  short uVar4;
  int iVar5;
  short uVar6;
  char *puVar7;
  char *piVar8;
  char *piVar9;
  char *psVar10;
  char *piVar11;
  int iStack_20;
  unsigned char *puStack_1c;
  int *piStack_18;
  int iStack_14;
  char *piStack_10;

  if (DAT_00a080f8 == 1) {
    iStack_20 = 1;
  }
  else {
    iStack_20 = DAT_00523410;
  }
  piVar9 = (char *)DAT_009a7b70;
  piVar11 = (char *)DAT_009e9670;
  puStack_1c = (unsigned char *)0;
  piStack_10 = (char *)DAT_009ed670;
  piStack_18 = DAT_009a8db0;
  do {
    if (piVar11[0x10] != '\0') {
      iStack_14 = 0;
      iVar1 = *piStack_18;
      piVar8 = piStack_10;
      if (0 < iVar1) {
        do {
          iVar5 = DAT_009e9658;
          if ((((iStack_20 == 0) || (DAT_00a080f8 == 1)) &&
               (LD32(piVar8 + 8) <= LD32(piVar11 + 0x58c))) &&
              (LD32(piVar8) == 0)) {
            FUN_004186e0((int)puStack_1c, (int *)piVar8);
            piVar8 = piVar8 + -0x12;
          }
          else {
            iVar2 = LD32(piVar8 + 8);
            DAT_009e9658 = iVar5;
            if (((iVar2 <= param_1) && ((iStack_20 != 0 || (LD32(piVar8 + 4) == 0)))) &&
                (iVar3 = LD32(piVar11 + 0x58c), iVar3 < iVar2)) {
              if ((iVar2 <= DAT_009e9658) && (LD32(piVar11) != -1)) {
                psVar10 = REC(LD32(piVar11));
                iVar2 = LD32(psVar10 + 0x1f);
                DAT_009e9658 = iVar5;
                if ((iVar2 == 0) || (LD32((char *)iVar2 + 0xa9) == 0)) goto LAB_004184a0;
                if ((LD16(piVar11 + 0x5aa) == LD16(psVar10)) &&
                    (piVar11[0x5ac] == piVar11[0x11])) {
                  if (iVar3 < LD32((char *)iVar2 + 2)) {
                    ST32((char *)iVar2 + 2, iVar3);
                    ST32(REC(LD32(piVar11)) + 4, LD32(piVar11 + 0x590));
                    ST32(REC(LD32(piVar11)) + 8, LD32(piVar11 + 0x594));
                    ST32(REC(LD32(piVar11)) + 0xc, LD32(piVar11 + 0x598));
                    ST16(REC(LD32(piVar11)) + 0x14, LD16(piVar11 + 0x59c));
                    ST16(REC(LD32(piVar11)) + 0x12, LD16(piVar11 + 0x59e));
                    ST16(REC(LD32(piVar11)) + 0x10, LD16(piVar11 + 0x5a0));
                    ST16(INNER(REC(LD32(piVar11))) + 0x81, LD16(piVar11 + 0x5a2));
                    ST16(INNER(REC(LD32(piVar11))) + 0x77, LD16(piVar11 + 0x5a8));
                    ST16(INNER(REC(LD32(piVar11))) + 0x79, LD16(piVar11 + 0x5a4));
                    ST16(INNER(REC(LD32(piVar11))) + 0x7b, LD16(piVar11 + 0x5a6));
                    ST16((char *)LD32(INNER(REC(LD32(piVar11))) + 0xa9) + 0xd,
                         LD16(piVar11 + 0x59e));
                    INNER(REC(LD32(piVar11)))[0x92] = 1;
                    INNER(REC(LD32(piVar11)))[0x8b] = 1;
                  }
                  iVar2 = LD32(INNER(REC(LD32(piVar11))) + 2);
                  if (iVar2 < DAT_009e9658) {
                    DAT_009e9658 = iVar2;
                  }
                }
                else {
                  if (iStack_20 != 0) goto LAB_004184a0;
                  ST32(piVar8 + 8, (int)((unsigned)DAT_009e9658 + 4u));
                  ST32(piVar11 + 0x58c, DAT_009e9658);
                }
              }
              uVar6 = DAT_009ec5fe;
              uVar4 = DAT_009d8112;
              iVar2 = LD32(piVar11);
              if (iVar2 != -1) {
                if (LD32(REC(iVar2) + 0x1f) != 0) {
                  DAT_009ec5fe = (short)LD32(piVar8 + 8) - (short)DAT_009e9658;
                  if (DAT_009ec5fe == 0) {
                    DAT_009d8112 = 0xec;
                  }
                  else {
                    DAT_009d8112 = (short)(0xec / (unsigned long long)(long long)(int)(unsigned int)(unsigned short)DAT_009ec5fe);
                  }
                  if (DAT_009d8112 == 0) {
                    DAT_009d8112 = 1;
                  }
                  DAT_009ec5d0 = iVar2;
                  FUN_004ace80();
                  FUN_00445570();
                  ST32(INNER(REC(DAT_009ec5d0)) + 2, LD32(piVar8 + 8));
                  puVar7 = REC(DAT_009ec5d0);
                  ST32(piVar11 + 0x590, LD32(puVar7 + 4));
                  ST32(piVar11 + 0x594, LD32(puVar7 + 8));
                  ST32(piVar11 + 0x598, LD32(puVar7 + 0xc));
                  ST16(piVar11 + 0x59c, LD16(puVar7 + 0x14));
                  ST16(piVar11 + 0x59e, LD16(puVar7 + 0x12));
                  ST16(piVar11 + 0x5a0, LD16(puVar7 + 0x10));
                  ST16(piVar11 + 0x5a2, LD16(INNER(puVar7) + 0x81));
                  ST16(piVar11 + 0x5a8, LD16(INNER(puVar7) + 0x77));
                  ST16(piVar11 + 0x5a4, LD16(INNER(puVar7) + 0x79));
                  ST16(piVar11 + 0x5a6, LD16(INNER(puVar7) + 0x7b));
                  ST16(piVar11 + 0x5aa, LD16(puVar7));
                  piVar11[0x5ac] = piVar11[0x11];
                }
                DAT_009ec5d0 = -1;
              }
              DAT_009ec5fe = (short)LD32(piVar8 + 8) - (short)LD32(piVar11 + 0x58c);
              if (DAT_009ec5fe < 4) {
                DAT_009ec5fe = 4;
              }
              if (DAT_009ec5fe == 0) {
                DAT_009d8112 = 0xec;
              }
              else {
                DAT_009d8112 = (short)(0xec / (unsigned long long)(long long)(int)(unsigned int)(unsigned short)DAT_009ec5fe);
              }
              if (DAT_009d8112 == 0) {
                DAT_009d8112 = 1;
              }
              ST32(piVar11 + 0x58c, LD32(piVar8 + 8));
              ST32(piVar9, LD32(piVar8 + 0xc));
              ST16(piVar9 + 4, LD32(piVar8 + 0x10));
              if ((iStack_20 != 0) && (LD32(piVar8 + 8) <= iVar5)) {
                piVar9[2] = 0;
                piVar9[5] = 0;
              }
              if (((1 < DAT_00a080f8) && (puStack_1c == DAT_009ecc34)) &&
                  (DAT_00523414 = 2, DAT_00a0085c < LD32(piVar8 + 8))) {
                DAT_00523414 = 1;
                DAT_00a0085c = LD32(piVar8 + 8);
              }
              FUN_0047a5e0(puStack_1c);
              DAT_00523414 = 0;
              DAT_009d8112 = uVar4;
              DAT_009e9658 = iVar5;
              DAT_009ec5fe = uVar6;
              if (piVar11[0x10] == '\0') break;
            }
          }
LAB_004184a0:
          piVar8 = piVar8 + 0x12;
          iStack_14 = iStack_14 + 1;
        } while (iStack_14 < iVar1);
      }
    }
    piVar9 = piVar9 + 6;
    piVar11 = piVar11 + 0x5bd;
    piStack_10 = piStack_10 + 0x7e9 * 4;
    piStack_18 = piStack_18 + 1;
    puStack_1c = puStack_1c + 1;
    if ((int *)0x9a8dcf < piStack_18) {
      return;
    }
  } while (1);
}
