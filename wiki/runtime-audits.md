# Measure runtime helpers

Install RGBDS and use Python 3.10 or newer. Run tests, then generate a report
with retained assembly, objects, ROMs, linker maps, and symbols:

```bash
python -B -m unittest discover -s tests -v
python -B tests/audit_runtime.py --output build/runtime-audit.json \
  --artifacts build/runtime-audit
```

The default report covers `mul16` and a fixture with nested calls, saved
registers, a ROM table, and scratch RAM. It uses five explicit input pairs,
recorded in the JSON. Select a helper and operands with:

```bash
python -B tests/audit_runtime.py --helper udiv16 --helper urem16 \
  --lhs 0x1234 --rhs 0x10 --output build/division-audit.json \
  --artifacts build/division-audit
```

The command exits with status 1 if a result or restored stack pointer is
incorrect. Existing division entry points have ABI/initialization defects; their
reports expose failures. The division execution tests separately enter the
current arithmetic loop with explicitly initialized registers. A helper that
reaches `$0010` is reported as `rst_10`; for division by zero, reaching that
vector is the expected outcome. The trap handler itself is not executed.

## Read the measurements

Each case reports the helper body and a synthetic caller using the backend ABI:
first operand low/high in B/C, second operand low/high in A/[SP+2], and result
low/high in A/B. The stack fixture returns A and preserves BC. These callers
pack constant operands, call the helper, and clean up the argument slot; they
are assembly fixtures rather than compiler-generated code.

- `cycles`: CPU T-cycles, including the helper's return. Caller measurements
  also include argument setup, CALL, and cleanup. The unit stays the same in CGB
  double-speed mode; elapsed hardware time changes.
- `helper_code_bytes`: linked helper section bytes, including sibling entry
  points in that section. This measures linked object cost rather than only
  dynamically executed instructions.
- `table_rom_bytes`: linked `Audit table` section bytes.
- `caller_code_bytes`: linked `Audit caller` section bytes.
- `scratch_ram_bytes`: declared WRAM0 allocation owned by the fixture.
- `scratch_written_bytes`: distinct allocated scratch bytes actually written.
  Writes to caller arguments and stack slots do not count as scratch allocation.
- `entry_stack_bytes`: stack already supplied at helper entry. Body measurements
  begin with a two-byte return address and a two-byte argument slot.
- `peak_additional_stack_bytes`: maximum stack growth after measurement starts.
- `peak_stack_bytes`: entry stack plus additional growth, including nested
  return addresses and saved registers.

ROM sizes come from RGBDS section maps, excluding address gaps and padding while
retaining zero-valued data. Source hashes, tool versions, commands, inputs,
register results, observed register changes, and artifact locations accompany
each case. Observed changes are evidence for that input, not a proof of which
registers a helper preserves. Keep the artifacts with the audit's testing log.

## Extend the fixtures

Use `assemble()` from `tests/runtime_harness.py` to assemble source snapshots
and obtain symbols, section sizes, and an initialized emulator. Pass an
`artifacts` directory to retain inputs and outputs; otherwise they are
temporary. The harness accepts ROM0 and WRAM0 sections in bank zero. Keep lookup
data in a separate section so instruction and table sizes can be measured
independently.

Set emulator registers and memory, then call `reset_metrics()` immediately
before execution. It resets cycles, peak stack growth, and written-address
tracking. Use `write8()` or `write16()` for tracked writes; direct memory setup
does not contribute to write tracking. Stack growth uses signed 16-bit SP deltas
and assumes each movement and total frame depth stay below 32 KiB.

The emulator models an instruction subset against flat memory. Unsupported
instructions fail explicitly. Add independent result, flag, memory, and timing
tests when extending the subset. Peripheral timing, interrupts, bank switching,
and bus-cycle effects are outside this helper's coverage. Instruction costs
follow the [RGBDS CPU reference](https://rgbds.gbdev.io/docs/master/gbz80.7).
