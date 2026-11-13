/* SPDX-License-Identifier: MIT */
/* z_bop 00405890 */
extern int DAT_009e9670;
extern int DAT_009a1fe8;
extern char DAT_009e9698;
extern short DAT_009ec5fe;
extern char DAT_009e9697;
extern char DAT_0051c5aa;
extern int DAT_009e9bfc;
extern int DAT_009ecc34;
extern short _DAT_00a07be0;
extern short _DAT_00a07be2;
extern int DAT_00523424;

void __cdecl FUN_00450650(int param_1, int param_2);
void __cdecl FUN_00406030(int param_1, uint param_2);
void __cdecl FUN_00405ac0(int param_1, uint param_2);

void __cdecl FUN_00405890(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  short sVar5;

  iVar1 = *(int *)((int)&DAT_009e9670 + param_1 * 0x5bd);
  iVar3 = param_1 * 0x5bd;
  if (iVar1 == -1) {
    return;
  }
  iVar2 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
  if ((*(int *)(iVar2 + 0x1d) != 0) &&
     ((*(char *)(iVar2 + 0x246) != '\x01' || (*(short *)(iVar2 + 0x248) == 0)))) {
    FUN_00450650(0x169, param_1);
    return;
  }
  if ((&DAT_009e9698)[iVar3] != '\0') {
    if ((short)((DAT_009ec5fe >> 1) + DAT_009ec5fe) <=
        *(short *)(iVar2 + 0x237 + (uint)(byte)(&DAT_009e9697)[iVar3] * 2)) {
      return;
    }
    if ((*(byte *)(iVar2 + 0x1e5) & 8) != 0) {
      FUN_00406030(iVar1, (uint)(byte)(&DAT_009e9697)[iVar3]);
      iVar1 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
      if ((uint)*(byte *)(iVar1 + 0x372 +
                         (uint)(byte)(&DAT_0051c5aa)
                                     [(uint)(byte)(&DAT_009e9697)[iVar3] +
                                      (uint)*(byte *)(iVar1 + 4) * 0x101] * 4) +
          (uint)*(byte *)(iVar1 + 0x376 +
                         (uint)(byte)(&DAT_0051c5aa)
                                     [(uint)(byte)(&DAT_009e9697)[iVar3] +
                                      (uint)*(byte *)(iVar1 + 4) * 0x101] * 4) != 0) {
        return;
      }
      (&DAT_009e9698)[iVar3] = 0;
      (&DAT_009e9697)[iVar3] = 0;
      *(undefined2 *)(iVar1 + 0x226) = 0x76;
      *(int *)(iVar1 + 0x22a) = *(int *)((int)&DAT_009e9bfc + iVar3) + 0x76;
      return;
    }
    if (param_1 != DAT_009ecc34) {
      return;
    }
    _DAT_00a07be0 = 0x5e;
    _DAT_00a07be2 = 0x57;
    FUN_00450650(0x56, param_1);
    return;
  }
  uVar4 = (uint)(byte)(&DAT_009e9697)[iVar3];
  sVar5 = *(short *)(iVar2 + 0x226 + uVar4 * 2);
  if (DAT_00523424 != 0) {
    if (sVar5 != 0) {
      if (*(int *)((int)&DAT_009e9bfc + iVar3) <= *(int *)(iVar2 + 0x22a + uVar4 * 4)) {
        sVar5 = DAT_009ec5fe * 2;
        goto LAB_00405958;
      }
      sVar5 = 0;
    }
    *(int *)(iVar2 + 0x22a + uVar4 * 4) = *(int *)((int)&DAT_009e9bfc + iVar3);
  }
LAB_00405958:
  if (sVar5 < (short)((DAT_009ec5fe >> 1) + DAT_009ec5fe)) {
    if ((*(byte *)(iVar2 + 0x1e5) & 0x10) != 0) {
      FUN_00405ac0(*(int *)((int)&DAT_009e9670 + iVar3), (uint)(byte)(&DAT_009e9697)[iVar3]);
      return;
    }
    if (param_1 == DAT_009ecc34) {
      _DAT_00a07be2 = 0x57;
      _DAT_00a07be0 = (byte)(&DAT_009e9697)[iVar3] + 0x5c;
      FUN_00450650(0x56, param_1);
      return;
    }
  }
  return;
}
